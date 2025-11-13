#!/bin/bash

set -e  # Exit on error

echo "=========================================="
echo "🚀 BETA TESTNET - CLEAN START"
echo "=========================================="
echo ""

# Get script directory
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR"

# Step 1: Stop any running validators
echo "1️⃣  Stopping any running validators..."
bash stop-beta-validators.sh 2>/dev/null || true
sleep 2
echo "   ✓ All validators stopped"
echo ""

# Step 2: Clean all chain data
echo "2️⃣  Cleaning chain data..."
rm -rf /tmp/beta-validator-{1..8}
rm -f /tmp/beta-validator-*.log
echo "   ✓ Chain data cleaned"
echo ""

# Step 3: Start FIRST validator (to create genesis)
echo "3️⃣  Starting first validator (genesis creator)..."
bash start-beta-validator-1.sh &
sleep 5
echo "   ✓ Validator 1 started"
echo ""

# Step 4: Check if first validator is running
echo "4️⃣  Checking first validator..."
if ! pgrep -f "pezkuwichain.*validator-1" > /dev/null; then
    echo "   ❌ ERROR: First validator failed to start!"
    echo "   Check logs: /tmp/beta-validator-1.log"
    exit 1
fi
echo "   ✓ First validator is running"
echo ""

# Step 5: Wait for RPC to be ready
echo "5️⃣  Waiting for RPC endpoint..."
for i in {1..30}; do
    if curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}' http://localhost:9944 > /dev/null 2>&1; then
        echo "   ✓ RPC endpoint is ready"
        break
    fi
    if [ $i -eq 30 ]; then
        echo "   ❌ ERROR: RPC endpoint not responding after 30 seconds!"
        exit 1
    fi
    sleep 1
done
echo ""

# Step 6: Start remaining validators
echo "6️⃣  Starting remaining validators (2-8)..."
for i in {2..8}; do
    bash start-beta-validator-$i.sh &
    echo "   ✓ Validator $i started"
    sleep 1
done
echo ""

# Step 7: Wait for all validators to start their RPC endpoints
echo "7️⃣  Waiting for all validator RPC endpoints..."
sleep 5
echo "   ✓ All validators should be running"
echo ""

# Step 8: Insert validator keys
echo "8️⃣  Inserting validator keys..."
echo "" | bash insert-all-beta-keys.sh
echo "   ✓ All validator keys inserted"
echo ""

# Step 9: Wait for block production
echo "9️⃣  Waiting for block production..."
for i in {1..60}; do
    BLOCK=$(curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"chain_getHeader"}' http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | cut -d'"' -f4)
    if [ ! -z "$BLOCK" ] && [ "$BLOCK" != "0x0" ]; then
        BLOCK_NUM=$((16#${BLOCK#0x}))
        echo "   ✓ Block production started! Current block: #$BLOCK_NUM"
        break
    fi
    if [ $i -eq 60 ]; then
        echo "   ❌ ERROR: No blocks produced after 60 seconds!"
        echo "   Check logs: /tmp/beta-validator-*.log"
        exit 1
    fi
    sleep 1
done
echo ""

# Step 10: Wait for finalized blocks
echo "🔟 Waiting for block finalization..."
sleep 10
for i in {1..30}; do
    FINALIZED=$(curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead"}' http://localhost:9944 2>/dev/null | grep -o '"result":"0x[^"]*"' | cut -d'"' -f4)
    if [ ! -z "$FINALIZED" ]; then
        FINALIZED_HEADER=$(curl -s -H "Content-Type: application/json" -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$FINALIZED\"]}" http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | cut -d'"' -f4)
        if [ ! -z "$FINALIZED_HEADER" ] && [ "$FINALIZED_HEADER" != "0x0" ]; then
            FINALIZED_NUM=$((16#${FINALIZED_HEADER#0x}))
            echo "   ✓ Blocks are being finalized! Current finalized: #$FINALIZED_NUM"
            break
        fi
    fi
    if [ $i -eq 30 ]; then
        echo "   ⚠️  WARNING: Blocks not finalized yet, but chain is producing blocks"
        echo "   This may be normal - finalization can take time"
        break
    fi
    sleep 1
done
echo ""

# Step 11: Final status check
echo "🎉 BETA TESTNET STARTED SUCCESSFULLY!"
echo "=========================================="
echo ""
echo "📊 Chain Status:"
BEST=$(curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"chain_getHeader"}' http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | cut -d'"' -f4)
BEST_NUM=$((16#${BEST#0x}))
echo "   Best block:      #$BEST_NUM"

FINALIZED=$(curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"chain_getFinalizedHead"}' http://localhost:9944 2>/dev/null | grep -o '"result":"0x[^"]*"' | cut -d'"' -f4)
FINALIZED_HEADER=$(curl -s -H "Content-Type: application/json" -d "{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[\"$FINALIZED\"]}" http://localhost:9944 2>/dev/null | grep -o '"number":"0x[^"]*"' | cut -d'"' -f4)
FINALIZED_NUM=$((16#${FINALIZED_HEADER#0x}))
echo "   Finalized block: #$FINALIZED_NUM"
echo ""

PEERS=$(curl -s -H "Content-Type: application/json" -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}' http://localhost:9944 2>/dev/null | grep -o '"peers":[0-9]*' | cut -d':' -f2)
echo "   Connected peers: $PEERS"
echo ""

echo "📁 Logs: /tmp/beta-validator-*.log"
echo "🛑 Stop: cd $SCRIPT_DIR && bash stop-beta-validators.sh"
echo ""
echo "=========================================="
