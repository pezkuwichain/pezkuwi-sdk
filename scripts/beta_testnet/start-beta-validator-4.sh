#!/usr/bin/env bash
# Beta Testnet Validator 4
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PEZKUWI_DIR="$(dirname "$(dirname "$SCRIPT_DIR")")"
BINARY="$PEZKUWI_DIR/target/release/pezkuwi"
BASE_PATH="/tmp/beta-validator-4"
LOG_FILE="/tmp/beta-validator-4.log"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}Starting Beta Testnet Validator 4${NC}"

if [ ! -f "$BINARY" ]; then
    echo -e "${RED}Error: Binary not found at $BINARY${NC}"
    exit 1
fi

mkdir -p "$BASE_PATH"

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
    --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
    --unsafe-force-node-key-generation \
    --prometheus-external \
    --prometheus-port 9618 \
    > "$LOG_FILE" 2>&1 &

NODE_PID=$!
echo "$NODE_PID" > /tmp/beta-validator-4.pid

echo -e "${GREEN}Validator 4 started with PID: $NODE_PID${NC}"
echo "P2P: 30336, RPC: 9947"
sleep 5

if ps -p $NODE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}Node is running${NC}"
else
    echo -e "${RED}Node failed to start${NC}"
    tail -30 "$LOG_FILE"
    exit 1
fi
