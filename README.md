# web-porto

A brutalist portfolio website built with Rust, WebAssembly, and Dioxus 0.7. Features an AI-powered chatbot that runs entirely in the browser using semantic search.

## Tech Stack

- **Rust + WebAssembly** — compiled to WASM via `wasm32-unknown-unknown`
- **Dioxus 0.7** — reactive UI framework with client-side routing
- **Transformers.js** — runs `all-MiniLM-L6-v2` (fp16) in-browser for semantic embeddings
- **Nix Flakes** — reproducible dev environment

## Pages

- **Home** — hero section with interactive elements
- **Projects** — project showcase with detail views
- **Papers** — research papers with detail views
- **Chat** — AI chatbot powered by semantic search over FAQ data

## AI Chatbot

The chat page features a FAQ-based chatbot that uses:

- **Semantic search** — user queries are embedded with `all-MiniLM-L6-v2` and matched against pre-computed FAQ embeddings via cosine similarity
- **TF-IDF fallback** — keyword-based matching when embeddings are unavailable
- **Conversational features** — greeting detection, follow-up handling, response variation
- **Privacy-first** — the ML model runs locally in the browser, no data leaves the device

The ~44MB model is loaded on-demand when the user clicks "Load AI Model".

## Project Structure

```
├── crates/
│   ├── app/          # Dioxus application (pages, styles, routing)
│   └── shared/       # Shared data types (FaqEntry, etc.)
├── content/          # TOML content files (about, projects, papers, FAQ, skills)
├── public/
│   ├── models/       # all-MiniLM-L6-v2 ONNX model files
│   ├── papers/       # PDF papers
│   └── photos/       # Photo assets
├── python/           # Offline embedding computation script
├── Dioxus.toml       # Dioxus build configuration
├── flake.nix         # Nix dev environment
└── index.html        # Entry point
```

## Development

```bash
# Enter nix dev shell
nix develop

# Start dev server
dx serve --package web-porto-app

# Pre-compute FAQ embeddings (only needed when FAQ content changes)
cd python && pip install sentence-transformers toml && python compute_embeddings.py
```

## Production Build

```bash
nix develop -c dx bundle --package web-porto-app
```

Output: `target/dx/web-porto-app/debug/web/public/` — serve this directory with any static file server.
