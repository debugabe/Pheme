use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use std::io::Write;
use std::process::{Command, Stdio};

use super::TtsProvider;

pub struct PiperProvider {
    pub piper_bin: String,
}

impl PiperProvider {
    pub fn new(piper_bin: Option<String>) -> Self {
        Self {
            piper_bin: piper_bin.unwrap_or_else(|| "piper".into()),
        }
    }
}

#[async_trait]
impl TtsProvider for PiperProvider {
    async fn synthesize(&self, text: &str, voice: &str) -> Result<Vec<u8>> {
        let piper_bin = self.piper_bin.clone();
        let text_owned = text.to_string();
        let voice_owned = voice.to_string();

        // Processamento síncrono offloaded para tokio spawn_blocking
        tokio::task::spawn_blocking(move || {
            let mut child = Command::new(&piper_bin)
                .arg("--model")
                .arg(&voice_owned)
                .arg("--output-raw")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .with_context(|| format!("Falha ao executar binário do Piper em '{}'", piper_bin))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin.write_all(text_owned.as_bytes())?;
            }

            let output = child.wait_with_output()?;
            if !output.status.success() {
                let err_msg = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow!("Execução do Piper falhou: {}", err_msg));
            }

            // Converter raw PCM em WAV se necessário, ou envelopar
            let pcm_data = output.stdout;
            let wav_bytes = wrap_raw_pcm_in_wav(&pcm_data, 22050, 1, 16)?;
            Ok(wav_bytes)
        })
        .await?
    }
}

fn wrap_raw_pcm_in_wav(pcm: &[u8], sample_rate: u32, channels: u16, bits_per_sample: u16) -> Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
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
