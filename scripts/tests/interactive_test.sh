#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
NC='\033[0m'

PEZKUWI_BIN="./target/release/pezkuwi"

clear
echo -e "${BLUE}╔═══════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     PezkuwiChain Interactive Test Suite          ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}This will start a dev node and guide you through${NC}"
echo -e "${CYAN}manual testing of all core functionality.${NC}"
echo ""

# Network selection
echo -e "${YELLOW}Select Network:${NC}"
echo "  1) Dev (1 validator - Alice)"
echo "  2) Local (2 validators - Alice + Bob)"
echo "  3) Alfa (4 validators)"
echo ""
read -p "Choice [1-3]: " NETWORK_CHOICE

case $NETWORK_CHOICE in
    1)
        CHAIN="dev"
        VALIDATOR_FLAG="--alice"
        ;;
    2)
        CHAIN="pezkuwichain-local"
        VALIDATOR_FLAG="--alice"
        ;;
    3)
        CHAIN="pezkuwichain-alfa-testnet"
        VALIDATOR_FLAG="--alice"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Starting $CHAIN network...${NC}"
echo ""

# Start node with block production
$PEZKUWI_BIN \
  --chain $CHAIN \
  --tmp \
  $VALIDATOR_FLAG \
  --rpc-cors all \
  --rpc-external \
  --rpc-methods=Unsafe \
  --port 30333 \
  --rpc-port 9944 --unsafe-force-node-key-generation &

NODE_PID=$!
echo -e "${BLUE}Node PID: $NODE_PID${NC}"

# Wait for node to start
echo -e "${YELLOW}Waiting for node to start...${NC}"
sleep 8

# Check if node is running
if ! ps -p $NODE_PID > /dev/null; then
    echo -e "${RED}❌ Node failed to start!${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Node started successfully!${NC}"
echo ""

# Display connection info
echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║          CONNECTION INFORMATION            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}🌐 Open: https://polkadot.js.org/apps${NC}"
echo -e "${GREEN}🔗 Connect to: ws://127.0.0.1:9944${NC}"
echo ""
# Start pool initialization in background
echo -e "${CYAN}🚀 Starting DEX pool initialization...${NC}"
./scripts/init/init_dex_pool.sh &

echo ""

# Rest of the script remains the same...
# (Test checklist functions)

echo -e "${YELLOW}Press Ctrl+C to stop the node...${NC}"
wait $NODE_PID
