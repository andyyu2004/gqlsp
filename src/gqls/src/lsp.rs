use anyhow::{Result};
use gqls_ide::{ChangeKind, Ide, Patch, Point, Range};
use lsp_types::*;
use parking_lot::Mutex;
use std::path::PathBuf;
use tower_lsp::jsonrpc;
use tower_lsp::{Client, LanguageServer};

pub struct Gqls {
    client: Client,
    ide: Mutex<Ide>,
}

impl Gqls {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            ide: Default::default(),
        }
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
        for (path, content) in paths {
            ide.change(path, ChangeKind::Set(content));
        }

        Ok(InitializeResult {
            capabilities: capabilities(),
            server_info: Some(ServerInfo {
                name: "gqls".to_owned(),
                version: None,
            }),
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
        fn convert_pos(pos: Position) -> Point {
            Point::new(pos.line as usize, pos.character as usize)
        }

        fn convert_range(range: lsp_types::Range) -> Range {
            Range {
                start: convert_pos(range.start),
                end: convert_pos(range.end),
            }
        }

        let mut ide = self.ide.lock();
        for change in params.content_changes {
            let change_kind = match change.range {
                Some(range) => ChangeKind::Patch(Patch {
                    range: convert_range(range),
                    with: change.text,
                }),
                None => ChangeKind::Set(change.text),
            };
            let path = params.text_document.uri.to_path()?;
            ide.change(path, change_kind);
        }
        Ok(())
    }
}
