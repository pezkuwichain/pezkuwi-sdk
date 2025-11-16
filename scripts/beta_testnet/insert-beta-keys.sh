#!/bin/bash

BINARY="$HOME/Pezkuwi-SDK/target/release/pezkuwi"
CHAIN_SPEC="$HOME/Pezkuwi-SDK/chain-specs/beta/beta-testnet-raw.json"
VALIDATORS_JSON="$HOME/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"

# Key types: [json_field]="key_type:scheme"
declare -A KEY_TYPES=(
    ["babe"]="babe:sr25519"
    ["grandpa"]="gran:ed25519"
    ["para_validator"]="para:sr25519"
    ["para_assignment"]="asgn:sr25519"
    ["authority_discovery"]="audi:sr25519"
    ["beefy"]="beef:ecdsa"
)

echo "=== BETA TESTNET KEY INSERTION ==="
echo "Chain: $CHAIN_SPEC"
echo "Binary: $BINARY"
echo ""

VALIDATORS=$(jq -c '.beta[]' "$VALIDATORS_JSON")
VALIDATOR_INDEX=1

while IFS= read -r validator; do
    VALIDATOR_NAME=$(echo "$validator" | jq -r '.name')
    
    echo "===================="
    echo "VALIDATOR $VALIDATOR_INDEX: $VALIDATOR_NAME"
    echo "===================="
    
    BASE_PATH="$HOME/pezkuwi-data/beta-testnet/validator-$VALIDATOR_INDEX"
    mkdir -p "$BASE_PATH"
    
    for json_field in "${!KEY_TYPES[@]}"; do
        IFS=':' read -r key_type scheme <<< "${KEY_TYPES[$json_field]}"
        
        seed_field="${json_field}_seed"
        seed=$(echo "$validator" | jq -r ".$seed_field")
        
        echo "  Inserting $key_type ($scheme)..."
        
        if "$BINARY" key insert \
            --base-path "$BASE_PATH" \
            --chain "$CHAIN_SPEC" \
            --scheme "$scheme" \
            --suri "$seed" \
            --key-type "$key_type" 2>/dev/null; then
            echo "    ✓ $key_type inserted"
        else
            echo "    ✗ FAILED to insert $key_type"
            "$BINARY" key insert \
                --base-path "$BASE_PATH" \
                --chain "$CHAIN_SPEC" \
                --scheme "$scheme" \
                --suri "$seed" \
                --key-type "$key_type"
            exit 1
        fi
    done
    
    echo ""
    VALIDATOR_INDEX=$((VALIDATOR_INDEX + 1))
    
done <<< "$VALIDATORS"

echo "=== ALL KEYS INSERTED SUCCESSFULLY ==="
