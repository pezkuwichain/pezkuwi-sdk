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

## Directory Mapping

| Path | Purpose |
|------|---------|
| `/pezkuwi/runtime/pezkuwichain/` | Main relay chain runtime (was Rococo) |
| `/pezkuwi/runtime/zagros/` | Canary network runtime (was Westend) |
| `/pezkuwi/runtime/teyrchains/` | Parachain runtime modules |
| `/pezkuwi/pallets/` | 12 custom pallets |

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