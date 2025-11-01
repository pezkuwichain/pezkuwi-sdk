#!/bin/bash
# Setup assets and pool for Instance3 runtime

set -e

ALICE="//Alice"
ALICE_ADDR="5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
WS="ws://localhost:9944"

echo "🔧 Setting up assets and pool..."
echo ""

# Create wHEZ (Asset ID 0)
echo "📦 Creating wHEZ asset (ID: 0)..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assets.create 0 "$ALICE_ADDR" 1000000000000 2>&1 | grep -E "(inBlock|dispatchError)" || true
sleep 2

# Create PEZ (Asset ID 1)
echo "📦 Creating PEZ asset (ID: 1)..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assets.create 1 "$ALICE_ADDR" 1000000000000 2>&1 | grep -E "(inBlock|dispatchError)" || true
sleep 2

# Mint wHEZ to Alice
echo "💰 Minting 1,000,000 wHEZ to Alice..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assets.mint 0 "$ALICE_ADDR" "1000000000000000000" 2>&1 | grep -E "(inBlock|dispatchError)" || true
sleep 2

# Mint PEZ to Alice
echo "💰 Minting 1,000,000 PEZ to Alice..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assets.mint 1 "$ALICE_ADDR" "1000000000000000000" 2>&1 | grep -E "(inBlock|dispatchError)" || true
sleep 2

echo ""
echo "✅ Assets created and minted!"
echo ""

# Create pool
echo "🏊 Creating wHEZ/PEZ pool..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assetConversion.createPool 0 1 2>&1 | grep -E "(inBlock|PoolCreated|dispatchError)" || true
sleep 2

echo ""
echo "💧 Adding liquidity (100 wHEZ + 500 PEZ)..."
polkadot-js-api --ws $WS --seed "$ALICE" \
  tx.assetConversion.addLiquidity \
  0 \
  1 \
  "100000000000000" \
  "500000000000000" \
  "95000000000000" \
  "475000000000000" \
  "$ALICE_ADDR" 2>&1 | grep -E "(inBlock|LiquidityAdded|dispatchError)" || true

echo ""
echo "🎉 Setup complete!"
echo "   - wHEZ (0) and PEZ (1) assets created"
echo "   - Pool created with liquidity"
echo "   - Ready to test swaps!"
