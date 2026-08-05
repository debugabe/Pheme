pub mod anthropic;
pub mod ollama_native;
pub mod openai_compatible;

use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptTurn {
    pub speaker: String, // "interviewer" ou "specialist"
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmScriptResponse {
    pub episode_title: String,
    pub summary: String,
    pub topics: Vec<String>,
    pub dialogue: Vec<ScriptTurn>,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn generate_script(&self, system_prompt: &str, user_prompt: &str) -> Result<LlmScriptResponse>;
}
