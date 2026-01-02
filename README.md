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
├── src/main.rs      # Axum backend
├── frontend/        # Leptos WASM frontend
│   ├── src/lib.rs
│   ├── index.html
│   └── style.css
├── start.sh         # Build & run
└── stop.sh

Browser ──HTTP──> daneel-web ──> frontend/dist/ (WASM)
        └─WS───> /ws endpoint ──> Redis + Qdrant
```

## Quick Start

```bash
./start.sh           # Builds frontend + backend, starts server
open http://localhost:3000
./stop.sh            # Stop server
```

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Leptos WASM frontend |
| `/health` | GET | Health check (JSON) |
| `/metrics` | GET | Current metrics snapshot (JSON) |
| `/ws` | WS | Real-time metrics push (200ms) |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `REDIS_URL` | `redis://localhost:6379` | Redis connection |
| `QDRANT_URL` | `http://localhost:6334` | Qdrant connection |
| `PORT` | `3000` | Server port |
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
RUST_LOG=debug ./start.sh
```

## Part of the DANEEL Family

- [daneel](https://github.com/royalbit/daneel) - The cognitive architecture
- [daneel-poster](https://github.com/royalbit/daneel-poster) - Social media automation

## License

- **Code**: AGPL-3.0-or-later (see [LICENSE](LICENSE))
- **Documentation**: CC-BY-SA-4.0 (see [DOCS_LICENSE.md](DOCS_LICENSE.md))

Copyright (C) 2025 Louis C. Tavares and contributors
