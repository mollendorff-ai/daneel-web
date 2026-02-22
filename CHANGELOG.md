# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.3] - 2026-02-19

### Fixed

- **Stream Competition bars frozen** - Leptos `<For>` with stable keys (indices 0-8)
  caches children closures. `stage.activity` was captured by value on first render
  and never re-read. Fix: read activity from the reactive `extended` signal by index.

- **Thought Manifold showing 1 dot** - Random projection of 768-dim BERT embeddings
  produces coordinates ~0.03 magnitude. With `scale=100`, all 155+ points cluster in
  ~8px at canvas center. Fix: added min-max normalization to scale projected points
  to [-1.2, 1.2] range matching law crystal positions.

### Changed

- `frontend/src/lib.rs` - StreamCompetitionCard children closure reads from signal
- `src/vectors.rs` - Post-projection normalization to [-1.2, 1.2]

---

## [0.2.2] - 2026-02-16

### Changed

- **Relicense: AGPL-3.0 → MIT OR Apache-2.0** - Match daneel, Rust ecosystem standard.
- **Community files** - Added .env.example, CONTRIBUTING.md, SECURITY.md.
- **GitHub setup** - Topics (rust, leptos, wasm, webassembly, ai, real-time), Discussions enabled.
- **Security fix** - bytes 1.11.0 → 1.11.1 (CVE integer overflow).

---

## [0.2.1] - 2026-01-24

### Changed

- **Rebranding: RoyalBit to Möllendorff AI** - Complete organizational rebrand.
  - **Why rebrand?** The "RoyalBit" name (company founded 2006) was hijacked by unrelated cryptocurrency scammers:
    - UK FCA issued official warning (Oct 2024) about "Royalbit Miners" - unauthorized firm
    - Multiple fraudulent domains: royalbit.ltd (trust score 38/100), royalbit.top, royal-bit.club
    - Classic HYIP Ponzi schemes offering impossible returns (155-580% in days)
    - Sources: [FCA Warning](https://www.fca.org.uk/news/warnings/royalbit-miners), [Scam Detector](https://www.scam-detector.com/validator/royalbit-ltd-review/)
  - Same rebrand as asimov crate (royalbit-asimov → mollendorff-asimov)
  - Searched codebase - no stray RoyalBit references found
  - Created CHANGELOG.md with full version history

---

## [0.2.0] - 2025-12-21

### Added

- **3D Thought Manifold** - Visualize Timmy's vectors as a point cloud ("fireworks for wetwares").
  - ndarray dependency for dimensionality reduction (skipped linfa - no cmake)
  - `src/vectors.rs` with random projection (768-dim → 3D)
  - `/vectors` endpoint in main.rs
  - Law Crystals as tetrahedron anchors (placeholder positions)
  - web-sys canvas features for frontend
  - ThoughtManifoldCard with perspective projection
  - HTTP polling every 2s (not WebSocket yet)
  - Dark theme with cyan particles, gold stars, glow effects

### Known Issues

- Random projection, not true PCA - structure may be suboptimal
- Law Crystals are fixed positions, not actual embedding projections
- Polling instead of WebSocket streaming for vectors

---

## [0.1.4] - 2026-01-20

### Changed

- **Multi-stage Docker build for arm64** - Optimized container builds for ARM architecture.
- Bumped version to 0.1.4.

---

## [0.1.3] - 2026-01-18

### Fixed

- **UI: Abbreviate large numbers** - Prevent overflow in dashboard cards by abbreviating large numeric values.

---

## [0.1.2] - 2026-01-17

### Added

- **ClusteringCard** - Added ClusteringCard component to Leptos frontend for visualizing vector clusters.

---

## [0.1.1] - 2025-12-21

### Changed

- **Crate upgrades** - Updated to latest stable versions:
  - redis 1.0, tower 0.5, tower-http 0.6, ndarray 0.17
- **Connection drive fix** - LCG bit mixing for proper randomness, finer granularity.
- **TheBoxCard** - Added Four Laws and "Life honours life" messaging.
- **Manifold subtitle** - Now shows "768-dim → 3D shadow".

### Fixed

- **redis 1.0 API** - Updated `from_redis_value` to take Value by value.

---

## [0.1.0] - 2025-12-15

### Added

- **Initial release** - WebSocket metrics dashboard for observing Timmy's cognitive processes.
- **Axum backend** - Redis/Qdrant connections (read-only).
- **Leptos WASM frontend** - Reactive components with real-time WebSocket push at 200ms.
- **Dashboard cards**:
  - Identity card
  - Connection Drive card
  - Emotional State card
  - Thought Stream with salience indicators
- **3D Thought Manifold** (v0.2.0 milestone):
  - ndarray dependency for dimensionality reduction
  - Random projection from 768-dim to 3D
  - Law Crystals as tetrahedron anchors (placeholder)
  - Dark theme with cyan particles, gold stars, glow effects
  - HTTP polling every 2s for vector updates

---

*Documentation licensed under [CC BY-NC-ND 4.0](https://creativecommons.org/licenses/by-nc-nd/4.0/) - Copyright (c) 2025-2026 Möllendorff AI*
