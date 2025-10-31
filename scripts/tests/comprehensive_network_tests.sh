#!/bin/bash
set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

PEZKUWI_BIN="./target/release/pezkuwi"
TEST_RESULTS=()

echo -e "${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  PezkuwiChain Comprehensive Network Tests ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
echo ""

log_test() {
    local status=$1
    local message=$2
    if [ "$status" == "PASS" ]; then
        echo -e "${GREEN}✅ PASS${NC}: $message"
        TEST_RESULTS+=("PASS: $message")
    else
        echo -e "${RED}❌ FAIL${NC}: $message"
        TEST_RESULTS+=("FAIL: $message")
    fi
}

# Test 1: Binary
echo -e "\n${YELLOW}═══ Test 1: Binary Validation ═══${NC}"
if [ -f "$PEZKUWI_BIN" ]; then
    VERSION=$($PEZKUWI_BIN --version 2>/dev/null || echo "unknown")
    log_test "PASS" "Binary exists: $VERSION"
else
    log_test "FAIL" "Binary not found"
    exit 1
fi

# Test 2: Chain Specs
echo -e "\n${YELLOW}═══ Test 2: Chain Spec Validation ═══${NC}"
NETWORKS=("dev" "local" "alfa" "beta" "staging" "mainnet")
EXPECTED_VALIDATORS=(1 2 4 8 20 100)

for i in "${!NETWORKS[@]}"; do
    NETWORK="${NETWORKS[$i]}"
    EXPECTED="${EXPECTED_VALIDATORS[$i]}"
    SPEC_FILE="chain-specs/${NETWORK}-spec.json"
    
    if [ -f "$SPEC_FILE" ]; then
        ACTUAL=$(cat "$SPEC_FILE" | jq '.genesis.runtimeGenesis.patch.session.keys | length' 2>/dev/null || echo "0")
        if [ "$ACTUAL" == "$EXPECTED" ]; then
            log_test "PASS" "$NETWORK: $ACTUAL validators (expected: $EXPECTED)"
        else
            log_test "FAIL" "$NETWORK: $ACTUAL validators (expected: $EXPECTED)"
        fi
    else
        log_test "FAIL" "$NETWORK: Chain spec not found"
    fi
done

# Test 3: Tokenomics
echo -e "\n${YELLOW}═══ Test 3: Genesis Tokenomics ═══${NC}"

BETA_HEZ=$(cat chain-specs/beta-spec.json | jq '[.genesis.runtimeGenesis.patch.balances.balances[][1]] | add' 2>/dev/null || echo "0")
if [ "$BETA_HEZ" == "2e+20" ]; then
    log_test "PASS" "Beta HEZ Total: 200M HEZ"
else
    log_test "FAIL" "Beta HEZ Total: $BETA_HEZ"
fi

BETA_PEZ=$(cat chain-specs/beta-spec.json | jq '[.genesis.runtimeGenesis.patch.assets.accounts[] | select(.[0] == 1) | .[2]] | add' 2>/dev/null || echo "0")
if [ "$BETA_PEZ" == "5e+21" ]; then
    log_test "PASS" "Beta PEZ Total: 5B PEZ"
else
    log_test "FAIL" "Beta PEZ Total: $BETA_PEZ"
fi

ASSETS=$(cat chain-specs/beta-spec.json | jq '.genesis.runtimeGenesis.patch.assets.assets | length' 2>/dev/null || echo "0")
if [ "$ASSETS" == "2" ]; then
    log_test "PASS" "Triple Token System: 2 assets (wHEZ + PEZ)"
else
    log_test "FAIL" "Triple Token System: $ASSETS assets"
fi

PEZ_TREASURY=$(cat chain-specs/beta-spec.json | jq '.genesis.runtimeGenesis.patch.pezTreasury.initializeTreasury' 2>/dev/null || echo "false")
[ "$PEZ_TREASURY" == "true" ] && log_test "PASS" "PezTreasury initialized" || log_test "FAIL" "PezTreasury not initialized"

PEZ_REWARDS=$(cat chain-specs/beta-spec.json | jq '.genesis.runtimeGenesis.patch.pezRewards.startRewardsSystem' 2>/dev/null || echo "false")
[ "$PEZ_REWARDS" == "true" ] && log_test "PASS" "PezRewards started" || log_test "FAIL" "PezRewards not started"

# Test 4: Node Startup
echo -e "\n${YELLOW}═══ Test 4: Node Startup Tests ═══${NC}"

test_node_startup() {
    local CHAIN=$1
    echo -e "  Testing $CHAIN node..."
    
    $PEZKUWI_BIN --chain $CHAIN --tmp --port 30333 > /tmp/pezkuwi-$CHAIN.log 2>&1 &
    local PID=$!
    sleep 15
    
    if ps -p $PID > /dev/null 2>&1; then
        if grep -q "error\|Error\|ERROR" /tmp/pezkuwi-$CHAIN.log; then
            log_test "FAIL" "$CHAIN node has errors"
            kill $PID 2>/dev/null || true
        else
            log_test "PASS" "$CHAIN node started (PID: $PID)"
            kill $PID 2>/dev/null || true
        fi
    else
        log_test "FAIL" "$CHAIN node failed to start"
    fi
    rm -f /tmp/pezkuwi-$CHAIN.log
}

test_node_startup "dev"
sleep 2
test_node_startup "pezkuwichain-local"
sleep 2
test_node_startup "pezkuwichain-alfa-testnet"
sleep 2

echo -e "  ${BLUE}ℹ${NC}  Skipping Beta/Staging/Mainnet (require validator keys)"

# Test 5: Validator Keys
echo -e "\n${YELLOW}═══ Test 5: Validator Keys Validation ═══${NC}"

check_validators() {
    local FILE=$1
    local KEY=$2
    local EXPECTED=$3
    
    if [ -f "$FILE" ]; then
        local COUNT=$(cat "$FILE" | jq ".$KEY | length" 2>/dev/null || echo "0")
        if [ "$COUNT" == "$EXPECTED" ]; then
            log_test "PASS" "$KEY validators: $COUNT"
        else
            log_test "FAIL" "$KEY validators: $COUNT (expected: $EXPECTED)"
        fi
    else
        log_test "FAIL" "$KEY validators JSON not found"
    fi
}

check_validators "pezkuwi/runtime/validators/beta_testnet_validators.json" "beta" "8"
check_validators "pezkuwi/runtime/validators/staging_validators.json" "staging" "20"
check_validators "pezkuwi/runtime/validators/mainnet_validators.json" "mainnet" "100"

# Summary
echo -e "\n${BLUE}╔════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           TEST RESULTS SUMMARY             ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════╝${NC}"
echo ""

TOTAL_TESTS=${#TEST_RESULTS[@]}
PASSED=$(printf '%s\n' "${TEST_RESULTS[@]}" | grep -c "^PASS:" || true)
FAILED=$(printf '%s\n' "${TEST_RESULTS[@]}" | grep -c "^FAIL:" || true)

echo -e "Total Tests: ${BLUE}$TOTAL_TESTS${NC}"
echo -e "Passed:      ${GREEN}$PASSED${NC}"
echo -e "Failed:      ${RED}$FAILED${NC}"
echo ""

if [ "$FAILED" -eq 0 ]; then
    echo -e "${GREEN}🎉 ALL TESTS PASSED!${NC}"
    exit 0
else
    echo -e "${RED}⚠️  $FAILED TESTS FAILED${NC}"
    printf '%s\n' "${TEST_RESULTS[@]}" | grep "^FAIL:"
    exit 1
fi
