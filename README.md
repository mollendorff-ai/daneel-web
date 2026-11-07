# DANEEL Web Dashboard

[![CI](https://github.com/mollendorff-ai/daneel-web/actions/workflows/ci.yml/badge.svg)](https://github.com/mollendorff-ai/daneel-web/actions/workflows/ci.yml)

> 📌 **R&D Prototype** — Interpret claims as hypotheses, not proven facts.

**The Observable Mind** - Real-time window into Timmy's cognitive processes.

A hobby project: the web observatory for [daneel](https://github.com/mollendorff-ai/daneel). Not production software.

## Overview

`daneel-web` is a unified web dashboard for DANEEL:
- **Backend**: Axum HTTP/WebSocket server connecting to Redis and Qdrant
- **Frontend**: Leptos WASM app (pure Rust, no JavaScript)

Dashboard endpoints are read-only. Two kin endpoints (`/inject`, `/embed`) accept POSTs and require a token verified by daneel core.

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
The server listens on `127.0.0.1:3000` by default; set `HOST=0.0.0.0` to expose it (for example behind a reverse proxy).

Or via Docker, from a `daneel` checkout that sits next to this one (`../daneel-web`):

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
| `/health` | GET | Liveness check |
| `/inject` | POST | Proxied to daneel core; core checks the HMAC token |
| `/recent_injections` | GET | Proxied to daneel core; token required |
| `/embed` | POST | Text → 768-dim vector; token verified by core, 503 if core is down |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REDIS_URL` | `redis://localhost:6379` | Redis connection |
| `QDRANT_URL` | `http://localhost:6334` | Qdrant connection |
| `DANEEL_CORE_URL` | `http://localhost:3030` | daneel core API |
| `PORT` | `3000` | Server port |
| `HOST` | `127.0.0.1` | Bind address (`0.0.0.0` to expose) |
| `CORS_ORIGINS` | *(none)* | Comma-separated origins allowed to call the API from a browser; the bundled frontend doesn't need any |
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

- No direct writes to Redis or Qdrant; writes happen only in daneel core via `/inject`
- `/inject`, `/recent_injections` and `/embed` require a kin token that daneel core verifies
- Listens on localhost by default; no cross-origin access unless `CORS_ORIGINS` is set
- Tokens are static bearer secrets (no expiry yet); rotate the keys in daneel core if one leaks

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
