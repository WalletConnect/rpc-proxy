use super::{ProviderKind, RpcProvider, RpcQueryParams};
use crate::error::{RpcError, RpcResult};
use async_trait::async_trait;
use hyper::{
    body::{self, Bytes},
    client::HttpConnector,
    Body, Client, Response,
};
use hyper_tls::HttpsConnector;
use std::collections::HashMap;

#[derive(Clone)]
pub struct PoktProvider {
    pub client: Client<HttpsConnector<HttpConnector>>,
    pub project_id: String,
    pub supported_chains: HashMap<String, String>,
}

#[async_trait]
impl RpcProvider for PoktProvider {
    async fn proxy(
        &self,
        method: hyper::http::Method,
        _path: warp::path::FullPath,
        query_params: RpcQueryParams,
        _headers: hyper::http::HeaderMap,
        body: hyper::body::Bytes,
    ) -> RpcResult<Response<Body>> {
        let chain = self
            .supported_chains
            .get(&query_params.chain_id.to_lowercase())
            .ok_or(RpcError::ChainNotFound)?;

        let uri = format!(
            "https://{}.gateway.pokt.network/v1/lb/{}",
            chain, self.project_id
        );

        let hyper_request = hyper::http::Request::builder()
            .method(method)
            .uri(uri)
            .header("Content-Type", "application/json")
            .body(hyper::body::Body::from(body))?;

        // TODO: map the response error codes properly
        // e.g. HTTP401 from target should map to HTTP500
        let response = self.client.request(hyper_request).await?;

        let (body_bytes, response) = copy_body_bytes(response).await.unwrap();

        if is_rate_limited(body_bytes) {
            return Err(RpcError::Throttled);
        }

        Ok(response)
    }

    fn supports_caip_chainid(&self, chain_id: &str) -> bool {
        self.supported_chains.contains_key(chain_id)
    }

    fn supported_caip_chainids(&self) -> Vec<String> {
        self.supported_chains.keys().cloned().collect()
    }

    fn provider_kind(&self) -> ProviderKind {
        ProviderKind::Pokt
    }

    fn project_id(&self) -> &str {
        &self.project_id
    }
}

fn is_rate_limited(body_bytes: Bytes) -> bool {
    let jsonrpc_response = serde_json::from_slice::<jsonrpc::Response>(&body_bytes);

    if jsonrpc_response.is_err() {
        return false;
    }

    if let Some(err) = jsonrpc_response.unwrap().error {
        // Code used by Pokt to indicate rate limited request
        // https://github.com/pokt-foundation/portal-api/blob/e06d1e50abfee8533c58768bb9b638c351b87a48/src/controllers/v1.controller.ts
        if err.code == -32068 {
            return true;
        }
    }
    false
}

async fn copy_body_bytes(
    response: Response<Body>,
) -> Result<(Bytes, Response<Body>), hyper::Error> {
    let (parts, body) = response.into_parts();
    let bytes = body::to_bytes(body).await?;

    let body = Body::from(bytes.clone());
    Ok((bytes, Response::from_parts(parts, body)))
}
