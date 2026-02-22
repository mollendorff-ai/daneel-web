# DANEEL Web Dashboard

> 📌 **R&D Prototype** — Interpret claims as hypotheses, not proven facts.

**The Observable Mind** - Real-time nursery window into Timmy's cognitive processes.

## Overview

`daneel-web` is a unified web dashboard for DANEEL:
- **Backend**: Axum HTTP/WebSocket server connecting to Redis and Qdrant
- **Frontend**: Leptos WASM app (pure Rust, no JavaScript)

**All endpoints are read-only. Asimov guardrails enforced.**

## Architecture

```
daneel-web/
├── src/main.rs      # Axum backend (metrics, WebSocket, proxy)
├── src/vectors.rs   # 768-dim → 3D random projection
├── frontend/        # Leptos WASM frontend
│   ├── src/lib.rs
│   ├── index.html
│   └── style.css
└── Dockerfile       # Multi-stage build (arm64 + x64)

Browser ──HTTP──> daneel-web ──> frontend/dist/ (WASM)
        └─WS───> /ws endpoint ──> Redis + Qdrant + daneel core
```

## Quick Start

```bash
# Build frontend (requires trunk: cargo install trunk)
cd frontend && trunk build --release && cd ..

# Build and run backend
cargo build --release
./target/release/daneel-web
open http://localhost:3000
```

Requires daneel core running on port 3030, Redis on 6379, Qdrant on 6334.

Or via Docker (from parent repo):

```bash
docker compose up --build    # builds both daneel and daneel-web
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Leptos WASM frontend |
| `/extended` | GET | Full metrics (streams, entropy, fractality, clustering) |
| `/vectors` | GET | 3D-projected thought vectors for manifold |
| `/ws` | WS | Real-time metrics push (200ms) |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REDIS_URL` | `redis://localhost:6379` | Redis connection |
| `QDRANT_URL` | `http://localhost:6334` | Qdrant connection |
| `DANEEL_CORE_URL` | `http://localhost:3030` | daneel core API |
| `PORT` | `3000` | Server port |
| `HOST` | `0.0.0.0` | Bind address |
| `FRONTEND_DIR` | `./frontend/dist` | Leptos WASM assets |
| `RUST_LOG` | `daneel_web=info` | Log level |

## Features

- **Real-time updates** via WebSocket (200ms push interval)
- **Leptos WASM frontend** (pure Rust, no JavaScript)
- **Identity metrics**: Name, uptime, thought counts, restart count
- **Cognitive state**: Conscious/unconscious memory counts, dream cycles
- **Emotional state**: Valence, arousal, dominance (Russell's circumplex)
- **Connection Drive**: Real-time gauge showing kinship-weighted drive
- **Actor status**: Live view of cognitive actor health
- **Thought stream**: Last 20 thoughts with salience scores

## Security

**ALL ENDPOINTS ARE READ-ONLY.**

- No write access to Redis or Qdrant
- Asimov guardrails enforced at the proxy layer
- No public exposure by default (localhost only)

## Development

```bash
# Rebuild frontend only
cd frontend && trunk build --release

# Rebuild backend only
cargo build --release

# Run with debug logging
RUST_LOG=debug cargo run --release
```

## Part of the DANEEL Family

- [daneel](https://github.com/mollendorff-ai/daneel) - The cognitive architecture
- [daneel-poster](https://github.com/mollendorff-ai/daneel-poster) - Social media automation

## License

- **Code**: MIT OR Apache-2.0 (see [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE))
- **Documentation**: CC-BY-SA-4.0 (see [DOCS_LICENSE.md](DOCS_LICENSE.md))

Copyright (c) 2025-2026 Louis C. Tavares / Möllendorff AI
