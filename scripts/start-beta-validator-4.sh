#!/usr/bin/env bash
# Beta Testnet Validator 4 Startup Script

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PEZKUWI_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$PEZKUWI_DIR/target/release/pezkuwi"
BASE_PATH="/tmp/beta-validator-4"
LOG_FILE="/tmp/beta-validator-4.log"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}🚀 Starting Beta Testnet Validator 4${NC}"

if [ ! -f "$BINARY" ]; then
    echo "Error: Pezkuwi binary not found at $BINARY"
    exit 1
fi

mkdir -p "$BASE_PATH"

echo -e "${GREEN}📡 Starting validator node...${NC}"
echo "Base path: $BASE_PATH"
echo "Log file: $LOG_FILE"
echo "P2P port: 30336"
echo "RPC port: 9947"
echo ""

nohup "$BINARY" \
    --chain pezkuwichain-beta-testnet \
    --base-path "$BASE_PATH" \
    --validator \
    --name "Validator-beta-4" \
    --port 30336 \
    --rpc-port 9947 \
    --rpc-cors all \
    --rpc-external \
    --rpc-methods=Unsafe \
    --unsafe-rpc-external \
    --unsafe-force-node-key-generation \
    --prometheus-external \
    --prometheus-port 9618 \
    --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWM5wShZSM9XQWFMvv46xSrgzWLSWzDnv2rDaVCSsVXJci" \
    > "$LOG_FILE" 2>&1 &

NODE_PID=$!
echo "$NODE_PID" > /tmp/beta-validator-4.pid

echo -e "${GREEN}✅ Validator started with PID: $NODE_PID${NC}"
sleep 10

if ps -p $NODE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Node is running!${NC}"
    tail -20 "$LOG_FILE"
    echo ""
    echo "RPC: http://127.0.0.1:9947"
else
    echo "❌ Node failed to start"
    tail -50 "$LOG_FILE"
    exit 1
fi
