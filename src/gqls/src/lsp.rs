use crate::config::{Config, DEFAULT_PROJECT};
use crate::convert::{self, PathExt};
use crate::{tokens, Convert, UrlExt};
use anyhow::Result;
use core::panic::{AssertUnwindSafe, UnwindSafe};
use gqls_ide::{
    Change, ChangeKind, Changeset, ChangesetSummary, FileId, Ide, Patch, Vfs, VfsProxy
};
use once_cell::sync::OnceCell;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tower_lsp::lsp_types::notification::PublishDiagnostics;
use tower_lsp::lsp_types::*;
use tower_lsp::{jsonrpc, Client, ClientSocket, LanguageServer, LspService};

pub struct Gqls {
    // FIXME not really a good thing to do as it's not really unwind safe
    client: AssertUnwindSafe<Client>,
    ide: AssertUnwindSafe<Mutex<Ide>>,
    workspace_folders: OnceCell<Vec<WorkspaceFolder>>,
}

impl Gqls {
    pub fn new(client: Client) -> Self {
        Self {
            client: AssertUnwindSafe(client),
            workspace_folders: Default::default(),
            // FIXME implements default in 1.62
            ide: AssertUnwindSafe(Default::default()),
        }
    }

    pub fn service() -> (LspService<Gqls>, ClientSocket) {
        LspService::build(Self::new).custom_method("gqls/syntaxTree", Gqls::syntax_tree).finish()
    }
}

pub fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Options(TextDocumentSyncOptions {
            open_close: Some(true),
            change: Some(TextDocumentSyncKind::INCREMENTAL),
            will_save: None,
            will_save_wait_until: None,
            save: Some(TextDocumentSyncSaveOptions::SaveOptions(SaveOptions {
                include_text: Some(true),
            })),
        })),
        definition_provider: Some(OneOf::Left(true)),
        references_provider: Some(OneOf::Left(true)),
        document_symbol_provider: Some(OneOf::Left(true)),
        workspace_symbol_provider: Some(OneOf::Left(true)),
        type_definition_provider: Some(TypeDefinitionProviderCapability::Simple(true)),
        implementation_provider: Some(ImplementationProviderCapability::Simple(true)),
        semantic_tokens_provider: Some(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: SemanticTokensLegend {
                    token_types: tokens::TYPES.to_vec(),
                    token_modifiers: tokens::MODIFIERS.to_vec(),
                },
                range: None,
                full: Some(SemanticTokensFullOptions::Bool(true)),
            }
            .into(),
        ),
        workspace: Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                did_create: None,
                will_create: None,
                did_rename: None,
                will_rename: None,
                did_delete: None,
                will_delete: Some(FileOperationRegistrationOptions {
                    filters: vec![FileOperationFilter {
                        scheme: Some(String::from("file")),
                        pattern: FileOperationPattern {
                            glob: String::from("**/*.graphql"),
                            matches: Some(FileOperationPatternKind::File),
                            options: None,
                        },
                    }],
                }),
            }),
        }),
        // hover_provider: Some(HoverProviderCapability::Simple(true)),
        completion_provider: Some(CompletionOptions {
            resolve_provider: Some(false),
            trigger_characters: Some(["@", ":", "|"].map(ToString::to_string).to_vec()),
            ..Default::default()
        }),
        rename_provider: Some(OneOf::Right(RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: Default::default(),
        })),
        ..Default::default()
    }
}

pub trait VfsExt {
    fn path(&self, url: &Url) -> jsonrpc::Result<FileId>;
}

impl VfsExt for Vfs {
    fn path(&self, url: &Url) -> jsonrpc::Result<FileId> {
        self.get(url.to_path()?)
            .ok_or_else(|| jsonrpc::Error::invalid_params(format!("unknown file: `{url}`")))
    }
}

impl VfsExt for VfsProxy {
    fn path(&self, url: &Url) -> jsonrpc::Result<FileId> {
        self.get(url.to_path()?)
            .ok_or_else(|| jsonrpc::Error::invalid_params(format!("unknown file: `{url}`")))
    }
}

impl Gqls {
    fn reinit(&self) -> jsonrpc::Result<ChangesetSummary> {
        self.init(self.workspace_folders.get().expect("called reinit before init").clone())
    }

    fn init(&self, workspaces: Vec<WorkspaceFolder>) -> jsonrpc::Result<ChangesetSummary> {
        let projects = discover_projects(workspaces).map_err(|err| {
            tracing::error!(%err);
            jsonrpc::Error::internal_error()
        })?;

        tracing::info!("discovered graphql projects");

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

        Ok(ide.apply(changeset))
    }

