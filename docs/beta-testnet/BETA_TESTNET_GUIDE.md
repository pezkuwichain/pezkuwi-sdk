# PezkuwiChain Beta Testnet Guide

## 🚀 Overview

PezkuwiChain Beta Testnet is a live testing environment featuring:
- **Token**: HEZ (12 decimals)
- **Genesis Supply**: 200M HEZ + 5B PEZ tokens
- **Validators**: 8-validator network
- **Block Time**: 6 seconds

## 📱 Wallet Support

Compatible with all Substrate/Polkadot wallets:
- Polkadot.js Extension
- Talisman Wallet
- SubWallet

## 🔗 Network Details

- **Chain ID**: pezkuwichain_beta_testnet
- **SS58 Format**: 42
- **RPC Endpoint**: ws://localhost:9944 (local)

## 🛠️ Getting Started

### Run Local Node
```bash
cd PezkuwiChain
./target/release/pezkuwi --chain pezkuwichain-beta-testnet --tmp --alice
```

### Connect with Polkadot.js Apps
1. Open https://polkadot.js.org/apps
2. Custom endpoint: ws://localhost:9944
3. Start testing!

## ⚠️ Testnet Notice
This is a testing environment. Tokens have no value.
