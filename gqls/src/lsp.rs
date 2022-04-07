use lsp_types::*;
use tower_lsp::jsonrpc::Result;
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

#[tower_lsp::async_trait]
impl LanguageServer for Gqls {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: capabilities(),
            server_info: Some(ServerInfo { name: "gqls".to_owned(), version: None }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client.log_message(MessageType::INFO, "server initialized!").await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
    }
}
