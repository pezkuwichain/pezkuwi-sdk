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
        CHAIN_FLAG="--dev"
        ;;
    2)
        CHAIN="pezkuwichain-local"
        CHAIN_FLAG="--chain pezkuwichain-local"
        ;;
    3)
        CHAIN="pezkuwichain-alfa-testnet"
        CHAIN_FLAG="--chain pezkuwichain-alfa-testnet"
        ;;
    *)
        echo "Invalid choice"
        exit 1
        ;;
esac

echo ""
echo -e "${GREEN}Starting $CHAIN network...${NC}"
echo ""

# Start node
$PEZKUWI_BIN \
  $CHAIN_FLAG \
  --tmp \
  --rpc-cors all \
  --rpc-external \
  --rpc-methods=Unsafe \
  --port 30333 \
  --rpc-port 9944 \
  --ws-port 9945 \
  --name "TestNode" \
  --validator \
  --alice &

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

echo -e "${GREEN}✅ Node is running!${NC}"
echo ""

# Display connection info
echo -e "${BLUE}╔═══════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              CONNECTION INFORMATION               ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${CYAN}📱 Polkadot.js Apps:${NC}"
echo -e "   ${GREEN}https://polkadot.js.org/apps${NC}"
echo ""
echo -e "${CYAN}🔗 WebSocket Endpoint:${NC}"
echo -e "   ${GREEN}ws://127.0.0.1:9944${NC}"
echo ""
echo -e "${CYAN}🔗 HTTP RPC Endpoint:${NC}"
echo -e "   ${GREEN}http://127.0.0.1:9944${NC}"
echo ""

# Wait for user to connect
echo -e "${YELLOW}Press ENTER when you've connected to the node...${NC}"
read

clear
echo -e "${BLUE}╔═══════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║              INTERACTIVE TEST GUIDE               ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════╝${NC}"
echo ""

# Test checklist
show_test() {
    local test_num=$1
    local test_name=$2
    local instructions=$3
    
    echo ""
    echo -e "${MAGENTA}═══════════════════════════════════════════════════${NC}"
    echo -e "${YELLOW}TEST $test_num: $test_name${NC}"
    echo -e "${MAGENTA}═══════════════════════════════════════════════════${NC}"
    echo ""
    echo -e "$instructions"
    echo ""
    echo -e "${CYAN}Did the test pass? [y/n]:${NC}"
    read -p "> " RESULT
    
    if [ "$RESULT" == "y" ] || [ "$RESULT" == "Y" ]; then
        echo -e "${GREEN}✅ TEST PASSED${NC}"
        return 0
    else
        echo -e "${RED}❌ TEST FAILED${NC}"
        return 1
    fi
}

# Test 1: Check Initial Balances
show_test "1" "Check Initial HEZ Balance" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Network → Explorer${NC}
2. Find block #0 (Genesis)
3. Expand genesis events
4. Look for ${YELLOW}balances.Endowed${NC} events

${CYAN}Expected:${NC}
- Alice should have ${GREEN}10,000,000,000 HEZ${NC} (10B HEZ)
- Multiple validator accounts with HEZ balances

${CYAN}Verify:${NC} Can you see Alice's 10B HEZ balance?"

# Test 2: Check PEZ Asset
show_test "2" "Verify PEZ Token (Asset ID: 1)" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Chain State${NC}
2. Select: ${YELLOW}assets${NC} pallet
3. Query: ${YELLOW}asset(1)${NC}

${CYAN}Expected Result:${NC}
{
  owner: Treasury Account
  issuer: Treasury Account
  admin: Treasury Account
  freezer: Treasury Account
  supply: ${GREEN}5,000,000,000,000,000,000,000${NC} (5B PEZ with 12 decimals)
  deposit: 0
  minBalance: 1
  isSufficient: true
  accounts: 3
  sufficients: 3
  approvals: 0
  status: Live
}

${CYAN}Verify:${NC} Does PEZ token exist with 5B supply?"

# Test 3: Check wHEZ Asset
show_test "3" "Verify wHEZ Token (Asset ID: 0)" \
"${CYAN}Steps:${NC}
1. Stay in: ${GREEN}Developer → Chain State → assets${NC}
2. Query: ${YELLOW}asset(0)${NC}

${CYAN}Expected Result:${NC}
{
  owner: Wrapper Pallet Account
  supply: ${GREEN}0${NC} (starts at 0, minted on wrap)
  minBalance: 1
  isSufficient: true
  status: Live
}

${CYAN}Verify:${NC} Does wHEZ token exist with 0 initial supply?"

# Test 4: HEZ Transfer
show_test "4" "HEZ Transfer (Alice → Bob)" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Extrinsics${NC}
2. Select account: ${YELLOW}ALICE${NC}
3. Select extrinsic: ${YELLOW}balances → transferKeepAlive${NC}
4. Destination: ${YELLOW}BOB${NC}
5. Amount: ${GREEN}1000000000000000${NC} (1000 HEZ)
6. Click ${GREEN}Submit Transaction${NC}
7. Sign and submit

${CYAN}Verify Bob's Balance:${NC}
1. Go to: ${GREEN}Developer → Chain State${NC}
2. Query: ${YELLOW}system → account(BOB)${NC}
3. Check: ${GREEN}data.free${NC} field

${CYAN}Expected:${NC} Bob should have 1000+ HEZ

