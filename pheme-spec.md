# Pheme — Especificação do Projeto

Projeto pessoal e open source: ferramenta CLI 100% Rust que gera episódios de
podcast (áudio) a partir de notícias de tecnologia, com dois personagens de
IA conversando — um entrevistador fixo e um especialista que muda por tema —,
com memória entre episódios via embeddings semânticos.

---

## 1. Fontes de notícias

- **Input**: qualquer link colado pelo usuário — não é fixo a um site específico.
- **Período configurável**: últimas 24h, 7 dias, 15 dias ou 30 dias.
- **Cascata de detecção de data de publicação**:
  1. **RSS/Atom autodiscovery** — procura `<link rel="alternate" type="application/rss+xml">` no HTML.
  2. **News sitemap** — tenta `sitemap-news.xml` / `sitemap.xml`.
  3. **Metadados estruturados** — `<meta property="article:published_time">` ou `datePublished` em JSON-LD.
  4. **LLM como último recurso** — lê o texto visível da página e tenta inferir a data.
  5. **Manual** — se tudo falhar, o usuário cola o texto do artigo (e a data, se souber) direto no terminal ou aponta um arquivo local.

---

## 2. Personas

- **Estrutura fixa de papéis**: sempre dois personagens — **Entrevistador(a)**
  (identidade fixa entre episódios, carrega memória e evolui tecnicamente) e
  **Entrevistado** (especialista, escolhido conforme o tema do dia).
- Cada episódio tem exatamente **um** entrevistador e **um** especialista —
  sem suporte a múltiplos especialistas por episódio.
- **Domínios de especialista 100% livres**: o usuário cria quantos quiser,
  com quaisquer nomes. O match entre tema do dia e especialista cadastrado
  usa a mesma infraestrutura de embeddings da memória entre episódios.
- **Mood configurável via seletor**: entrevistador e especialista têm mood
  escolhido a partir de uma lista pré-definida (ex: cético, entusiasta,
  direto, didático), persistente até o usuário mudar.
- Internamente, cada mood mapeia pra uma combinação dos 6 eixos de
  personalidade (conciso↔expansivo, didático↔direto, cético↔entusiasta,
  formal↔descontraído, analítico↔storytelling, provocador↔conciliador) —
  isso fica escondido do usuário final.

---

## 3. Formato do episódio

- Duração: **3 presets** — curto, médio, longo — a LLM decide a minutagem
  aproximada dentro de cada faixa, conforme volume/profundidade do material.

---

## 4. Memória entre episódios

- A cada episódio, um resumo estruturado é salvo (tópicos, resumo, data).
- Antes de gerar um novo episódio, busca episódios anteriores relacionados
  ao tema do dia via **embeddings semânticos**.
- Cita episódios antigos só quando há relevância real.
- **Embeddings**: `nomic-embed-text` via Ollama (local) ou
  `text-embedding-3-small` via API — substitui TF-IDF do protótipo inicial.

---

## 5. Camada de LLM (geração de roteiro)

- **Agnóstica / plugável, sem limite de provedores.**
- Distinção arquitetural entre:
  - **Compatível com OpenAI** (`/chat/completions`): cobre OpenAI, OpenRouter,
    Ollama Cloud, Groq, DeepSeek, Together e qualquer serviço que siga esse
    padrão — um único módulo genérico, configurado via `base_url` / `api_key`
    / `model` no TOML, sem código novo por provider.
  - **Formato nativo**: Anthropic (Messages API) e Ollama local, cada um com
    implementação própria.
- Sem provedor padrão — precisa ser configurado explicitamente.
- Chave de API sempre via variável de ambiente, nunca hardcoded.
- Permite 100% gratuito via Ollama local, ou tiers grátis de terceiros
  (OpenRouter `:free`, Ollama Cloud) — sujeitos a limites de taxa e rotação
  de modelos disponíveis.

---

## 6. Camada de TTS (voz)

- Também agnóstica / plugável.
- Provedores curados para v1: **Piper** (local) e **ElevenLabs** (API).
- Sem engine padrão — precisa ser configurada explicitamente.
- Concatenação de áudio via `hound` (manipulação de WAV puro), evitando
  dependência de `ffmpeg`.

---

## 7. Execução

- Ferramenta de linha de comando (CLI).
- Execução manual é o padrão.
- Automação (GitHub Actions, cron) é opcional e externa, não assumida.
- Output padrão: pasta local (áudio + transcript). Também é possível expor
  a pasta via servidor local (ex: `http.server`, Tailscale/ngrok pra acesso
  remoto) — fica documentado como opção, não como feature obrigatória do CLI.

---

## 8. Configuração

- Tudo explícito, nada assumido por padrão — campo obrigatório ausente falha
  com mensagem de erro clara.
- Wizard de configuração (`pheme init`) guia o setup: idioma (PT/EN),
  provider de LLM, chave de API, provider de TTS, personas (mood via
  seletor), fontes/período padrão.
- Formato do arquivo de config: **TOML**.

---

## 9. Stack técnica

