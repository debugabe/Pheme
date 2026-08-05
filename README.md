# Pheme

100% Rust CLI tool to generate audio podcast episodes from tech news using AI personas and semantic vector memory.

[![Crates.io](https://img.shields.io/crates/v/pheme.svg)](https://crates.io/crates/pheme)
[![CI](https://github.com/usuario/pheme/actions/workflows/ci.yml/badge.svg)](https://github.com/usuario/pheme/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/language-Rust-orange.svg)](https://www.rust-lang.org/)

---

## Features

- **100% Pure Rust**: Zero heavy runtime dependencies (pure WAV audio concatenation via `hound`).
- **Agnostic LLM Layer**: Supports OpenAI, OpenRouter, Groq, DeepSeek, Together, Anthropic Messages API, and local Ollama.
- **Agnostic TTS Layer**: Fast local synthesis via **Piper** (`.onnx`) and cloud synthesis via **ElevenLabs** API.
- **Semantic Vector Memory**: SQLite storage (`rusqlite`) with vector embeddings (`text-embedding-3-small` or `nomic-embed-text`) and cosine similarity search.
- **Quality & Anti-Hallucination Reviewers**:
  - **News Reviewer**: Detects paywalls, empty articles, and scraping errors.
  - **Fidelity Reviewer**: Audits JSON script alignment against source article.
  - **Audio Technical Reviewer**: Validates silence, duration, and sampling rate.

---

## Installation

Install directly from **crates.io**:

```bash
cargo install pheme
```

Or build from source:

```bash
git clone https://github.com/usuario/pheme.git
cd pheme
cargo build --release
```

---

## Usage & Commands

Executing `pheme` without arguments displays the interactive command guide:

```bash
pheme
```

### 1. Interactive Setup (`pheme init`)
Launches the interactive setup wizard:

```bash
pheme init
```

Generates your active configuration file at `~/.config/pheme/pheme.toml`.

### 2. Generate Podcast Episode (`pheme generate`)
Pass any news article URL:

```bash
pheme generate "https://example.com/tech-news" --duration medium
# Aliases: `pheme gen`, `pheme g`, `pheme gerar`
```

Duration presets: `short`, `medium` (default), `long`.

### 3. Check Configuration (`pheme config`)

```bash
pheme config --show
```

### 4. Interactive Help (`pheme help` or `pheme /help`)

```bash
pheme /help
# Or force language:
pheme help --lang en
pheme help --lang pt
```

---

## Publishing to Crates.io

To publish a new version to crates.io:

```bash
cargo login <your-crates-io-token>
cargo publish
```

---

## Contributing

Contributions are welcome! Read [`CONTRIBUTING.md`](CONTRIBUTING.md) for architecture details.

---

## License

Distributed under the MIT License. See [`LICENSE`](LICENSE) for details.
