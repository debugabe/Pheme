# Guia de Contribuição — Pheme

Agradecemos o seu interesse em contribuir para o **Pheme**! Este projeto é 100% código aberto sob a licença MIT e busca criar a melhor ferramenta CLI para transformação de notícias em podcasts educativos.

---

## Princípios Arquiteturais do Projeto

- **Monolito Modular em Rust**: Toda a aplicação roda em um único binário compilado.
- **Traits de Abstração Plugáveis**:
  - `LlmProvider` (`src/llm/mod.rs`): Para provedores de roteiro.
  - `EmbeddingProvider` (`src/embeddings/mod.rs`): Para geração de vetores semânticos.
  - `TtsProvider` (`src/tts/mod.rs`): Para síntese de áudio.
- **Isolamento de Conhecimento**: O módulo `episode` é o único orquestrador que conhece todas as camadas. Módulos individuais de TTS e LLM não conversam diretamente com a memória.
- **Erros Explícitos**: Não faça suposições silenciosas em campos de configuração ausentes.

---

## Como Adicionar um Novo Provedor

### 1. Novo Provedor de LLM
- Se o serviço segue a especificação OpenAI (`/chat/completions`), basta usar a classe `OpenAiCompatibleProvider` alterando a `base_url` no `pheme.toml`.
- Se o serviço exige payload/headers específicos, crie um novo arquivo em `src/llm/<provedor>.rs`, implemente o trait `LlmProvider` e registre a variante em `src/llm/mod.rs` e `src/episode/mod.rs`.

### 2. Novo Provedor de TTS
- Crie um arquivo em `src/tts/<provedor>.rs`.
- Implemente `TtsProvider` assegurando o retorno de um buffer contendo áudio em formato **WAV**.

---

## Rodando Testes Localmente

Antes de abrir um Pull Request, certifique-se de que todos os testes passem:

```bash
cargo fmt --all -- --check
cargo clippy -- -D warnings
cargo test
```

---

## Fluxo de Pull Request

1. Faça um Fork do repositório.
2. Crie uma branch para sua funcionalidade: `git checkout -b minha-feature`.
3. Escreva testes para cobrir suas alterações.
4. Faça commit das mudanças com mensagens claras.
5. Envie um Pull Request apontando para a branch `main`.

Muito obrigado por ajudar a evoluir o Pheme!
