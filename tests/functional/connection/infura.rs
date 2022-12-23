use hyper::{http, Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;
use test_context::test_context;

use crate::{
    context::ServerContext, functional::connection::INFURA_CHAIN_DECOMISSIONED_ERROR_CODE,
    utils::send_jsonrpc_request, JSONRPC_VERSION,
};

#[test_context(ServerContext)]
#[tokio::test]
async fn health_check(ctx: &mut ServerContext) {
    let addr = format!("{}/health", ctx.server.public_addr);

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(addr)
        .body(Body::default())
        .unwrap();

    let response = client.request(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK)
}

#[test_context(ServerContext)]
#[tokio::test]
async fn metrics_check(ctx: &mut ServerContext) {
    let addr = format!("{}/metrics", ctx.server.public_addr);

    let client = Client::builder().build::<_, hyper::Body>(HttpsConnector::new());

    let request = Request::builder()
        .method(Method::GET)
        .uri(addr)
        .body(Body::default())
        .unwrap();

    let response = client.request(request).await.unwrap();

    assert_eq!(response.status(), http::StatusCode::OK)
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_1_mainnet_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:1", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x1")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_3_ropsten_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:3", request).await;

    // Verify that HTTP communication returns error
    assert_eq!(status, StatusCode::GONE);

    // Verify the error code is for
    // "Network decommissioned, please use Goerli or Sepolia instead"
    assert_eq!(
        rpc_response.error.unwrap().code,
        INFURA_CHAIN_DECOMISSIONED_ERROR_CODE
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_42_kovan_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:42", request).await;

    // Verify that HTTP communication returns error
    assert_eq!(status, StatusCode::GONE);

    // Verify the error code is for
    // "Network decommissioned, please use Goerli or Sepolia instead"
    assert_eq!(
        rpc_response.error.unwrap().code,
        INFURA_CHAIN_DECOMISSIONED_ERROR_CODE
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_4_rinkeby_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:4", request).await;

    // Verify that HTTP communication returns error
    assert_eq!(status, StatusCode::GONE);

    // Verify the error code is for
    // "Network decommissioned, please use Goerli or Sepolia instead"
    assert_eq!(
        rpc_response.error.unwrap().code,
        INFURA_CHAIN_DECOMISSIONED_ERROR_CODE
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_5_goerli_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:5", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x5")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_137_polygon_mainnet_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:137", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x89")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_80001_polygon_mumbai_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:80001", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x13881")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_10_optimism_mainnet_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:10", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0xa")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_69_optimism_kovan_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:69", request).await;

    // Verify that HTTP communication returns error
    assert_eq!(status, StatusCode::GONE);

    // Verify the error code is for
    // "message":"Network decommissioned, please use Goerli",
    assert_eq!(
        rpc_response.error.unwrap().code,
        INFURA_CHAIN_DECOMISSIONED_ERROR_CODE
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_420_optimism_goerli_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:420", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x1A4")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_42161_arbitrum_mainnet_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:42161", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0xa4b1")
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_421611_arbitrum_rinkeby_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:421611", request).await;

    // Verify that HTTP communication returns error
    assert_eq!(status, StatusCode::GONE);

    // Verify the error code is for
    // "message":"Network decommissioned, please use Goerli",
    assert_eq!(
        rpc_response.error.unwrap().code,
        INFURA_CHAIN_DECOMISSIONED_ERROR_CODE
    );
}

#[test_context(ServerContext)]
#[tokio::test]
async fn eip155_421613_arbitrum_goerli_infura(ctx: &mut ServerContext) {
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

    let (status, rpc_response) = send_jsonrpc_request(client, addr, "eip155:421613", request).await;

    // Verify that HTTP communication was successful
    assert_eq!(status, StatusCode::OK);

    // Verify there was no error in rpc
    assert!(rpc_response.error.is_none());

    // Check chainId
    assert_eq!(rpc_response.result::<String>().unwrap(), "0x66eed")
}
