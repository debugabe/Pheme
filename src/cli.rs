use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pheme")]
#[command(author = "Pheme Contributors")]
#[command(version = "0.1.0")]
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

pub fn print_help_guide(lang: &str) {
    if lang.starts_with("en") {
        println!(
            "Pheme CLI — Available Commands Guide\n\n\
            1. pheme init\n\
               Launches an interactive wizard to configure LLM, TTS, Embeddings, Personas and language.\n\n\
            2. pheme generate <URL> [--duration short|medium|long]\n\
               Generates a complete podcast episode from a news article URL.\n\
               Aliases: `pheme gen <URL>`, `pheme g <URL>`\n\n\
            3. pheme config [--show]\n\
               Validates active configuration file or displays full TOML settings.\n\n\
            4. pheme help [--lang en|pt] (or `pheme /help`)\n\
               Displays this comprehensive command guide."
        );
    } else {
        println!(
            "Pheme CLI — Guia Completo de Comandos\n\n\
            1. pheme init\n\
               Inicia o assistente interativo para configurar LLM, TTS, Embeddings, Personas e idioma.\n\n\
            2. pheme generate <URL> [--duration curto|medio|longo]\n\
               Gera um episódio completo de podcast a partir da URL de uma notícia.\n\
               Atalhos: `pheme gen <URL>`, `pheme g <URL>`, `pheme gerar <URL>`\n\n\
            3. pheme config [--show]\n\
               Valida o arquivo de configuração ativo ou exibe todas as definições TOML.\n\n\
            4. pheme help [--lang en|pt] (ou `pheme /help`)\n\
               Exibe este guia detalhado de comandos."
        );
    }
}