    // dirty hack to retry a request if it fails by reinitializing
    // this is to work around deletions/renames/creations for now
    // also, this crate shouldn't have a direct dependency on salsa
    fn with_ide<R: UnwindSafe>(
        &self,
        mut f: impl FnMut(&mut Ide) -> jsonrpc::Result<R> + UnwindSafe,
    ) -> jsonrpc::Result<R> {
        let start = std::time::Instant::now();
        // FIXME hacking unwind safety
        // HACK when just any panics or errors, reinit and try once more
        let r = match std::panic::catch_unwind(AssertUnwindSafe(|| f(&mut self.ide.lock()))) {
            Ok(Ok(res)) => Ok(res),
            _ => {
                tracing::warn!("retrying failed request");
                self.reinit()?;
                f(&mut self.ide.lock())
            }
        };
        tracing::info!("took {}ms", start.elapsed().as_millis());
        r
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    #[tracing::instrument(skip_all)]
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        tracing::info!("gqls initializing");

        // TODO should probably check client capabilities, but going to assume they have everything we need for now

        let workspaces = params.workspace_folders.unwrap_or_default();
        self.workspace_folders.set(workspaces.clone()).expect("initialize called twice");
        let summary = self.init(workspaces)?;
        self.send_diagnostics(summary).await;

        Ok(InitializeResult {
            capabilities: capabilities(),
            server_info: Some(ServerInfo { name: "gqls".to_owned(), version: None }),
        })
    }

    #[tracing::instrument(skip_all)]
    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("gqls initialized");
    }

    #[tracing::instrument(skip_all)]
    async fn shutdown(&self) -> jsonrpc::Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        // maybe we can make sure text is not desynced by doing a hard reset with Change::Set using `params.text`
        if let Ok(summary) = self.with_ide(|ide| {
            let uri = params.text_document.uri.to_path()?;
            let changeset =
                Changeset::single(Change::set(ide.intern_path(uri), params.text.clone().unwrap()));
            Ok(ide.apply(changeset))
        }) {
            self.send_diagnostics(summary).await;
        }
    }

    #[tracing::instrument(skip_all)]
    async fn did_change_workspace_folders(&self, _params: DidChangeWorkspaceFoldersParams) {
        todo!("did_change_workspace_folders")
    }

    #[tracing::instrument(skip_all)]
    async fn symbol(
        &self,
        params: WorkspaceSymbolParams,
    ) -> jsonrpc::Result<Option<Vec<SymbolInformation>>> {
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let symbols = snapshot.workspace_symbols(&params.query);
            Ok(Some(symbols.convert()))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn will_create_files(
        &self,
        _params: CreateFilesParams,
    ) -> jsonrpc::Result<Option<WorkspaceEdit>> {
        Ok(None)
    }

    #[tracing::instrument(skip_all)]
    async fn will_rename_files(
        &self,
        _params: RenameFilesParams,
    ) -> jsonrpc::Result<Option<WorkspaceEdit>> {
        let _ = self.reinit();
        Ok(None)
    }

    #[tracing::instrument(skip_all)]
    async fn will_delete_files(
        &self,
        params: DeleteFilesParams,
    ) -> jsonrpc::Result<Option<WorkspaceEdit>> {
        let summary = self.with_ide(|ide| {
            let mut changeset = Changeset::default();
            for file in &params.files {
                let uri = Url::parse(&file.uri).expect("invalid uri");
                let change = Change::set(ide.intern_path(uri.to_path()?), String::new());
                changeset = changeset.with_change(change);
            }
            Ok(ide.apply(changeset))
        })?;
        self.send_diagnostics(summary).await;
        Ok(None)
    }

    #[tracing::instrument(skip_all)]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let summary = self.with_ide(|ide| {
            let file = ide.intern_path(params.text_document.uri.to_path()?);
            Ok(ide.apply(Change::set(file, params.text_document.text.clone())))
        });
        match summary {
            Ok(summary) => self.send_diagnostics(summary).await,
            Err(err) => tracing::error!(%err),
        }
    }

    #[tracing::instrument(skip_all)]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Err(err) = self.handle_did_change(params).await {
            tracing::error!(%err);
        }
    }

    #[tracing::instrument(skip_all)]
    async fn completion(
        &self,
        params: CompletionParams,
    ) -> jsonrpc::Result<Option<CompletionResponse>> {
        let position = params.text_document_position;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let completions = snapshot.completions(position.convert()?);
            Ok(Some(CompletionResponse::Array(completions.convert())))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let position = params.text_document_position_params;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let locations = snapshot.goto_definition(position.convert()?);
            Ok(convert::locations_to_goto_definition_response(&locations))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn goto_type_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let position = params.text_document_position_params;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let locations = snapshot.goto_type_definition(position.convert()?);
            Ok(convert::locations_to_goto_definition_response(&locations))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn goto_implementation(
        &self,
        params: GotoDefinitionParams,
    ) -> jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let position = params.text_document_position_params;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let locations = snapshot.goto_implementation(position.convert()?);
            Ok(convert::locations_to_goto_definition_response(&locations))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn references(&self, params: ReferenceParams) -> jsonrpc::Result<Option<Vec<Location>>> {
        let position = params.text_document_position;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let locations = snapshot.find_references(position.convert()?);
            match &locations[..] {
                [] => Ok(None),
                locations => Ok(Some(locations.convert())),
            }
        })
    }

    #[tracing::instrument(skip_all)]
    async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> jsonrpc::Result<Option<DocumentSymbolResponse>> {
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let path = ide.vfs().path(&params.text_document.uri)?;
            let symbols = snapshot.document_symbols(path);
            Ok(Some(DocumentSymbolResponse::Nested(symbols.convert())))
        })
    }

    #[tracing::instrument(skip_all)]
    async fn prepare_rename(
        &self,
        params: TextDocumentPositionParams,
    ) -> jsonrpc::Result<Option<PrepareRenameResponse>> {
        self.with_ide(|ide| match ide.snapshot().prepare_rename(params.convert()?) {
            Ok(range) => Ok(Some(PrepareRenameResponse::Range(range.convert()))),
            Err(_) => Ok(None),
        })
    }

    #[tracing::instrument(skip_all)]
    async fn rename(&self, params: RenameParams) -> jsonrpc::Result<Option<WorkspaceEdit>> {
        let position = params.text_document_position;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            match snapshot.rename(position.convert()?, &params.new_name) {
                Ok(edits) => Ok(Some(WorkspaceEdit {
                    document_changes: Some(DocumentChanges::Edits(edits.convert())),
                    ..Default::default()
                })),
                Err(err) => Err(err.convert()),
            }
        })
    }

    #[tracing::instrument(skip_all)]
    async fn semantic_tokens_full(
        &self,
        params: SemanticTokensParams,
    ) -> jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        self.with_ide(|ide| {
            let snapshot = ide.snapshot();
            let path = ide.vfs().path(&uri)?;
            Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
                data: tokens::convert(&snapshot.semantic_tokens(path)),
                result_id: None,
            })))
        })
    }
}

