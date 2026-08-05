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

    let mode_lbl = if is_english {
        "Select setup mode / Selecione o modo de configuração:"
    } else {
        "Selecione o modo de configuração:"
    };

    let mode_options = if is_english {
        vec!["API KEY (Cloud / Nuvem)", "LOCAL (Offline / Local)"]
    } else {
        vec!["API KEY (Nuvem)", "LOCAL (Offline / Local)"]
    };

    let chosen_mode = Select::new(mode_lbl, mode_options)
        .prompt()
        .context("Failed to select setup mode")?;

    let is_cloud_mode = chosen_mode.contains("API KEY") || chosen_mode.contains("Nuvem");

    // 1. LLM Setup
    let (llm_provider, llm_base_url, llm_api_key_env, llm_model) = if is_cloud_mode {
        if is_english {
            println!("\n--- Cloud LLM Configuration (Script Generation) ---");
        } else {
            println!("\n--- Configuração de LLM em Nuvem (Roteiro) ---");
        }

        let llm_providers = vec!["openai_compatible", "anthropic"];
        let llm_provider = Select::new(
            if is_english {
                "LLM Provider:"
            } else {
                "Provedor de LLM:"
            },
            llm_providers,
        )
        .prompt()?
        .to_string();

        let base_url = if llm_provider == "openai_compatible" {
            Some(
                Text::new(if is_english {
                    "LLM API Base URL:"
                } else {
                    "URL Base da API LLM:"
                })
                .with_default("https://api.openai.com/v1")
                .prompt()?,
            )
        } else {
            Some("https://api.anthropic.com".to_string())
        };

        let default_env_var = if llm_provider == "anthropic" {
            "ANTHROPIC_API_KEY"
        } else {
            "OPENAI_API_KEY"
        };

        let api_key_env = Text::new(if is_english {
            "Environment variable for API key:"
        } else {
            "Variável de ambiente para chave de API:"
        })
        .with_default(default_env_var)
        .prompt()?;

        let default_model = if llm_provider == "anthropic" {
            "claude-3-5-sonnet-20241022"
        } else {
            "gpt-4o-mini"
        };

        let model = Text::new(if is_english {
            "LLM Model:"
        } else {
            "Modelo de LLM:"
        })
        .with_default(default_model)
        .prompt()?;

        (llm_provider, base_url, Some(api_key_env), model)
    } else {
        if is_english {
            println!("\n--- Local LLM Configuration (Ollama) ---");
        } else {
            println!("\n--- Configuração de LLM Local (Ollama) ---");
        }

        let base_url = Text::new(if is_english {
            "Ollama Base URL:"
        } else {
            "URL Base do Ollama:"
        })
        .with_default("http://localhost:11434")
        .prompt()?;

        let model = Text::new(if is_english {
            "Ollama Model:"
        } else {
            "Modelo no Ollama:"
        })
        .with_default("llama3")
        .prompt()?;

        ("ollama_native".to_string(), Some(base_url), None, model)
    };

    // 2. Embedding Setup
    let (emb_provider, emb_base_url, emb_api_key_env, emb_model) = if is_cloud_mode {
        if is_english {
            println!("\n--- Cloud Embedding Configuration ---");
        } else {
            println!("\n--- Configuração de Embedding em Nuvem ---");
        }

        let base_url = Text::new(if is_english {
            "Embedding API Base URL:"
        } else {
            "URL Base do Embedding:"
        })
        .with_default("https://api.openai.com/v1")
        .prompt()?;

        let api_key_env = Text::new(if is_english {
            "Environment variable for Embedding key:"
        } else {
            "Variável de ambiente para chave de Embedding:"
        })
        .with_default("OPENAI_API_KEY")
        .prompt()?;

        let model = Text::new(if is_english {
            "Embedding Model:"
        } else {
            "Modelo de Embedding:"
        })
        .with_default("text-embedding-3-small")
        .prompt()?;

        (
            "openai".to_string(),
            Some(base_url),
            Some(api_key_env),
            model,
        )
    } else {
        if is_english {
            println!("\n--- Local Embedding Configuration (Ollama) ---");
        } else {
            println!("\n--- Configuração de Embedding Local (Ollama) ---");
        }

        let base_url = Text::new(if is_english {
            "Ollama Base URL:"
        } else {
            "URL Base do Ollama:"
        })
        .with_default("http://localhost:11434")
        .prompt()?;

        let model = Text::new(if is_english {
            "Ollama Embedding Model:"
        } else {
            "Modelo de Embedding no Ollama:"
        })
        .with_default("nomic-embed-text")
        .prompt()?;

        ("ollama".to_string(), Some(base_url), None, model)
    };

    // 3. TTS Setup
    let (tts_provider, piper_path, interviewer_voice, specialist_voice, elevenlabs_api_key_env) =
        if is_cloud_mode {
            if is_english {
                println!("\n--- Cloud TTS Configuration (ElevenLabs) ---");
            } else {
                println!("\n--- Configuração de TTS em Nuvem (ElevenLabs) ---");
            }

            let api_key_env = Text::new(if is_english {
                "Environment variable for ElevenLabs key:"
            } else {
                "Variável de ambiente com chave ElevenLabs:"
            })
            .with_default("ELEVENLABS_API_KEY")
            .prompt()?;

            let int_voice = Text::new(if is_english {
                "Interviewer Voice ID:"
            } else {
                "Voice ID do(a) Entrevistador(a):"
            })
            .with_default("21m00Tcm4TlvDq8ikWAM")
            .prompt()?;

            let spec_voice = Text::new(if is_english {
                "Interviewee Voice ID:"
            } else {
                "Voice ID do(a) Entrevistado(a):"
            })
            .with_default("AZnzlk1XvdvUeBnXmlld")
            .prompt()?;

            (
                "elevenlabs".to_string(),
                None,
                int_voice,
                spec_voice,
                Some(api_key_env),
            )
        } else {
            if is_english {
                println!("\n--- Local TTS Configuration (Piper) ---");
            } else {
                println!("\n--- Configuração de TTS Local (Piper) ---");
            }

            let p_path = Text::new(if is_english {
                "Path to Piper executable binary:"
            } else {
                "Caminho para o executável do Piper:"
            })
            .with_default("piper")
            .prompt()?;

            let int_voice = Text::new(if is_english {
                "Interviewer ONNX Voice file:"
            } else {
                "Arquivo de voz ONNX do(a) Entrevistador(a):"
            })
            .with_default("pt_BR-faber-medium.onnx")
            .prompt()?;

            let spec_voice = Text::new(if is_english {
                "Interviewee ONNX Voice file:"
            } else {
                "Arquivo de voz ONNX do(a) Entrevistado(a):"
            })
            .with_default("pt_BR-carlos-medium.onnx")
            .prompt()?;

            (
                "piper".to_string(),
                Some(p_path),
                int_voice,
                spec_voice,
                None,
            )
        };

    // 4. Personas Setup
    if is_english {
        println!("\n--- Personas Configuration ---");
    } else {
        println!("\n--- Configuração das Personas ---");
    }

    let interviewer_name = Text::new(if is_english {
        "Interviewer name:"
    } else {
        "Nome do(a) Entrevistador(a):"
    })
    .with_default(if is_english {
        "Interviewer"
    } else {
        "Entrevistador"
    })
    .prompt()?;

    let available_moods = get_available_moods();

    let interviewer_mood = Select::new(
        if is_english {
            "Interviewer mood:"
        } else {
            "Mood do(a) Entrevistador(a):"
        },
        available_moods.clone(),
    )
    .prompt()?
    .to_string();

    let specialist_name = Text::new(if is_english {
        "Interviewee / Specialist name:"
    } else {
        "Nome do(a) Entrevistado(a) / Especialista:"
    })
    .with_default(if is_english {
        "Specialist"
    } else {
        "Especialista"
    })
    .prompt()?;

    let specialist_mood = Select::new(
        if is_english {
            "Interviewee / Specialist mood:"
        } else {
            "Mood do(a) Entrevistado(a) / Especialista:"
        },
        available_moods.clone(),
    )
    .prompt()?
    .to_string();

    // 5. Defaults
    if is_english {
        println!("\n--- General Defaults ---");
    } else {
        println!("\n--- Padrões Gerais ---");
    }

    let output_dir = Text::new(if is_english {
        "Episode output directory:"
    } else {
        "Diretório de saída dos episódios:"
    })
    .with_default("./episodes")
    .prompt()?;

    let db_path = Text::new(if is_english {
        "SQLite memory database path:"
    } else {
        "Caminho do banco de memória SQLite:"
    })
    .with_default("~/.config/pheme/memory.db")
    .prompt()?;

    let duration_options = if is_english {
        vec!["short", "medium", "long"]
    } else {
        vec!["curto", "medio", "longo"]
    };

    let duration_preset = Select::new(
        if is_english {
            "Default episode duration:"
        } else {
            "Duração padrão dos episódios:"
        },
        duration_options,
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
