use anyhow::Result;
use futures::StreamExt;
use gqls::{Convert, Gqls};
use gqls_ide::DiagnosticKind;
use lsp_types::notification::Notification as _;
use lsp_types::request::Request as _;
use lsp_types::*;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::atomic::AtomicI64;
use tower::{Service, ServiceExt};
use tower_lsp::jsonrpc::{Request, Response};
use tower_lsp::LspService;

macro_rules! make_service {
    () => {{
        let (service, socket) = LspService::new(Gqls::new);
        (Box::leak(Box::new(service)).ready().await.unwrap(), socket)
    }};
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

macro_rules! notify {
    ($service:ident: $notification:tt, $params:expr) => {{
        let response = $service.call(build_notification!($notification, $params)).await?;
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
            .params(verify::<<Notification as lsp_types::notification::Notification>::Params>(
                serde_json::to_value($params).unwrap(),
            ))
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
    ($name:literal) => {{ lsp_types::Url::from_file_path(fixture_path!($name)).unwrap() }};
}

macro_rules! url {
    ($name:literal.$file:literal) => {{ fixture!($name).join($file).unwrap() }};
}

macro_rules! workspaces {
    ($workspace:literal) => {
        json!([{
            "uri": fixture!($workspace),
            "name": $workspace,
        }])
    };
}

macro_rules! request_init {
    ($service:ident: $workspace:literal) => {{
        request!($service: "initialize", json!({
            "capabilities": {},
            "workspaceFolders": workspaces!($workspace),
        }))
    }}
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

#[test]
fn test_fixture_path_macro() {
    assert!(fixture_path!("simple").ends_with("tests/lsp/fixtures/simple"));
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_init() -> Result<()> {
    let (service, _) = make_service!();
    let response = request!(service: "initialize", json!({ "capabilities": {} }));
    assert_eq!(response.capabilities, gqls::capabilities());
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_init_with_graphql_files() -> Result<()> {
    let (service, _) = make_service!();
    let response = request_init!(service: "simple");
    assert_eq!(response.capabilities, gqls::capabilities());
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_document_change() -> Result<()> {
    let (service, _socket) = make_service!();
    request_init!(service: "empty");
    let params = lsp_types::DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: url!("empty"."empty.graphql"),
            version: next_id() as i32,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Default::default(),
            range_length: None,
            text: "{}".to_owned(),
        }],
    };
    notify!(service: "textDocument/didChange", params);
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_diagnostics() -> Result<()> {
    let (service, mut socket) = make_service!();
    request_init!(service: "empty");
    let uri = url!("empty"."empty.graphql");
    let params = lsp_types::DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: uri.clone(),
            version: next_id() as i32,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: Default::default(),
            range_length: None,
            text: "bad gql".to_owned(),
        }],
    };

    notify!(service: "textDocument/didChange", params);
    let req = socket.next().await.unwrap();

    assert_eq!(
        req,
        build_notification!(
            "textDocument/publishDiagnostics",
            json!({
                "uri": uri,
                "diagnostics": [
                    gqls_ide::Diagnostic { range: gqls_ide::range!(0:0..0:7), kind: DiagnosticKind::Syntax }.convert()
                ]
            })
        )
    );
    Ok(())
}

#[tokio::test]
#[tracing_test::traced_test]
async fn test_lsp_configuration() -> Result<()> {
    let (service, _) = make_service!();
    request_init!(service: "config");
    let _foo = url!("config"."foo.graphql");

    Ok(())
}
