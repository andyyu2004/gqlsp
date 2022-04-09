use std::path::PathBuf;

use lsp_types::*;
use tower_lsp::jsonrpc::{self, Result};
use tower_lsp::{Client, LanguageServer};

pub struct Gqls {
    client: Client,
}

impl Gqls {
    pub fn new(client: Client) -> Self {
        Self { client }
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

fn uri_to_path(uri: &Url) -> Result<PathBuf> {
    if uri.scheme() != "file" {
        return Err(jsonrpc::Error::invalid_params(
            "Only file URIs are supported for workspace folders: `{uri}`",
        ));
    }
    uri.to_file_path()
        .map_err(|()| jsonrpc::Error::invalid_params(format!("Invalid file path: `{uri}`")))
}

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    #[tracing::instrument(skip(self))]
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // TODO should probably check client capabilities, but going to assume they have everything we need for now
        fn find_graphql_files(
            folders: impl IntoIterator<Item = WorkspaceFolder>,
        ) -> anyhow::Result<Vec<PathBuf>> {
            let mut paths = vec![];
            for folder in folders {
                let path = uri_to_path(&folder.uri)?;
                for entry in walkdir::WalkDir::new(&path).into_iter() {
                    let entry = entry?;
                    if entry.path().extension() != Some("graphql".as_ref()) {
                        continue;
                    }
                    paths.push(entry.path().to_path_buf());
                }
            }
            Ok(paths)
        }

        let paths = find_graphql_files(params.workspace_folders.into_iter().flatten())
            .map_err(|_| jsonrpc::Error::internal_error())?;
        tracing::error!(?paths);

        Ok(InitializeResult {
            capabilities: capabilities(),
            server_info: Some(ServerInfo { name: "gqls".to_owned(), version: None }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        tracing::info!("gqls initialized");
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    #[tracing::instrument(skip(self))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
    }

    #[tracing::instrument(skip(self))]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        for change in params.content_changes {
            match change.range {
                Some(range) => {
                    tracing::error!(?range);
                }
                None => {}
            }
        }
    }
}
