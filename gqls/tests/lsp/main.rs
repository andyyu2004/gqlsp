use std::sync::atomic::AtomicI64;

use anyhow::Result;
use gqls::Gqls;
use lsp_types::request::Request as _;
use lsp_types::{lsp_request, InitializeResult};
use serde::de::DeserializeOwned;
use serde_json::json;
use tower::{Service, ServiceExt};
use tower_lsp::jsonrpc::{Request, Response};
use tower_lsp::LspService;

macro_rules! make_service {
    ($ident:ident) => {
        let (mut service, _) = LspService::new(Gqls::new);
        let $ident = service.ready().await.unwrap();
    };
}

fn next_id() -> i64 {
    static NEXT: AtomicI64 = AtomicI64::new(0);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

macro_rules! request {
    ($request:tt, $params:expr) => {{ Request::build(<lsp_request!($request)>::METHOD).id(next_id()).params($params).finish() }};
}

trait ResponseExt {
    fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error>;
}

impl ResponseExt for Response {
    fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(self.result().unwrap().clone())
    }
}

#[tokio::test]
async fn test_lsp_init() -> Result<()> {
    make_service!(service);
    let response =
        service.call(request!("initialize", json!({ "capabilities": {} }))).await?.unwrap();
    let initialize_result = response.json::<InitializeResult>()?;
    assert_eq!(initialize_result.capabilities, gqls::capabilities());
    Ok(())
}
