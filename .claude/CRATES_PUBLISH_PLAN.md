# Crates.io Publish Plan - Pezkuwi SDK

**Tarih:** 2025-12-27
**Toplam Workspace Crate:** 606
**Publishable Crate:** 453
**Not Publishable (internal/test):** 153
**Registry:** https://crates.io
**Owner:** https://crates.io/users/SatoshiQaziMuhammed

---

## Mevcut Durum

| Durum | Sayı | Version |
|-------|------|---------|
| Placeholder (crates.io'da mevcut) | 130 | 0.1.0 |
| Yeni publish edilecek | 323 | 0.44.0 |
| **Toplam publish edilecek** | **453** | |

**Not:** 130 placeholder crate gerçek içerikle değiştirilecek (0.1.0 → 0.44.0 version bump)

---

## Kategori Dağılımı

| Prefix | Sayı | Açıklama |
|--------|------|----------|
| pezsp-* | 54 | Substrate Primitives (en düşük seviye) |
| pezsc-* | 55 | Substrate Client |
| pezframe-* | 17 | FRAME support |
| pezpallet-* | 128 | Pallets |
| pezcumulus-* | 34 | Cumulus (parachain support) |
| pezkuwi-* | 64 | Polkadot node components |
| pezsnowbridge-* | 22 | Snowbridge (Ethereum bridge) |
| bp-* | 16 | Bridge primitives |
| pezbridge-* | 4 | Bridge hub runtimes |
| pezbp-* | 1 | Bridge runtime primitives |
| pezmmr-* | 2 | Merkle Mountain Range |
| pezstaging-* | 7 | Staging/test tools |
| zombienet-* | 6 | Zombienet SDK |
| bizinikiwi-* | 9 | Bizinikiwi utilities |
| xcm-* | 4 | XCM tools |
| *-runtime | 17 | Runtime crates |
| Other | 30 | Diğer |

---

## Publish Sırası (Dependency Order)

### LEVEL 0: Core Primitives (Bağımsız)
Bu crate'ler dış bağımlılığı olmayan veya sadece external crate'lere bağımlı.

```
pezsp-std
pezsp-panic-handler
pezsp-tracing
pezsp-debug-derive
pezsp-externalities
pezsp-storage
pezsp-core-hashing
pezsp-core-hashing-proc-macro
pezsp-crypto-hashing
pezsp-crypto-hashing-proc-macro
pezsp-wasm-interface
```

### LEVEL 1: Core Runtime Primitives
pezsp-std ve temel primitive'lere bağımlı.

```
pezsp-arithmetic
pezsp-io
pezsp-runtime-interface
pezsp-runtime-interface-proc-macro
pezsp-core
pezsp-keyring
pezsp-weights
pezsp-version
pezsp-version-proc-macro
pezsp-application-crypto
pezsp-runtime
pezsp-staking
pezsp-state-machine
pezsp-trie
pezsp-database
pezsp-maybe-compressed-blob
```

### LEVEL 2: Extended Primitives
Level 1'e bağımlı primitives.

```
pezsp-api
pezsp-api-proc-macro
pezsp-blockchain
pezsp-consensus
pezsp-consensus-aura
pezsp-consensus-babe
pezsp-consensus-beefy
pezsp-consensus-grandpa
pezsp-consensus-pow
pezsp-consensus-slots
pezsp-inherents
pezsp-keystore
pezsp-offchain
pezsp-session
pezsp-timestamp
pezsp-transaction-pool
pezsp-genesis-builder
pezsp-block-builder
pezsp-authority-discovery
pezsp-npos-elections
pezsp-mmr-primitives
pezsp-mixnet
pezsp-metadata-ir
pezsp-rpc
pezsp-statement-store
pezsp-transaction-storage-proof
pezsp-crypto-ec-utils
```

### LEVEL 3: FRAME Support
pezsp-* primitive'lere bağımlı.

```
pezframe-support-procedural-tools
pezframe-support-procedural-tools-derive
pezframe-support-procedural
pezframe-support
pezframe-system
pezframe-system-rpc-runtime-api
pezframe-metadata
pezframe-metadata-hash-extension
pezframe-executive
pezframe-election-provider-solution-type
pezframe-election-provider-support
pezframe-try-runtime
pezframe-benchmarking
pezframe-benchmarking-pezpallet-pov
pezframe-system-benchmarking
pezframe-benchmarking-cli
pezframe-omni-bencher
```

### LEVEL 4: Basic Pallets (No Cross-Pallet Deps)
pezframe-*'a bağımlı, diğer pallet'lere bağımlı olmayan.

```
pezpallet-aura
pezpallet-authorship
pezpallet-authority-discovery
pezpallet-babe
pezpallet-balances
pezpallet-collective
pezpallet-democracy
pezpallet-elections-phragmen
pezpallet-grandpa
pezpallet-identity
pezpallet-im-online
pezpallet-indices
pezpallet-lottery
pezpallet-membership
pezpallet-multisig
pezpallet-nfts
pezpallet-nfts-runtime-api
pezpallet-offences
pezpallet-preimage
pezpallet-proxy
pezpallet-recovery
pezpallet-remark
pezpallet-scheduler
pezpallet-session
pezpallet-session-benchmarking
pezpallet-society
pezpallet-sudo
pezpallet-timestamp
pezpallet-tips
pezpallet-transaction-payment
pezpallet-transaction-payment-rpc
pezpallet-transaction-payment-rpc-runtime-api
pezpallet-treasury
pezpallet-uniques
pezpallet-utility
pezpallet-vesting
pezpallet-whitelist
pezpallet-insecure-randomness-collective-flip
pezpallet-message-queue
pezpallet-migrations
pezpallet-paged-list
pezpallet-parameters
pezpallet-safe-mode
pezpallet-scored-pool
pezpallet-state-trie-migration
pezpallet-statement
pezpallet-tx-pause
pezpallet-verify-signature
```

### LEVEL 5: Advanced Pallets (Cross-Pallet Deps)
Diğer pallet'lere bağımlı.

```
pezpallet-assets
pezpallet-assets-freezer
pezpallet-assets-holder
pezpallet-asset-conversion
pezpallet-asset-conversion-ops
pezpallet-asset-conversion-tx-payment
pezpallet-asset-rate
pezpallet-asset-rewards
pezpallet-asset-tx-payment
pezpallet-atomic-swap
pezpallet-bags-list
pezpallet-beefy
pezpallet-beefy-mmr
pezpallet-bounties
pezpallet-broker
pezpallet-child-bounties
pezpallet-collator-selection
pezpallet-collective-content
pezpallet-contracts
pezpallet-contracts-mock-network
pezpallet-contracts-proc-macro
pezpallet-contracts-uapi
pezpallet-conviction-voting
pezpallet-core-fellowship
pezpallet-delegated-staking
pezpallet-derivatives
pezpallet-election-provider-multi-block
pezpallet-election-provider-multi-phase
pezpallet-election-provider-support-benchmarking
pezpallet-fast-unstake
pezpallet-glutton
pezpallet-mmr
pezpallet-multi-asset-bounties
pezpallet-nft-fractionalization
pezpallet-nis
pezpallet-node-authorization
pezpallet-nomination-pools
pezpallet-nomination-pools-benchmarking
pezpallet-nomination-pools-runtime-api
pezpallet-offences-benchmarking
pezpallet-oracle
pezpallet-oracle-runtime-api
pezpallet-origin-restriction
pezpallet-people
pezpallet-ranked-collective
pezpallet-referenda
pezpallet-root-offences
pezpallet-root-testing
pezpallet-salary
pezpallet-skip-feeless-payment
pezpallet-staking
pezpallet-staking-reward-curve
pezpallet-staking-reward-fn
pezpallet-staking-runtime-api
```

### LEVEL 6: Substrate Client
pezsp-* ve bazı pezframe-*'a bağımlı.

```
pezsc-allocator
pezsc-utils
pezsc-tracing
pezsc-tracing-proc-macro
pezsc-telemetry
pezsc-sysinfo
pezsc-executor-common
pezsc-executor-polkavm
pezsc-executor-wasmtime
pezsc-executor
pezsc-state-db
pezsc-keystore
pezsc-client-api
pezsc-client-db
pezsc-block-builder
pezsc-proposer-metrics
pezsc-informant
pezsc-chain-spec
pezsc-chain-spec-derive
pezsc-transaction-pool-api
pezsc-transaction-pool
pezsc-offchain
pezsc-network-types
pezsc-network-common
pezsc-network
pezsc-network-gossip
pezsc-network-light
pezsc-network-statement
pezsc-network-sync
pezsc-network-transactions
pezsc-consensus
pezsc-consensus-slots
pezsc-consensus-aura
pezsc-consensus-babe
pezsc-consensus-babe-rpc
pezsc-consensus-beefy
pezsc-consensus-beefy-rpc
pezsc-consensus-epochs
pezsc-consensus-grandpa
pezsc-consensus-grandpa-rpc
pezsc-consensus-manual-seal
pezsc-consensus-pow
pezsc-authority-discovery
pezsc-basic-authorship
pezsc-storage-monitor
pezsc-statement-store
pezsc-mixnet
pezsc-rpc-api
pezsc-rpc-server
pezsc-rpc
pezsc-rpc-spec-v2
pezsc-sync-state-rpc
pezsc-runtime-utilities
pezsc-service
pezsc-cli
```

### LEVEL 7: Bridge Primitives
pezsp-* ve pezframe-*'a bağımlı.

```
bp-header-pez-chain
bp-messages
bp-relayers
bp-test-utils
bp-teyrchains
bp-pezkuwi-core
bp-pezkuwichain
bp-zagros
bp-pezkuwi-bulletin
bp-asset-hub-pezkuwichain
bp-asset-hub-zagros
bp-bridge-hub-pezcumulus
bp-bridge-hub-pezkuwichain
bp-bridge-hub-zagros
bp-xcm-bridge-hub
bp-xcm-bridge-hub-router
pezbp-runtime
```

### LEVEL 8: Bridge Pallets
bp-* ve pezpallet-*'a bağımlı.

```
pezpallet-bridge-grandpa
pezpallet-bridge-messages
pezpallet-bridge-relayers
pezpallet-bridge-teyrchains
pezpallet-xcm
pezpallet-xcm-benchmarks
pezpallet-xcm-bridge-hub
pezpallet-xcm-bridge-hub-router
```

### LEVEL 9: Cumulus Primitives
pezsp-*, pezframe-* ve pezpallet-*'a bağımlı.

```
pezcumulus-primitives-core
pezcumulus-primitives-aura
pezcumulus-primitives-proof-size-hostfunction
pezcumulus-primitives-storage-weight-reclaim
pezcumulus-primitives-teyrchain-inherent
pezcumulus-primitives-timestamp
pezcumulus-primitives-utility
```

### LEVEL 10: Cumulus Pallets & Client

```
pezcumulus-pezpallet-aura-ext
pezcumulus-pezpallet-dmp-queue
pezcumulus-pezpallet-session-benchmarking
pezcumulus-pezpallet-solo-to-para
pezcumulus-pezpallet-teyrchain-system
pezcumulus-pezpallet-teyrchain-system-proc-macro
pezcumulus-pezpallet-weight-reclaim
pezcumulus-pezpallet-xcm
pezcumulus-pezpallet-xcmp-queue
pezcumulus-client-bootnodes
pezcumulus-client-cli
pezcumulus-client-collator
pezcumulus-client-consensus-aura
pezcumulus-client-consensus-common
pezcumulus-client-consensus-proposer
pezcumulus-client-consensus-relay-chain
pezcumulus-client-network
pezcumulus-client-pov-recovery
pezcumulus-client-service
pezcumulus-client-teyrchain-inherent
pezcumulus-relay-chain-inprocess-interface
pezcumulus-relay-chain-interface
pezcumulus-relay-chain-minimal-node
pezcumulus-relay-chain-rpc-interface
pezcumulus-relay-chain-streams
pezcumulus-ping
pezcumulus-pov-validator
```

### LEVEL 11: Snowbridge

```
pezsnowbridge-beacon-primitives
pezsnowbridge-core
pezsnowbridge-ethereum
pezsnowbridge-inbound-queue-primitives
pezsnowbridge-merkle-tree
pezsnowbridge-outbound-queue-primitives
pezsnowbridge-verification-primitives
pezsnowbridge-outbound-queue-runtime-api
pezsnowbridge-outbound-queue-v2-runtime-api
pezsnowbridge-system-runtime-api
pezsnowbridge-system-v2-runtime-api
pezsnowbridge-pezpallet-ethereum-client
pezsnowbridge-pezpallet-inbound-queue
pezsnowbridge-pezpallet-inbound-queue-v2
pezsnowbridge-pezpallet-outbound-queue
pezsnowbridge-pezpallet-outbound-queue-v2
pezsnowbridge-pezpallet-system
pezsnowbridge-pezpallet-system-frontend
pezsnowbridge-pezpallet-system-v2
pezsnowbridge-runtime-common
pezsnowbridge-runtime-test-common
pezsnowbridge-test-utils
```

### LEVEL 12: Polkadot Node Components

```
pezkuwi-core-primitives
pezkuwi-primitives
pezkuwi-primitives-test-helpers
pezkuwi-pez-node-primitives
pezkuwi-runtime-common
pezkuwi-runtime-metrics
pezkuwi-runtime-teyrchains
pezkuwi-teyrchain-primitives
pez-tracing-gum
pez-tracing-gum-proc-macro
pezkuwi-erasure-coding
pezkuwi-statement-table
pezkuwi-node-metrics
pezkuwi-node-network-protocol
pezkuwi-node-subsystem-types
pezkuwi-node-subsystem
pezkuwi-node-subsystem-util
pezkuwi-overseer
pezkuwi-node-core-approval-voting
pezkuwi-node-core-approval-voting-parallel
pezkuwi-node-core-av-store
pezkuwi-node-core-backing
pezkuwi-node-core-bitfield-signing
pezkuwi-node-core-candidate-validation
pezkuwi-node-core-chain-api
pezkuwi-node-core-chain-selection
pezkuwi-node-core-dispute-coordinator
pezkuwi-node-core-prospective-teyrchains
pezkuwi-node-core-provisioner
pezkuwi-node-core-pvf
pezkuwi-node-core-pvf-checker
pezkuwi-node-core-pvf-common
pezkuwi-node-core-pvf-execute-worker
pezkuwi-node-core-pvf-prepare-worker
pezkuwi-node-core-runtime-api
pezkuwi-node-core-teyrchains-inherent
pezkuwi-approval-distribution
pezkuwi-availability-bitfield-distribution
pezkuwi-availability-distribution
pezkuwi-availability-recovery
pezkuwi-collator-protocol
pezkuwi-dispute-distribution
pezkuwi-gossip-support
pezkuwi-network-bridge
pezkuwi-node-collation-generation
pezkuwi-statement-distribution
pezkuwi-rpc
pezkuwi-service
pezkuwi-cli
pezkuwi-omni-node-lib
pezkuwi-omni-node
pezkuwi-teyrchain-bin
```

### LEVEL 13: Revive & Staking Async

```
pezpallet-revive-proc-macro
pezpallet-revive-uapi
pezpallet-revive
pezpallet-revive-eth-rpc
pezpallet-staking-async
pezpallet-staking-async-ah-client
pezpallet-staking-async-rc-client
pezpallet-staking-async-rc-runtime-constants
pezpallet-staking-async-reward-fn
pezpallet-staking-async-runtime-api
pezpallet-ah-ops
pezpallet-meta-tx
pezpallet-dummy-dim
pezpallet-assets-precompiles
pezpallet-xcm-precompiles
```

### LEVEL 14: XCM & Staging

```
pezstaging-xcm
pezstaging-xcm-builder
pezstaging-xcm-executor
xcm-pez-procedural
xcm-runtime-pezapis
xcm-pez-emulator
xcm-pez-simulator
pezstaging-chain-spec-builder
pezstaging-node-inspect
pezstaging-teyrchain-info
pezstaging-tracking-allocator
```

### LEVEL 15: Utilities & MMR

```
pezmmr-gadget
pezmmr-rpc
pez-fork-tree
pez-generate-bags
pez-slot-range-helper
pez-subkey
pez-ethereum-standards
pez-revive-dev-runtime
binary-merkle-tree
bizinikiwi-bip39
bizinikiwi-build-script-utils
bizinikiwi-frame-rpc-support
bizinikiwi-frame-rpc-system
bizinikiwi-prometheus-endpoint
bizinikiwi-rpc-client
bizinikiwi-state-trie-migration-rpc
bizinikiwi-wasm-builder
bizinikiwi-txtesttool
frame-remote-externalities
frame-storage-access-test-runtime
pezkuwi-voter-bags
pezkuwi-ss58-registry
```

### LEVEL 16: Subxt

```
pezkuwi-subxt-core
pezkuwi-subxt-metadata
pezkuwi-subxt-codegen
pezkuwi-subxt-macro
pezkuwi-subxt-lightclient
pezkuwi-subxt-rpcs
pezkuwi-subxt-signer
pezkuwi-subxt-utils-fetchmetadata
pezkuwi-subxt-utils-stripmetadata
pezkuwi-subxt
```

### LEVEL 17: Zombienet SDK

```
zombienet-configuration
zombienet-prom-metrics-parser
zombienet-provider
zombienet-support
zombienet-orchestrator
zombienet-sdk
```

### LEVEL 18: Common & Bridge Runtime

```
assets-common
bridge-hub-common
teyrchains-common
testnet-teyrchains-constants
teyrchains-runtimes-test-utils
pezbridge-runtime-common
pezbridge-hub-test-utils
```

### LEVEL 19: Runtimes

```
pezkuwichain-runtime-constants
zagros-runtime-constants
pezkuwichain-runtime
zagros-runtime
asset-hub-zagros-runtime
collectives-zagros-runtime
coretime-pezkuwichain-runtime
coretime-zagros-runtime
glutton-zagros-runtime
people-zagros-runtime
pezbridge-hub-pezkuwichain-runtime
pezbridge-hub-zagros-runtime
pezkuwichain-teyrchain-runtime
yet-another-teyrchain-runtime
asset-test-pezutils
```

### LEVEL 20: Umbrella Crates (En Son)

```
pezkuwi-sdk-frame
pezkuwi-sdk
pezkuwi
```

---

## Publish Komutu

Her crate için:
```bash
cargo publish -p <crate-name> --registry crates-io
```

veya dry-run test:
```bash
cargo publish -p <crate-name> --dry-run
```

---

## Publish Script

```bash
#!/bin/bash
# publish_all.sh

CRATES=(
  # LEVEL 0
  "pezsp-std"
  "pezsp-panic-handler"
  # ... (tüm crate'ler sırayla)
)

for crate in "${CRATES[@]}"; do
  echo "Publishing $crate..."
  cargo publish -p "$crate" --registry crates-io
  if [ $? -ne 0 ]; then
    echo "ERROR: Failed to publish $crate"
    exit 1
  fi
  sleep 30  # Rate limiting için
done
```

---

## Önemli Notlar

1. **Crates.io Token:** `CARGO_REGISTRY_TOKEN` environment variable gerekli
2. **Rate Limiting:** Her publish arasında 30 saniye bekle
3. **Dry Run:** Önce tüm crate'leri `--dry-run` ile test et
4. **Version:** Workspace version: 0.44.0 (bazı crate'ler farklı version kullanabilir)
5. **Metadata:** Her crate'in `description`, `license`, `repository` alanları dolu olmalı

---

## Tahmini Süre

- 453 crate × 30 saniye = 226.5 dakika (~4 saat)
- Hata düzeltmeleri için ek süre gerekebilir

---

*Son güncelleme: 2025-12-27*
