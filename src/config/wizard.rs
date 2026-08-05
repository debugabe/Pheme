use anyhow::{Context, Result};
use inquire::{Select, Text};
use std::fs;

use super::{
    Config, DefaultsConfig, EmbeddingConfig, LlmConfig, PersonaEntryConfig, PersonasConfig,
    TtsConfig,
};
use crate::personas::presets::get_available_moods;

pub fn run_wizard() -> Result<()> {
    println!("=== Assistente de Configuração do Pheme ===\n");

    let language = Select::new("Selecione o idioma dos episódios:", vec!["pt-BR", "en-US"])
        .prompt()
        .context("Falha ao selecionar idioma")?
        .to_string();

    // LLM Setup
    println!("\n--- Configuração de LLM (Roteiro) ---");
    let llm_providers = vec!["openai_compatible", "ollama_native", "anthropic"];
    let llm_provider = Select::new("Provedor de LLM:", llm_providers)
        .prompt()
        .context("Falha ao selecionar provedor de LLM")?
        .to_string();

    let llm_base_url = if llm_provider == "openai_compatible" {
        Some(
            Text::new("URL Base da API LLM:")
                .with_default("https://api.openai.com/v1")
                .prompt()?,
        )
    } else if llm_provider == "anthropic" {
        Some("https://api.anthropic.com".to_string())
    } else {
        None
    };

    let llm_api_key_env = if llm_provider != "ollama_native" {
        Some(
            Text::new("Nome da variável de ambiente com a chave de API:")
                .with_default(if llm_provider == "anthropic" {
                    "ANTHROPIC_API_KEY"
                } else {
                    "OPENAI_API_KEY"
                })
                .prompt()?,
        )
    } else {
        None
    };

    let default_model = match llm_provider.as_str() {
        "anthropic" => "claude-3-5-sonnet-20241022",
        "ollama_native" => "llama3",
        _ => "gpt-4o-mini",
    };

    let llm_model = Text::new("Modelo de LLM:")
        .with_default(default_model)
        .prompt()?;

    // Embedding Setup
    println!("\n--- Configuração de Embeddings (Memória Semântica) ---");
    let emb_providers = vec!["openai", "ollama"];
    let emb_provider = Select::new("Provedor de Embeddings:", emb_providers)
        .prompt()?
        .to_string();

    let emb_base_url = if emb_provider == "openai" {
        Some(
            Text::new("URL Base da API de Embedding:")
                .with_default("https://api.openai.com/v1")
                .prompt()?,
        )
    } else {
        None
    };

    let emb_api_key_env = if emb_provider == "openai" {
        Some(
            Text::new("Variável de ambiente para chave de Embedding:")
                .with_default("OPENAI_API_KEY")
                .prompt()?,
        )
    } else {
        None
    };

    let emb_model = Text::new("Modelo de Embedding:")
        .with_default(if emb_provider == "openai" {
            "text-embedding-3-small"
        } else {
            "nomic-embed-text"
        })
        .prompt()?;

    // TTS Setup
    println!("\n--- Configuração de TTS (Voz) ---");
    let tts_providers = vec!["piper", "elevenlabs"];
    let tts_provider = Select::new("Provedor de TTS:", tts_providers)
        .prompt()?
        .to_string();

    let piper_path = if tts_provider == "piper" {
        Some(
            Text::new("Caminho para o binário executável do Piper:")
                .with_default("piper")
                .prompt()?,
        )
    } else {
        None
    };

    let interviewer_voice = Text::new("Voz do(a) Entrevistador(a):")
        .with_default(if tts_provider == "piper" {
            "pt_BR-faber-medium.onnx"
        } else {
            "21m00Tcm4TlvDq8ikWAM"
        })
        .prompt()?;

    let specialist_voice = Text::new("Voz do(a) Especialista:")
        .with_default(if tts_provider == "piper" {
            "pt_BR-carlos-medium.onnx"
        } else {
            "AZnzlk1XvdvUeBnXmlld"
        })
        .prompt()?;

    let elevenlabs_api_key_env = if tts_provider == "elevenlabs" {
        Some(
            Text::new("Variável de ambiente com chave ElevenLabs:")
                .with_default("ELEVENLABS_API_KEY")
                .prompt()?,
        )
    } else {
        None
    };

    // Personas Setup
    println!("\n--- Configuração das Personas ---");
    let interviewer_name = Text::new("Nome do(a) Entrevistador(a):")
        .with_default("Alex")
        .prompt()?;

    let available_moods = get_available_moods();

    let interviewer_mood = Select::new(
        "Mood do Entrevistador(a):",
        available_moods.clone(),
    )
    .prompt()?
    .to_string();

    let specialist_name = Text::new("Nome Padrão do Especialista:")
        .with_default("Dr. Silva")
        .prompt()?;

    let specialist_domain = Text::new("Domínio Padrão do Especialista:")
        .with_default("Inteligência Artificial")
        .prompt()?;

    let specialist_mood = Select::new(
        "Mood Padrão do Especialista:",
        available_moods.clone(),
    )
    .prompt()?
    .to_string();

    // Defaults
    println!("\n--- Padrões Gerais ---");
    let output_dir = Text::new("Diretório de saída dos episódios:")
        .with_default("./episodes")
        .prompt()?;

    let db_path = Text::new("Caminho do banco de memória SQLite:")
        .with_default("~/.config/pheme/memory.db")
        .prompt()?;

    let duration_preset = Select::new(
        "Duração padrão dos episódios:",
        vec!["curto", "medio", "longo"],
    )
    .prompt()?
    .to_string();

    let config = Config {
        language,
        llm: LlmConfig {
            provider: llm_provider,
            base_url: llm_base_url,
            api_key_env: llm_api_key_env,
            model: llm_model,
        },
        embedding: EmbeddingConfig {
            provider: emb_provider,
            base_url: emb_base_url,
            api_key_env: emb_api_key_env,
            model: emb_model,
        },
        tts: TtsConfig {
            provider: tts_provider,
            piper_path,
            interviewer_voice,
            specialist_voice,
            elevenlabs_api_key_env,
        },
        personas: PersonasConfig {
            interviewer: PersonaEntryConfig {
                name: interviewer_name,
                domain: Some("Tecnologia".into()),
                mood: interviewer_mood,
            },
            specialist: PersonaEntryConfig {
                name: specialist_name,
                domain: Some(specialist_domain),
                mood: specialist_mood,
            },
        },
        defaults: DefaultsConfig {
            period_days: 7,
            duration_preset,
            output_dir,
            db_path,
        },
    };

    let target_path = Config::default_config_path();
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let toml_str = toml::to_string_pretty(&config)?;
    fs::write(&target_path, toml_str)?;

    println!(
        "\nConfiguração salva com sucesso em: {:?}\nVocê já pode executar `pheme generate <URL>`!",
        target_path
    );

    Ok(())
}
