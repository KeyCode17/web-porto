# web-porto

A brutalist portfolio website built with Rust, WebAssembly, and Dioxus 0.7. Features an AI-powered chatbot that runs entirely in the browser using semantic search.

## Tech Stack

- **Rust + WebAssembly** — compiled to WASM via `wasm32-unknown-unknown`
- **Dioxus 0.7** — reactive UI framework with client-side routing
- **Transformers.js** — runs `all-MiniLM-L6-v2` (fp16) in-browser for semantic embeddings
- **Nix Flakes** — reproducible dev environment and NixOS deployment module

## Pages

- **Home** — hero section with interactive particle canvas
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
├── porto-app/           # Main Dioxus WASM application
│   └── src/
│       ├── canvas/      # Interactive canvas animations
│       ├── components/  # Shared UI components (navbar, cursor)
│       ├── pages/       # Route pages (home, projects, papers, chat)
│       └── styles/      # Theme and global styles
├── porto-shared/        # Shared data types
│   └── src/
├── content/             # TOML content files (about, projects, papers, FAQ)
├── public/              # Static assets (photos, papers, models, favicon)
├── nix/
│   └── module.nix       # NixOS module (nginx, ACME, security headers)
├── Cargo.toml           # Workspace root
├── Dioxus.toml          # Dioxus build configuration
└── flake.nix            # Nix devShell, packages, nixosModules.default
```

## Development

```bash
nix develop
dx serve --package porto-app
```

## Production Build

```bash
nix develop -c dx bundle --package porto-app
```

Output: `target/dx/porto-app/debug/web/public/`

## NixOS Deployment

This flake exports `nixosModules.default` for use in a NixOS configuration:

```nix
# In your flake inputs:
web-porto.url = "github:KeyCode17/web-porto";

# In your machine config:
imports = [ web-porto.nixosModules.default ];

services.web-porto = {
  enable = true;
  domain = "daffakaryudi.web.id";
  acmeEmail = "m.daffa.karyudi@gmail.com";
};
```
