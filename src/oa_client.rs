use std::sync::Arc;

use async_openai::{config::OpenAIConfig, Client};

use crate::Result;

pub type OaClient = Arc<Client<OpenAIConfig>>;

pub fn new_oa_client() -> Result<OaClient> {
    let config = OpenAIConfig::new()
        .with_api_base("http://localhost:11434/v1")
        .with_api_key("ollama");
    Ok(Client::with_config(config).into())
}
