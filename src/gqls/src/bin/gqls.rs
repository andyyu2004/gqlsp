use anyhow::Result;
use gqls::Gqls;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let mut builder = tracing_subscriber::fmt().with_writer(std::io::stderr);
    if atty::isnt(atty::Stream::Stderr) {
        builder = builder.with_ansi(false);
    }
    builder.init();
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(Gqls::new);
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
