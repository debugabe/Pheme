use anyhow::Result;
use clap::Parser;
use colored::*;
use inquire::{Select, Text};
use pheme::cli::{print_banner, print_help_guide, Cli, Commands};
use pheme::config::{self, Config};
use pheme::episode::EpisodeGenerator;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init) => {
            print_banner();
            config::wizard::run_wizard()?;
        }
        Some(Commands::Generate { url, duration }) => {
            print_banner();
            let mut cfg = Config::load()?;
            if let Some(dur) = duration {
                cfg.defaults.duration_preset = dur;
            }

            let generator = EpisodeGenerator::new(cfg)?;
            let episode_path = generator.run_pipeline(&url).await?;
            println!(
                "\n{}\n   {}\n",
                "Episode successfully generated at:".green().bold(),
                episode_path.display().to_string().yellow()
            );
        }
        Some(Commands::Config { show }) => {
            print_banner();
            let cfg = Config::load()?;
            if show {
                let toml_str = toml::to_string_pretty(&cfg)?;
                println!(
                    "{}\n{}",
                    "--- Active Configuration ---".bold().cyan(),
                    toml_str.bright_black()
                );
            } else {
                let path = Config::find_config_file()?;
                println!(
                    "{} {:?}",
                    "Valid configuration found at:".green().bold(),
                    path
                );
            }
        }
        Some(Commands::Help { lang }) => {
            let selected_lang = lang
                .or_else(|| Config::load().map(|c| c.language).ok())
                .unwrap_or_else(|| "pt-BR".to_string());
            print_help_guide(&selected_lang);
        }
        None => {
            print_banner();
            let cfg = Config::load().ok();
            let lang = cfg.as_ref().map(|c| c.language.as_str()).unwrap_or("pt-BR");
            let is_english = lang.starts_with("en");

            let options = if is_english {
                vec![
                    "1. Generate Podcast Episode (Enter URL)",
                    "2. Run Setup Wizard (pheme init)",
                    "3. View Active Configuration (pheme config)",
                    "4. View Command Guide (pheme help)",
                    "5. Exit",
                ]
            } else {
                vec![
                    "1. Gerar Episódio de Podcast (Digitar URL)",
                    "2. Executar Assistente de Configuração (pheme init)",
                    "3. Visualizar Configuração Ativa (pheme config)",
                    "4. Guia Completo de Comandos (pheme help)",
                    "5. Sair",
                ]
            };

            let choice = Select::new(
                if is_english {
                    "Select an action:"
                } else {
                    "Selecione uma ação:"
                },
                options,
            )
            .prompt()?;

            if choice.starts_with("1") {
                let url = Text::new(if is_english {
                    "Enter news article URL:"
                } else {
                    "Digite a URL da notícia:"
                })
                .prompt()?;

                if !url.trim().is_empty() {
                    let active_cfg = Config::load()?;
                    let generator = EpisodeGenerator::new(active_cfg)?;
                    let episode_path = generator.run_pipeline(url.trim()).await?;
                    println!(
                        "\n{}\n   {}\n",
                        "Episode successfully generated at:".green().bold(),
                        episode_path.display().to_string().yellow()
                    );
                }
            } else if choice.starts_with("2") {
                config::wizard::run_wizard()?;
            } else if choice.starts_with("3") {
                let active_cfg = Config::load()?;
                let toml_str = toml::to_string_pretty(&active_cfg)?;
                println!(
                    "{}\n{}",
                    "--- Active Configuration ---".bold().cyan(),
                    toml_str.bright_black()
                );
            } else if choice.starts_with("4") {
                print_help_guide(lang);
            }
        }
    }

    Ok(())
}
