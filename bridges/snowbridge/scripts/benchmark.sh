#!/usr/bin/env bash

# Example command for updating pallet benchmarking
pushd ../cumulus
cargo run --release --bin pezkuwi-teyrchain \
--features runtime-benchmarks \
-- \
benchmark pallet \
--chain=bridge-hub-pezkuwichain-dev \
--pallet=snowbridge_pallet_ethereum_client \
--extrinsic="*" \
--execution=wasm --wasm-execution=compiled \
--steps 50 --repeat 20 \
--output ./teyrchains/runtimes/bridge-hubs/bridge-hub-pezkuwichain/src/weights/snowbridge_pallet_ethereum_client.rs
popd
