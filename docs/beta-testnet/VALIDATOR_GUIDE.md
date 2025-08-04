# PezkuwiChain Beta Testnet - Validator Guide

## Validator Requirements
- 4+ CPU cores, 8GB+ RAM, 100GB+ SSD  
- Ubuntu 20.04+ or similar Linux
- Rust toolchain and Git

## Quick Setup

### 1. Build Node
cd PezkuwiChain
cargo build --release

### 2. Generate Keys  
./target/release/pezkuwi key generate-node-key --chain pezkuwichain-beta-testnet --base-path ~/validator-data

### 3. Start Validator
./target/release/pezkuwi --chain pezkuwichain-beta-testnet --base-path ~/validator-data --validator --name "My-Validator" --rpc-port 9944 --port 30333

## Check Status
- Look for "Imported #X" in logs
- Monitor peer connections  
- Verify block production

Community channels coming soon.
