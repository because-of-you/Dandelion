use rmcp::schemars;
use serde::{Deserialize, Serialize};

/// Parameters for the echo tool
#[derive(Debug, Deserialize, schemars::JsonSchema)]
#[schemars(description = "Input parameters for the echo tool")]
pub struct EchoParameters {
    #[schemars(description = "The message to echo back")]
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EchoOutput {
    pub message: String,
}