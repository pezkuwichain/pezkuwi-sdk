# Pezkuwi-SDK

**TeyrChain - The Kurdistan Blockchain Network**

A sovereign blockchain parachain built for the Kurdistan region on Polkadot SDK v1.15.6.

## Overview

**TeyrChain (تێیرچەین)** is a production-ready Substrate-based parachain runtime featuring:

- **11 Custom Pallets**: Presale, Governance, Education, Identity, Treasury, and more
- **Dual Token Economics**: HEZ (native) + PEZ (governance)
- **XCM Integration**: Cross-chain asset transfers (wUSDT from Asset Hub)
- **Democratic Governance**: Welati - Digital democracy system
- **Educational Platform**: Perwerde - Learning and certification
- **Identity System**: KYC and citizen verification
- **Economic Tools**: PEZ treasury and rewards system

## Key Features

### 🪙 Token Economics
- **HEZ Token**: Native gas token (inflationary, Polkadot SDK standard)
- **PEZ Token**: Governance token (5B fixed supply, presale distribution)
- **wUSDT**: Bridged USDT from Polkadot Asset Hub (Asset ID 1000)

### 🏛️ Custom Pallets
1. **Presale**: Multi-presale launchpad with soft/hard caps, vesting, bonus tiers
2. **Tiki**: NFT-based social and economic features
3. **Identity-KYC**: Decentralized identity and compliance
4. **Referral**: Incentivized referral system
5. **Perwerde**: Educational platform and certification
6. **Token Wrapper**: Asset wrapping (wUSDT, etc.)
7. **Welati**: Democratic governance and voting
8. **Staking Score**: Reputation-based staking rewards
9. **Trust**: Decentralized trust and reputation
10. **PEZ Treasury**: Community treasury management
11. **PEZ Rewards**: Staking and participation rewards

### 🌉 Cross-Chain
- XCM v5 implementation
- Asset Hub USDT bridge (reserve-backed)
- Polkadot/Kusama parachain ready
- HRMP channels for system parachains

## Documentation

- **[Whitepaper](./WHITEPAPER.md)**: Complete technical specification
- **Runtime**: `pezkuwi/runtime/parachain/` (TeyrChain runtime)
- **Pallets**: `pezkuwi/pallets/` (11 custom pallets)
- **Parachain Node**: `cumulus/pezkuwi-parachain/` (Cumulus collator)

## Quick Start

### Build from Source

```bash
# Clone repository
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk

# Build release binary
cargo build --release

# Binary location
./target/release/pezkuwi-parachain
```

### Run Local Development

```bash
# Start local relay chain (Alice + Bob validators)
# See scripts/devlocalfa-testnet/ for setup

# Start parachain collator
./target/release/pezkuwi-parachain \
  --collator \
  --alice \
  --chain=local \
  --base-path=/tmp/parachain/alice \
  --port 40333 \
  --rpc-port 8844 \
  -- \
  --chain=rococo-local \
  --port 30343 \
  --rpc-port 9977
```

## Network Stages

**Current:** Alfa Testnet (4 validators) 🔄
**Next:** Beta Testnet (8 validators) 📅
**Future:** Staging → Mainnet (Polkadot/Kusama parachain)

## Architecture

```
Polkadot/Kusama Relay Chain
    │
    └─── TeyrChain Parachain (ParaID TBD)
          ├─ Runtime: teyrchain-runtime
          ├─ Consensus: Aura + GRANDPA (via relay)
          ├─ Block Time: 6 seconds
          └─ 11 Custom Pallets
               ├─ XCM Bridge (Asset Hub USDT)
               └─ Democratic Governance
```

## Technology Stack

- **Framework**: Substrate (Polkadot SDK v1.15.6)
- **Parachain**: Cumulus
- **Language**: Rust
- **Runtime**: WASM compilation
- **Consensus**: Aura (PoA)
- **Finality**: GRANDPA (relay chain)

## Use Cases

1. **Token Launches**: Multi-presale platform with compliance
2. **Digital Governance**: Community voting and proposals (Welati)
3. **Education**: Online courses and certifications (Perwerde)
4. **Identity**: KYC and trust systems
5. **Cross-Chain Finance**: USDT bridge, asset swaps

## Official Links

- **Website**: https://pezkuwichain.io
- **Explorer**: https://explorer.pezkuwichain.io
- **RPC**: wss://rpc.pezkuwichain.io
- **Network Dashboard**: https://network.pezkuwichain.io

## Community

- **Telegram**: @pezkuwichain
- **Discord**: discord.gg/pezkuwichain
- **Twitter**: @pezkuwichain

## License

Apache 2.0 - Open Source

---

**Built for the Kurdish Nation**

*TeyrChain (تێیرچەین) - Empowering Kurdistan through blockchain technology*
