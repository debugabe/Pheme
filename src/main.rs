use anyhow::Result;
use clap::Parser;
use colored::*;
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

            println!(
                "{} {}",
                "Starting Pheme pipeline for URL:".bold().cyan(),
                url.yellow().underline()
            );
            let generator = EpisodeGenerator::new(cfg)?;
            let episode_path = generator.run_pipeline(&url).await?;
            println!(
                "\n{}\n   {}",
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
            let selected_lang = Config::load()
                .map(|c| c.language)
                .unwrap_or_else(|_| "pt-BR".into());
            print_help_guide(&selected_lang);
        }
    }

    Ok(())
}
