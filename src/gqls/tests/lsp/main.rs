use anyhow::Result;
use gqls::Gqls;
use lsp_types::notification::Notification as _;
use lsp_types::request::Request as _;
use lsp_types::{lsp_request, TextDocumentContentChangeEvent, VersionedTextDocumentIdentifier};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
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
        let response = $service
            .call(build_request!($request, $params))
            .await?
            .unwrap();
        response.json::<<lsp_request!($request) as lsp_types::request::Request>::Result>()?
    }};
}

macro_rules! notify {
    ($service:ident: $notification:tt, $params:expr) => {{
        let response = $service
            .call(build_notification!($notification, $params))
            .await?;
        assert!(response.is_none());
    }};
}

fn verify<T: Serialize + DeserializeOwned>(value: Value) -> Value {
    serde_json::to_value(serde_json::from_value::<T>(value).unwrap()).unwrap()
}

macro_rules! build_notification {
    ($request:tt, $params:expr) => {{
        type Notification = lsp_types::lsp_notification!($request);
        Request::build(<Notification>::METHOD)
            .params(verify::<
                <Notification as lsp_types::notification::Notification>::Params,
            >(serde_json::to_value($params).unwrap()))
            .finish()
    }};
}

macro_rules! build_request {
    ($request:tt, $params:expr) => {{
        type Req = lsp_types::lsp_request!($request);
        Request::build(<Req>::METHOD)
            .id(next_id())
            .params(verify::<<Req as lsp_types::request::Request>::Params>(
                serde_json::to_value($params).unwrap(),
            ))
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
        assert!(
            path.exists(),
            "fixture `{}` does not exist (path `{}`)",
            $name,
            path.display()
        );
        path
    }};
}

macro_rules! fixture {
    ($name:literal) => {{
        url::Url::from_file_path(fixture_path!($name)).unwrap()
    }};
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
            "name": "simple",
        }]
    }));
    assert_eq!(response.capabilities, gqls::capabilities());
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_document_change() -> Result<()> {
    make_service!(service);
    request!(service: "initialize", json!({
        "capabilities": {},
        "workspaceFolders": [{
            "uri": fixture!("empty"),
            "name": "empty",
        }]
    }));
    let params = lsp_types::DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: fixture!("empty").join("empty.graphql").unwrap(),
            version: next_id() as i32,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Default::default(),
            range_length: None,
            text: "test".to_owned(),
        }],
    };
    notify!(service: "textDocument/didChange", params);
    Ok(())
}
