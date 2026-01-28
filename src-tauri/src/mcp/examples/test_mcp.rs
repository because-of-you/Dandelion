use mcp::create_server;
use rmcp::transport::streamable_http_server::session::local::LocalSessionManager;
use rmcp::transport::streamable_http_server::StreamableHttpServerConfig;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting MCP server test on port 3000...");

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let service = rmcp::transport::streamable_http_server::StreamableHttpService::new(
        || Ok(create_server()),
        Arc::new(LocalSessionManager::default()),
        StreamableHttpServerConfig::default(),
    );

    let app = axum::Router::new().nest_service("/mcp", service);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("MCP server is running at http://{}", addr);
    println!("MCP endpoint: http://{}/mcp", addr);
    println!("\nNote: MCP servers use the MCP protocol, not regular HTTP.");
    println!("Use an MCP client (like Cursor, Claude Desktop, or MCP Inspector) to connect.");
    println!("\nExample MCP Inspector command:");
    println!("  npx @modelcontextprotocol/inspector http://localhost:3000/mcp");

    axum::serve(listener, app).await?;

    Ok(())
}