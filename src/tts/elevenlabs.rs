use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

use super::TtsProvider;

pub struct ElevenLabsProvider {
    pub api_key: String,
}

impl ElevenLabsProvider {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl TtsProvider for ElevenLabsProvider {
    async fn synthesize(&self, text: &str, voice: &str) -> Result<Vec<u8>> {
        let client = Client::new();
        let endpoint = format!(
            "https://api.elevenlabs.io/v1/text-to-speech/{}/stream?output_format=pcm_24000",
            voice
        );

        let resp = client
            .post(&endpoint)
            .header("xi-api-key", &self.api_key)
            .json(&json!({
                "text": text,
                "model_id": "eleven_multilingual_v2",
                "voice_settings": {
                    "stability": 0.5,
                    "similarity_boost": 0.75
                }
            }))
            .send()
            .await
            .with_context(|| format!("Falha na chamada ElevenLabs para voz '{}'", voice))?;

        if !resp.status().is_success() {
            let err_text = resp.text().await.unwrap_or_default();
            return Err(anyhow!("Erro retornado pela ElevenLabs API: {}", err_text));
        }

        let pcm_bytes = resp.bytes().await?.to_vec();
        let wav_bytes = wrap_pcm_24k_in_wav(&pcm_bytes)?;

        Ok(wav_bytes)
    }
}

fn wrap_pcm_24k_in_wav(pcm: &[u8]) -> Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 24000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = std::io::Cursor::new(Vec::new());
    let mut writer = hound::WavWriter::new(&mut cursor, spec)?;

    let mut i = 0;
    while i + 1 < pcm.len() {
        let sample = i16::from_le_bytes([pcm[i], pcm[i + 1]]);
        writer.write_sample(sample)?;
        i += 2;
    }

    writer.finalize()?;
    Ok(cursor.into_inner())
}
