use anyhow::Result;
use gqls::Gqls;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() -> Result<()> {
    let builder = tracing_subscriber::fmt().with_writer(std::io::stderr);
    if atty::is(atty::Stream::Stderr) {
        builder.init();
    } else {
        builder.json().init();
    }
    tracing::error!("hello");
    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = LspService::new(Gqls::new);
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
