#!/usr/bin/env bash
# Beta Testnet Validator 1 - Bootnode
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PEZKUWI_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
BINARY="$PEZKUWI_DIR/target/release/pezkuwi"
BASE_PATH="/tmp/beta-validator-1"
LOG_FILE="/tmp/beta-validator-1.log"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Starting Beta Testnet Validator 1 (Bootnode)${NC}"

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}Error: Binary not found at $BINARY${NC}"
    exit 1
fi

mkdir -p "$BASE_PATH"

nohup "$BINARY" \
    --chain /home/mamostehp/Pezkuwi-SDK/chainspecs/beta-testnet-raw.json \
    --base-path "$BASE_PATH" \
    --validator \
    --name "Validator-beta-1" \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --rpc-external \
    --rpc-methods=Unsafe \
    --unsafe-rpc-external \
    --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
    --prometheus-external \
    --prometheus-port 9615 \
    > "$LOG_FILE" 2>&1 &

NODE_PID=$!
echo "$NODE_PID" > /tmp/beta-validator-1.pid

echo -e "${GREEN}Validator 1 started with PID: $NODE_PID${NC}"
echo "Base: $BASE_PATH"
echo "Log: $LOG_FILE"
echo "P2P: 30333, RPC: 9944"
sleep 5

if ps -p $NODE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}Node is running${NC}"
else
    echo -e "${RED}Node failed to start${NC}"
    tail -30 "$LOG_FILE"
    exit 1
fi
