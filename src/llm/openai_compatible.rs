use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::{LlmProvider, LlmScriptResponse};

pub struct OpenAiCompatibleProvider {
    pub base_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl OpenAiCompatibleProvider {
    pub fn new(base_url: String, api_key: Option<String>, model: String) -> Self {
        Self {
            base_url,
            api_key,
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiCompatibleProvider {
    async fn generate_script(&self, system_prompt: &str, user_prompt: &str) -> Result<LlmScriptResponse> {
        let client = Client::new();
        let endpoint = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));

        let mut req = client.post(&endpoint).json(&json!({
            "model": self.model,
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": user_prompt }
            ],
            "response_format": { "type": "json_object" }
        }));

        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req
            .send()
            .await
            .with_context(|| format!("Falha na chamada HTTP para {}", endpoint))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro retornado pela API OpenAI-Compatible: {}", err_text));
        }

        let body: serde_json::Value = resp.json().await?;
        let content = body["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Formato de resposta inesperado da API (sem content)"))?;

        let parsed: LlmScriptResponse = serde_json::from_str(content)
            .with_context(|| format!("Falha ao parsear JSON do roteiro retornado pela LLM: {}", content))?;

        Ok(parsed)
    }
}