- **100% Rust** (primeiro projeto do autor na linguagem), sem mistura com
  Python — prioriza leveza e binário único sem runtime externo.
- Crates principais: `clap` (CLI), `reqwest` + `tokio` (HTTP/async),
  `serde` + `toml` (config), `feed-rs` (RSS), `scraper` (HTML/JSON-LD),
  `rusqlite` (memória local), `hound` (WAV).
- TTS local (Piper) chamado via `std::process::Command` — único ponto que
  depende de binário externo não-Rust.

---

## 10. Nome, licença e distribuição

- **Nome do projeto**: Pheme (nome grego da personificação da voz/boato que
  se espalha — Calliope, Saga, Fama e Fabula foram considerados, mas já
  estavam registrados no crates.io).
- **Licença**: MIT.
- Plano: publicar como open source no GitHub, configurável para outros
  usuários.

---

## 11. Estrutura de pastas do repositório (monolito modular)

```
pheme/
├── Cargo.toml
├── README.md
├── LICENSE                      # MIT
├── pheme.toml.example           # config de exemplo (nunca a real, com chaves)
│
├── src/
│   ├── main.rs                  # entrypoint, parsing de CLI (clap)
│   ├── cli.rs                   # definição dos comandos: init, gerar, config
│   │
│   ├── config/
│   │   ├── mod.rs               # struct Config, load/validate — falha explícita se campo obrigatório faltar
│   │   └── wizard.rs            # lógica do `pheme init` — pergunta idioma, providers, chaves, personas
│   │
│   ├── news/
│   │   ├── mod.rs
│   │   ├── fetch.rs              # busca o HTML/conteúdo do link colado
│   │   └── date_detection.rs     # cascata: RSS → sitemap → meta/JSON-LD → LLM → paste manual
│   │
│   ├── personas/
│   │   ├── mod.rs                # struct Persona (papel: entrevistador/especialista, mood)
│   │   └── presets.rs            # moods pré-definidos (cético, entusiasta, direto, didático) via seletor
│   │
│   ├── llm/
│   │   ├── mod.rs                # trait LlmProvider
│   │   ├── openai_compatible.rs  # genérico: cobre OpenAI, OpenRouter, Ollama Cloud, Groq, DeepSeek, Together etc — só muda base_url/api_key/model no config, sem código novo
│   │   ├── ollama_native.rs      # Ollama local, formato nativo (sem api_key)
│   │   └── anthropic.rs          # Messages API, formato próprio
│   │
│   ├── embeddings/
│   │   ├── mod.rs                # trait EmbeddingProvider
│   │   ├── ollama.rs              # nomic-embed-text
│   │   └── openai.rs              # text-embedding-3-small
│   │
│   ├── tts/
│   │   ├── mod.rs                # trait TtsProvider
│   │   ├── piper.rs               # subprocess local (binário externo, não-Rust)
│   │   └── elevenlabs.rs
│   │
│   ├── memory/
│   │   ├── mod.rs
│   │   ├── store.rs               # rusqlite: salvar/buscar episódios (tópicos, resumo, data)
│   │   └── similarity.rs          # busca por embeddings, cita episódio antigo só se relevância real
│   │
│   ├── episode/
│   │   ├── mod.rs                 # orquestrador — única camada que conhece todas as outras
│   │   └── script.rs              # geração do diálogo (prompt building), aplica preset curto/médio/longo
│   │
│   ├── audio/
│   │   ├── mod.rs
│   │   └── wav.rs                 # concatenação via crate `hound`, sem depender de ffmpeg
│   │
│   └── output/
│       └── mod.rs                 # salva pasta local: áudio + transcript
│
└── tests/
```

### Regras de design acordadas

- Cada camada plugável (`llm/`, `tts/`, `embeddings/`) segue o padrão: uma
  `trait` no `mod.rs` + implementações concretas. Adicionar provider novo =
  criar um arquivo, nunca mexer no resto.
- `llm/`: distingue "compatível" de "nativo" — provedores que falam o
  formato OpenAI (`/chat/completions`) usam um único `openai_compatible.rs`
  genérico, configurável só por `base_url`/`api_key`/`model` no TOML, sem
  exigir código Rust novo por provider. Só formatos realmente diferentes
  (Anthropic Messages API, Ollama nativo) ganham arquivo dedicado. Isso
  permite plugar qualquer provider OpenAI-compatible (OpenRouter, Ollama
  Cloud, Groq, etc.) sem limite artificial de quantidade.
- `episode/` é o único módulo que conhece todos os outros — evita
  acoplamento cruzado (ex: `tts/` nunca sabe de `memory/`).
- `config/` é lido por quase todo mundo, cada camada usa só o pedaço que
  precisa da struct `Config`.
- Sem pasta `utils/` genérica — código sem lar óbvio indica que falta um
  módulo, não uma gaveta.

---

## Nota sobre o protótipo anterior

O primeiro rascunho de código (zip com Ollama fixo, Piper fixo, personas
fixas e GitHub Actions obrigatório) reflete uma versão antiga do design e
**não está alinhado** com as decisões acima. Deve ser descartado ou
reescrito do zero quando a implementação começar.
