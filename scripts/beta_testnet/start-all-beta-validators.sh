#!/usr/bin/env bash
# Start all 8 Beta Testnet validators on single computer

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   PezkuwiChain Beta Testnet - 8 Validator Launcher${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}⚠️  Important Notes:${NC}"
echo "1. Make sure you have built the binary with: cargo build --release"
echo "2. Validator keys must be inserted after starting"
echo "3. This starts all 8 validators on single computer"
echo ""

# Start Validator 1 (bootnode)
echo -e "${GREEN}Starting Validator 1 (Bootnode)...${NC}"
"$SCRIPT_DIR/start-beta-validator-1.sh"
echo ""
echo "Waiting for bootnode to stabilize..."
sleep 8
echo ""

# Start remaining validators
for i in 2 3 4 5 6 7 8; do
  echo -e "${GREEN}Starting Validator $i...${NC}"
  "$SCRIPT_DIR/start-beta-validator-$i.sh"
  echo ""
  sleep 3
done

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ All 8 validators started!${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📝 Validator Endpoints:${NC}"
echo "   Validator 1: ws://127.0.0.1:9944 (Bootnode)"
echo "   Validator 2: ws://127.0.0.1:9945"
echo "   Validator 3: ws://127.0.0.1:9946"
echo "   Validator 4: ws://127.0.0.1:9947"
echo "   Validator 5: ws://127.0.0.1:9948"
echo "   Validator 6: ws://127.0.0.1:9949"
echo "   Validator 7: ws://127.0.0.1:9950"
echo "   Validator 8: ws://127.0.0.1:9951"
echo ""
echo -e "${YELLOW}📝 To insert keys:${NC}"
echo "   ./scripts/insert-all-beta-keys.sh"
echo ""
echo -e "${YELLOW}📝 To stop all validators:${NC}"
echo "   ./scripts/stop-beta-validators.sh"
echo ""
