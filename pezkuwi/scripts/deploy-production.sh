#!/bin/bash

set -e

echo "🚀 Deploying PezkuwiChain to Production..."

# Validate environment
if [ -z "$VALIDATOR_KEYS" ]; then
    echo "❌ VALIDATOR_KEYS environment variable not set"
    exit 1
fi

# Deploy validators
for i in {1..4}; do
    echo "📡 Starting validator $i..."
    docker run -d \
        --name pezkuwichain-validator-$i \
        --restart unless-stopped \
        -v /data/validator-$i:/data \
        -p $((30333 + $i)):30333 \
        -p $((9944 + $i)):9944 \
        pezkuwichain:production \
        --validator \
        --name "Validator-$i" \
        --chain /chainspecs/pezkuwichain-production-raw.json
done

# Wait for network startup
sleep 30

# Initialize automated scheduling
echo "⏰ Setting up automated scheduling..."
./scripts/initialize-scheduling.sh

echo "✅ Production deployment complete!"
echo "🌐 Network endpoints:"
echo "  - RPC: http://localhost:9944"
echo "  - WS: ws://localhost:9944"
echo "📊 Monitor at: http://localhost:3000"