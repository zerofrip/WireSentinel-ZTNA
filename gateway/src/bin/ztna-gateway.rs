//! Standalone ZTNA service gateway (Phase 15-F).

use gateway::ServiceGateway;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let gateway = ServiceGateway::new();
    let addr = gateway.start_http_stub().await?;
    tracing::info!(%addr, "ztna-gateway listening");

    tokio::signal::ctrl_c().await?;
    Ok(())
}
