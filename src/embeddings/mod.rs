pub mod ollama;
pub mod openai;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    async fn generate_embedding(&self, text: &str) -> Result<Vec<f32>>;
}
