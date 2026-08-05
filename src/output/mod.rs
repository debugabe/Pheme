use anyhow::{Context, Result};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::fs;
use std::path::{Path, PathBuf};

use crate::llm::LlmScriptResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpisodeMetadata {
    pub title: String,
    pub summary: String,
    pub topics: Vec<String>,
    pub source_url: String,
    pub published_at: Option<String>,
    pub generated_at: String,
    pub duration_preset: String,
    pub interviewer_name: String,
    pub specialist_name: String,
    pub specialist_domain: String,
}

pub fn save_episode_output(
    base_output_dir: &Path,
    title: &str,
    summary: &str,
    topics: &[String],
    source_url: &str,
    published_at: Option<chrono::DateTime<Utc>>,
    duration_preset: &str,
    interviewer_name: &str,
    specialist_name: &str,
    specialist_domain: &str,
    script_resp: &LlmScriptResponse,
    audio_wav_bytes: &[u8],
) -> Result<PathBuf> {
    let date_prefix = published_at
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| Utc::now().format("%Y-%m-%d").to_string());

    let title_slug = slugify(title);
    let folder_name = format!("{}-{}", date_prefix, title_slug);
    let episode_dir = base_output_dir.join(&folder_name);

    fs::create_dir_all(&episode_dir)
        .with_context(|| format!("Falha ao criar diretório do episódio em {:?}", episode_dir))?;

    // 1. Salvar Audio WAV
    let audio_path = episode_dir.join("audio.wav");
    fs::write(&audio_path, audio_wav_bytes)
        .with_context(|| format!("Falha ao salvar áudio em {:?}", audio_path))?;

    // 2. Salvar Metadata JSON
    let metadata = EpisodeMetadata {
        title: title.to_string(),
        summary: summary.to_string(),
        topics: topics.to_vec(),
        source_url: source_url.to_string(),
        published_at: published_at.map(|d| d.to_rfc3339()),
        generated_at: Utc::now().to_rfc3339(),
        duration_preset: duration_preset.to_string(),
        interviewer_name: interviewer_name.to_string(),
        specialist_name: specialist_name.to_string(),
        specialist_domain: specialist_domain.to_string(),
    };

    let meta_json = serde_json::to_string_pretty(&metadata)?;
    let meta_path = episode_dir.join("metadata.json");
    fs::write(&meta_path, meta_json)?;

    // 3. Salvar Transcript Markdown
    let mut md = String::new();
    md.push_str(&format!("# {}\n\n", title));
    md.push_str(&format!("**Data da Notícia**: {}\n", date_prefix));
    md.push_str(&format!("**Fonte**: {}\n", source_url));
    md.push_str(&format!("**Participantes**: {} (Entrevistador) e {} (Especialista em {})\n\n", interviewer_name, specialist_name, specialist_domain));
    md.push_str("## Resumo do Episódio\n\n");
    md.push_str(summary);
    md.push_str("\n\n## Tópicos Abordados\n\n");
    for t in topics {
        md.push_str(&format!("- {}\n", t));
    }
    md.push_str("\n---\n\n## Transcrição do Diálogo\n\n");

    for turn in &script_resp.dialogue {
        let speaker_display_name = if turn.speaker == "interviewer" {
            interviewer_name
        } else {
            specialist_name
        };
        md.push_str(&format!("**{}**: {}\n\n", speaker_display_name, turn.text));
    }

    let transcript_path = episode_dir.join("transcript.md");
    fs::write(&transcript_path, md)?;

    Ok(episode_dir)
}
