# Kurdistan SDK — Sovereign Blockchain Infrastructure

**Developed and maintained by Kurdistan Tech Institute**

Kurdistan SDK is a fully independent blockchain development framework, providing all the tools needed to build sovereign, interoperable blockchain networks for stateless nations and distributed communities.

---

## Origin

Kurdistan SDK originated as a fork of Polkadot SDK (stable2512). It has since been completely rebranded and restructured as an independent project under Kurdistan Tech Institute stewardship. All crate names, dependencies, and documentation have been updated to reflect this independence.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Kurdistan SDK Ecosystem                   │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   ┌─────────────────────┐    ┌─────────────────────────┐    │
│   │   Bizinikiwi        │    │      PezCumulus         │    │
│   │   (Core Framework)  │    │     (Parachain SDK)     │    │
│   │                     │    │                         │    │
│   │  • Runtime Engine   │    │  • Collator Framework   │    │
│   │  • Consensus        │    │  • XCM Integration      │    │
│   │  • Networking       │    │  • Relay Chain Bridge   │    │
│   └─────────────────────┘    └─────────────────────────┘    │
│              │                          │                    │
│              └──────────┬───────────────┘                    │
│                         │                                    │
│              ┌──────────▼───────────┐                        │
│              │   PezkuwiChain       │                        │
│              │   (Reference Impl)   │                        │
│              │   • TeyrChain        │                        │
│              │   • Zagros Runtime   │                        │
│              └──────────────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Crate Naming Convention

Kurdistan SDK uses a distinct naming scheme to ensure complete independence:

| Component | Prefix | Example |
|-----------|--------|---------|
| Core Framework (ex-Bizinikiwi) | `bizinikiwi-` | `bizinikiwi-runtime` |
| Parachain SDK (ex-Pezcumulus) | `pezcumulus-` | `pezcumulus-client` |
| Client Crates | `pezsc-` | `pezsc-network`, `pezsc-consensus` |
| Primitives | `pezsp-` | `pezsp-runtime`, `pezsp-core` |
| Framework | `pezframe-` | `pezpezframe-support`, `pezpezframe-system` |
| Pallets | `pezpallet-` | `pezpallet-balances`, `pezpallet-staking` |
| Staging | `pezstaging-` | `pezstaging-xcm` |

---

## PezkuwiChain — Reference Implementation

PezkuwiChain is the flagship blockchain built on Kurdistan SDK, designed for Kurdish digital sovereignty.

### Token Economics

**HEZ — Native Gas Token**
- Purpose: Transaction fees, staking, network security
- Model: Inflationary with staking rewards
- Distribution: 85% staking rewards, 15% treasury

**PEZ — Governance Token**
- Total Supply: 5,000,000,000 PEZ
- Model: Fixed supply with halving cycles
- Access: Citizenship-gated rewards

### Custom Pallets

| Pallet | Purpose |
|--------|---------|
| `pezpallet-presale` | Multi-round token launches with vesting |
| `pezpallet-identity-kyc` | Decentralized identity verification |
| `pezpallet-welati` | Democratic governance and voting |
| `pezpallet-perwerde` | Educational platform and certificates |
| `pezpallet-pez-treasury` | Community treasury with halving |
| `pezpallet-pez-rewards` | Trust-based staking rewards |
| `pezpallet-validator-pool` | Validator participation |
| `pezpallet-staking-score` | Reputation-based metrics |
| `pezpallet-trust` | Peer-to-peer trust system |
| `pezpallet-referral` | Referral incentives |
| `pezpallet-tiki` | NFT-based citizenship |
| `pezpallet-token-wrapper` | Asset wrapping |

### TNPoS Consensus

Trust-enhanced Nominated Proof-of-Stake combines:
- Economic Stake (HEZ)
- Social Trust metrics
- Performance scoring

---

## Quick Start

```bash
# Clone repository
git clone https://github.com/pezkuwichain/kurdistan-sdk.git
cd kurdistan-sdk

# Build release
cargo build --release

# Build with benchmarks
cargo build --release --features runtime-benchmarks

# Run development node
./target/release/pezkuwi-node --dev
```

---

## Documentation

