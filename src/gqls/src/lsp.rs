use crate::config::{Config, DEFAULT_PROJECT};
use crate::convert::PathExt;
use crate::{Convert, UrlExt};
use anyhow::Result;
use gqls_ide::{Change, ChangeSummary, Ide, Patch, VfsPath};
use lsp_types::notification::PublishDiagnostics;
use lsp_types::*;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tower_lsp::{jsonrpc, Client, ClientSocket, LanguageServer, LspService};

pub struct Gqls {
    client: Client,
    ide: Mutex<Ide>,
}

impl Gqls {
    pub fn new(client: Client) -> Self {
        Self { client, ide: Default::default() }
    }

    pub fn service() -> (LspService<Gqls>, ClientSocket) {
        LspService::build(Self::new).custom_method("gqls/syntaxTree", Gqls::syntax_tree).finish()
    }
}

pub fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        definition_provider: Some(OneOf::Left(true)),
        // hover_provider: Some(HoverProviderCapability::Simple(true)),
        // completion_provider: Some(CompletionOptions::default()),
        ..Default::default()
    }
}

pub trait IdeExt {
    fn path(&self, url: &Url) -> jsonrpc::Result<VfsPath>;
}

impl IdeExt for Ide {
    fn path(&self, url: &Url) -> jsonrpc::Result<VfsPath> {
        self.vfs()
            .get(url.to_path()?)
            .ok_or_else(|| jsonrpc::Error::invalid_params(format!("unknown file: `{url}`")))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        // TODO should probably check client capabilities, but going to assume they have everything we need for now

        fn read_config(path: &Path) -> anyhow::Result<Option<Config>> {
            assert!(path.is_dir());
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if entry.file_type()?.is_file()
                    && path.file_name() == Some(".graphqlrc".as_ref())
                    && (path.extension() == Some("yaml".as_ref())
                        || path.extension() == Some("toml".as_ref()))
                {
                    return Ok(Some(Config::read(&path)?));
                }
            }
            Ok(None)
        }

        fn find_graphql_files(
            workspaces: impl IntoIterator<Item = WorkspaceFolder>,
        ) -> anyhow::Result<HashMap<String, HashSet<PathBuf>>, Vec<(PathBuf, String)>> {
            let mut paths = vec![];
            for workspace in workspaces {
                let path = workspace.uri.to_path()?;
                let config = read_config(&path)?;
                for entry in walkdir::WalkDir::new(&path)
                    .into_iter()
                    .filter_entry(|entry| entry.file_type().is_file())
                {
                    let entry = entry?;
                    let projects = match &config {
                        Some(config) => config.project_matches(entry.path()),
                        // If configuration file is found, then all `*.graphql` files are assigned to the default project
                        None => (entry.path().extension() == Some("graphql".as_ref()))
                            .then(|| DEFAULT_PROJECT)
                            .into_iter()
                            .collect(),
                    };
                    let path = entry.path().to_path_buf();
                    let content = std::fs::read_to_string(&path)?;
                    paths.push((path, content));
                }
            }
            Ok(paths)
        }

        let paths =
            find_graphql_files(params.workspace_folders.into_iter().flatten()).map_err(|err| {
                tracing::error!(%err);
                jsonrpc::Error::internal_error()
            })?;

        let mut ide = self.ide.lock();
        let changesets = paths
            .into_iter()
            .map(|(path, content)| ide.make_changeset(&path, vec![Change::Set(content)]))
            .collect::<Vec<_>>();
        for summary in ide.apply_changesets(changesets) {
            self.diagnostics(summary).await;
        }

        Ok(InitializeResult {
            capabilities: capabilities(),
            server_info: Some(ServerInfo { name: "gqls".to_owned(), version: None }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("gqls initialized");
    }

    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Err(err) = self.handle_did_change(params).await {
            tracing::error!(%err);
        }
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let ide = self.ide.lock();
        let position = params.text_document_position_params;
        let path = ide.path(&position.text_document.uri)?;
        let analysis = ide.analysis();
        let locations = analysis.goto_definition(path, position.position.convert());
        match &locations[..] {
            [] => Ok(None),
            [location] => Ok(Some(GotoDefinitionResponse::Scalar(location.convert()))),
            locations => Ok(Some(GotoDefinitionResponse::Array(locations.convert()))),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyntaxTreeParams {
    pub text_document: VersionedTextDocumentIdentifier,
}

impl Gqls {
    async fn syntax_tree(&self, params: SyntaxTreeParams) -> jsonrpc::Result<String> {
        let ide = self.ide.lock();
        let path = ide.path(&params.text_document.uri)?;
        let analysis = ide.analysis();
        Ok(analysis.syntax_tree(path))
    }

    async fn handle_did_change(&self, params: DidChangeTextDocumentParams) -> Result<()> {
        let path = params.text_document.uri.to_path().unwrap();
        let changes = params
            .content_changes
            .into_iter()
            .map(|change| match change.range {
                Some(range) => Change::Patch(Patch { range: range.convert(), with: change.text }),
                None => Change::Set(change.text),
            })
            .collect();
        let summary = self.ide.lock().changeset(&path, changes);
        self.diagnostics(summary).await;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn diagnostics(&self, summary: ChangeSummary) {
        tracing::info!("emitting diagnostics");
        let diagnostics = summary.diagnostics.iter().map(Convert::convert).collect::<Vec<_>>();
        self.client
            .send_notification::<PublishDiagnostics>(PublishDiagnosticsParams {
                uri: summary.path.to_url(),
                diagnostics,
                version: None,
            })
            .await;
    }
}
