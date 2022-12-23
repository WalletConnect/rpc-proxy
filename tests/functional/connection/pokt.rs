use hyper::{Client, StatusCode};
use hyper_tls::HttpsConnector;
use test_context::test_context;

use crate::{context::ServerContext, utils::send_jsonrpc_request, JSONRPC_VERSION};

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_43114_avax(ctx: &mut ServerContext) {
    let addr = format!(
        "{}/v1?projectId={}&chainId=",
        ctx.server.public_addr, ctx.server.project_id
    );

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let request = jsonrpc::Request {
        method: "eth_chainId",
        params: &[],
        id: serde_json::Value::Number(1.into()),
        jsonrpc: JSONRPC_VERSION,
    };

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:43114", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0xa86a")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_100_gnosis(ctx: &mut ServerContext) {
    let addr = format!(
        "{}/v1?projectId={}&chainId=",
        ctx.server.public_addr, ctx.server.project_id
    );

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let request = jsonrpc::Request {
        method: "eth_chainId",
        params: &[],
        id: serde_json::Value::Number(1.into()),
        jsonrpc: JSONRPC_VERSION,
    };

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:100", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x64")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn solana_mainnet(ctx: &mut ServerContext) {
    let addr = format!(
        "{}/v1?projectId={}&chainId=",
        ctx.server.public_addr, ctx.server.project_id
    );

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());
    let request = jsonrpc::Request {
        method: "getHealth",
        params: &[],
        id: serde_json::Value::Number(1.into()),
        jsonrpc: JSONRPC_VERSION,
    };

    let (status, rpc_response) = send_jsonrpc_request(
        client,
        addr,
        "solana:4sgjmw1sunhzsxgspuhpqldx6wiyjntz",
        request,
    )
    .await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "ok")
}
