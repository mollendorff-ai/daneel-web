# DANEEL-WEB - Observation Dashboard
# Multi-stage build: compiles inside Docker for correct architecture
#
# Usage: docker build -t daneel-web .

# === Build Stage ===
FROM debian:bookworm AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl \
    build-essential \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup (gets latest stable)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
ENV PATH="/root/.cargo/bin:${PATH}"

# Install trunk and wasm target
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /build

# Copy manifests and source
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Build backend
RUN cargo build --release

# Build WASM frontend
COPY frontend ./frontend
RUN cd frontend && trunk build --release

# === Runtime Stage ===
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/daneel-web /app/daneel-web

# Copy WASM frontend
COPY --from=builder /build/frontend/dist /app/frontend/dist

# fastembed cache directory (mounted as volume)
RUN mkdir -p /root/.cache/fastembed

EXPOSE 3000

ENTRYPOINT ["/app/daneel-web"]
