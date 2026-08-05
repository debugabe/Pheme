pub mod elevenlabs;
pub mod piper;

use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait TtsProvider: Send + Sync {
    async fn synthesize(&self, text: &str, voice: &str) -> Result<Vec<u8>>;
}
