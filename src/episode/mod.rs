pub mod reviewer;
pub mod script;

use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, warn};

use crate::audio::{concatenate_wav_buffers, AudioTechnicalReviewer};
use crate::config::Config;
use crate::embeddings::{
    ollama::OllamaEmbeddingProvider, openai::OpenAiEmbeddingProvider, EmbeddingProvider,
};
use crate::llm::{
    anthropic::AnthropicProvider, ollama_native::OllamaNativeProvider,
    openai_compatible::OpenAiCompatibleProvider, LlmProvider,
};
use crate::memory::MemoryStore;
use crate::news::{load_article_from_url, NewsReviewer};
use crate::output::save_episode_output;
use crate::personas::{Persona, Role};
use crate::tts::{elevenlabs::ElevenLabsProvider, piper::PiperProvider, TtsProvider};
use reviewer::ScriptFidelityReviewer;

pub struct EpisodeGenerator {
    config: Config,
    llm: Arc<dyn LlmProvider>,
    embedding: Arc<dyn EmbeddingProvider>,
    tts: Arc<dyn TtsProvider>,
    memory_store: MemoryStore,
}

impl EpisodeGenerator {
    pub fn new(config: Config) -> Result<Self> {
        let llm: Arc<dyn LlmProvider> = match config.llm.provider.as_str() {
            "openai_compatible" => {
                let base_url = config
                    .llm
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.openai.com/v1".into());
                let api_key = config.llm.resolve_api_key();
                Arc::new(OpenAiCompatibleProvider::new(
                    base_url,
                    api_key,
                    config.llm.model.clone(),
                ))
            }
            "ollama_native" => Arc::new(OllamaNativeProvider::new(
                config.llm.base_url.clone(),
                config.llm.model.clone(),
            )),
            "anthropic" => {
                let api_key = config.llm.resolve_api_key().ok_or_else(|| {
                    anyhow!("Variável de ambiente para a API Key da Anthropic não definida")
                })?;
                Arc::new(AnthropicProvider::new(api_key, config.llm.model.clone()))
            }
            provider => return Err(anyhow!("Provedor LLM desconhecido: {}", provider)),
        };

        let embedding: Arc<dyn EmbeddingProvider> = match config.embedding.provider.as_str() {
            "openai" => {
                let base_url = config
                    .embedding
                    .base_url
                    .clone()
                    .unwrap_or_else(|| "https://api.openai.com/v1".into());
                let api_key = config.embedding.resolve_api_key();
                Arc::new(OpenAiEmbeddingProvider::new(
                    base_url,
                    api_key,
                    config.embedding.model.clone(),
                ))
            }
            "ollama" => Arc::new(OllamaEmbeddingProvider::new(
                config.embedding.base_url.clone(),
                config.embedding.model.clone(),
            )),
            provider => return Err(anyhow!("Provedor de Embedding desconhecido: {}", provider)),
        };

        let tts: Arc<dyn TtsProvider> = match config.tts.provider.as_str() {
            "piper" => Arc::new(PiperProvider::new(config.tts.piper_path.clone())),
            "elevenlabs" => {
                let api_key = config.tts.resolve_elevenlabs_api_key().ok_or_else(|| {
                    anyhow!("Variável de ambiente para a ElevenLabs API Key não definida")
                })?;
                Arc::new(ElevenLabsProvider::new(api_key))
            }
            provider => return Err(anyhow!("Provedor de TTS desconhecido: {}", provider)),
        };

        let db_path = config.resolve_db_path();
        let memory_store = MemoryStore::open(&db_path)?;

        Ok(Self {
            config,
            llm,
            embedding,
            tts,
            memory_store,
        })
    }

