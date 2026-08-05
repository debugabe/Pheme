pub mod wizard;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmConfig {
    pub provider: String,
    pub base_url: Option<String>,
    pub api_key_env: Option<String>,
    pub model: String,
}

impl LlmConfig {
    pub fn resolve_api_key(&self) -> Option<String> {
        self.api_key_env
            .as_ref()
            .and_then(|env_var| std::env::var(env_var).ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddingConfig {
    pub provider: String,
    pub base_url: Option<String>,
    pub api_key_env: Option<String>,
    pub model: String,
}

impl EmbeddingConfig {
    pub fn resolve_api_key(&self) -> Option<String> {
        self.api_key_env
            .as_ref()
            .and_then(|env_var| std::env::var(env_var).ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub provider: String,
    pub piper_path: Option<String>,
    pub interviewer_voice: String,
    pub specialist_voice: String,
    pub elevenlabs_api_key_env: Option<String>,
}

impl TtsConfig {
    pub fn resolve_elevenlabs_api_key(&self) -> Option<String> {
        self.elevenlabs_api_key_env
            .as_ref()
            .and_then(|env_var| std::env::var(env_var).ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaEntryConfig {
    pub name: String,
    pub domain: Option<String>,
    pub mood: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonasConfig {
    pub interviewer: PersonaEntryConfig,
    pub specialist: PersonaEntryConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultsConfig {
    pub period_days: u32,
    pub duration_preset: String, // curto, medio, longo
    pub output_dir: String,
    pub db_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub language: String,
    pub llm: LlmConfig,
    pub embedding: EmbeddingConfig,
    pub tts: TtsConfig,
    pub personas: PersonasConfig,
    pub defaults: DefaultsConfig,
}

impl Config {
    pub fn default_config_path() -> PathBuf {
        if let Some(mut home) = dirs::config_dir() {
            home.push("pheme");
            home.push("pheme.toml");
            home
        } else {
            PathBuf::from("pheme.toml")
        }
    }

    pub fn load() -> Result<Self> {
        let path = Self::find_config_file()?;
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Falha ao ler o arquivo de configuração em {:?}", path))?;

        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Falha no parsing TOML do arquivo {:?}", path))?;

        config.validate()?;
        Ok(config)
    }

    pub fn find_config_file() -> Result<PathBuf> {
        let local_path = PathBuf::from("pheme.toml");
        if local_path.exists() {
            return Ok(local_path);
        }

        let user_config = Self::default_config_path();
        if user_config.exists() {
            return Ok(user_config);
        }

        Err(anyhow!(
            "Arquivo de configuração `pheme.toml` não encontrado em ./pheme.toml nem em {:?}.\nExecute `pheme init` para criar uma nova configuração.",
            user_config
        ))
    }

    pub fn validate(&self) -> Result<()> {
        if self.language.trim().is_empty() {
            return Err(anyhow!("Campo `language` é obrigatório na configuração."));
        }
        if self.llm.model.trim().is_empty() {
            return Err(anyhow!("Campo `llm.model` é obrigatório."));
        }
        if self.embedding.model.trim().is_empty() {
            return Err(anyhow!("Campo `embedding.model` é obrigatório."));
        }
        if self.personas.interviewer.name.trim().is_empty() {
            return Err(anyhow!("Campo `personas.interviewer.name` é obrigatório."));
        }
        if self.personas.specialist.name.trim().is_empty() {
            return Err(anyhow!("Campo `personas.specialist.name` é obrigatório."));
        }
        Ok(())
    }

    pub fn resolve_db_path(&self) -> PathBuf {
        expand_path(&self.defaults.db_path)
    }

    pub fn resolve_output_dir(&self) -> PathBuf {
        expand_path(&self.defaults.output_dir)
    }
}

pub fn expand_path(p: &str) -> PathBuf {
    if p.starts_with("~/") || p == "~" {
        if let Some(home) = dirs::home_dir() {
            return home.join(p.trim_start_matches("~/").trim_start_matches("~"));
        }
    }
    PathBuf::from(p)
}
