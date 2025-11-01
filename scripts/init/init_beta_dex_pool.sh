#!/bin/bash
# DEX Pool Initialization for Beta Testnet
# This script initializes liquidity pools on beta testnet after validator startup

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

# Beta testnet uses validator 1 as primary RPC
RPC_PORT=${RPC_PORT:-9944}
RPC_URL="http://localhost:${RPC_PORT}"

echo -e "${CYAN}🔄 Beta Testnet - DEX Pool Initialization${NC}"
echo -e "${CYAN}   RPC: ${RPC_URL}${NC}"
echo ""

# Wait for node to be ready
echo -e "${YELLOW}⏳ Waiting for beta testnet to start producing blocks...${NC}"
echo -e "${YELLOW}   This may take a few minutes as validators sync...${NC}"
sleep 15

# Check if node is responding
MAX_RETRIES=60
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s -H "Content-Type: application/json" \
           -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
           "$RPC_URL" > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Node is responding!${NC}"
        break
    fi

    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo -e "${YELLOW}Retry $RETRY_COUNT/$MAX_RETRIES...${NC}"
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${RED}❌ Beta testnet did not start in time!${NC}"
    echo ""
    echo "Check validator logs:"
    echo "  tail -f /tmp/beta-validator-1.log"
    exit 1
fi

# Check if blocks are being produced
echo ""
echo -e "${YELLOW}⏳ Waiting for block production...${NC}"
sleep 10

CURRENT_BLOCK=$(curl -s -H "Content-Type: application/json" \
    -d '{"id":1,"jsonrpc":"2.0","method":"chain_getBlock","params":[]}' \
    "$RPC_URL" | jq -r '.result.block.header.number' 2>/dev/null)

if [ -z "$CURRENT_BLOCK" ] || [ "$CURRENT_BLOCK" == "null" ]; then
    echo -e "${YELLOW}⚠️  Warning: Cannot verify block production${NC}"
    echo -e "${YELLOW}   Continuing anyway...${NC}"
else
    echo -e "${GREEN}✅ Block #${CURRENT_BLOCK} detected${NC}"
fi

echo ""
echo -e "${CYAN}🏊 Initializing DEX Pools for Beta Testnet...${NC}"
echo ""

# Check if Python script exists
if [ -f "./scripts/init/init_pools.py" ]; then
    echo -e "${GREEN}✅ Using Python initialization script${NC}"
    echo ""

    # Check if virtual environment exists
    if [ -d "./venv" ]; then
        echo -e "${GREEN}✅ Using virtual environment${NC}"
        source ./venv/bin/activate
    else
        echo -e "${YELLOW}⚠️  No virtual environment found, using system Python${NC}"
    fi

    # Run Python script with beta testnet RPC
    python3 ./scripts/init/init_pools.py --rpc-url "$RPC_URL"

    if [ $? -eq 0 ]; then
        echo ""
        echo -e "${GREEN}✅ DEX pools initialized successfully on beta testnet!${NC}"
    else
        echo ""
        echo -e "${RED}❌ DEX pool initialization failed${NC}"
        echo "Check the logs above for details"
        exit 1
    fi
else
    echo -e "${RED}❌ init_pools.py not found${NC}"
    echo "Expected location: ./scripts/init/init_pools.py"
    exit 1
fi

echo ""
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Beta Testnet DEX is ready!${NC}"
echo -e "${CYAN}═══════════════════════════════════════════════════════${NC}"
echo ""
echo "Available pools:"
echo "  - HEZ/PEZ liquidity pool"
echo "  - Initial liquidity provided"
echo ""
echo "Connect your DKSweb to: wss://beta.pezkuwichain.io"
