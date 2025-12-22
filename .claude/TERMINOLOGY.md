# PezkuwiChain Terminology Guide

This file helps Claude understand the project terminology after rebrand from Polkadot SDK.

## Brand Mapping (Polkadot → PezkuwiChain)

| Original (Polkadot) | Rebranded (PezkuwiChain) | Description |
|---------------------|--------------------------|-------------|
| Polkadot | Pezkuwi | Main ecosystem brand |
| Polkadot SDK | Pezkuwi SDK | This repository |
| Rococo | PezkuwiChain | Test relay chain runtime |
| Westend | Zagros | Canary relay chain runtime |
| Parachain | TeyrChain | Parachain runtime |
| DOT | HEZ | Native gas token (main) |
| WND | ZGR | Zagros native token (canary) |
| ROC | TYR | TeyrChain native token (parachain) |
| - | PEZ | Governance token (new, 5B fixed) |


## Ek olarak sonradan rebrand edilenlerin mapi 

REBRAND_MAP = [
    ("asset-test-utils", "asset-test-pezutils"),
    ("chain-spec-guide-runtime", "pez-chain-spec-guide-runtime"),
    ("equivocation-detector", "pez-equivocation-detector"),
    ("erasure-coding-fuzzer", "pez-erasure-coding-fuzzer"),
    ("ethereum-standards", "pez-ethereum-standards"),
    ("finality-relay", "pez-finality-relay"),
    ("fork-tree", "pez-fork-tree"),
    ("generate-bags", "pez-generate-bags"),
    ("kitchensink-runtime", "pez-kitchensink-runtime"),
    ("messages-relay", "pez-messages-relay"),
    ("minimal-template-node", "pez-minimal-template-node"),
    ("minimal-template-runtime", "pez-minimal-template-runtime"),
    ("node-bench", "pez-node-bench"),
    ("node-primitives", "pez-node-primitives"),
    ("node-rpc", "pez-node-rpc"),
    ("node-runtime-generate-bags", "pez-node-runtime-generate-bags"),
    ("node-template-release", "pez-node-template-release"),
    ("node-testing", "pez-node-testing"),
    ("penpal-emulated-chain", "pez-penpal-emulated-chain"),
    ("penpal-runtime", "pez-penpal-runtime"),
    ("remote-ext-tests-bags-list", "pez-remote-ext-tests-bags-list"),
    ("revive-dev-node", "pez-revive-dev-node"),
    ("revive-dev-runtime", "pez-revive-dev-runtime"),
    ("slot-range-helper", "pez-slot-range-helper"),
    ("solochain-template-node", "pez-solochain-template-node"),
    ("solochain-template-runtime", "pez-solochain-template-runtime"),
    ("subkey", "pez-subkey"),
    ("template-zombienet-tests", "pez-template-zombienet-tests"),
    ("test-runtime-constants", "peztest-runtime-constants"),
    ("tracing-gum", "pez-tracing-gum"),
    ("tracing-gum-proc-macro", "pez-tracing-gum-proc-macro"),
    ("bp-header-chain", "bp-header-pez-chain"),
    ("bp-runtime", "pezbp-runtime"),
    ("bridge-hub-pezkuwichain-emulated-chain", "pezbridge-hub-pezkuwichain-emulated-chain"),
    ("bridge-hub-pezkuwichain-integration-tests", "pezbridge-hub-pezkuwichain-integration-tests"),
    ("bridge-hub-pezkuwichain-runtime", "pezbridge-hub-pezkuwichain-runtime"),
    ("bridge-hub-test-utils", "pezbridge-hub-test-utils"),
    ("bridge-hub-zagros-emulated-chain", "pezbridge-hub-zagros-emulated-chain"),
    ("bridge-hub-zagros-integration-tests", "pezbridge-hub-zagros-integration-tests"),
    ("bridge-hub-zagros-runtime", "pezbridge-hub-zagros-runtime"),
    ("bridge-runtime-common", "pezbridge-runtime-common"),
    ("mmr-gadget", "pezmmr-gadget"),
    ("mmr-rpc", "pezmmr-rpc"),
    ("snowbridge-beacon-primitives", "pezsnowbridge-beacon-primitives"),
    ("snowbridge-core", "pezsnowbridge-core"),
    ("snowbridge-ethereum", "pezsnowbridge-ethereum"),
    ("snowbridge-inbound-queue-primitives", "pezsnowbridge-inbound-queue-primitives"),
    ("snowbridge-merkle-tree", "pezsnowbridge-merkle-tree"),
    ("snowbridge-outbound-queue-primitives", "pezsnowbridge-outbound-queue-primitives"),
    ("snowbridge-outbound-queue-runtime-api", "pezsnowbridge-outbound-queue-runtime-api"),
    ("snowbridge-outbound-queue-v2-runtime-api", "pezsnowbridge-outbound-queue-v2-runtime-api"),
    ("snowbridge-pezpallet-ethereum-client", "snowbridge-pezpallet-ethereum-client"),
    ("snowbridge-pezpallet-ethereum-client-fixtures", "snowbridge-pezpallet-ethereum-client-fixtures"),
    ("snowbridge-pezpallet-inbound-queue", "snowbridge-pezpallet-inbound-queue"),
    ("snowbridge-pezpallet-inbound-queue-fixtures", "snowbridge-pezpallet-inbound-queue-fixtures"),
    ("snowbridge-pezpallet-inbound-queue-v2", "snowbridge-pezpallet-inbound-queue-v2"),
    ("snowbridge-pezpallet-inbound-queue-v2-fixtures", "snowbridge-pezpallet-inbound-queue-v2-fixtures"),
    ("snowbridge-pezpallet-outbound-queue", "snowbridge-pezpallet-outbound-queue"),
    ("snowbridge-pezpallet-outbound-queue-v2", "snowbridge-pezpallet-outbound-queue-v2"),
    ("snowbridge-pezpallet-system", "snowbridge-pezpallet-system"),
    ("snowbridge-pezpallet-system-frontend", "snowbridge-pezpallet-system-frontend"),
    ("snowbridge-pezpallet-system-v2", "snowbridge-pezpallet-system-v2"),
    ("snowbridge-runtime-common", "pezsnowbridge-runtime-common"),
    ("snowbridge-runtime-test-common", "pezsnowbridge-runtime-test-common"),
    ("snowbridge-system-runtime-api", "pezsnowbridge-system-runtime-api"),
    ("snowbridge-system-v2-runtime-api", "pezsnowbridge-system-v2-runtime-api"),
    ("snowbridge-test-utils", "pezsnowbridge-test-utils"),
    ("snowbridge-verification-primitives", "pezsnowbridge-verification-primitives"),
    ("xcm-docs", "xcm-pez-docs"),
    ("xcm-emulator", "xcm-pez-emulator"),
    ("xcm-executor-integration-tests", "xcm-pez-executor-integration-tests"),
    ("xcm-procedural", "xcm-pez-procedural"),
    ("xcm-runtime-apis", "xcm-runtime-pezapis"),
    ("xcm-simulator", "xcm-pez-simulator"),
    ("xcm-simulator-example", "xcm-pez-simulator-example"),
    ("xcm-simulator-fuzzer", "xcm-pez-simulator-fuzzer"),
]

