use anyhow::Result;
use chrono::Utc;
use clap::Parser;
use pheme::audio::{concatenate_wav_buffers, AudioTechnicalReviewer};
use pheme::cli::{Cli, Commands};
use pheme::memory::MemoryStore;
use pheme::news::{Article, NewsReviewer};
use pheme::personas::{Persona, Role};
use tempfile::tempdir;

#[test]
fn test_cli_parsing_english_commands() {
    let args = vec![
        "pheme",
        "generate",
        "https://example.com/news",
        "--duration",
        "short",
    ];
    let cli = Cli::try_parse_from(args).unwrap();
    if let Some(Commands::Generate { url, duration }) = cli.command {
        assert_eq!(url, "https://example.com/news");
        assert_eq!(duration, Some("short".into()));
    } else {
        panic!("Esperava comando Generate");
    }

    // Alias 'gen'
    let args_alias = vec!["pheme", "gen", "https://example.com/news"];
    let cli_alias = Cli::try_parse_from(args_alias).unwrap();
    assert!(matches!(cli_alias.command, Some(Commands::Generate { .. })));

    // Alias 'g'
    let args_g = vec!["pheme", "g", "https://example.com/news"];
    let cli_g = Cli::try_parse_from(args_g).unwrap();
    assert!(matches!(cli_g.command, Some(Commands::Generate { .. })));

    // Comando 'help' e alias '/help'
    let args_help = vec!["pheme", "/help"];
    let cli_help = Cli::try_parse_from(args_help).unwrap();
    assert!(matches!(cli_help.command, Some(Commands::Help { .. })));

    // Sem subcomandos (apenas `pheme`)
    let args_bare = vec!["pheme"];
    let cli_bare = Cli::try_parse_from(args_bare).unwrap();
    assert!(cli_bare.command.is_none());
}

#[test]
fn test_persona_axes_generation() {
    let persona = Persona {
        name: "Alex".into(),
        role: Role::Interviewer,
        mood: "cetico".into(),
    };

    let prompt = persona.build_prompt_instructions();
    assert!(prompt.contains("Alex"));
    assert!(prompt.contains("Ceticismo vs Entusiasmo"));
}

#[test]
fn test_memory_store_end_to_end() -> Result<()> {
    let dir = tempdir()?;
    let db_path = dir.path().join("test_memory.db");

    let store = MemoryStore::open(&db_path)?;

    let embedding1 = vec![0.1, 0.2, 0.9, 0.4];
    let embedding2 = vec![0.1, 0.25, 0.85, 0.38];
    let embedding3 = vec![-0.9, 0.0, 0.1, -0.2];

    store.save_episode(
        "Episódio 1 - IA",
        "Resumo sobre inteligência artificial",
        &["IA".into()],
        Utc::now(),
        &embedding1,
    )?;
    store.save_episode(
        "Episódio 2 - Robótica",
        "Resumo sobre robôs",
        &["Robôs".into()],
        Utc::now(),
        &embedding2,
    )?;
    store.save_episode(
        "Episódio 3 - Culinária",
        "Receita de bolo",
        &["Comida".into()],
        Utc::now(),
        &embedding3,
    )?;

    let query_embedding = vec![0.1, 0.2, 0.88, 0.4];
    let similar = store.find_similar_episodes(&query_embedding, 0.6, 10)?;

    assert_eq!(similar.len(), 2);
    assert_eq!(similar[0].0.title, "Episódio 1 - IA");
    assert_eq!(similar[1].0.title, "Episódio 2 - Robótica");

    Ok(())
}

#[test]
fn test_audio_concatenation_and_review() -> Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 22050,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor1 = std::io::Cursor::new(Vec::new());
    let mut writer1 = hound::WavWriter::new(&mut cursor1, spec)?;
    for i in 0..11025 {
        writer1.write_sample((i % 50) as i16 * 100)?;
    }
    writer1.finalize()?;
    let buf1 = cursor1.into_inner();

    let mut cursor2 = std::io::Cursor::new(Vec::new());
    let mut writer2 = hound::WavWriter::new(&mut cursor2, spec)?;
    for i in 0..11025 {
        writer2.write_sample((i % 50) as i16 * 100)?;
    }
    writer2.finalize()?;
    let buf2 = cursor2.into_inner();

    let merged = concatenate_wav_buffers(&[buf1, buf2], 300)?;

    let report = AudioTechnicalReviewer::review_wav_buffer(&merged, 5)?;
    assert!(report.is_valid);
    assert!(report.duration_seconds >= 1.25 && report.duration_seconds <= 1.35);

    Ok(())
}

#[test]
fn test_news_reviewer_validation() {
    let valid_art = Article {
        url: "https://tech.example.com/ai-update".into(),
        title: "Atualização no Setor de IA".into(),
        content: "Engenheiros apresentaram hoje uma nova arquitetura para redes neurais que reduz consumo de energia em 40%. A novidade foi amplamente elogiada pela comunidade científica internacional.".into(),
        published_at: Some(Utc::now()),
    };

    let report = NewsReviewer::review_article(&valid_art);
    assert!(report.is_valid);
    assert_eq!(report.score, 1.0);

    let paywalled_art = Article {
        url: "https://paywall.example.com".into(),
        title: "Notícia Bloqueada".into(),
        content: "Para continuar lendo este artigo exclusivo, assine para ler na nossa plataforma."
            .into(),
        published_at: None,
    };

    let report_paywall = NewsReviewer::review_article(&paywalled_art);
    assert!(!report_paywall.is_valid);
}
