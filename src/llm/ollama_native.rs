use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::{LlmProvider, LlmScriptResponse};

pub struct OllamaNativeProvider {
    pub base_url: String,
    pub model: String,
}

impl OllamaNativeProvider {
    pub fn new(base_url: Option<String>, model: String) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".into()),
            model,
        }
    }
}

#[async_trait]
impl LlmProvider for OllamaNativeProvider {
    async fn generate_script(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<LlmScriptResponse> {
        let client = Client::new();
        let endpoint = format!("{}/api/generate", self.base_url.trim_end_matches('/'));

        let prompt = format!("SYSTEM:\n{}\n\nUSER:\n{}", system_prompt, user_prompt);

        let resp = client
            .post(&endpoint)
            .json(&json!({
                "model": self.model,
                "prompt": prompt,
                "format": "json",
                "stream": false
            }))
            .send()
            .await
            .with_context(|| format!("Falha ao conectar no Ollama local em {}", endpoint))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro retornado pelo Ollama: {}", err_text));
        }

        let body: serde_json::Value = resp.json().await?;
        let response_text = body["response"]
            .as_str()
            .ok_or_else(|| anyhow!("Resposta do Ollama sem o campo `response`"))?;

        let parsed: LlmScriptResponse = serde_json::from_str(response_text).with_context(|| {
            format!(
                "Falha ao parsear o JSON de resposta do Ollama: {}",
                response_text
            )
        })?;

        Ok(parsed)
    }
}
