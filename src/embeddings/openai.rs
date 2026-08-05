use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::EmbeddingProvider;

pub struct OpenAiEmbeddingProvider {
    pub base_url: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl OpenAiEmbeddingProvider {
    pub fn new(base_url: String, api_key: Option<String>, model: String) -> Self {
        Self {
            base_url,
            api_key,
            model,
        }
    }
}

#[async_trait]
impl EmbeddingProvider for OpenAiEmbeddingProvider {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let client = Client::new();
        let endpoint = format!("{}/embeddings", self.base_url.trim_end_matches('/'));

        let mut req = client.post(&endpoint).json(&json!({
            "model": self.model,
            "input": text
        }));

        if let Some(key) = &self.api_key {
            req = req.bearer_auth(key);
        }

        let resp = req
            .send()
            .await
            .with_context(|| format!("Falha ao solicitar embeddings em {}", endpoint))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro retornado na API de Embeddings: {}", err_text));
        }

        let body: serde_json::Value = resp.json().await?;
        let vec_vals = body["data"][0]["embedding"]
            .as_array()
            .ok_or_else(|| anyhow!("Resposta de embedding sem array de valores"))?;

        let floats = vec_vals
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        Ok(floats)
    }
}
