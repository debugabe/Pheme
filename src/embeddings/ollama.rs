use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::EmbeddingProvider;

pub struct OllamaEmbeddingProvider {
    pub base_url: String,
    pub model: String,
}

impl OllamaEmbeddingProvider {
    pub fn new(base_url: Option<String>, model: String) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".into()),
            model,
        }
    }
}

#[async_trait]
impl EmbeddingProvider for OllamaEmbeddingProvider {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let client = Client::new();
        let endpoint = format!("{}/api/embeddings", self.base_url.trim_end_matches('/'));

        let resp = client
            .post(&endpoint)
            .json(&json!({
                "model": self.model,
                "prompt": text
            }))
            .send()
            .await
            .with_context(|| format!("Falha ao conectar no Ollama embeddings em {}", endpoint))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro de embeddings no Ollama: {}", err_text));
        }

        let body: serde_json::Value = resp.json().await?;
        let vec_vals = body["embedding"]
            .as_array()
            .ok_or_else(|| anyhow!("Resposta sem campo `embedding` no Ollama"))?;

        let floats = vec_vals
            .iter()
            .filter_map(|v| v.as_f64().map(|f| f as f32))
            .collect();

        Ok(floats)
    }
}
