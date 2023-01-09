use std::env;

use async_trait::async_trait;
use test_context::AsyncTestContext;

use self::server::RpcProxy;

mod server;

pub struct ServerContext {
    pub server: RpcProxy,
}

#[async_trait]
impl AsyncTestContext for ServerContext {
    async fn setup() -> Self {
        #[cfg(feature = "test-localhost")]
        let server = RpcProxy::start().await;

        #[cfg(not(feature = "test-localhost"))]
        let server = {
            let project_id = env::var("PROJECT_ID").expect("PROJECT_ID must be set");
            RpcProxy {
                public_addr: "https://staging.rpc.walletconnect.com".into(),
                port: None,
                project_id,
                shutdown_signal: None,
                is_shutdown: false,
            }
        };
        Self { server }
    }

    async fn teardown(mut self) {
        #[cfg(feature = "test-localhost")]
        self.server.shutdown().await;
    }
}

pub type TestResult<T> = Result<T, TestError>;

#[derive(Debug, thiserror::Error)]
pub enum TestError {
    #[error(transparent)]
    Elapsed(#[from] tokio::time::error::Elapsed),

    #[error(transparent)]
    RpcProxy(#[from] rpc_proxy::error::RpcError),
}
