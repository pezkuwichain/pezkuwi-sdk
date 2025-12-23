# PezkuwiChain Infrastructure

## Domains

### Primary Domain
| Property | Value |
|----------|-------|
| Domain | pezkuwichain.io |
| IP | 37.60.230.9 |
| TTL | 3600 |

### Service Subdomains
| Subdomain | Full URL | Description |
|-----------|----------|-------------|
| www | www.pezkuwichain.io | Main website |
| network | network.pezkuwichain.io | Network portal |
| api | api.pezkuwichain.io | API endpoint |
| rpc | rpc.pezkuwichain.io | RPC endpoint |
| ws | ws.pezkuwichain.io | WebSocket endpoint |
| explorer | explorer.pezkuwichain.io | Block explorer |
| docs | docs.pezkuwichain.io | Documentation |
| wiki | wiki.pezkuwichain.io | Wiki |
| wiki.network | wiki.network.pezkuwichain.io | Network wiki |
| faucet | faucet.pezkuwichain.io | Test token faucet |
| telemetry | telemetry.pezkuwichain.io | Telemetry dashboard |
| forum | forum.pezkuwichain.io | Community forum |
| grants | grants.pezkuwichain.io | Grant program |
| developers | developers.pezkuwichain.io | Developer portal |
| mail | mail.pezkuwichain.io | Email service |

### Network RPC Endpoints
| Network | Subdomain | Full URL |
|---------|-----------|----------|
| Mainnet | mainnet | mainnet.pezkuwichain.io |
| Mainnet | pezkuwichain-rpc | pezkuwichain-rpc.pezkuwichain.io |
| Zagros | zagros | zagros.pezkuwichain.io |
| Zagros | zagros-rpc | zagros-rpc.pezkuwichain.io |
| Zagros Asset Hub | zagros-asset-hub-rpc | zagros-asset-hub-rpc.pezkuwichain.io |
| Zagros Bridge Hub | zagros-bridge-hub-rpc | zagros-bridge-hub-rpc.pezkuwichain.io |
| Zagros Collectives | zagros-collectives-rpc | zagros-collectives-rpc.pezkuwichain.io |
| Zagros Coretime | zagros-coretime-rpc | zagros-coretime-rpc.pezkuwichain.io |
| Testnet | testnet | testnet.pezkuwichain.io |
| Staging | staging | staging.pezkuwichain.io |
| Beta | beta | beta.pezkuwichain.io |
| Beta RPC | beta-rpc | beta-rpc.pezkuwichain.io |
| Try Runtime | try-runtime | try-runtime.pezkuwichain.io |
| Try Runtime Zagros | try-runtime-zagros | try-runtime-zagros.pezkuwichain.io |

---

## GitHub Repositories

**Organization:** https://github.com/pezkuwichain

### Core SDK Repositories
| Repository | Description | Language | Upstream |
|------------|-------------|----------|----------|
| pezkuwi-sdk | Digital state for Kurdish nation on blockchain | Rust | paritytech/polkadot-sdk |
| polkadot-sdk | The Parity Polkadot Blockchain SDK (fork) | Rust | paritytech/polkadot-sdk |
| pezkuwi-subxt | Interact with Substrate based nodes | Rust | paritytech/subxt |
| pezkuwi-zombienet-sdk | ZombieNet SDK | Rust | paritytech/zombienet-sdk |

### Template Repositories
| Repository | Description | Language | Upstream |
|------------|-------------|----------|----------|
| pezkuwi-sdk-minimal-template | Minimal Testing/Learning Template | Rust | paritytech/polkadot-sdk-minimal-template |
| pezkuwi-sdk-solochain-template | Solochain-Ready Template | Rust | paritytech/polkadot-sdk-solochain-template |
| pezkuwi-sdk-teyrchain-template | Teyrchain template for Pezkuwi SDK | Rust | paritytech/polkadot-sdk-parachain-template |
| polkadot-runtime-templates | Runtime Templates for Pezkuwi Teyrchains | Rust | - |

