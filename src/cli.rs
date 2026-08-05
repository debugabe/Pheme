use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser, Debug)]
#[command(name = "pheme")]
#[command(author = "Pheme Contributors")]
#[command(version = "0.1.5")]
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

// Crimson Red Color: RGB (220, 20, 60)
fn crimson<T: AsRef<str>>(s: T) -> ColoredString {
    s.as_ref().truecolor(220, 20, 60)
}

pub fn print_banner() {
    let title_art = r#"
.______   __    __   _______ .___  ___.  _______ 
|   _  \ |  |  |  | |   ____||   \/   | |   ____|
|  |_)  ||  |__|  | |  |__   |  \  /  | |  |__   
|   ___/ |   __   | |   __|  |  |\/|  | |   __|  
|  |     |  |  |  | |  |____ |  |  |  | |  |____ 
| _|     |__|  |__| |_______||__|  |__| |_______|
"#;

    println!("{}", crimson(title_art).bold());
}

pub fn print_help_guide(lang: &str) {
    print_banner();

    let is_english = lang.starts_with("en");

    // Arte ASCII derivada da imagem pheme.png
    let emblem = vec![
        r#"########################"#,
        r#"################## @####"#,
        r#"###########@@@###  ##  @"#,
        r#"### ####* % # ####.@  ##"#,
        r#"###       #  @  ## #@###"#,
        r#"### #  ## #@@%# @@ #####"#,
        r#"### # ##@@@@@##%@@#@####"#,
        r#"### -  #@# ###@@@   ####"#,
        r#"####@#  =@@@#@@# #@ ####"#,
        r#"###@#@@@ #=  #    ######"#,
        r#"      .------.          "#,
        r#"     (  AUDIO AI )      "#,
    ];

    let version_header = " Pheme v0.1.5 (2026) ";

    let box_width = 90;
    let header_fill = box_width - 3 - version_header.len();
    let top_border = format!("┌{} {}┐", "─".repeat(header_fill), version_header);

    println!("{}", crimson(&top_border).bold());

    let mut right_lines: Vec<ColoredString> = Vec::new();

    if is_english {
        right_lines.push(crimson("Available Commands").bold());
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme generate <URL>".yellow().bold(),
                "Generate audio podcast from news URL".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme init".yellow().bold(),
                "Interactive setup wizard for LLM/TTS/Personas".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme config [--show]".yellow().bold(),
                "Validate or print active configuration".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme help / /help".yellow().bold(),
                "Display this interactive commands guide".white()
            )
            .into(),
        );
        right_lines.push("".into());
        right_lines.push(crimson("Supported Engines & Features").bold());
        right_lines.push(
            format!(
                "  {} : OpenAI, OpenRouter, Groq, DeepSeek, Anthropic, Ollama",
                "LLM Providers".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : Piper (Local ONNX), ElevenLabs (Cloud API)",
                "TTS Providers".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : SQLite vector database with cosine similarity",
                "Memory Engine".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : News Reviewer, Script Fidelity & Technical Audio Audit",
                "Safety Reviewers".yellow()
            )
            .into(),
        );
    } else {
        right_lines.push(crimson("Comandos Disponíveis").bold());
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme generate <URL>".yellow().bold(),
                "Gera podcast em áudio a partir de notícia".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme init".yellow().bold(),
                "Assistente de configuração interativo de LLM/TTS".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme config [--show]".yellow().bold(),
                "Valida ou exibe as definições ativas do TOML".white()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : {}",
                "pheme help / /help".yellow().bold(),
                "Exibe este guia interativo de comandos".white()
            )
            .into(),
        );
        right_lines.push("".into());
        right_lines.push(crimson("Provedores e Recursos").bold());
        right_lines.push(
            format!(
                "  {} : OpenAI, OpenRouter, Groq, DeepSeek, Anthropic, Ollama",
                "Provedores LLM".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : Piper (Local ONNX), ElevenLabs (API Nuvem)",
                "Provedores TTS".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : Banco de vetores SQLite com similaridade de cosseno",
                "Memória Vetorial".yellow()
            )
            .into(),
        );
        right_lines.push(
            format!(
                "  {} : Revisor de Notícias, Fidelidade de Roteiro e Auditoria WAV",
                "Revisores de Qualidade".yellow()
            )
            .into(),
        );
    }

    let max_rows = emblem.len().max(right_lines.len());
    let empty_line = "".into();

    for i in 0..max_rows {
        let left_part = if i < emblem.len() {
            emblem[i]
        } else {
            "                        "
        };

        let right_part = if i < right_lines.len() {
            &right_lines[i]
        } else {
            &empty_line
        };

        println!(
            "{} {:<24} │ {:<58} {}",
            crimson("│"),
            crimson(left_part).bold(),
            right_part,
            crimson("│")
        );
    }

    let bottom_border = format!("└{}┘", "─".repeat(box_width - 2));
    println!("{}\n", crimson(&bottom_border).bold());
}
