use anyhow::{Context, Result};
use inquire::{Select, Text};
use std::fs;

use super::{
    Config, DefaultsConfig, EmbeddingConfig, LlmConfig, PersonaEntryConfig, PersonasConfig,
    TtsConfig,
};
use crate::personas::presets::get_available_moods;

pub fn run_wizard() -> Result<()> {
    println!("=== Pheme Setup Wizard / Assistente de Configuração ===\n");

    let language = Select::new(
        "Select episode language / Selecione o idioma dos episódios:",
        vec!["pt-BR", "en-US"],
    )
    .prompt()
    .context("Failed to select language / Falha ao selecionar idioma")?
    .to_string();

    let is_english = language.starts_with("en");

    // LLM Setup
    if is_english {
        println!("\n--- LLM Configuration (Script Generation) ---");
    } else {
        println!("\n--- Configuração de LLM (Roteiro) ---");
    }

    let llm_providers = vec!["openai_compatible", "ollama_native", "anthropic"];
    let llm_prompt_lbl = if is_english {
        "LLM Provider:"
    } else {
        "Provedor de LLM:"
    };

    let llm_provider = Select::new(llm_prompt_lbl, llm_providers)
        .prompt()
        .context("Failed to select LLM provider")?
        .to_string();

    let base_url_lbl = if is_english {
        "LLM API Base URL:"
    } else {
        "URL Base da API LLM:"
    };

    let llm_base_url = if llm_provider == "openai_compatible" {
        Some(
            Text::new(base_url_lbl)
                .with_default("https://api.openai.com/v1")
                .prompt()?,
        )
    } else if llm_provider == "anthropic" {
        Some("https://api.anthropic.com".to_string())
    } else {
        None
    };

    let api_key_lbl = if is_english {
        "Environment variable name for API key:"
    } else {
        "Nome da variável de ambiente com a chave de API:"
    };

    let llm_api_key_env = if llm_provider != "ollama_native" {
        Some(
            Text::new(api_key_lbl)
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

    let model_lbl = if is_english {
        "LLM Model:"
    } else {
        "Modelo de LLM:"
    };

    let llm_model = Text::new(model_lbl).with_default(default_model).prompt()?;

    // Embedding Setup
    if is_english {
        println!("\n--- Embedding Configuration (Semantic Memory) ---");
    } else {
        println!("\n--- Configuração de Embeddings (Memória Semântica) ---");
    }

    let emb_providers = vec!["openai", "ollama"];
    let emb_prov_lbl = if is_english {
        "Embedding Provider:"
    } else {
        "Provedor de Embeddings:"
    };

    let emb_provider = Select::new(emb_prov_lbl, emb_providers)
        .prompt()?
        .to_string();

    let emb_url_lbl = if is_english {
        "Embedding API Base URL:"
    } else {
        "URL Base da API de Embedding:"
    };

    let emb_base_url = if emb_provider == "openai" {
        Some(
            Text::new(emb_url_lbl)
                .with_default("https://api.openai.com/v1")
                .prompt()?,
        )
    } else {
        None
    };

    let emb_key_lbl = if is_english {
        "Environment variable for Embedding API key:"
    } else {
        "Variável de ambiente para chave de Embedding:"
    };

    let emb_api_key_env = if emb_provider == "openai" {
        Some(
            Text::new(emb_key_lbl)
                .with_default("OPENAI_API_KEY")
                .prompt()?,
        )
    } else {
        None
    };

    let emb_model_lbl = if is_english {
        "Embedding Model:"
    } else {
        "Modelo de Embedding:"
    };

    let emb_model = Text::new(emb_model_lbl)
        .with_default(if emb_provider == "openai" {
            "text-embedding-3-small"
        } else {
            "nomic-embed-text"
        })
        .prompt()?;

    // TTS Setup
    if is_english {
        println!("\n--- TTS Configuration (Voice) ---");
    } else {
        println!("\n--- Configuração de TTS (Voz) ---");
    }

    let tts_providers = vec!["piper", "elevenlabs"];
    let tts_prov_lbl = if is_english {
        "TTS Provider:"
    } else {
        "Provedor de TTS:"
    };

    let tts_provider = Select::new(tts_prov_lbl, tts_providers)
        .prompt()?
        .to_string();

    let piper_path_lbl = if is_english {
        "Path to Piper executable binary:"
    } else {
        "Caminho para o binário executável do Piper:"
    };

    let piper_path = if tts_provider == "piper" {
        Some(Text::new(piper_path_lbl).with_default("piper").prompt()?)
    } else {
        None
    };

    let int_voice_lbl = if is_english {
        "Interviewer voice model / Voice ID:"
    } else {
        "Voz do(a) Entrevistador(a):"
    };

    let interviewer_voice = Text::new(int_voice_lbl)
        .with_default(if tts_provider == "piper" {
            "pt_BR-faber-medium.onnx"
        } else {
            "21m00Tcm4TlvDq8ikWAM"
        })
        .prompt()?;

    let spec_voice_lbl = if is_english {
        "Interviewee / Specialist voice model / Voice ID:"
    } else {
        "Voz do(a) Entrevistado(a) / Especialista:"
    };

    let specialist_voice = Text::new(spec_voice_lbl)
        .with_default(if tts_provider == "piper" {
            "pt_BR-carlos-medium.onnx"
        } else {
            "AZnzlk1XvdvUeBnXmlld"
        })
        .prompt()?;

    let elevenlabs_key_lbl = if is_english {
        "Environment variable with ElevenLabs key:"
    } else {
        "Variável de ambiente com chave ElevenLabs:"
    };

    let elevenlabs_api_key_env = if tts_provider == "elevenlabs" {
        Some(
            Text::new(elevenlabs_key_lbl)
                .with_default("ELEVENLABS_API_KEY")
                .prompt()?,
        )
    } else {
        None
    };

    // Personas Setup
    if is_english {
        println!("\n--- Personas Configuration ---");
    } else {
        println!("\n--- Configuração das Personas ---");
    }

    let int_name_lbl = if is_english {
        "Interviewer name:"
    } else {
        "Nome do(a) Entrevistador(a):"
    };

    let interviewer_name = Text::new(int_name_lbl)
        .with_default(if is_english {
            "Interviewer"
        } else {
            "Entrevistador"
        })
        .prompt()?;

    let available_moods = get_available_moods();

    let int_mood_lbl = if is_english {
        "Interviewer mood:"
    } else {
        "Mood do(a) Entrevistador(a):"
    };

    let interviewer_mood = Select::new(int_mood_lbl, available_moods.clone())
        .prompt()?
        .to_string();

    let spec_name_lbl = if is_english {
        "Interviewee / Specialist name:"
    } else {
        "Nome do(a) Entrevistado(a) / Especialista:"
    };

    let specialist_name = Text::new(spec_name_lbl)
        .with_default(if is_english {
            "Specialist"
        } else {
            "Especialista"
        })
        .prompt()?;

    let spec_mood_lbl = if is_english {
        "Interviewee / Specialist mood:"
    } else {
        "Mood do(a) Entrevistado(a) / Especialista:"
    };

    let specialist_mood = Select::new(spec_mood_lbl, available_moods.clone())
        .prompt()?
        .to_string();

    // Defaults
    if is_english {
        println!("\n--- General Defaults ---");
    } else {
        println!("\n--- Padrões Gerais ---");
    }

    let out_dir_lbl = if is_english {
        "Episode output directory:"
    } else {
        "Diretório de saída dos episódios:"
    };

    let output_dir = Text::new(out_dir_lbl).with_default("./episodes").prompt()?;

    let db_path_lbl = if is_english {
        "SQLite memory database path:"
    } else {
        "Caminho do banco de memória SQLite:"
    };

    let db_path = Text::new(db_path_lbl)
        .with_default("~/.config/pheme/memory.db")
        .prompt()?;

    let duration_lbl = if is_english {
        "Default episode duration:"
    } else {
        "Duração padrão dos episódios:"
    };

    let duration_options = if is_english {
        vec!["short", "medium", "long"]
    } else {
        vec!["curto", "medio", "longo"]
    };

    let duration_preset = Select::new(duration_lbl, duration_options)
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
                domain: None,
                mood: interviewer_mood,
            },
            specialist: PersonaEntryConfig {
                name: specialist_name,
                domain: None,
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

    if is_english {
        println!(
            "\nConfiguration successfully saved at: {:?}\nYou can now run `pheme generate <URL>`!",
            target_path
        );
    } else {
        println!(
            "\nConfiguração salva com sucesso em: {:?}\nVocê já pode executar `pheme generate <URL>`!",
            target_path
        );
    }

    Ok(())
}
