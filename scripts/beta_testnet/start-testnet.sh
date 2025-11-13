#!/bin/bash

echo "🚀 Starting Beta Testnet..."
echo "=============================="

# Get the script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Start all 8 validators in background
echo "Starting validators..."
for i in {1..8}; do
    bash start-beta-validator-$i.sh &
    echo "  ✓ Validator $i started"
done

# Wait for chain to initialize
echo ""
echo "⏳ Waiting for chain to initialize and RPC to be ready (20 seconds)..."
sleep 20

# Insert all validator keys
echo ""
echo "🔑 Inserting validator keys..."
echo "" | bash insert-all-beta-keys.sh

echo ""
echo "✅ Beta testnet started successfully!"
echo "   You can check logs at /tmp/beta-validator-*.log"
echo "   To stop: bash stop-beta-validators.sh"
