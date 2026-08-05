use anyhow::{anyhow, Context, Result};
use hound::{SampleFormat, WavReader, WavWriter};
use std::io::Cursor;

pub fn concatenate_wav_buffers(buffers: &[Vec<u8>], silence_duration_ms: u32) -> Result<Vec<u8>> {
    if buffers.is_empty() {
        return Err(anyhow!("Nenhum buffer de áudio fornecido para concatenação"));
    }

    // Lê as especificações do primeiro buffer para definir a saída
    let first_reader = WavReader::new(Cursor::new(&buffers[0]))
        .context("Falha ao abrir primeiro buffer WAV")?;
    let target_spec = first_reader.spec();

    let mut output_cursor = Cursor::new(Vec::new());
    let mut writer = WavWriter::new(&mut output_cursor, target_spec)?;

    let num_silence_samples = (target_spec.sample_rate as u64 * silence_duration_ms as u64 / 1000) as usize;

    for (idx, buf) in buffers.iter().enumerate() {
        let mut reader = WavReader::new(Cursor::new(buf))
            .with_context(|| format!("Falha ao ler o buffer de áudio #{}", idx))?;
        
        let current_spec = reader.spec();

        match (current_spec.sample_format, target_spec.sample_format) {
            (SampleFormat::Int, SampleFormat::Int) => {
                for sample in reader.samples::<i32>() {
                    let s = sample?;
                    writer.write_sample(s)?;
                }
            }
            (SampleFormat::Float, SampleFormat::Float) => {
                for sample in reader.samples::<f32>() {
                    let s = sample?;
                    writer.write_sample(s)?;
                }
            }
            _ => {
                return Err(anyhow!("Formatos de amostra incompatíveis na concatenação WAV"));
            }
        }

        // Insere silêncio entre as falas (exceto após a última fala)
        if idx + 1 < buffers.len() {
            for _ in 0..num_silence_samples {
                for _ in 0..target_spec.channels {
                    match target_spec.sample_format {
                        SampleFormat::Int => writer.write_sample(0i32)?,
                        SampleFormat::Float => writer.write_sample(0.0f32)?,
                    }
                }
            }
        }
    }

    writer.finalize()?;
    Ok(output_cursor.into_inner())
}
