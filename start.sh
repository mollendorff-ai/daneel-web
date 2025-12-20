#!/bin/bash
# DANEEL Web Dashboard - Start Script
# Serves Leptos WASM frontend + WebSocket metrics from Redis/Qdrant

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

# Default environment variables
export REDIS_URL="${REDIS_URL:-redis://localhost:6379}"
export QDRANT_URL="${QDRANT_URL:-http://localhost:6334}"
export PORT="${PORT:-3000}"
export FRONTEND_DIR="${FRONTEND_DIR:-./frontend/dist}"
export RUST_LOG="${RUST_LOG:-daneel_web=info,tower_http=debug}"

echo "=== DANEEL Web Dashboard ==="
echo "Redis:    $REDIS_URL"
echo "Qdrant:   $QDRANT_URL"
echo "Port:     $PORT"
echo ""

# Build frontend if needed
if [ ! -f "$FRONTEND_DIR/index.html" ]; then
    echo "Building frontend (Leptos WASM)..."
    cd frontend
    trunk build --release
    cd ..
fi

# Build backend if needed
if [ ! -f target/release/daneel-web ]; then
    echo "Building backend..."
    cargo build --release
fi

# Start server
echo "Starting daneel-web on http://localhost:$PORT"
./target/release/daneel-web &
PID=$!
echo $PID > .daneel-web.pid
echo "PID: $PID (saved to .daneel-web.pid)"
