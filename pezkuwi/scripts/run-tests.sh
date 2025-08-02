#!/bin/bash

set -e

echo "🧪 Running PezkuwiChain Test Suite..."

# Runtime tests
echo "📋 Running runtime tests..."
cargo test -p pezkuwichain-runtime --release

# Economic integration tests
echo "💰 Running economic integration tests..."
cargo test -p pezkuwichain-runtime economic_integration_tests --release

# Pallet tests
echo "🔧 Running pallet tests..."
cargo test -p pallet-pez-treasury --release
cargo test -p pallet-pez-rewards --release  
cargo test -p pallet-trust --release
cargo test -p pallet-welati --release

# Benchmarks (opsiyonel)
if [ "$1" = "--bench" ]; then
    echo "⚡ Running benchmarks..."
    cargo test -p pezkuwichain-runtime --release --features runtime-benchmarks
fi

echo "✅ All tests passed!"