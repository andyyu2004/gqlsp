use anyhow::Result;
use gqls::Gqls;
use lsp_types::lsp_request;
use lsp_types::request::Request as _;
use serde::de::DeserializeOwned;
use serde_json::json;
use std::sync::atomic::AtomicI64;
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
    ($service:ident: $request:tt, $params:expr) => {{
        let response = $service.call(build_request!($request, $params)).await?.unwrap();
        response.json::<<lsp_request!($request) as lsp_types::request::Request>::Result>()?
    }};
}

macro_rules! build_request {
    ($request:tt, $params:expr) => {{
        type Req = lsp_request!($request);
        Request::build(<Req>::METHOD)
            .id(next_id())
            .params(
                // Convert to the expected request type (then back again) to check its well formed
                serde_json::to_value(
                    serde_json::from_value::<<Req as lsp_types::request::Request>::Params>($params)
                        .unwrap(),
                )
                .unwrap(),
            )
            .finish()
    }};
}

trait ResponseExt {
    fn json<T: DeserializeOwned>(self) -> Result<T>;
}

impl ResponseExt for Response {
    fn json<T: DeserializeOwned>(self) -> Result<T> {
        match self.into_parts().1 {
            Ok(res) => Ok(serde_json::from_value(res)?),
            Err(err) => return Err(err.into()),
        }
    }
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_init() -> Result<()> {
    make_service!(service);
    let response = request!(service: "initialize", json!({ "capabilities": {} }));
    assert_eq!(response.capabilities, gqls::capabilities());
    Ok(())
}

macro_rules! fixture_path {
    ($name:literal) => {{
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/lsp/fixtures")
            .join($name);
        assert!(path.exists(), "fixture `{}` does not exist (path `{}`)", $name, path.display());
        path
    }};
}

macro_rules! fixture {
    ($name:literal) => {{ url::Url::from_file_path(fixture_path!($name)).unwrap() }};
}

#[test]
fn test_fixture_path_macro() {
    assert!(fixture_path!("simple").ends_with("tests/lsp/fixtures/simple"));
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_init_with_graphql_files() -> Result<()> {
    make_service!(service);
    let response = request!(service: "initialize", json!({
        "capabilities": {},
        "workspaceFolders": [{
            "uri": fixture!("simple"),
            "name": "main.rs",
        }]
    }));
    assert_eq!(response.capabilities, gqls::capabilities());
    Ok(())
}
