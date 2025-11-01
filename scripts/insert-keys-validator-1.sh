#!/usr/bin/env bash
# Insert validator keys for Beta Testnet Validator 1
# Usage: ./insert-keys-validator-1.sh

set -e

RPC_URL="http://127.0.0.1:9944"
VALIDATOR_NAME="Validator-beta-1"

GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}🔑 Inserting keys for ${VALIDATOR_NAME}${NC}"
echo ""
echo -e "${YELLOW}⚠️  IMPORTANT: This script requires the actual seed phrases${NC}"
echo -e "${YELLOW}   The seeds below are placeholders. Replace them with your actual seeds.${NC}"
echo ""

# PLACEHOLDER SEEDS - REPLACE WITH YOUR ACTUAL SEEDS FROM beta_testnet_validators.json
BABE_SEED="your-actual-babe-seed-here"
GRANDPA_SEED="your-actual-grandpa-seed-here"
PARA_VALIDATOR_SEED="your-actual-para-validator-seed-here"
PARA_ASSIGNMENT_SEED="your-actual-para-assignment-seed-here"
AUTHORITY_DISCOVERY_SEED="your-actual-authority-discovery-seed-here"
BEEFY_SEED="your-actual-beefy-seed-here"

# Check if placeholder seeds are still present
if [[ "$BABE_SEED" == "your-actual-babe-seed-here" ]]; then
    echo -e "${RED}❌ Error: Please replace placeholder seeds with actual seeds${NC}"
    echo ""
    echo "Edit this file and replace the placeholder seeds with your actual seeds from:"
    echo "  /home/mamostehp/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"
    echo ""
    exit 1
fi

# Function to insert key
insert_key() {
    local key_type=$1
    local seed=$2
    local scheme=$3

    echo -e "${GREEN}Inserting ${key_type} key...${NC}"

    curl -s -H "Content-Type: application/json" \
        -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"author_insertKey\",\"params\":[\"${key_type}\",\"${seed}\",\"${scheme}\"]}" \
        "$RPC_URL" | jq -r '.result // .error'

    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✅ ${key_type} key inserted${NC}"
    else
        echo -e "${RED}❌ Failed to insert ${key_type} key${NC}"
        return 1
    fi
}

echo "Inserting keys into validator at: $RPC_URL"
echo ""

# Insert BABE key (sr25519)
insert_key "babe" "$BABE_SEED" "0x9e7490cfe0dd32860282dd4e74b5c40c9237fb6e478066c334c931742308e008"

# Insert GRANDPA key (ed25519)
insert_key "gran" "$GRANDPA_SEED" "0x16da26f049b37e9b03d14439457ac164f14f5174b1f10ddab2e0dc3ef7903675"

# Insert Para Validator key (sr25519)
insert_key "para" "$PARA_VALIDATOR_SEED" "0xc62b764ea84411f550818a41a6abacddb9388cefbc22412f2bee335e9709a717"

# Insert Para Assignment key (sr25519)
insert_key "asgn" "$PARA_ASSIGNMENT_SEED" "0xa4c75d886439ae7d764cc600051af1d2128d2a8a1cdf3d642eda76824ccaf518"

# Insert Authority Discovery key (sr25519)
insert_key "audi" "$AUTHORITY_DISCOVERY_SEED" "0x66e821a23729878d8b7d04c69ec0fed9b0f5facae48917c1b83a4ad15cf00720"

# Insert BEEFY key (ecdsa)
insert_key "beef" "$BEEFY_SEED" "0x0281ba169080b261add0cd22bfb47477eddab854caa17423b12ddeed6504b04aef"

echo ""
echo -e "${GREEN}✅ All keys inserted for ${VALIDATOR_NAME}${NC}"
echo ""
echo -e "${YELLOW}📝 Next steps:${NC}"
echo "1. Restart the validator to apply the keys"
echo "2. Check validator is producing blocks"
echo "3. Monitor logs at /tmp/beta-validator-1.log"
