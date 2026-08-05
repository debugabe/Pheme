use anyhow::{Context, Result};
use hound::WavReader;
use serde::{Deserialize, Serialize};
use std::io::Cursor;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioReviewReport {
    pub is_valid: bool,
    pub duration_seconds: f32,
    pub sample_rate: u32,
    pub channels: u16,
    pub max_amplitude: f32,
    pub warnings: Vec<String>,
}

pub struct AudioTechnicalReviewer;

impl AudioTechnicalReviewer {
    pub fn review_wav_buffer(wav_bytes: &[u8], word_count: usize) -> Result<AudioReviewReport> {
        let mut reader = WavReader::new(Cursor::new(wav_bytes))
            .context("Falha ao abrir e decodificar os bytes da faixa WAV")?;

        let spec = reader.spec();
        let num_samples = reader.len();
        let duration_seconds =
            num_samples as f32 / (spec.sample_rate as f32 * spec.channels as f32);

        let mut max_amplitude: f32 = 0.0;
        let mut warnings = Vec::new();

        // Amostragem parcial para verificar amplitude máxima e detecção de áudio mudo
        let mut has_non_zero = false;

        for sample_res in reader.samples::<i32>() {
            let sample = sample_res.unwrap_or(0);
            let abs_val = sample.abs() as f32;
            if abs_val > max_amplitude {
                max_amplitude = abs_val;
            }
            if sample != 0 {
                has_non_zero = true;
            }
        }

        if !has_non_zero || max_amplitude < 10.0 {
            warnings.push("O arquivo de áudio gerado está completamente mudo ou com volume extremamente baixo.".into());
        }

        if duration_seconds < 1.0 {
            warnings.push("A duração total do áudio é inferior a 1 segundo.".into());
        }

        // Estimativa de palavras por segundo (velocidade média de fala ~ 2 a 3.5 palavras/segundo)
        if word_count > 0 && duration_seconds > 0.0 {
            let words_per_sec = word_count as f32 / duration_seconds;
            if words_per_sec > 6.0 {
                warnings.push(format!("Velocidade de fala anormalmente alta ({:.1} palavras/seg). O áudio pode ter sido cortado.", words_per_sec));
            } else if words_per_sec < 0.5 && duration_seconds > 10.0 {
                warnings.push(format!("Velocidade de fala anormalmente baixa ({:.1} palavras/seg). Áudio contendo pausas excessivas.", words_per_sec));
            }
        }

        let is_valid = has_non_zero && duration_seconds >= 1.0;

        Ok(AudioReviewReport {
            is_valid,
            duration_seconds,
            sample_rate: spec.sample_rate,
            channels: spec.channels,
            max_amplitude,
            warnings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hound::{WavSpec, WavWriter};

    #[test]
    fn test_audio_reviewer_valid_buffer() {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 22050,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut cursor = std::io::Cursor::new(Vec::new());
        let mut writer = WavWriter::new(&mut cursor, spec).unwrap();

        // 1 segundo de áudio (22050 amostras)
        for i in 0..22050 {
            let sample = (i % 100) as i16 * 10;
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();

        let bytes = cursor.into_inner();
        let report = AudioTechnicalReviewer::review_wav_buffer(&bytes, 3).unwrap();

        assert!(report.is_valid);
        assert!((report.duration_seconds - 1.0).abs() < 0.05);
    }
}
