use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser, Debug)]
#[command(name = "pheme")]
#[command(author = "Pheme Contributors")]
#[command(version = "0.1.3")]
#[command(about = "AI Audio Podcast Generator from News with Semantic Memory", long_about = None)]
#[command(disable_help_subcommand = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize configuration interactively
    Init,

    /// Generate a new podcast episode from a news article URL
    #[command(alias = "gen", alias = "g", alias = "gerar")]
    Generate {
        /// News article URL
        #[arg(required = true)]
        url: String,

        /// Optional duration preset: short, medium, long (or curto, medio, longo)
        #[arg(short, long)]
        duration: Option<String>,
    },

    /// Show or validate active configuration
    Config {
        /// Display active configuration in TOML format
        #[arg(short, long)]
        show: bool,
    },

    /// Display detailed command help in configured language (or Portuguese/English)
    #[command(name = "help", alias = "/help")]
    Help {
        /// Specify language explicitly: pt-BR or en-US
        #[arg(short, long)]
        lang: Option<String>,
    },
}

pub fn print_banner() {
    let banner = r#"
  ____  _   _ _____ __  __ _____ 
 |  _ \| | | | ____|  \/  | ____|
 | |_) | |_| |  _| | |\/| |  _|  
 |  __/|  _  | |___| |  | | |___ 
 |_|   |_| |_|_____|_|  |_|_____|
"#;
    println!("{}", banner.cyan().bold());
    println!(
        "  {}\n",
        "AI Audio Podcast Generator from News & Semantic Memory"
            .bright_black()
            .italic()
    );
}

pub fn print_help_guide(lang: &str) {
    print_banner();

    if lang.starts_with("en") {
        println!(
            "{}",
            "Pheme CLI — Available Commands Guide".bold().underline()
        );
        println!();
        println!("  1. {}", "pheme init".cyan().bold());
        println!(
            "     {}",
            "Launches an interactive wizard to configure LLM, TTS, Embeddings, Personas and language.".bright_black()
        );
        println!();
        println!(
            "  2. {} {} [{}]",
            "pheme generate".cyan().bold(),
            "<URL>".yellow(),
            "--duration short|medium|long".bright_black()
        );
        println!(
            "     {}",
            "Generates a complete podcast episode from a news article URL.".bright_black()
        );
        println!(
            "     {}",
            "Aliases: pheme gen <URL>, pheme g <URL>".dimmed()
        );
        println!();
        println!(
            "  3. {} [{}]",
            "pheme config".cyan().bold(),
            "--show".yellow()
        );
        println!(
            "     {}",
            "Validates active configuration file or displays full TOML settings.".bright_black()
        );
        println!();
        println!(
            "  4. {} [{}] (or {})",
            "pheme help".cyan().bold(),
            "--lang en|pt".yellow(),
            "pheme /help".cyan()
        );
        println!(
            "     {}\n",
            "Displays this comprehensive command guide.".bright_black()
        );
    } else {
        println!(
            "{}",
            "Pheme CLI — Guia Completo de Comandos".bold().underline()
        );
        println!();
        println!("  1. {}", "pheme init".cyan().bold());
        println!(
            "     {}",
            "Inicia o assistente interativo para configurar LLM, TTS, Embeddings, Personas e idioma.".bright_black()
        );
        println!();
        println!(
            "  2. {} {} [{}]",
            "pheme generate".cyan().bold(),
            "<URL>".yellow(),
            "--duration curto|medio|longo".bright_black()
        );
        println!(
            "     {}",
            "Gera um episódio completo de podcast a partir da URL de uma notícia.".bright_black()
        );
        println!(
            "     {}",
            "Atalhos: pheme gen <URL>, pheme g <URL>, pheme gerar <URL>".dimmed()
        );
        println!();
        println!(
            "  3. {} [{}]",
            "pheme config".cyan().bold(),
            "--show".yellow()
        );
        println!(
            "     {}",
            "Valida o arquivo de configuração ativo ou exibe todas as definições TOML."
                .bright_black()
        );
        println!();
        println!(
            "  4. {} [{}] (ou {})",
            "pheme help".cyan().bold(),
            "--lang en|pt".yellow(),
            "pheme /help".cyan()
        );
        println!(
            "     {}\n",
            "Exibe este guia detalhado de comandos.".bright_black()
        );
    }
}