    pub async fn run_pipeline(&self, url: &str) -> Result<PathBuf> {
        info!("1/7 Buscando matéria em: {}", url);
        let article = load_article_from_url(url).await?;

        info!("2/7 [Revisor de Notícias] Validando qualidade do artigo capturado...");
        let news_report = NewsReviewer::review_article(&article);
        if !news_report.is_valid {
            for warn_msg in &news_report.warnings {
                warn!("Aviso na matéria: {}", warn_msg);
            }
            return Err(anyhow!(
                "A matéria capturada foi rejeitada pelo Revisor de Notícias (Score: {:.2}). Motivos: {:?}",
                news_report.score,
                news_report.warnings
            ));
        } else if !news_report.warnings.is_empty() {
            for warn_msg in &news_report.warnings {
                warn!("Aviso de qualidade na notícia: {}", warn_msg);
            }
        }

        info!("3/7 Pesquisando memórias de episódios anteriores...");
        let article_embedding = self.embedding.generate_embedding(&article.content).await?;
        let related_memories = self
            .memory_store
            .find_similar_episodes(&article_embedding, 0.70, 3)
            .unwrap_or_default();

        info!("4/7 Gerando roteiro via LLM ({})", self.config.llm.model);
        let interviewer_persona = Persona {
            name: self.config.personas.interviewer.name.clone(),
            role: Role::Interviewer,
            domain: self.config.personas.interviewer.domain.clone(),
            mood: self.config.personas.interviewer.mood.clone(),
        };

        let specialist_persona = Persona {
            name: self.config.personas.specialist.name.clone(),
            role: Role::Specialist,
            domain: self.config.personas.specialist.domain.clone(),
            mood: self.config.personas.specialist.mood.clone(),
        };

        let sys_prompt = script::build_system_prompt(
            &interviewer_persona,
            &specialist_persona,
            &self.config.defaults.duration_preset,
            &self.config.language,
        );

        let pub_date_str = article.published_at.map(|d| d.to_rfc3339());
        let user_prompt = script::build_user_prompt(
            &article.title,
            &article.content,
            pub_date_str.as_deref(),
            &related_memories,
        );

        let script_resp = self.llm.generate_script(&sys_prompt, &user_prompt).await?;
        info!("Roteiro gerado! Título: '{}'", script_resp.episode_title);

        info!("5/7 [Revisor de Fidelidade] Auditando roteiro contra a notícia original...");
        let fidelity_report = ScriptFidelityReviewer::review_fidelity(
            self.llm.as_ref(),
            &article.title,
            &article.content,
            &script_resp,
        )
        .await?;

        if !fidelity_report.is_coherent {
            return Err(anyhow!(
                "O roteiro gerado falhou na auditoria de fidelidade com a notícia: {:?}",
                fidelity_report.observations
            ));
        }

        info!(
            "6/7 Sintetizando falas em áudio com TTS ({})",
            self.config.tts.provider
        );
        let mut audio_buffers = Vec::new();
        let mut total_word_count = 0;

        for (i, turn) in script_resp.dialogue.iter().enumerate() {
            total_word_count += turn.text.split_whitespace().count();
            let voice = if turn.speaker == "interviewer" {
                &self.config.tts.interviewer_voice
            } else {
                &self.config.tts.specialist_voice
            };

            info!(
                "Sintetizando fala {}/{} [{}]",
                i + 1,
                script_resp.dialogue.len(),
                turn.speaker
            );
            let wav_buf = self.tts.synthesize(&turn.text, voice).await?;
            audio_buffers.push(wav_buf);
        }

        let final_audio_wav = concatenate_wav_buffers(&audio_buffers, 300)?;

        info!("7/7 [Revisor Técnico de Áudio] Inspecionando integridade da faixa WAV gerada...");
        let audio_report =
            AudioTechnicalReviewer::review_wav_buffer(&final_audio_wav, total_word_count)?;

        if !audio_report.is_valid {
            return Err(anyhow!(
                "O áudio final gerado falhou na revisão técnica: {:?}",
                audio_report.warnings
            ));
        }

        info!(
            "Áudio aprovado pelo Revisor! Duração: {:.1}s, Freq: {}Hz, Canais: {}",
            audio_report.duration_seconds, audio_report.sample_rate, audio_report.channels
        );

        let episode_embedding = self
            .embedding
            .generate_embedding(&script_resp.summary)
            .await?;

        self.memory_store.save_episode(
            &script_resp.episode_title,
            &script_resp.summary,
            &script_resp.topics,
            article.published_at.unwrap_or_else(chrono::Utc::now),
            &episode_embedding,
        )?;

        let out_dir = self.config.resolve_output_dir();
        let episode_dir = save_episode_output(
            &out_dir,
            &script_resp.episode_title,
            &script_resp.summary,
            &script_resp.topics,
            url,
            article.published_at,
            &self.config.defaults.duration_preset,
            &interviewer_persona.name,
            &specialist_persona.name,
            specialist_persona.domain.as_deref().unwrap_or("Tecnologia"),
            &script_resp,
            &final_audio_wav,
        )?;

        Ok(episode_dir)
    }
}