#[derive(Debug, Eq, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SyntaxTreeParams {
    pub text_document: VersionedTextDocumentIdentifier,
}

impl Gqls {
    async fn syntax_tree(&self, params: SyntaxTreeParams) -> jsonrpc::Result<String> {
        self.with_ide(|ide| {
            let path = ide.vfs().path(&params.text_document.uri)?;
            let snapshot = ide.snapshot();
            Ok(snapshot.syntax_tree(path))
        })
    }

    async fn handle_did_change(&self, params: DidChangeTextDocumentParams) -> Result<()> {
        let path = params.text_document.uri.to_path()?;
        tracing::info!("path: {path:?}");
        let summary = self.with_ide(|ide| {
            let path = ide.intern_path(path.clone());
            let changes = params
                .content_changes
                .iter()
                .map(|change| match change.range {
                    Some(range) => ChangeKind::Patch(Patch {
                        range: range.convert(),
                        with: change.text.clone(),
                    }),
                    None => ChangeKind::Set(change.text.clone()),
                })
                .map(|kind| Change::new(path, kind))
                .collect();
            let changeset = Changeset::new(changes);
            Ok(ide.apply(changeset))
        })?;
        self.send_diagnostics(summary).await;
        Ok(())
    }

    #[tracing::instrument(skip_all)]
    async fn send_diagnostics(&self, summary: ChangesetSummary) {
        tracing::info!("emitting diagnostics");
        for (path, diagnostics) in summary.diagnostics {
            let diagnostics = diagnostics.iter().map(Convert::convert).collect::<Vec<_>>();
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

#[tracing::instrument(skip_all)]
fn discover_projects(
    workspaces: impl IntoIterator<Item = WorkspaceFolder>,
) -> anyhow::Result<HashMap<String, Vec<(PathBuf, String)>>> {
    let mut projects = HashMap::default();
    for workspace in workspaces {
        let path = workspace.uri.to_path()?;
        let config = read_config(&path)?;
        // FIXME drop this dependency
        for entry in ignore::Walk::new(&path) {
            let entry = entry?;
            if !entry.file_type().expect("should exist as not stdin").is_file() {
                continue;
            }

            let file_projects = match &config {
                Some(config) => config.project_matches(entry.path().strip_prefix(&path).unwrap()),
                // If configuration file is found, then all `*.graphql` files are assigned to the default project
                None => (entry.path().extension() == Some("graphql".as_ref()))
                    .then_some(DEFAULT_PROJECT)
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
