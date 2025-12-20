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
export FRONTEND_DIR="${FRONTEND_DIR:-../daneel-web-ui/dist}"
export RUST_LOG="${RUST_LOG:-daneel_web=info,tower_http=debug}"

echo "=== DANEEL Web Dashboard ==="
echo "Redis:    $REDIS_URL"
echo "Qdrant:   $QDRANT_URL"
echo "Port:     $PORT"
echo "Frontend: $FRONTEND_DIR"
echo ""

# Build if needed
if [ ! -f target/release/daneel-web ]; then
    echo "Building release binary..."
    cargo build --release
fi

# Check frontend exists
if [ ! -f "$FRONTEND_DIR/index.html" ]; then
    echo "WARNING: Frontend not found at $FRONTEND_DIR"
    echo "Run: cd ../daneel-web-ui && trunk build --release"
fi

# Start server
echo "Starting daneel-web on http://localhost:$PORT"
./target/release/daneel-web &
PID=$!
echo $PID > .daneel-web.pid
echo "PID: $PID (saved to .daneel-web.pid)"
