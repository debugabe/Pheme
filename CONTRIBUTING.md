# Contributing to Pheme

Thank you for your interest in contributing to **Pheme**! This project is 100% open source under the MIT License and aims to build the best CLI tool for transforming tech news articles into engaging audio podcasts.

---

## Architectural Guidelines

- **Modular Monolith in Rust**: The entire application runs as a single compiled binary crate.
- **Pluggable Trait Abstractions**:
  - `LlmProvider` (`src/llm/mod.rs`): Script generation providers.
  - `EmbeddingProvider` (`src/embeddings/mod.rs`): Semantic vector generation.
  - `TtsProvider` (`src/tts/mod.rs`): Audio synthesis.
- **Strict Isolation**: The `episode` module is the single orchestrator aware of all layers. Individual LLM, TTS, or news modules do not cross-depend or directly touch memory storage.
- **Explicit Failures**: Do not silently assume missing required configuration fields. Return descriptive errors.

---

## How to Add a New Provider

### 1. New LLM Provider
- If the provider follows OpenAI's specification (`/chat/completions`), use `OpenAiCompatibleProvider` and adjust `base_url` in `pheme.toml`.
- If custom API payloads or headers are required, create a file at `src/llm/<provider>.rs`, implement `LlmProvider`, and register it in `src/llm/mod.rs` and `src/episode/mod.rs`.

### 2. New TTS Provider
- Create a file at `src/tts/<provider>.rs`.
- Implement `TtsProvider`, ensuring the output is a valid **WAV** byte buffer.

---

## Running Tests Locally

Before submitting a Pull Request, ensure formatting, linting, and all tests pass:

```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
```

---

## Pull Request Workflow

1. Fork the repository.
2. Create a feature branch: `git checkout -b my-feature`.
3. Add tests covering your changes.
4. Commit your changes with clear messages.
5. Submit a Pull Request targeting the `main` branch.

Thank you for helping improve Pheme!
