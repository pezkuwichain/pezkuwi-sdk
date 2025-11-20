#!/bin/bash

# Pezkuwi Rococo Deployment Script
# This script helps deploy Pezkuwi parachain to Rococo testnet

set -e

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;36m'
NC='\033[0m' # No Color

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SDK_DIR="$(dirname "$SCRIPT_DIR")"

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Pezkuwi Rococo Deployment${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if hex files exist
if [ ! -f "$SCRIPT_DIR/pezkuwi-genesis-state.hex" ]; then
    echo -e "${RED}Error: pezkuwi-genesis-state.hex not found${NC}"
    exit 1
fi

if [ ! -f "$SCRIPT_DIR/pezkuwi-runtime.wasm.hex" ]; then
    echo -e "${RED}Error: pezkuwi-runtime.wasm.hex not found${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Deployment files found${NC}"
echo -e "   Genesis state hex: $(wc -c < $SCRIPT_DIR/pezkuwi-genesis-state.hex) chars"
echo -e "   Runtime WASM hex: $(wc -c < $SCRIPT_DIR/pezkuwi-runtime.wasm.hex) chars"
echo ""

# Read hex files
GENESIS_HEX="0x$(cat $SCRIPT_DIR/pezkuwi-genesis-state.hex)"
WASM_HEX="0x$(cat $SCRIPT_DIR/pezkuwi-runtime.wasm.hex)"

echo -e "${BLUE}Deployment Information:${NC}"
echo -e "   Genesis hex preview: ${GENESIS_HEX:0:66}..."
echo -e "   WASM hex preview: ${WASM_HEX:0:66}..."
echo ""

# Ask for ParaId
echo -e "${YELLOW}Please provide your reserved ParaId from Rococo:${NC}"
echo -e "   (Visit https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io#/parachains/parathreads)"
read -p "ParaId: " PARA_ID

if [ -z "$PARA_ID" ]; then
    echo -e "${RED}Error: ParaId is required${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}Using ParaId: $PARA_ID${NC}"
echo ""

# Generate deployment info
cat > "$SCRIPT_DIR/deployment-info.txt" <<EOF
===========================================
Pezkuwi Rococo Deployment Information
===========================================

Para ID: $PARA_ID

Genesis State (hex):
$GENESIS_HEX

Validation Code (WASM hex):
$WASM_HEX

===========================================
Registration Instructions
===========================================

1. Go to Polkadot.js Apps (Rococo):
   https://polkadot.js.org/apps/?rpc=wss://rococo-rpc.polkadot.io

2. Navigate to: Developer → Sudo

3. Submit the following extrinsic:
   parasSudoWrapper.sudoScheduleParaInitialize(
     id: $PARA_ID,
     genesisHead: [paste Genesis State hex above],
     validationCode: [paste Validation Code hex above],
     paraKind: true
   )

4. Sign and submit the transaction

===========================================
Collator Node Command
===========================================

After parachain is registered, start the collator:

cd $SDK_DIR

./target/release/pezkuwi \\
  --collator \\
  --name "Pezkuwi-Collator-1" \\
  --base-path /tmp/pezkuwi-rococo \\
  --chain $SCRIPT_DIR/pezkuwi-rococo-raw.json \\
  --port 30333 \\
  --rpc-port 9944 \\
  --rpc-cors all \\
  --rpc-external \\
  --ws-external \\
  --prometheus-external \\
  --prometheus-port 9615 \\
  -- \\
  --chain rococo \\
  --port 30334 \\
  --rpc-port 9945 \\
  --execution wasm

===========================================
HRMP Channel Setup (Asset Hub)
===========================================

After collator is running, open HRMP channel to Asset Hub:

1. Connect to your Pezkuwi parachain via Polkadot.js Apps
2. Go to Developer → Extrinsics
3. Submit:
   xcmPallet.send(
     dest: { V3: { parents: 1, interior: Here } },
     message: {
       V3: [{
         Transact: {
           originKind: 'Native',
           requireWeightAtMost: { refTime: 1000000000, proofSize: 0 },
           call: {
             encoded: hrmp.hrmpInitOpenChannel(
               recipient: 1000,
               proposedMaxCapacity: 1000,
               proposedMaxMessageSize: 102400
             )
           }
         }
       }]
     }
   )

4. Wait for Asset Hub to accept the channel

===========================================
Next Steps
===========================================

See docs/XCM_ROCOCO_DEPLOYMENT.md for complete instructions
EOF

echo -e "${GREEN}✅ Deployment info saved to: deployment-info.txt${NC}"
echo ""
echo -e "${YELLOW}Next steps:${NC}"
echo -e "1. Review deployment-info.txt"
echo -e "2. Go to Polkadot.js Apps and register the parachain"
echo -e "3. Start the collator node using the command in deployment-info.txt"
echo -e "4. Setup HRMP channel with Asset Hub"
echo ""

# Ask if user wants to see the file
read -p "Display deployment-info.txt now? (y/n): " SHOW_INFO

if [ "$SHOW_INFO" = "y" ]; then
    cat "$SCRIPT_DIR/deployment-info.txt"
fi

echo ""
echo -e "${GREEN}Deployment preparation complete!${NC}"
