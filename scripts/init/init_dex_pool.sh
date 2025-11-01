#!/bin/bash

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

echo -e "${CYAN}🔄 DEX Pool Initialization Script${NC}"
echo ""

# Wait for node to be ready
echo -e "${YELLOW}⏳ Waiting for node to start...${NC}"
sleep 10

# Check if node is responding
MAX_RETRIES=30
RETRY_COUNT=0

while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if curl -s -H "Content-Type: application/json" \
           -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
           http://localhost:9944 > /dev/null 2>&1; then
        echo -e "${GREEN}✅ Node is ready!${NC}"
        break
    fi
    
    RETRY_COUNT=$((RETRY_COUNT + 1))
    echo -e "${YELLOW}Retry $RETRY_COUNT/$MAX_RETRIES...${NC}"
    sleep 2
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo -e "${RED}❌ Node did not start in time!${NC}"
    exit 1
fi

echo ""
echo -e "${CYAN}🏊 Initializing DEX Pools automatically...${NC}"
echo ""

# Try Python script first (more reliable)
if [ -f "./scripts/init/init_pools.py" ]; then
    echo -e "${GREEN}✅ Using Python initialization script${NC}"
    echo ""

    # Check if virtual environment exists
    if [ -d "./venv" ]; then
        echo -e "${GREEN}✅ Using virtual environment${NC}"
        PYTHON_CMD="./venv/bin/python3"
    else
        PYTHON_CMD="python3"
    fi

    # Check if Python dependencies are installed
    if $PYTHON_CMD -c "import substrateinterface" 2>/dev/null; then
        echo -e "${CYAN}Running Python pool initializer...${NC}"
        $PYTHON_CMD ./scripts/init/init_pools.py

        if [ $? -eq 0 ]; then
            echo ""
            echo -e "${GREEN}✅ Pools initialized successfully via Python!${NC}"
            exit 0
        else
            echo -e "${YELLOW}⚠️  Python script failed, trying alternative methods...${NC}"
        fi
    else
        echo -e "${YELLOW}⚠️  Python substrate-interface not installed${NC}"
        echo -e "${CYAN}To install in virtual environment:${NC}"
        echo -e "  ${YELLOW}python3 -m venv venv${NC}"
        echo -e "  ${YELLOW}source venv/bin/activate${NC}"
        echo -e "  ${YELLOW}pip install substrate-interface${NC}"
        echo ""
    fi
fi

# Fallback to polkadot-js-api
echo -e "${CYAN}Trying polkadot-js-api method...${NC}"
echo ""

# ⚠️ IMPORTANT: AssetConversion only supports Asset ↔ Asset swaps!
# Native HEZ ↔ wHEZ uses TokenWrapper.wrap/unwrap
# Only valid pool: wHEZ ↔ PEZ

ALICE_SEED="//Alice"

# Check if @polkadot/api-cli is installed
if command -v polkadot-js-api &> /dev/null; then
    echo -e "${GREEN}✅ @polkadot/api-cli found!${NC}"
    echo ""

    echo -e "${CYAN}ℹ️  Creating wHEZ/PEZ pool (only valid Asset-Asset pool)${NC}"
    echo -e "${YELLOW}   Note: Native HEZ ↔ wHEZ uses TokenWrapper.wrap/unwrap${NC}"
    echo ""

    # ONLY POOL: wHEZ (Asset 0) ↔ PEZ (Asset 1)
    echo -e "${CYAN}Creating wHEZ/PEZ pool...${NC}"
    polkadot-js-api \
      --ws ws://localhost:9944 \
      --seed "$ALICE_SEED" \
      tx.assetConversion.createPool \
      0 \
      1

    sleep 3

    echo ""
    echo -e "${GREEN}🎉 Pool created successfully!${NC}"

    # Add initial liquidity
    echo ""
    echo -e "${CYAN}💧 Adding initial liquidity to wHEZ/PEZ pool...${NC}"
    echo -e "${YELLOW}   Amount: 100K wHEZ + 500K PEZ (1:5 ratio)${NC}"

    # Smaller amounts to avoid JavaScript number limit
    # 100,000 tokens = 100000 * 10^12 = 100000000000000000
    polkadot-js-api \
      --ws ws://localhost:9944 \
      --seed "$ALICE_SEED" \
      tx.assetConversion.addLiquidity \
      0 \
      1 \
      "100000000000000000" \
      "500000000000000000" \
      "95000000000000000" \
      "475000000000000000" \
      "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"

    echo ""
    echo -e "${GREEN}✅ wHEZ/PEZ pool initialized and funded!${NC}"

else
    echo -e "${RED}❌ @polkadot/api-cli not found!${NC}"
    echo ""
    echo -e "${CYAN}To install:${NC}"
    echo -e "  ${YELLOW}npm install -g @polkadot/api-cli${NC}"
    echo ""
    echo -e "${CYAN}📋 OR create pool manually:${NC}"
    echo -e "  1. Open: ${GREEN}https://polkadot.js.org/apps${NC}"
    echo -e "  2. Connect to: ${GREEN}ws://127.0.0.1:9944${NC}"
    echo -e "  3. Developer → Extrinsics"
    echo -e "  4. ${YELLOW}assetConversion → createPool${NC}"
    echo -e "     - asset1: ${GREEN}0${NC} (wHEZ)"
    echo -e "     - asset2: ${GREEN}1${NC} (PEZ)"
    echo -e "  5. Submit Transaction (Alice)"
    echo ""
    echo -e "  ${CYAN}ℹ️  Only wHEZ ↔ PEZ pool is valid!${NC}"
    echo -e "  ${YELLOW}Native HEZ ↔ wHEZ uses TokenWrapper.wrap/unwrap${NC}"
fi

echo ""
echo -e "${GREEN}✅ Pool initialization script complete!${NC}"
