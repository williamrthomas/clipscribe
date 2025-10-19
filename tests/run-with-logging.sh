#!/bin/bash

# Run ClipScribe with comprehensive logging
# Usage: ./tests/run-with-logging.sh

echo "Starting ClipScribe with logging enabled..."
echo "Logs will be saved to logs/tauri-console-$(date +%Y%m%d-%H%M%S).log"

LOG_FILE="logs/tauri-console-$(date +%Y%m%d-%H%M%S).log"

# Ensure logs directory exists
mkdir -p logs

# Run with logging
npm run tauri:dev 2>&1 | tee "$LOG_FILE"
