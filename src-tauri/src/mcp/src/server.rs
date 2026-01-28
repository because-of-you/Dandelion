use rmcp::{
    handler::server::tool::ToolRouter,
    handler::server::wrapper::Parameters,
    model::{CallToolResult, Content, Implementation, InitializeResult, ProtocolVersion, ServerCapabilities, ServerInfo},
    tool, tool_handler, tool_router, ServerHandler,
};
use rmcp::ErrorData as McpError;

use crate::types::{EchoOutput, EchoParameters};

#[derive(Clone)]
pub struct MyMcpServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl MyMcpServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Echo back the input message
    #[tool(description = "Echo back the input message to test the MCP server")]
    async fn echo(
        Parameters(EchoParameters { message }): Parameters<EchoParameters>,
    ) -> Result<CallToolResult, McpError> {
        let output = EchoOutput {
            message: format!("Echo: {}", message),
        };

        let content = Content::json(output)
            .map_err(|e| McpError::internal_error(format!("Error serializing output: {}", e), None))?;

        Ok(CallToolResult::success(vec![content]))
    }
}

#[tool_handler]
impl ServerHandler for MyMcpServer {
    fn get_info(&self) -> rmcp::model::ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2025_06_18,
            capabilities: ServerCapabilities::builder()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "I am a simple MCP server that provides an echo tool for testing purposes.

                The available tools are:
                - echo: Echo back a message
                ".to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _request: rmcp::model::InitializeRequestParams,
        _context: rmcp::service::RequestContext<rmcp::RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(self.get_info())
    }
}

pub fn create_server() -> MyMcpServer {
    MyMcpServer::new()
}