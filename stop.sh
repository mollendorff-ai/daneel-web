#!/bin/bash
# DANEEL Web Dashboard - Stop Script

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

if [ -f .daneel-web.pid ]; then
    PID=$(cat .daneel-web.pid)
    if kill -0 "$PID" 2>/dev/null; then
        echo "Stopping daneel-web (PID: $PID)..."
        kill "$PID"
        rm .daneel-web.pid
        echo "Stopped."
    else
        echo "Process $PID not running."
        rm .daneel-web.pid
    fi
else
    echo "No PID file found. Checking for running process..."
    pkill -f "daneel-web" && echo "Killed." || echo "Not running."
fi