| Resource | URL |
|----------|-----|
| Main Docs | [docs.pezkuwichain.io](https://docs.pezkuwichain.io) |
| API Reference | [api.pezkuwichain.io](https://api.pezkuwichain.io) |
| Website | [pezkuwichain.io](https://pezkuwichain.io) |

---

## Community

| Platform | Link |
|----------|------|
| Telegram | [@pezkuwichain](https://t.me/pezkuwichain) |
| Discord | [discord.gg/pezkuwichain](https://discord.gg/pezkuwichain) |
| Twitter | [@pezkuwichain](https://twitter.com/pezkuwichain) |
| GitHub | [github.com/pezkuwichain](https://github.com/pezkuwichain) |

---

## Contributing

Kurdistan SDK is developed by Kurdistan Tech Institute with community contributions.

See [CONTRIBUTING.md](./docs/contributor/CONTRIBUTING.md) for guidelines.

---

## License

Kurdistan SDK is licensed under Apache 2.0. See [LICENSE](./LICENSE) for details.

---

<div align="center">

**Kurdistan Tech Institute**

*Building sovereign infrastructure for stateless nations*

</div>
```


## TNPoS Consensus — World's First Trust-Augmented PoS

**Trust-enhanced Nominated Proof-of-Stake** combines:
- **Economic Stake**: Traditional staking (HEZ)
- **Social Trust**: Peer endorsements, citizenship level, governance participation
- **Performance Metrics**: Uptime, finality participation, historical behavior

**Key Innovations**:
- 🔬 First implementation of social trust in consensus (academic contribution)
- 🏛️ Parliamentary NFT System: 201 non-transferable governance seats
- 🔐 Sybil-resistant via citizenship verification (Tiki pallet)
- 📊 Multi-dimensional validator scoring

---

## Cross-Chain Integration (XCM v5)

```
Polkadot Asset Hub (USDT)
         │
         │ XCM Reserve Transfer
         ▼
TeyrChain Parachain
         │
         ├─► Presale contributions
         ├─► DEX trading (future)
         └─► Withdraw back to Asset Hub
```

- Full XCM v5 implementation
- Reserve-backed wUSDT bridge
- HRMP channels for system teyrchains
- Future bridges: wETH, wBTC, Ethereum, Tron, BSC

---

## Network Roadmap

| Stage | Validators | Status |
|-------|------------|--------|
| Dev Mode | 1 | ✅ Complete |
| Local Testnet | 2 | ✅ Complete |
| **Alfa Testnet** | 4 | 🔄 Current |
| Beta Testnet | 8 | Q1 2026 |
| Staging | 20 | Q1 2026 |
| **Mainnet** | 100 | Q2 2026 |

---

## Multi-Nation Platform Vision

PezkuwiChain is designed to host **multiple digital nations** simultaneously:

| Nation Type | Example | Target Population |
|-------------|---------|-------------------|
| **Ethnic** | Kurdish (PEZ) | 40M+ globally |
| **Ethnic** | Catalan | 10M+ |
| **Ethnic** | Tibetan | 6M+ diaspora |
| **Ethnic** | Uyghur | 12M+ |
| **Ethnic** | Basque | 3M+ |
| **Cultural** | Armenian diaspora | 8M+ |
| **Cultural** | Romani | 10M+ worldwide |
| **Indigenous** | Various tribes | Land rights, heritage |

**Shared Infrastructure**: All nations use HEZ for transactions
**Nation-Specific Tokens**: Each group issues citizenship-gated tokens (PEZ model)
**Interoperable Governance**: Cross-nation commerce, diplomacy on single blockchain

---

## Use Cases

1. **Digital Governance** — Parliamentary NFTs, on-chain voting, treasury management
2. **Token Launches** — Compliant multi-round presales with vesting
3. **Education** — Blockchain-verified certificates (Perwerde)
4. **Identity** — Self-sovereign KYC and citizenship verification
5. **Cross-Chain Finance** — wUSDT bridge, DeFi primitives
6. **Cultural Preservation** — Language, heritage, oral tradition on-chain
7. **Remittance** — Low-cost diaspora→homeland transfers ($20B+ Kurdish annual flows)

---

## Quick Start

```bash
# Clone repository
git clone https://github.com/pezkuwichain/pezkuwi-sdk.git
cd pezkuwi-sdk

# Build release
cargo build --release

# Build with benchmarks
cargo build --release --features runtime-benchmarks

# Run local testnet
./target/release/pezkuwi-node --dev
```

---

## Links

| Resource | URL |
|----------|-----|
| Website | [pezkuwichain.io](https://pezkuwichain.io) |
| App | [pezkuwichain.app](https://pezkuwichain.app) |
| Explorer | [explorer.pezkuwichain.io](https://explorer.pezkuwichain.io) |
| Documentation | [docs.pezkuwichain.io](https://docs.pezkuwichain.io) |
| RPC Endpoint | `wss://rpc.pezkuwichain.io` |

## Community

| Platform | Link |
|----------|------|
| Telegram | [@pezkuwichain](https://t.me/pezkuwichain) |
| Discord | [discord.gg/pezkuwichain](https://discord.gg/pezkuwichain) |
| Twitter | [@pezkuwichain](https://twitter.com/pezkuwichain) |
| GitHub | [github.com/pezkuwichain](https://github.com/pezkuwichain) |
| Medium | [@pezkuwichain](https://medium.com/@pezkuwichain) |
| Facebook | [PezkuwiChain](https://www.facebook.com/profile.php?id=61582484611719) |

---

<div align="center">

![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_White.png#gh-dark-mode-only)
![SDK Logo](./docs/images/Pezkuwi_Logo_Horizontal_Pink_Black.png#gh-light-mode-only)

# Pezkuwi SDK

![GitHub stars](https://img.shields.io/github/stars/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub
forks](https://img.shields.io/github/forks/pezkuwichain/pezkuwi-sdk)

<!-- markdownlint-disable-next-line MD013 -->
[![StackExchange](https://img.shields.io/badge/StackExchange-Community%20&%20Support-222222?logo=stackexchange)](https://pezkuwichain.app/community)&nbsp;&nbsp;![GitHub contributors](https://img.shields.io/github/contributors/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub commit activity](https://img.shields.io/github/commit-activity/m/pezkuwichain/pezkuwi-sdk)&nbsp;&nbsp;![GitHub last commit](https://img.shields.io/github/last-commit/pezkuwichain/pezkuwi-sdk)

> The Pezkuwi SDK repository provides all the components needed to start building on the
> [PezkuwiChain](https://pezkuwichain.app/) network, a multi-chain blockchain platform that enables
> different blockchains to interoperate and share information in a secure and scalable way.

</div>

## ⚡ Quickstart
If you want to get an example node running quickly you can execute the following getting started script:

```
curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/pezkuwichain/pezkuwi-sdk/master/scripts/getting-started.sh | bash
```

## 👩🏽‍💻 Building

In order to build this project you need to install some dependencies, follow the instructions in [this guide](https://docs.pezkuwichain.io/develop/teyrchains/install-pezkuwi-sdk).

## 📚 Documentation

- [Pezkuwi Documentation Portal](https://docs.pezkuwichain.io)
- [rust-docs](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/index.html): Where we keep track of
the API docs of our Rust crates. Includes:
  - [Introduction](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/index.html)
    to each component of the Pezkuwi SDK: Bizinikiwi, FRAME, Pezcumulus, and XCM
  - [Guides](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/guides/index.html),
    namely how to build your first FRAME pallet
  - [Templates](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/pezkuwi_sdk/templates/index.html)
    for starting a new project.
  - [External Resources](https://pezkuwichain.github.io/pezkuwi-sdk/master/pezkuwi_sdk_docs/external_resources/index.html)
- Have a question? You can ask in the Pezkuwi SDK Developers Chat.
Messages from either of these channels are bridged to the other, so you can use whichever one you like.
  - [Telegram](https://t.me/pezkuwidevs)
  - [Matrix](https://matrix.to/#/#pezkuwidevs:matrix.org)
  - [Discord](https://discord.gg/Y3VyEC6h8W)
  - [Pezkuwi and Bizinikiwi StackExchange](https://pezkuwichain.app/community)

## 🚀 Releases

<!-- markdownlint-disable-next-line MD013 -->
![Current Stable Release](https://raw.githubusercontent.com/pezkuwichain/release-registry/main/badges/pezkuwi-sdk-latest.svg)&nbsp;&nbsp;![Next Stable Release](https://raw.githubusercontent.com/pezkuwichain/release-registry/main/badges/pezkuwi-sdk-next.svg)

The Pezkuwi SDK is released every three months as a `Pezkuwi stableYYMM` release. Each stable release is supported for
one year with patches. See the next upcoming versions in the [Release
Registry](https://github.com/pezkuwichain/release-registry/) and more docs in [RELEASE.md](./docs/RELEASE.md).

You can use [`psvm`](https://github.com/pezkuwichain/psvm) to update all dependencies to a specific
version without needing to manually select the correct version for each crate.

## 🛠️ Tooling

[Pezkuwi SDK Version Manager](https://github.com/pezkuwichain/psvm):
A simple tool to manage and update the Pezkuwi SDK dependencies in any Cargo.toml file.
It will automatically update the Pezkuwi SDK dependencies to their correct crates.io version.

## 🔐 Security

The security policy and procedures can be found in
[docs/contributor/SECURITY.md](./docs/contributor/SECURITY.md).

## 🤍 Contributing & Code of Conduct

Ensure you follow our [contribution guidelines](./docs/contributor/CONTRIBUTING.md). In every
interaction and contribution, this project adheres to the [Contributor Covenant Code of
Conduct](./docs/contributor/CODE_OF_CONDUCT.md).

### 👾 Ready to Contribute?

Take a look at the issues labeled with [`mentor`](https://github.com/pezkuwichain/pezkuwi-sdk/labels/C1-mentor)
(or alternatively [this](https://mentor.tasty.limo/) page, created by one of the maintainers) label to get started!
We always recognize valuable contributions by proposing an on-chain tip to the PezkuwiChain network as a token of our
appreciation.

## Pezkuwi Fellowship

Development in this repo usually goes hand in hand with the `fellowship` organization. In short,
this repository provides all the SDK pieces needed to build both PezkuwiChain and its teyrchains. But,
the actual PezkuwiChain runtime lives in the `fellowship/runtimes` repository. Read more about the
fellowship, this separation, the RFC process
[here](https://pezkuwi-fellows.github.io/dashboard/).

## History

This repository is the amalgamation of 3 separate repositories that used to make up Pezkuwi SDK,
namely Bizinikiwi, Pezkuwi and Pezcumulus. Read more about the merge and its history
[here](https://pezkuwi-public.notion.site/Pezkuwi-SDK-FAQ-fbc4cecc2c46443fb37b9eeec2f0d85f).
