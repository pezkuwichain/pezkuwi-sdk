#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <validator_number> [additional_args]"
    echo "Example: $0 1"
    exit 1
fi

VALIDATOR_NUM=$1
shift

BINARY="$HOME/Pezkuwi-SDK/target/release/pezkuwi"
CHAIN_SPEC="$HOME/Pezkuwi-SDK/chain-specs/beta/beta-testnet-raw.json"
BASE_PATH="$HOME/pezkuwi-data/beta-testnet/validator-$VALIDATOR_NUM"
VALIDATORS_JSON="$HOME/Pezkuwi-SDK/pezkuwi/runtime/validators/beta_testnet_validators.json"

# Get validator name
VALIDATOR_NAME=$(jq -r ".beta[$((VALIDATOR_NUM - 1))].name" "$VALIDATORS_JSON")

# Port configuration
RPC_PORT=$((9944 + VALIDATOR_NUM - 1))
WS_PORT=$((9944 + VALIDATOR_NUM - 1))
P2P_PORT=$((30333 + VALIDATOR_NUM - 1))

# Bootnode (Validator 1)
BOOTNODE="/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWRyg1V1ay7aFbHWdpzYMnT3Nk6RLdM8GceqVQzp1GoEgZ"

echo "=========================================="
echo "Starting Validator #$VALIDATOR_NUM: $VALIDATOR_NAME"
echo "=========================================="
echo "RPC Port: $RPC_PORT"
echo "WS Port:  $WS_PORT"
echo "P2P Port: $P2P_PORT"
echo "Base Path: $BASE_PATH"
echo "=========================================="

if [ "$VALIDATOR_NUM" -eq 1 ]; then
    # Validator 1 - no bootnode needed
    "$BINARY" \
        --base-path "$BASE_PATH" \
        --chain "$CHAIN_SPEC" \
        --name "$VALIDATOR_NAME" \
        --validator \
        --rpc-port $RPC_PORT \
        --port $P2P_PORT \
        --rpc-cors all \
        --rpc-external \
        --rpc-methods=Unsafe \
        --no-mdns \
        "$@"
else
    # Validator 2-8 - connect to Validator 1
    "$BINARY" \
        --base-path "$BASE_PATH" \
        --chain "$CHAIN_SPEC" \
        --name "$VALIDATOR_NAME" \
        --validator \
        --rpc-port $RPC_PORT \
        --port $P2P_PORT \
        --rpc-cors all \
        --rpc-external \
        --rpc-methods=Unsafe \
        --no-mdns \
        --bootnodes "$BOOTNODE" \
        "$@"
fi
