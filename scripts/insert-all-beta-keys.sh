#!/usr/bin/env bash
# Insert all validator keys for Beta Testnet (Validators 1-4)
# This is a template script - actual seeds must be provided

set -e

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   Beta Testnet - Bulk Key Insertion${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}⚠️  SECURITY WARNING${NC}"
echo "This script will insert validator keys for all 4 local validators."
echo "Make sure:"
echo "  1. All validators are running"
echo "  2. You have the actual seed phrases ready"
echo "  3. Seeds are stored securely (cold wallet)"
echo ""
echo -e "${RED}⚠️  This is a TEMPLATE script with placeholder seeds${NC}"
echo -e "${RED}   You MUST edit this file and replace placeholders with actual seeds${NC}"
echo ""
read -p "Press Ctrl+C to cancel, or Enter to continue..."
echo ""

# Function to insert key
insert_key() {
    local validator_num=$1
    local rpc_port=$2
    local key_type=$3
    local seed=$4
    local public_key=$5

    local rpc_url="http://127.0.0.1:${rpc_port}"

    echo -e "${GREEN}Validator ${validator_num}: Inserting ${key_type} key...${NC}"

    result=$(curl -s -H "Content-Type: application/json" \
        -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"${key_type}\",\"${seed}\",\"${public_key}\"]}" \
        "$rpc_url")

    if echo "$result" | grep -q '"result":null'; then
        echo -e "${GREEN}  ✅ Success${NC}"
        return 0
    else
        echo -e "${RED}  ❌ Failed${NC}"
        echo "  Response: $result"
        return 1
    fi
}

# =============================================================================
# VALIDATOR 1 KEYS (Port 9944)
# =============================================================================
echo -e "${BLUE}🔑 Validator 1 (Port 9944)${NC}"

# Replace these with actual seeds from beta_testnet_validators.json
V1_BABE_SEED="your-validator-1-babe-seed"
V1_BABE_PUB="0x9e7490cfe0dd32860282dd4e74b5c40c9237fb6e478066c334c931742308e008"

V1_GRANDPA_SEED="your-validator-1-grandpa-seed"
V1_GRANDPA_PUB="0x16da26f049b37e9b03d14439457ac164f14f5174b1f10ddab2e0dc3ef7903675"

V1_PARA_VALIDATOR_SEED="your-validator-1-para-validator-seed"
V1_PARA_VALIDATOR_PUB="0xc62b764ea84411f550818a41a6abacddb9388cefbc22412f2bee335e9709a717"

V1_PARA_ASSIGNMENT_SEED="your-validator-1-para-assignment-seed"
V1_PARA_ASSIGNMENT_PUB="0xa4c75d886439ae7d764cc600051af1d2128d2a8a1cdf3d642eda76824ccaf518"

V1_AUTHORITY_DISCOVERY_SEED="your-validator-1-authority-discovery-seed"
V1_AUTHORITY_DISCOVERY_PUB="0x66e821a23729878d8b7d04c69ec0fed9b0f5facae48917c1b83a4ad15cf00720"

V1_BEEFY_SEED="your-validator-1-beefy-seed"
V1_BEEFY_PUB="0x0281ba169080b261add0cd22bfb47477eddab854caa17423b12ddeed6504b04aef"

# Check if still placeholder
if [[ "$V1_BABE_SEED" == "your-validator-1-babe-seed" ]]; then
    echo -e "${RED}❌ Error: Placeholder seeds detected!${NC}"
    echo ""
    echo "Please edit this script and replace all placeholder seeds with actual seeds from:"
    echo "  /home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"
    echo ""
    exit 1
fi

insert_key 1 9944 "babe" "$V1_BABE_SEED" "$V1_BABE_PUB"
insert_key 1 9944 "gran" "$V1_GRANDPA_SEED" "$V1_GRANDPA_PUB"
insert_key 1 9944 "para" "$V1_PARA_VALIDATOR_SEED" "$V1_PARA_VALIDATOR_PUB"
insert_key 1 9944 "asgn" "$V1_PARA_ASSIGNMENT_SEED" "$V1_PARA_ASSIGNMENT_PUB"
insert_key 1 9944 "audi" "$V1_AUTHORITY_DISCOVERY_SEED" "$V1_AUTHORITY_DISCOVERY_PUB"
insert_key 1 9944 "beef" "$V1_BEEFY_SEED" "$V1_BEEFY_PUB"

echo ""

# =============================================================================
# VALIDATOR 2-4 (Add similar blocks for other validators)
# =============================================================================
echo -e "${YELLOW}Note: Add Validator 2-4 key insertion blocks following the same pattern${NC}"
echo -e "${YELLOW}      Validator 2: Port 9945${NC}"
echo -e "${YELLOW}      Validator 3: Port 9946${NC}"
echo -e "${YELLOW}      Validator 4: Port 9947${NC}"
echo ""

echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Key insertion complete!${NC}"
echo -e "${GREEN}═══════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${YELLOW}📝 Next steps:${NC}"
echo "1. Restart all validators to apply keys"
echo "2. Wait for network to start producing blocks"
echo "3. Check logs for 'Prepared block for proposing'"
echo ""
echo "Validator logs:"
echo "  Validator 1: tail -f /tmp/beta-validator-1.log"
echo "  Validator 2: tail -f /tmp/beta-validator-2.log"
echo "  Validator 3: tail -f /tmp/beta-validator-3.log"
echo "  Validator 4: tail -f /tmp/beta-validator-4.log"
