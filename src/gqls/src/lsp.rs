use crate::config::{Config, DEFAULT_PROJECT};
use crate::convert::PathExt;
use crate::{Convert, UrlExt};
use anyhow::Result;
use gqls_ide::{Change, ChangeKind, Changeset, ChangesetSummary, FileId, Ide, Patch};
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
        references_provider: Some(OneOf::Left(true)),
        // hover_provider: Some(HoverProviderCapability::Simple(true)),
        // completion_provider: Some(CompletionOptions::default()),
        ..Default::default()
    }
}

pub trait IdeExt {
    fn path(&self, url: &Url) -> jsonrpc::Result<FileId>;
}

impl IdeExt for Ide {
    fn path(&self, url: &Url) -> jsonrpc::Result<FileId> {
        self.vfs()
            .get(url.to_path()?)
            .ok_or_else(|| jsonrpc::Error::invalid_params(format!("unknown file: `{url}`")))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        // TODO should probably check client capabilities, but going to assume they have everything we need for now
        let projects =
            discover_projects(params.workspace_folders.into_iter().flatten()).map_err(|err| {
                tracing::error!(%err);
                jsonrpc::Error::internal_error()
            })?;

        let mut ide = self.ide.lock();
        let mut changeset = Changeset::default().with_projects(
            projects
                .iter()
                .map(|(k, v)| {
                    (
                        ide.intern_project(k.to_owned()),
                        v.iter().map(|(path, _)| ide.intern_path(path.clone())).collect(),
                    )
                })
                .collect(),
        );

        for (_, files) in projects {
            for (path, content) in files {
                changeset = changeset.with_change(Change::set(ide.intern_path(path), content))
            }
        }

        let summary = ide.apply_changeset(changeset);
        self.diagnostics(summary).await;

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

    async fn references(&self, params: ReferenceParams) -> jsonrpc::Result<Option<Vec<Location>>> {
        let ide = self.ide.lock();
        let position = params.text_document_position;
        let path = ide.path(&position.text_document.uri)?;
        let analysis = ide.analysis();
        let locations = analysis.find_references(path, position.position.convert());
        match &locations[..] {
            [] => Ok(None),
            locations => Ok(Some(locations.convert())),
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
        let path = params.text_document.uri.to_path()?;
        let mut ide = self.ide.lock();
        let path = ide.intern_path(path);
        let changes = params
            .content_changes
            .into_iter()
            .map(|change| match change.range {
                Some(range) =>
                    ChangeKind::Patch(Patch { range: range.convert(), with: change.text }),
                None => ChangeKind::Set(change.text),
            })
            .map(|kind| Change::new(path, kind))
            .collect();
        let changeset = Changeset::new(changes);
        let changeset_summary = ide.apply_changeset(changeset);
        self.diagnostics(changeset_summary).await;
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn diagnostics(&self, summary: ChangesetSummary) {
        tracing::info!("emitting diagnostics");
        for (path, change_summary) in summary {
            let diagnostics =
                change_summary.diagnostics.iter().map(Convert::convert).collect::<Vec<_>>();
            self.client
                .send_notification::<PublishDiagnostics>(PublishDiagnosticsParams {
                    uri: path.to_url(),
                    diagnostics,
                    version: None,
                })
                .await;
        }
    }
}

fn read_config(path: &Path) -> anyhow::Result<Option<Config>> {
    assert!(path.is_dir());
    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_file() && path.file_stem() == Some(".graphqlrc".as_ref()) {
            return Ok(Some(Config::read(&path)?));
        }
    }
    Ok(None)
}

fn discover_projects(
    workspaces: impl IntoIterator<Item = WorkspaceFolder>,
) -> anyhow::Result<HashMap<String, Vec<(PathBuf, String)>>> {
    let mut projects = HashMap::default();
    for workspace in workspaces {
        let path = workspace.uri.to_path()?;
        let config = read_config(&path)?;
        for entry in walkdir::WalkDir::new(&path)
            .into_iter()
            .filter_entry(|entry| !entry.path().ends_with(".git"))
        {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }
            let file_projects = match &config {
                Some(config) => config.project_matches(entry.path().strip_prefix(&path).unwrap()),
                // If configuration file is found, then all `*.graphql` files are assigned to the default project
                None => (entry.path().extension() == Some("graphql".as_ref()))
                    .then(|| DEFAULT_PROJECT)
                    .into_iter()
                    .collect(),
            };

            if file_projects.is_empty() {
                continue;
            }
            let path = entry.path().to_path_buf();
            let content = std::fs::read_to_string(&path).unwrap();
            // FIXME shouldn't have to clone everything
            for file_project in file_projects {
                projects
                    .entry(file_project.to_owned())
                    .or_insert_with(Vec::new)
                    .push((path.clone(), content.clone()));
            }
        }
    }
    Ok(projects)
}

#[cfg(test)]
mod tests;
