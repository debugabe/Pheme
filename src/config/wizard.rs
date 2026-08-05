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
        "Select setup mode / Selecione o modo de execução:"
    } else {
        "Selecione o modo de execução:"
    };

    let mode_options = if is_english {
        vec!["API KEY (Cloud / Remote API)", "LOCAL (Offline / Local)"]
    } else {
        vec!["API KEY (Nuvem / Remote API)", "LOCAL (Offline / Local)"]
    };

    let chosen_mode = Select::new(mode_lbl, mode_options)
        .prompt()
        .context("Failed to select setup mode")?;

    let is_cloud_mode = chosen_mode.contains("API KEY") || chosen_mode.contains("Nuvem");

    // 1. LLM Setup
    let (llm_provider, llm_base_url, llm_api_key_env, llm_model) = if is_cloud_mode {
        if is_english {
            println!("\n--- API Provider LLM Setup (Script Generation) ---");
        } else {
            println!("\n--- Configuração de Provedor de API LLM (Roteiro) ---");
        }

        let provider_choices = vec![
            "OpenAI (api.openai.com)",
            "OpenRouter (openrouter.ai)",
            "Groq (api.groq.com)",
            "DeepSeek (api.deepseek.com)",
            "Ollama Remote (Custom URL + API Key)",
            "Anthropic (api.anthropic.com)",
            "Custom OpenAI-Compatible Endpoint",
        ];

        let chosen_p = Select::new(
            if is_english {
                "Select API Service:"
            } else {
                "Selecione o Serviço de API:"
            },
            provider_choices,
        )
        .prompt()?;

        match chosen_p {
            "OpenAI (api.openai.com)" => {
                let model = Text::new(if is_english {
                    "OpenAI Model:"
                } else {
                    "Modelo da OpenAI:"
                })
                .with_default("gpt-4o-mini")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OPENAI_API_KEY")
                .prompt()?;
                (
                    "openai_compatible".to_string(),
                    Some("https://api.openai.com/v1".to_string()),
                    Some(env_var),
                    model,
                )
            }
            "OpenRouter (openrouter.ai)" => {
                let model = Text::new(if is_english {
                    "OpenRouter Model:"
                } else {
                    "Modelo do OpenRouter:"
                })
                .with_default("anthropic/claude-3.5-sonnet")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OPENROUTER_API_KEY")
                .prompt()?;
                (
                    "openai_compatible".to_string(),
                    Some("https://openrouter.ai/api/v1".to_string()),
                    Some(env_var),
                    model,
                )
            }
            "Groq (api.groq.com)" => {
                let model = Text::new(if is_english {
                    "Groq Model:"
                } else {
                    "Modelo no Groq:"
                })
                .with_default("llama-3.3-70b-versatile")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("GROQ_API_KEY")
                .prompt()?;
                (
                    "openai_compatible".to_string(),
                    Some("https://api.groq.com/openai/v1".to_string()),
                    Some(env_var),
                    model,
                )
            }
            "DeepSeek (api.deepseek.com)" => {
                let model = Text::new(if is_english {
                    "DeepSeek Model:"
                } else {
                    "Modelo no DeepSeek:"
                })
                .with_default("deepseek-chat")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("DEEPSEEK_API_KEY")
                .prompt()?;
                (
                    "openai_compatible".to_string(),
                    Some("https://api.deepseek.com/v1".to_string()),
                    Some(env_var),
                    model,
                )
            }
            "Ollama Remote (Custom URL + API Key)" => {
                let base_url = Text::new(if is_english {
                    "Ollama Remote Base URL:"
                } else {
                    "URL Base do Ollama Remoto:"
                })
                .with_default("https://your-ollama-host:11434")
                .prompt()?;
                let model = Text::new(if is_english {
                    "Ollama Model:"
                } else {
                    "Modelo no Ollama:"
                })
                .with_default("llama3")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "Ollama API Key Env Var:"
                } else {
                    "Variável da Chave de API Ollama:"
                })
                .with_default("OLLAMA_API_KEY")
                .prompt()?;
                (
                    "ollama_native".to_string(),
                    Some(base_url),
                    Some(env_var),
                    model,
                )
            }
            "Anthropic (api.anthropic.com)" => {
                let model = Text::new(if is_english {
                    "Anthropic Model:"
                } else {
                    "Modelo da Anthropic:"
                })
                .with_default("claude-3-5-sonnet-20241022")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("ANTHROPIC_API_KEY")
                .prompt()?;
                (
                    "anthropic".to_string(),
                    Some("https://api.anthropic.com".to_string()),
                    Some(env_var),
                    model,
                )
            }
            _ => {
                let base_url = Text::new(if is_english { "Base URL:" } else { "URL Base:" })
                    .with_default("https://api.openai.com/v1")
                    .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OPENAI_API_KEY")
                .prompt()?;
                let model = Text::new(if is_english {
                    "Model Name:"
                } else {
                    "Nome do Modelo:"
                })
                .prompt()?;
                (
                    "openai_compatible".to_string(),
                    Some(base_url),
                    Some(env_var),
                    model,
                )
            }
        }
    } else {
        if is_english {
            println!("\n--- Local LLM Configuration (Ollama Local) ---");
        } else {
            println!("\n--- Configuração de LLM Local (Ollama Local) ---");
        }

        let base_url = Text::new(if is_english {
            "Ollama Local Base URL:"
        } else {
            "URL Base do Ollama Local:"
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
            println!("\n--- Embedding Setup ---");
        } else {
            println!("\n--- Configuração de Embedding ---");
        }

        let emb_choices = vec![
            "OpenAI Embedding (text-embedding-3-small)",
            "Ollama Remote Embedding (nomic-embed-text)",
            "Custom OpenAI-Compatible Embedding",
        ];

        let chosen_e = Select::new(
            if is_english {
                "Select Embedding Provider:"
            } else {
                "Selecione o Provedor de Embedding:"
            },
            emb_choices,
        )
        .prompt()?;

        match chosen_e {
            "OpenAI Embedding (text-embedding-3-small)" => {
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OPENAI_API_KEY")
                .prompt()?;
                (
                    "openai".to_string(),
                    Some("https://api.openai.com/v1".to_string()),
                    Some(env_var),
                    "text-embedding-3-small".to_string(),
                )
            }
            "Ollama Remote Embedding (nomic-embed-text)" => {
                let base_url = Text::new(if is_english {
                    "Ollama Remote Base URL:"
                } else {
                    "URL Base do Ollama Remoto:"
                })
                .with_default("https://your-ollama-host:11434")
                .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OLLAMA_API_KEY")
                .prompt()?;
                (
                    "ollama".to_string(),
                    Some(base_url),
                    Some(env_var),
                    "nomic-embed-text".to_string(),
                )
            }
            _ => {
                let base_url = Text::new(if is_english { "Base URL:" } else { "URL Base:" })
                    .with_default("https://api.openai.com/v1")
                    .prompt()?;
                let env_var = Text::new(if is_english {
                    "API Key Env Var:"
                } else {
                    "Variável da Chave de API:"
                })
                .with_default("OPENAI_API_KEY")
                .prompt()?;
                let model = Text::new(if is_english {
                    "Embedding Model Name:"
                } else {
                    "Nome do Modelo de Embedding:"
                })
                .with_default("text-embedding-3-small")
                .prompt()?;
                ("openai".to_string(), Some(base_url), Some(env_var), model)
            }
        }
    } else {
        if is_english {
            println!("\n--- Local Embedding Configuration (Ollama Local) ---");
        } else {
            println!("\n--- Configuração de Embedding Local (Ollama Local) ---");
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