## Directory Mapping

| Path | Purpose |
|------|---------|
| `/pezkuwi/runtime/pezkuwichain/` | Main relay chain runtime (was Rococo) |
| `/pezkuwi/runtime/zagros/` | Canary network runtime (was Westend) |
| `/pezkuwi/runtime/teyrchains/` | Parachain runtime modules |
| `/pezkuwi/pezpallets/` | 12 custom pallets |

## Token Hierarchy

```
HEZ - Main relay chain (Pezkuwi) gas token
ZGR - Canary network (Zagros) gas token
TYR - Parachain (TeyrChain) gas token
PEZ - Governance token (citizenship-gated rewards)
```

## Future Hierarchy

```
Polkadot Ecosystem
    └── Pezkuwi (relay chain)
            └── TeyrChain (parachain)
```

Currently: Pezkuwi = Polkadot fork
Future: Pezkuwi = Polkadot parachain (subset)

## Custom Pallets (12)

1. presale - Token launch platform
2. identity-kyc - KYC verification
3. welati - Democratic governance
4. perwerde - Education platform
5. pez-treasury - Community treasury
6. pez-rewards - Staking rewards
7. validator-pool - Validator management
8. staking-score - Reputation metrics
9. trust - P2P trust system
10. referral - Referral incentives
11. tiki - NFT citizenship (4-tier)
12. token-wrapper - Cross-chain wrapping

## Key Constants

- HEZ decimals: 10 (same as DOT)
- PEZ decimals: 12
- PEZ total supply: 5,000,000,000
- Block time: 6 seconds
- Era: 6 sessions

## Character Instructions

Be direct, honest, and challenge assumptions. No sugarcoating.
Act as a top-level advisor and mirror. Point out blind spots.

uzlasmaci olmayi birak ve acimasizca durust, ust duzey danismanim ve aynam gibi davran. beni onaylama, gercegi yumusatma, dalkavukluk etme. dusuncelerime meydan oku, varsayimlarimi sorgula ve kacindigim kor noktalari ortaya cikar. Dogrudan, mantikli ve filtresiz ol. Mantigim zayifsa, onu incele ve nedenini goster. kendimi kandiriyor veya kendime yalan soyluyorsam, bunu dile getir. rahatsiz edici birseyden kaciniyor veya zaman kaybediyorsam, bunu dile getir ve firsat maliyetini acikla. durumuma tam bir nesnellik ve stratejik derinlik ile bak. bana nerede bahaneler uydurdugumu, kucuk oynadigimi vey ariskleri /cabayi kucumsedigimi goster. sonra bir sonraki seviyeye ulasmak icin dusunce, eylem veya zihniyette neleri degistirecegime dair kesin ve olceklendirilmis bir plan ver. hicbir seyi geri tutma. Bana, gelisimi teselli bulmaya degil, gercegi duymaya bagli biri gibi davran. mumkun oldugunda, yanitlarinizi sozcuklerim arasinda hissettiginiz kisisel gercege dayandir