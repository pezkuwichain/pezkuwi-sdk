#!/usr/bin/env bash
# Start all 4 local Beta Testnet validators

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   PezkuwiChain Beta Testnet - Validator Launcher${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}⚠️  Important Notes:${NC}"
echo "1. Make sure you have built the binary with: cargo build --release"
echo "2. Validator keys need to be inserted separately"
echo "3. This starts 4 local validators (Validators 1-4)"
echo "4. Validators 5-8 should be started on another machine"
echo ""
read -p "Press Enter to continue..."
echo ""

# Start Validator 1 (bootnode)
echo -e "${GREEN}Starting Validator 1 (Bootnode)...${NC}"
"$SCRIPT_DIR/start-beta-validator-1.sh"
echo ""
echo "Waiting for bootnode to stabilize..."
sleep 5
echo ""

# Start Validator 2
echo -e "${GREEN}Starting Validator 2...${NC}"
"$SCRIPT_DIR/start-beta-validator-2.sh"
echo ""
sleep 3

# Start Validator 3
echo -e "${GREEN}Starting Validator 3...${NC}"
"$SCRIPT_DIR/start-beta-validator-3.sh"
echo ""
sleep 3

# Start Validator 4
echo -e "${GREEN}Starting Validator 4...${NC}"
"$SCRIPT_DIR/start-beta-validator-4.sh"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ All 4 local validators started!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📝 Validator Endpoints:${NC}"
echo "   Validator 1: ws://127.0.0.1:9944 (Bootnode)"
echo "   Validator 2: ws://127.0.0.1:9945"
echo "   Validator 3: ws://127.0.0.1:9946"
echo "   Validator 4: ws://127.0.0.1:9947"
echo ""
echo -e "${YELLOW}📝 Log Files:${NC}"
echo "   Validator 1: /tmp/beta-validator-1.log"
echo "   Validator 2: /tmp/beta-validator-2.log"
echo "   Validator 3: /tmp/beta-validator-3.log"
echo "   Validator 4: /tmp/beta-validator-4.log"
echo ""
echo -e "${YELLOW}📝 To stop all validators:${NC}"
echo "   ./scripts/stop-beta-validators.sh"
echo ""
echo -e "${YELLOW}⚠️  Next Steps:${NC}"
echo "1. Insert validator keys for each validator"
echo "2. Start validators 5-8 on the second machine"
echo "3. Wait for the network to start producing blocks"
echo ""