${CYAN}Verify:${NC} Did Bob receive HEZ?"

# Test 5: PEZ Transfer
show_test "5" "PEZ Transfer (Treasury → Alice)" \
"${CYAN}Steps:${NC}
1. First, check Treasury PEZ balance:
   - ${GREEN}Developer → Chain State${NC}
   - ${YELLOW}assets → account(1, TREASURY_ADDRESS)${NC}
   
2. Transfer PEZ to Alice:
   - ${GREEN}Developer → Extrinsics${NC}
   - Use ${YELLOW}//Alice${NC} account (needs sudo)
   - ${YELLOW}sudo → sudo${NC}
   - Inner call: ${YELLOW}assets → transfer${NC}
     - id: ${GREEN}1${NC} (PEZ)
     - target: ${YELLOW}ALICE${NC}
     - amount: ${GREEN}1000000000000000${NC} (1000 PEZ)
   - Submit and sign

3. Verify Alice received PEZ:
   - ${YELLOW}assets → account(1, ALICE)${NC}

${CYAN}Expected:${NC} Alice should have 1000 PEZ

${CYAN}Verify:${NC} Did Alice receive PEZ?"

# Test 6: Wrap HEZ → wHEZ
show_test "6" "Wrap HEZ → wHEZ" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Extrinsics${NC}
2. Select: ${YELLOW}ALICE${NC}
3. Select: ${YELLOW}tokenWrapper → wrap${NC}
4. Amount: ${GREEN}500000000000000${NC} (500 HEZ)
5. Submit and sign

${CYAN}Verify wHEZ Received:${NC}
1. ${GREEN}Developer → Chain State${NC}
2. Query: ${YELLOW}assets → account(0, ALICE)${NC}

${CYAN}Expected:${NC}
- Alice's HEZ balance decreased by 500
- Alice's wHEZ balance = ${GREEN}500000000000000${NC} (500 wHEZ)

${CYAN}Verify:${NC} Did Alice receive 500 wHEZ?"

# Test 7: Unwrap wHEZ → HEZ
show_test "7" "Unwrap wHEZ → HEZ" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Extrinsics${NC}
2. Select: ${YELLOW}ALICE${NC}
3. Select: ${YELLOW}tokenWrapper → unwrap${NC}
4. Amount: ${GREEN}200000000000000${NC} (200 wHEZ)
5. Submit and sign

${CYAN}Verify:${NC}
- Alice's wHEZ balance decreased by 200
- Alice's HEZ balance increased by 200

${CYAN}Check:${NC}
1. ${YELLOW}assets → account(0, ALICE)${NC} (wHEZ: should be 300)
2. ${YELLOW}system → account(ALICE)${NC} (HEZ: increased)

${CYAN}Verify:${NC} Did unwrap work correctly?"

# Test 8: Check PEZ Treasury
show_test "8" "Verify PEZ Treasury Status" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Chain State${NC}
2. Query: ${YELLOW}pezTreasury → treasuryAccount()${NC}
3. Note the treasury account address

4. Check Treasury PEZ balance:
   - ${YELLOW}assets → account(1, TREASURY_ACCOUNT)${NC}

${CYAN}Expected:${NC}
- Treasury initialized: ${GREEN}true${NC}
- Treasury has ${GREEN}~4.8B PEZ${NC}

${CYAN}Verify:${NC} Is PEZ Treasury properly configured?"

# Test 9: Check PEZ Rewards
show_test "9" "Verify PEZ Rewards System" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Developer → Chain State${NC}
2. Query: ${YELLOW}pezRewards → currentEpoch()${NC}
3. Query: ${YELLOW}pezRewards → totalStaked()${NC}

${CYAN}Expected:${NC}
- Epoch should be >= ${GREEN}0${NC}
- Rewards system initialized

${CYAN}Verify:${NC} Is PEZ Rewards system active?"

# Test 10: Check Session/Validators
show_test "10" "Verify Validators & Session" \
"${CYAN}Steps:${NC}
1. Go to: ${GREEN}Network → Staking${NC}
2. View active validators

${CYAN}Expected for $CHAIN:${NC}"

case $CHAIN in
    "dev")
        echo -e "- ${GREEN}1 validator${NC} (Alice)"
        ;;
    "pezkuwichain-local")
        echo -e "- ${GREEN}2 validators${NC} (Alice, Bob)"
        ;;
    "pezkuwichain-alfa-testnet")
        echo -e "- ${GREEN}4 validators${NC} (Alice, Bob, Charlie, Dave)"
        ;;
esac

echo ""
echo -e "${CYAN}Verify:${NC} Are all validators active and producing blocks?"

# Summary
echo ""
echo -e "${BLUE}╔═══════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║            INTERACTIVE TESTS COMPLETE             ║${NC}"
echo -e "${BLUE}╚═══════════════════════════════════════════════════╝${NC}"
echo ""
echo -e "${GREEN}All tests completed!${NC}"
echo ""
echo -e "${YELLOW}Press ENTER to stop the node and exit...${NC}"
read

# Cleanup
echo ""
echo -e "${YELLOW}Stopping node (PID: $NODE_PID)...${NC}"
kill $NODE_PID 2>/dev/null || true
sleep 2

echo -e "${GREEN}✅ Node stopped. Tests complete!${NC}"
echo ""
