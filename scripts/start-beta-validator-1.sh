#!/usr/bin/env bash
# Beta Testnet Validator 1 Startup Script
# This is the bootnode for the beta testnet

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PEZKUWI_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$PEZKUWI_DIR/target/release/pezkuwi"
BASE_PATH="/tmp/beta-validator-1"
LOG_FILE="/tmp/beta-validator-1.log"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Starting Beta Testnet Validator 1 (Bootnode)${NC}"
echo -e "${YELLOW}⚠️  Make sure you have inserted the validator keys before starting${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BINARY" ]; then
    echo "Error: Pezkuwi binary not found at $BINARY"
    echo "Please run: cargo build --release"
    exit 1
fi

# Create base path
mkdir -p "$BASE_PATH"

# Start validator
echo -e "${GREEN}📡 Starting validator node...${NC}"
echo "Base path: $BASE_PATH"
echo "Log file: $LOG_FILE"
echo "P2P port: 30333"
echo "RPC port: 9944"
echo "WS port: 9944"
echo ""

nohup "$BINARY" \
    --chain pezkuwichain-beta-testnet \
    --base-path "$BASE_PATH" \
    --validator \
    --name "Validator-beta-1" \
    --port 30333 \
    --rpc-port 9944 \
    --rpc-cors all \
    --rpc-external \
    --rpc-methods=Unsafe \
    --unsafe-rpc-external \
    --unsafe-force-node-key-generation \
    --prometheus-external \
    --telemetry-url "wss://telemetry.pezkuwichain.io/submit/ 0" \
    > "$LOG_FILE" 2>&1 &

NODE_PID=$!
echo "$NODE_PID" > /tmp/beta-validator-1.pid

echo -e "${GREEN}✅ Validator started with PID: $NODE_PID${NC}"
echo ""
echo "Waiting for node to initialize..."
sleep 10

# Check if node is running
if ps -p $NODE_PID > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Node is running!${NC}"
    echo ""
    echo "📊 Recent logs:"
    tail -20 "$LOG_FILE"
    echo ""
    echo -e "${BLUE}🔗 Connections:${NC}"
    echo "   RPC: http://127.0.0.1:9944"
    echo "   WebSocket: ws://127.0.0.1:9944"
    echo ""
    echo -e "${YELLOW}📝 To insert keys, use:${NC}"
    echo "   ./scripts/insert-keys-validator-1.sh"
    echo ""
    echo -e "${YELLOW}📝 To stop the validator:${NC}"
    echo "   kill $NODE_PID"
    echo "   # or"
    echo "   kill \$(cat /tmp/beta-validator-1.pid)"
else
    echo -e "${RED}❌ Node failed to start${NC}"
    echo "Check logs at: $LOG_FILE"
    tail -50 "$LOG_FILE"
    exit 1
fi