### Library Forks
| Repository | Description | Language | Upstream |
|------------|-------------|----------|----------|
| parity-db | Experimental blockchain database | Rust | paritytech/parity-db |
| parity-scale-codec | Binary serialization codec | Rust | paritytech/parity-scale-codec |
| pezframe-metadata | FRAME metadata parser tools | Rust | paritytech/frame-metadata |
| merkle-mountain-range | MMR implementation | Rust | nervosnetwork/merkle-mountain-range |
| wasm-instrument | WASM module instrumentation | Rust | parity/wasm-instrument |
| consensus | Consensus for Web3 | TeX | parity/consensus |

### Community & Documentation
| Repository | Description | Language | Upstream |
|------------|-------------|----------|----------|
| awesome-hez | Awesome resources in Polkadot ecosystem | - | parity/awesome-substrate |
| pezkuwi-fellows | Fellowship proposals | Handlebars | polkadot-fellows/rfc |
| kurdistan_blockchain-akademy | Blockchain Academy Rust Qualifier | Rust | polkadot-blockchain-academy |
| docs | PezkuwiChain documentation | Rust | - |
| labels | Label rules for repositories | Shell | paritytech/labels |

### Applications
| Repository | Description | Language |
|------------|-------------|----------|
| DKSweb | Sovereign Blockchain for Kurdish Nation | TypeScript |
| pezkuwi-mobile-app | Official Mobile App | TypeScript |
| pwap | Web and mobile applications monorepo | TypeScript |
| pezkuwi-validator-v1.0.0 | Validator installer scripts | Shell |
| testnet | Testnet configuration | Shell |
| workflow-stopper | CI/CD workflow management | - |

---

## URL Mapping Reference

### Repositories We Own (use pezkuwichain URLs)
```
github.com/pezkuwichain/pezkuwi-sdk
github.com/pezkuwichain/pezkuwi-subxt
github.com/pezkuwichain/pezkuwi-zombienet-sdk
github.com/pezkuwichain/pezkuwi-fellows
github.com/pezkuwichain/awesome-hez
github.com/pezkuwichain/parity-db
github.com/pezkuwichain/parity-scale-codec
github.com/pezkuwichain/pezframe-metadata
github.com/pezkuwichain/merkle-mountain-range
github.com/pezkuwichain/wasm-instrument
github.com/pezkuwichain/kurdistan_blockchain-akademy
```

### External Repositories (use original URLs)
| Our Reference | Original URL |
|---------------|--------------|
| xcm-format | github.com/polkadot-fellows/xcm-format |
| try-runtime-cli | github.com/paritytech/try-runtime-cli |
| trie | github.com/paritytech/trie |
| srtool | github.com/paritytech/srtool |
| parity-common | github.com/paritytech/parity-common |
| arkworks-extensions | github.com/paritytech/arkworks-extensions |
| chainextension-registry | github.com/parity/chainextension-registry |
| substrate-developer-hub | github.com/substrate-developer-hub |

### External Websites (use original URLs)
| Our Reference | Original URL |
|---------------|--------------|
| shawntabrizi.com/bizinikiwi-* | shawntabrizi.com/substrate-* |
| pezkuwichain.github.io | paritytech.github.io |

---

## Code Usage Examples

```rust
// Website URLs
"https://pezkuwichain.io"
"https://network.pezkuwichain.io"
"https://docs.pezkuwichain.io"

// RPC Endpoints
"wss://rpc.pezkuwichain.io"         // Mainnet WS
"wss://zagros-rpc.pezkuwichain.io"  // Zagros WS

// GitHub Repositories
"https://github.com/pezkuwichain/pezkuwi-sdk"
"https://github.com/pezkuwichain/awesome-hez"
```

---

*Last updated: 2025-12-23*
