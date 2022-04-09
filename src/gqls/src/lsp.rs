use anyhow::Result;
use gqls_ide::{Change, ChangeSummary, Ide, Patch, Point, Range};
use lsp_types::*;
use parking_lot::Mutex;
use std::path::{Path, PathBuf};
use tower_lsp::{jsonrpc, Client, LanguageServer};

pub struct Gqls {
    client: Client,
    ide: Mutex<Ide>,
}

impl Gqls {
    pub fn new(client: Client) -> Self {
        Self { client, ide: Default::default() }
    }
}

pub fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::INCREMENTAL,
        )),
        // hover_provider: Some(HoverProviderCapability::Simple(true)),
        // completion_provider: Some(CompletionOptions::default()),
        ..Default::default()
    }
}

trait PathExt {
    fn to_url(&self) -> Url;
}

impl PathExt for Path {
    fn to_url(&self) -> Url {
        Url::from_file_path(self).unwrap()
    }
}

trait LspRangeExt {
    fn convert(self) -> Range;
}

impl LspRangeExt for lsp_types::Range {
    fn convert(self) -> Range {
        fn convert_pos(pos: Position) -> Point {
            Point::new(pos.line as usize, pos.character as usize)
        }

        Range { start: convert_pos(self.start), end: convert_pos(self.end) }
    }
}

trait RangeExt {
    fn convert(self) -> lsp_types::Range;
}

impl RangeExt for Range {
    fn convert(self) -> lsp_types::Range {
        fn convert_pos(pos: Point) -> Position {
            Position::new(pos.row as u32, pos.column as u32)
        }

        lsp_types::Range { start: convert_pos(self.start), end: convert_pos(self.end) }
    }
}

trait UrlExt {
    fn to_path(&self) -> jsonrpc::Result<PathBuf>;
}

impl UrlExt for Url {
    fn to_path(&self) -> jsonrpc::Result<PathBuf> {
        if self.scheme() != "file" {
            return Err(jsonrpc::Error::invalid_params(
                "Only file URIs are supported for workspace folders: `{uri}`",
            ));
        }
        self.to_file_path()
            .map_err(|()| jsonrpc::Error::invalid_params(format!("Invalid file path: `{self}`")))
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    #[tracing::instrument(skip(self))]
    async fn initialize(&self, params: InitializeParams) -> jsonrpc::Result<InitializeResult> {
        // TODO should probably check client capabilities, but going to assume they have everything we need for now
        fn find_graphql_files(
            folders: impl IntoIterator<Item = WorkspaceFolder>,
        ) -> anyhow::Result<Vec<(PathBuf, String)>> {
            let mut paths = vec![];
            for folder in folders {
                let path = folder.uri.to_path()?;
                for entry in walkdir::WalkDir::new(&path).into_iter() {
                    let entry = entry?;
                    if entry.path().extension() != Some("graphql".as_ref()) {
                        continue;
                    }
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
            self.diagnostics(&summary).await;
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

    #[tracing::instrument(skip(self))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
    }

    #[tracing::instrument(skip(self))]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        if let Err(err) = self.handle_did_change(params).await {
            tracing::error!(%err);
        }
    }
}

impl Gqls {
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
        let mut ide = self.ide.lock();
        let changeset = ide.make_changeset(&path, changes);
        let summary = ide.apply_changeset(changeset);
        self.diagnostics(&summary).await;
        Ok(())
    }

    async fn diagnostics(&self, summary: &ChangeSummary) {
        let mut diagnostics = vec![];
        for &range in &summary.error_ranges {
            diagnostics.push(Diagnostic {
                range: range.convert(),
                message: "SYNTAX_ERROR".to_owned(),
                ..Default::default()
            });
        }

        if diagnostics.is_empty() {
            return;
        }
        let uri = self.ide.lock().vfs().lookup(summary.file).to_url();
        self.client
            .send_notification::<lsp_types::notification::PublishDiagnostics>(
                PublishDiagnosticsParams { uri, diagnostics, version: None },
            )
            .await;
    }
}
