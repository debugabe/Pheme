use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::{LlmProvider, LlmScriptResponse};

pub struct AnthropicProvider {
    pub api_key: String,
    pub model: String,
}

impl AnthropicProvider {
    pub fn new(api_key: String, model: String) -> Self {
        Self { api_key, model }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn generate_script(&self, system_prompt: &str, user_prompt: &str) -> Result<LlmScriptResponse> {
        let client = Client::new();
        let endpoint = "https://api.anthropic.com/v1/messages";

        let resp = client
            .post(endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": self.model,
                "max_tokens": 4096,
                "system": system_prompt,
                "messages": [
                    { "role": "user", "content": user_prompt }
                ]
            }))
            .send()
            .await
            .with_context(|| "Falha ao enviar requisição para a Anthropic Messages API")?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro retornado pela Anthropic API: {}", err_text));
        }

        let body: serde_json::Value = resp.json().await?;
        let content_text = body["content"][0]["text"]
            .as_str()
            .ok_or_else(|| anyhow!("Resposta sem texto no formato esperado da Anthropic"))?;

        // Tenta extrair bloco JSON da resposta
        let json_str = clean_json_text(content_text);
        let parsed: LlmScriptResponse = serde_json::from_str(&json_str)
            .with_context(|| format!("Falha ao parsear JSON retornado pela Anthropic: {}", content_text))?;

        Ok(parsed)
    }
}

fn clean_json_text(raw: &str) -> String {
    let trimmed = raw.trim();
    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}')) {
        trimmed[start..=end].to_string()
    } else {
        trimmed.to_string()
    }
}
