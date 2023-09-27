use anyhow::Result;
use gqls::Gqls;
use tower_lsp::Server;
use tracing::metadata::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    Targets::new();

    let mut layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_span_events(FmtSpan::FULL);
    if atty::isnt(atty::Stream::Stderr) {
        layer = layer.with_ansi(false);
    }
    let filtered_layer = layer.with_filter(
        EnvFilter::builder()
            .with_env_var("GQLS_LOG")
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy(),
    );

    let targets =
        Targets::new().with_target("salsa", LevelFilter::WARN).with_default(LevelFilter::TRACE);
    tracing_subscriber::registry().with(filtered_layer).with(targets).init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    let (service, socket) = Gqls::service();
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
