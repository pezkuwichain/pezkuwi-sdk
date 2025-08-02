#!/bin/bash

set -e

echo "🚀 Building PezkuwiChain for Production..."

# Clean previous builds
cargo clean

# Build optimized release
RUSTFLAGS="-C target-cpu=native" cargo build --release --features runtime-benchmarks

# Generate production chainspec
./target/release/pezkuwichain build-spec \
    --chain production \
    --raw \
    --disable-default-bootnode \
    > chainspecs/pezkuwichain-production-raw.json

# Build Docker image
docker build -f docker/Dockerfile.production -t pezkuwichain:production .

echo "✅ Production build complete!"
echo "📋 Next steps:"
echo "  1. Test chainspec: ./target/release/pezkuwichain --chain chainspecs/pezkuwichain-production-raw.json --tmp"
echo "  2. Deploy validators with docker run pezkuwichain:production"
echo "  3. Initialize automated scheduling after first block"