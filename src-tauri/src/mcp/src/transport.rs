use crate::server::create_server;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use std::net::SocketAddr;
use std::sync::Arc;

/// Run the MCP server with Streamable HTTP transport
pub async fn run_streamable_http_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {
    use axum::Router;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let service = rmcp::transport::streamable_http_server::StreamableHttpService::new(
        || Ok(create_server()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    let app = Router::new().nest_service("/mcp", service);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}