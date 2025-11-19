# PezkuwiChain Whitepaper
**Version:** 3.0
**Date:** November 17, 2025
**Status:** Production Ready - Beta Testnet Operational
**Previous Version:** November 13, 2025 (v2.0)

---

## Table of Contents

1. [Executive Summary](#executive-summary)
2. [Vision & Mission](#vision--mission)
3. [Problem Statement](#problem-statement)
4. [Competitive Landscape](#competitive-landscape)
5. [Solution Architecture](#solution-architecture)
6. [Consensus Mechanism (TNPoS)](#consensus-mechanism-tnpos)
7. [Current Implementation](#current-implementation)
8. [Future Development](#future-development)
9. [Governance Model](#governance-model)
10. [Economic Model (Dual-Token)](#economic-model-dual-token)
11. [Pre-sale & Initial Distribution](#pre-sale--initial-distribution)
12. [Technical Specifications](#technical-specifications)
13. [Environmental Sustainability](#environmental-sustainability)
14. [Risks & Mitigations](#risks--mitigations)
15. [Roadmap](#roadmap)

---

## Executive Summary

PezkuwiChain is a Substrate-based blockchain designed to serve as a **digital state infrastructure** for the Kurdish people. It provides decentralized governance, economic sovereignty, and digital citizenship in a censorship-resistant, transparent, and democratic framework.

**Core Principles:**
- Decentralized democratic governance
- Economic independence and transparency
- Digital identity and citizenship
- Cultural preservation and heritage protection
- Censorship-resistant communication
- Community-driven development

**Current Status:**
- ✅ Production-ready blockchain infrastructure
- ✅ 11 custom pallets deployed and operational
- ✅ 8 validators running on beta testnet (VPS production environment)
- ✅ Frontend (Web + Mobile) 90%+ complete
- ✅ 6-language internationalization (Kurdish, Turkish, Arabic, Persian, English)
- ✅ Comprehensive operational documentation
- ✅ XCM cross-chain infrastructure ready

---

## Vision & Mission

### Vision
To create the world's first **stateless digital nation** powered by blockchain technology, providing Kurdish people worldwide with democratic governance, economic tools, and cultural preservation mechanisms regardless of geographic location.

### Mission
Build a decentralized infrastructure that:
1. Enables direct democratic participation for all Kurdish people
2. Provides economic tools for financial sovereignty
3. Preserves Kurdish language, culture, and heritage
4. Connects diaspora communities with their roots
5. Operates transparently and cannot be censored or controlled by any centralized authority

---

## Problem Statement

### 1. Lack of State Sovereignty
Kurdish people, despite being one of the largest stateless nations (40+ million), lack a unified governance structure and political representation.

### 2. Economic Challenges
- Remittance costs from diaspora are high (5-10% fees)
- Limited access to banking and financial services
- Economic dependency on surrounding states
- Lack of transparent public finance management

### 3. Cultural Erosion
- Language suppression in various regions
- Limited preservation of cultural heritage
- Disconnection between diaspora and homeland
- Risk of losing oral traditions and historical records

### 4. Democratic Deficit
- Limited political participation mechanisms
- Lack of transparent governance structures
- No unified decision-making framework
- Centralized control by various authorities

**Solution:** A blockchain-based digital state that addresses these challenges through decentralization, transparency, and community governance.

---

## Competitive Landscape

### Comparison with Leading Platforms

| Feature | PezkuwiChain | Ethereum | Polkadot | Cardano |
|---------|--------------|----------|----------|---------|
| **Consensus** | TNPoS (Trust-enhanced NPoS) | PoS (Casper FFG) | NPoS (GRANDPA+BABE) | Ouroboros PoS |
| **Block Time** | 6 seconds | 12-14 seconds | 6 seconds | 20 seconds |
| **Finality** | ~30 seconds | ~15 minutes | ~30 seconds | ~5 minutes |
| **TPS Target** | 100+ | 15-30 | 1000+ (parachains) | 250+ |
| **Governance** | Parliamentary + Direct Democracy | Off-chain (EIPs) | On-chain (OpenGov) | On-chain (Voltaire) |
| **Forkless Upgrades** | ✅ Yes (Substrate) | ❌ No (Hard forks) | ✅ Yes | ✅ Yes |
| **Smart Contracts** | Native Pallets | Solidity (EVM) | Ink! (WASM) | Plutus/Marlowe |
| **Identity System** | Built-in (Citizenship NFTs) | External (ENS, etc.) | Built-in (Identity pallet) | Atala PRISM |
| **Treasury** | Automated, Governance-controlled | External (DAOs) | Built-in | Built-in |
| **Focus** | Digital State Infrastructure | General-purpose | Interoperability | Academic rigor |
| **Energy Efficiency** | High (PoS) | High (PoS) | High (PoS) | High (PoS) |
| **Carbon Footprint** | ~0.001 tCO₂/year | ~0.01 tCO₂/year | ~0.001 tCO₂/year | ~0.001 tCO₂/year |

### Key Differentiators

**1. Trust-enhanced Consensus (TNPoS)**
- World's first implementation of social trust in validator selection
- Combines staking with citizenship reputation scores
- Prevents Sybil attacks through identity verification

**2. Parliamentary NFT System**
- 201 unique NFTs providing governance rights
- 10% of staking rewards allocated to parliament members
- Non-transferable to prevent vote buying

**3. Built for Stateless Nations**
- Purpose-built for communities without geographic sovereignty
- Cultural preservation mechanisms (language, heritage)
- Diaspora-focused economic tools (remittances, cooperatives)

**4. Dual-Token Economy**
- HEZ: Inflationary staking token
- PEZ: Fixed-supply governance token (5 billion)
- Clear separation of concerns

**5. XCM-Ready Architecture**
- Full XCM (Cross-Consensus Messaging) implementation (306 lines)
- Parachain-ready infrastructure from day one
- Seamless bridge capability to Ethereum, Bitcoin, Polkadot ecosystem
- Future-proof for cross-chain interoperability

---

## Solution Architecture

### Blockchain Foundation
**Technology Stack:** Substrate Framework (Polkadot SDK)

**Why Substrate?**
- Battle-tested by Polkadot ecosystem
- Forkless upgradability (runtime upgrades)
- High performance (100+ tps target)
- Interoperability with other chains
- Robust security guarantees
- Active developer community

### Core Architecture Layers

```
┌─────────────────────────────────────────────────┐
│         Application Layer (dApps)               │
│  Mobile App, Web UI, Third-party Applications   │
└─────────────────────────────────────────────────┘
                      ▼
┌─────────────────────────────────────────────────┐
│         Runtime Layer (Pallets)                 │
│  Governance │ Treasury │ Citizenship │ Future   │
└─────────────────────────────────────────────────┘
                      ▼
┌─────────────────────────────────────────────────┐
│         Consensus Layer (GRANDPA + BABE)        │
│     Validator Network, Block Production         │
└─────────────────────────────────────────────────┘
                      ▼
┌─────────────────────────────────────────────────┐
│         Network Layer (P2P)                     │
│     Libp2p, Gossip Protocol, Sync               │
└─────────────────────────────────────────────────┘
```

---

## Consensus Mechanism (TNPoS)

### Trust-enhanced Nominated Proof-of-Stake

PezkuwiChain implements the world's first **Trust-enhanced Nominated Proof-of-Stake (TNPoS)** consensus mechanism, which augments traditional NPoS with social reputation and identity verification.

### How TNPoS Works

#### 1. Validator Selection Process

```
┌─────────────────────────────────────────────────┐
│  Step 1: Validator Nomination                  │
│  - Token holders nominate validators           │
│  - Stake HEZ tokens to support candidates      │
│  - Minimum stake: 10,000 HEZ                    │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│  Step 2: Trust Score Calculation               │
│  - Citizenship verification (Tiki pallet)       │
│  - Historical behavior analysis                 │
│  - Community reputation                         │
│  Trust Score = f(citizenship, uptime, slashing) │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│  Step 3: Weighted Validator Selection          │
│  Final Score = (Stake × 0.7) + (Trust × 0.3)   │
│  - Top validators by Final Score selected      │
│  - Minimum validator set: 4                     │
│  - Target validator set: 8-16                   │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│  Step 4: Block Production (BABE)               │
│  - Validators produce blocks in 6-second slots  │
│  - VRF-based slot assignment                    │
│  - Multiple validators per epoch                │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│  Step 5: Finality (GRANDPA)                    │
│  - Byzantine Fault Tolerant finality            │
│  - 66% supermajority required                   │
│  - Finalization in ~30 seconds                  │
└─────────────────────────────────────────────────┘
```

#### 2. Trust Score Components

**Formula:**
```
TrustScore(v) = 0.4 × CitizenshipLevel(v)
              + 0.3 × UptimeScore(v)
              + 0.2 × CommunityEndorsements(v)
              + 0.1 × GovernanceParticipation(v)
```

**Components:**
- **Citizenship Level** (0-100): Based on Tiki pallet verification
  - Applicant: 0
  - Citizen: 50
  - Legislator: 75
  - Core Team: 100

- **Uptime Score** (0-100): Historical validator availability
  - Last 30 days block production rate
  - Penalized for downtime

- **Community Endorsements** (0-100): Number of unique citizen nominators
  - Each verified citizen nomination adds weight
  - Prevents Sybil attacks through identity verification

- **Governance Participation** (0-100): Involvement in Welati governance
  - Proposal votes
  - Referendum participation
  - Parliament membership

#### 3. Validator Economics

**Block Rewards:**
```
Total Block Reward: 100 HEZ per block (adjustable via governance)

Distribution:
├── 70% → Validator operator
├── 20% → Nominators (proportional to stake)
└── 10% → Parliamentary NFT holders (201 NFTs)
```

**Parliamentary NFT System:**
- **Collection ID:** 100
- **Total NFTs:** 201 (representing parliament seats)
- **NFT IDs:** 1-201
- **Characteristics:**
  - Non-transferable (soulbound)
  - Issued through governance election
  - 10% of all block rewards distributed to holders
  - Provides voting rights in Welati pallet
  - Automatic rewards via Treasury pallet

**Slashing Conditions:**
- **Double-signing:** 100% of stake slashed
- **Unresponsiveness:** 0.1% per missed block
- **Malicious behavior:** Up to 100% (governance decision)

#### 4. Security Advantages

**Compared to Traditional PoS:**
| Aspect | Traditional PoS | TNPoS (PezkuwiChain) |
|--------|----------------|---------------------|
| Sybil Resistance | Stake-based only | Stake + Identity verification |
| Validator Quality | Economic only | Economic + Reputation |
| Long-term Alignment | Limited | Strong (citizenship requirement) |
| Attack Cost | Buy stake | Buy stake + Build reputation |
| Community Trust | Low | High (verified identities) |

**Byzantine Fault Tolerance:**
- Can tolerate up to 33% of validators being malicious
- GRANDPA finality ensures no chain reversions
- Trust scores make it harder to become a malicious validator

### TNPoS vs NPoS Comparison

```
Traditional NPoS:           TNPoS (PezkuwiChain):
═══════════════════         ═══════════════════════════════
Stake → Selection           Stake + Trust → Selection
                           │
Anyone can stake    →       Citizenship verification required
                           │
Economic game only  →       Economic + Social game
                           │
No identity check   →       KYC via Tiki pallet
                           │
No parliament       →       201 Parliamentary NFTs
                           │
Simple rewards      →       Multi-tier rewards (Validators,
                           Nominators, Parliament)
```

### Technical Constants (From Implementation)

```rust
// From runtime/pezkuwichain/src/lib.rs
const BABE_GENESIS_EPOCH_CONFIG: sp_consensus_babe::BabeEpochConfiguration =
    sp_consensus_babe::BabeEpochConfiguration {
        c: (1, 4),  // 25% secondary slots
        allowed_slots: sp_consensus_babe::AllowedSlots::PrimaryAndSecondaryVRFSlots,
    };

// Block time: 6 seconds
const MILLISECS_PER_BLOCK: u64 = 6000;

// Session length: 10 minutes (100 blocks)
const SESSION_LENGTH: BlockNumber = 100;

// Era length: 1 hour (600 blocks)
const ERA_LENGTH: BlockNumber = 600;

// Epoch length: 10 minutes (same as session)
const EPOCH_DURATION_IN_BLOCKS: BlockNumber = 100;

// Minimum validator count: 4
const MIN_VALIDATOR_COUNT: u32 = 4;

// Maximum validator count: 16 (adjustable via governance)
const MAX_VALIDATOR_COUNT: u32 = 16;
```

### Parliamentary NFT Technical Details

```rust
// From runtime configuration
const PARLIAMENT_COLLECTION_ID: u32 = 100;
const TOTAL_PARLIAMENT_SEATS: u32 = 201;
const PARLIAMENT_REWARD_PERCENTAGE: Percent = Percent::from_percent(10);

// NFT Distribution in block rewards
fn distribute_block_rewards(total_reward: Balance) {
    let validator_share = total_reward * 70 / 100;      // 70%
    let nominator_share = total_reward * 20 / 100;      // 20%
    let parliament_share = total_reward * 10 / 100;     // 10%

    // Parliament share divided among 201 NFT holders
    let per_parliament_member = parliament_share / 201;
}
```

---

## Current Implementation

### Pallet Overview

PezkuwiChain consists of 17 specialized pallets (11 custom production-ready, 6 future planned):

#### **Custom Pallets (Production - Deployed)**

| # | Pallet Name | Runtime Index | Purpose | Status | Frontend Integration |
|---|------------|---------------|---------|--------|---------------------|
| 1 | **pallet-tiki** | 42 | Citizenship NFTs & Role-based access (45 role types) | ✅ Production | ✅ Web + Mobile |
| 2 | **pallet-welati** | 75 | Democratic governance, elections, parliament (201 seats) | ✅ Production | ✅ Web + Mobile |
| 3 | **pallet-perwerde** | 48 | Education platform (courses, enrollment, achievements) | ✅ Production | ✅ Web |
| 4 | **pallet-identity-kyc** | 46 | KYC verification, citizenship application workflow | ✅ Production | ✅ Web + Mobile |
| 5 | **pallet-referral** | 47 | Referral system, community growth incentives | ✅ Production | ✅ Web + Mobile |
| 6 | **pallet-trust** | 69 | Composite trust score (staking + referral + education + citizenship) | ✅ Production | ✅ Web + Mobile |
| 7 | **pallet-staking-score** | 49 | Time-weighted staking metrics for trust calculation | ✅ Production | ✅ Web + Mobile |
| 8 | **pallet-validator-pool** | 103 | Validator selection (3 categories: Stake, Parliamentary, Merit) | ✅ Production | ✅ Web |
| 9 | **pallet-pez-treasury** | 101 | Treasury management, automated monthly releases | ✅ Production | ✅ Web |
| 10 | **pallet-pez-rewards** | 102 | Monthly PEZ distribution (90% citizens, 10% parliament) | ✅ Production | ✅ Web + Mobile |
| 11 | **pallet-token-wrapper** | 76 | HEZ ↔ wHEZ conversion for DEX trading | ✅ Production | ✅ Web |

#### **Future Pallets (Planned)**

| # | Pallet Name | Purpose | Timeline |
|---|------------|---------|----------|
| 12 | **pallet-free-media** | Censorship-resistant journalism & content publishing | Q2 2026 |
| 13 | **pallet-heritage** | Cultural preservation, language content, historical archives | Q3 2026 |
| 14 | **pallet-cooperatives** | Worker cooperative management, collective decision-making | Q4 2026 |
| 15 | **pallet-diaspora** | Diaspora connectivity, remittance channels | 2027 |
| 16 | **pallet-education** | Educational credentials, decentralized learning | 2027-2028 |
| 17 | **pallet-privacy** | Zero-knowledge proofs, private voting (research phase) | 2028+ |

**Current Implementation Status:**
- **Custom Pallets Deployed:** 11/17 (65%)
- **Total Runtime Pallets:** 77 (11 custom + 66 standard Substrate)
- **Beta Testnet:** 8 validators operational
- **Block Production:** 6-second block time, ~30-second finality
- **Network Health:** 100% uptime (7 days+)
- **Frontend Integration:** 90% Web, 95% Mobile

---

### Phase 1: Core Infrastructure ✅ COMPLETED

#### 1. Pallet Tiki (Citizenship & Roles)
**Runtime Index:** 42 | **Status:** ✅ Production Ready

**Purpose:** Digital citizenship and role-based access control system

**Features:**
- **Citizenship NFTs:** Non-transferable soulbound tokens (Collection ID: 100)
- **45 Role Types:** Complete governance structure
  - Governance: Serok (President), SerokWeziran (Prime Minister), 8 Minister types
  - Judicial: Dadger (Prosecutor), Dozger (Judge), Hiquqnas (Lawyer)
  - Administrative: Qeydkar (Registrar), Xezinedar (Treasurer)
  - Educational: Mamoste (Teacher), Perwerdekar (Educator), Rewsenbîr (Intellectual)
  - Parliamentary: Parlementer (Parliament Member), SerokiMeclise (Speaker)
  - Community: Axa (Elder), Pêseng (Leader), Hekem (Wise), ModeratorêCivakê (Moderator)
- **Automatic NFT Minting:** Upon KYC approval
- **Role Granting:** Via elections (Welati) or merit-based achievements
- **Max 20 roles** per user

**Integration:** Hooks into pallet-identity-kyc for automatic citizenship NFT minting

---

#### 2. Pallet Welati (Democratic Governance)
**Runtime Index:** 75 | **Status:** ✅ Production Ready

**Purpose:** Complete democratic governance framework with elections and legislative system

**Features:**
- **Election Types:**
  - Presidential (Serok): 50%+1 majority, runoff if needed
  - Parliamentary: 201 seats, district-based representation
  - Speaker Election (SerokiMeclise): Parliament selects
  - Constitutional Court (Diwan): Advisory council (50 members)
- **Legislative System:**
  - Proposal submission and voting
  - Multiple voting thresholds (simple majority, supermajority)
  - Trust-score weighted voting
  - Transparent on-chain voting records
- **Parliament Management:**
  - 201 parliamentary NFTs (non-transferable)
  - Endorsement requirements (1000 for president, 100 for parliament)
  - Campaign and voting periods
  - Automatic role assignment via pallet-tiki

**Governance Functions:**
- Constitutional amendments
- Budget allocation
- Minister appointments (7 cabinet positions)
- Treasury spending approval
- Policy proposals

---

#### 3. Pallet Perwerde (Education Platform)
**Runtime Index:** 48 | **Status:** ✅ Production Ready

**Purpose:** Decentralized education system with course management

**Features:**
- **Course Management:** Create, enroll, track completion
- **IPFS Content Storage:** Decentralized course materials
- **Points-Based System:** Achievement tracking
- **Hybrid Architecture:** Blockchain + Supabase database sync
- **Course Archive:** Deactivate outdated content

**Integration:** Education points contribute to trust score (pallet-trust)

---

#### 4. Pallet Identity-KYC (Citizenship Application)
**Runtime Index:** 46 | **Status:** ✅ Production Ready

**Purpose:** KYC verification and citizenship application workflow

**Features:**
- **Identity Registration:** Name + email (one-time)
- **KYC Application:** Document submission (IPFS CIDs + commitment hash)
- **KYC Statuses:** NotStarted → Pending → Approved/Rejected/Revoked
- **Zero-Knowledge Design:** Encrypted documents, commitment hashes
- **Admin Controls:** Approve/reject/revoke citizenship

**Workflow:**
1. User sets identity
2. Submits KYC application (deposits reserved)
3. Admin reviews documents
4. Approval triggers: Citizenship NFT mint + referral rewards

**Integration Hooks:**
- `CitizenNftProvider`: Triggers pallet-tiki NFT minting
- `OnKycApproved`: Notifies pallet-referral for rewards

---

#### 5. Pallet Referral (Community Growth)
**Runtime Index:** 47 | **Status:** ✅ Production Ready

**Purpose:** Incentivize community growth through referral system

**Features:**
- **Referral Tracking:** Link between referrer and referred
- **Automatic Processing:** Rewards on KYC approval
- **Referral Score:** Contributes to trust calculation
- **Multi-level Tracking:** Referral chains

**Integration:** Provides referral score component for pallet-trust

---

#### 6. Pallet Trust (Composite Trust Score)
**Runtime Index:** 69 | **Status:** ✅ Production Ready

**Purpose:** Calculate holistic trust score from multiple reputation sources

**Trust Score Formula:**
```
trust_score = (staking_score × 100 + referral_score × 300 +
               perwerde_score × 300 + tiki_score × 300) ×
              staking_score / multiplier_base
```

**Components:**
- **Staking Score:** From pallet-staking-score (time-weighted)
- **Referral Score:** From pallet-referral (community growth)
- **Perwerde Score:** From pallet-perwerde (education completion)
- **Tiki Score:** From pallet-tiki (role-based citizenship level)

**Features:**
- Batch update mechanism (100-500 users per batch)
- Periodic recalculation (scheduled)
- Manual recalculation (admin)
- Total active trust score tracking

**Usage:** Validator selection (pallet-validator-pool), voting weight (pallet-welati), rewards (pallet-pez-rewards)

---

#### 7. Pallet Staking-Score (Time-Weighted Staking)
**Runtime Index:** 49 | **Status:** ✅ Production Ready

**Purpose:** Track time-weighted staking metrics for trust calculation

**Features:**
- User-activated score tracking
- Time-based staking measurements
- Provides staking component for trust score

---

#### 8. Pallet Validator-Pool (Validator Selection)
**Runtime Index:** 103 | **Status:** ✅ Production Ready

**Purpose:** Decentralized validator selection with multiple categories

**Validator Categories:**
1. **StakeValidator:** Minimum stake + trust score ≥ 500
2. **ParliamentaryValidator:** Requires Parlementer tiki (parliament member)
3. **MeritValidator:** Special tikis + high community support (referrals)

**Selection Algorithm:**
- **Target Distribution:** 10 stake, 6 parliamentary, 5 merit (out of 21 total)
- **Rotation Rule:** Cannot be selected in 3 consecutive eras
- **Performance Threshold:** 70% reputation minimum
- **Randomized Selection:** VRF-based from eligible pool

**Features:**
- Join/leave validator pool
- Update category
- Performance metrics tracking (blocks produced, missed blocks, points)
- Era-based validator rotation
- Selection history (last 5 eras)

**Integration:** Implements `SessionManager` for automatic validator set updates

---

#### 9. Pallet Pez-Treasury (Treasury Management)
**Runtime Index:** 101 | **Status:** ✅ Production Ready

**Purpose:** Automated treasury management with scheduled releases

**Features:**
- **Automated Monthly Releases:** Scheduled via pallet-scheduler
- **Genesis Distribution:** One-time initial allocation
- **Fund Categories:**
  - Validator rewards
  - Governance treasury
  - Incentive pool (for pallet-pez-rewards)
  - Development fund

**Integration:** Transfers funds to pallet-pez-rewards incentive pot monthly

---

#### 10. Pallet Pez-Rewards (Monthly Distribution)
**Runtime Index:** 102 | **Status:** ✅ Production Ready

**Purpose:** Monthly PEZ token distribution to citizens and parliament

**Epoch System:**
- **Duration:** 30 days (432,000 blocks)
- **Snapshot:** Users record trust score during epoch
- **Finalization:** Automatic via scheduler (end of epoch)
- **Claim Window:** 7 days after epoch ends
- **Unclaimed:** Returns to incentive pot

**Distribution:**
- **90% Citizens:** Proportional to trust score
- **10% Parliament:** Automatic distribution to 201 parliamentary NFT holders

**Features:**
- User-initiated snapshot recording
- Automatic epoch finalization
- Claim rewards (per epoch)
- Close epoch (claw back unclaimed after 7 days)
- Parliamentary NFT holder tracking

**Integration:** Receives monthly funding from pallet-pez-treasury

---

#### 11. Pallet Token-Wrapper (HEZ ↔ wHEZ)
**Runtime Index:** 76 | **Status:** ✅ Production Ready

**Purpose:** Convert native HEZ to wrapped wHEZ for DEX trading

**Features:**
- **Wrap:** Lock native HEZ, mint wHEZ asset (Asset ID: 0)
- **Unwrap:** Burn wHEZ, unlock native HEZ
- **1:1 Backing:** HEZ locked in pallet account
- **Total Locked Tracking:** Monitor wrapped supply

**Integration:** Enables HEZ trading on DEX (pallet-asset-conversion)

---

## Future Development

### Phase 2: Cultural & Social Infrastructure (Planned)

See: [FUTURE_PALLETS_ROADMAP.md](./FUTURE_PALLETS_ROADMAP.md) for detailed specifications.

**Planned Pallets:**

#### 1. Pallet Free-Media
- Censorship-resistant content publishing
- Journalist protection mechanisms
- Community-based fact-checking
- IPFS-based storage
- Decentralized news distribution

**Status:** Documented, implementation planned for Q2 2026

#### 2. Pallet Heritage
- Cultural asset preservation
- Historical document archiving
- Language content storage (Kurmancî, Soranî, Zazakî, Feylî)
- NFT-based cultural tokens
- Oral history recording

**Status:** Documented, implementation planned for Q3 2026

#### 3. Pallet Cooperatives
- Worker cooperative management
- Collective decision-making
- Profit-sharing mechanisms
- Producer cooperative marketplace
- On-chain governance for coops

**Status:** Documented, implementation planned for Q4 2026

#### 4. Pallet Privacy
- Zero-knowledge proof integration
- Private voting mechanisms
- Selective disclosure identity
- Anonymous transactions (opt-in)

**Status:** Research phase, implementation planned for 2027

---

## Governance Model

### Democratic Structure

```
┌─────────────────────────────────────────────┐
│           Citizens (Token Holders)          │
│         Direct voting on referendums        │
└─────────────────┬───────────────────────────┘
                  │
                  ▼ (Elect)
┌─────────────────────────────────────────────┐
│      Parliament (Welati Legislators)        │
│   - Review proposals                        │
│   - Submit legislation                      │
│   - Approve treasury spending               │
└─────────────────┬───────────────────────────┘
                  │
                  ▼ (Propose/Execute)
┌─────────────────────────────────────────────┐
│         Executive Functions                 │
│   - Treasury management                     │
│   - Technical operations                    │
│   - Emergency responses                     │
└─────────────────────────────────────────────┘
```

### Voting Mechanisms

**1. Direct Democracy (Referendums)**
- Any citizen can initiate a referendum with sufficient support
- Majority vote required for passage
- Voting power based on citizenship tier
- Transparent on-chain voting records

**2. Representative Democracy (Parliament)**
- Elected legislators serve fixed terms
- Vote on behalf of constituents
- Can be removed through no-confidence votes
- Subject to re-election

**3. Liquid Democracy (Future)**
- Delegate voting power to trusted individuals
- Revocable delegation
- Topic-specific delegation possible

---

## Economic Model (Dual-Token)

### Dual-Token System

PezkuwiChain implements a **dual-token economy** to separate utility and governance functions:

#### Token 1: HEZ (Hez)
**Purpose:** Inflationary staking and utility token

**Characteristics:**
- **Supply:** Inflationary (no hard cap)
- **Inflation Rate:** 10% annual (adjustable via governance)
- **Primary Use:** Staking, validator rewards, transaction fees
- **Distribution:** Block rewards to validators, nominators, parliament

**Issuance:**
```
Block Reward: 100 HEZ per block
├── 70% → Validator operator (70 HEZ)
├── 20% → Nominators (20 HEZ)
└── 10% → Parliament NFT holders (10 HEZ)

Annual Issuance: ~525,600 blocks/year × 100 HEZ = 52.56M HEZ/year
```

**Functions:**
- Stake to nominate validators
- Pay transaction fees
- Collateral for governance proposals
- Reward for network participation

---

#### Token 2: PEZ (Pez)
**Purpose:** Fixed-supply governance and citizen distribution token

**Characteristics:**
- **Total Supply:** 5,000,000,000 PEZ (5 billion, fixed)
- **Inflation:** None (deflationary via halving mechanism)
- **Primary Use:** Governance voting, citizen dividends
- **Distribution:** Halving-based citizen distribution

**Supply Distribution:**
```
Total: 5,000,000,000 PEZ

├── 40% → Citizen Distribution (2,000,000,000 PEZ)
│   └── Monthly distribution with 4-year halving
├── 30% → Treasury Reserve (1,500,000,000 PEZ)
│   └── Governance-controlled spending
├── 20% → Development Fund (1,000,000,000 PEZ)
│   └── Core team and developer grants
└── 10% → Presale/Initial Funding (500,000,000 PEZ)
    └── Early supporters and genesis validators
```

**Halving Schedule (Synthetic):**
```
Initial Monthly Distribution: D₀
Year 0-4:     D₀ (base distribution)
Year 4-8:     D₀ / 2 (first halving)
Year 8-12:    D₀ / 4 (second halving)
Year 12-16:   D₀ / 8 (third halving)
...
```

**Calculation:**
```
Citizen Distribution Pool: 2,000,000,000 PEZ
First Period (48 months): 100% distribution
Distribution per month: 2B / 48 = 41,666,666 PEZ/month

After first halving (months 49-96):
Distribution per month: 20,833,333 PEZ/month

After second halving (months 97-144):
Distribution per month: 10,416,666 PEZ/month
```

**Governance Functions:**
- Vote on Welati proposals (1 PEZ = 1 vote)
- Submit governance proposals (minimum 10,000 PEZ deposit)
- Referendum participation
- Treasury spending approval

---

### Why Dual-Token?

| Aspect | Single-Token Problem | Dual-Token Solution |
|--------|---------------------|-------------------|
| **Inflation** | Can't satisfy both staking rewards and scarcity | HEZ inflates for rewards, PEZ scarce for value |
| **Governance** | High volatility affects decision-making | PEZ stable supply for predictable governance |
| **Validator Economics** | Fixed supply limits validator incentives | HEZ unlimited rewards for validators |
| **Citizen Benefits** | Dilution from staking rewards | PEZ halving protects citizen value |
| **Decoupling** | Governance tied to staking | Separate governance (PEZ) from security (HEZ) |

### Token Interaction Flow

```
┌─────────────────────────────────────────────┐
│           User Actions                      │
└───────────┬─────────────────┬───────────────┘
            │                 │
    Stake HEZ               Hold PEZ
            │                 │
            ▼                 ▼
┌─────────────────┐   ┌─────────────────┐
│  Validator      │   │  Governance     │
│  Nomination     │   │  Participation  │
│                 │   │                 │
│  Earn HEZ       │   │  Vote with PEZ  │
│  (inflation)    │   │  (1 PEZ = 1 vote)│
└─────────────────┘   └─────────────────┘
```

### Economic Parameters (From Implementation)

```rust
// HEZ Token Configuration
const HEZ_DECIMALS: u8 = 18;
const HEZ_INITIAL_SUPPLY: Balance = 0; // Minted via block rewards
const HEZ_BLOCK_REWARD: Balance = 100_000_000_000_000_000_000; // 100 HEZ

// PEZ Token Configuration
const PEZ_DECIMALS: u8 = 18;
const PEZ_TOTAL_SUPPLY: Balance = 5_000_000_000_000_000_000_000_000_000; // 5B PEZ
const PEZ_CITIZEN_POOL: Balance = 2_000_000_000_000_000_000_000_000_000; // 2B PEZ
const PEZ_HALVING_PERIOD_MONTHS: u32 = 48; // 4 years
```

### Treasury Management

**Revenue Sources:**
1. Transaction fees (variable)
2. Governance penalties (slashing)
3. External grants/donations
4. Service fees (KYC, NFT minting, etc.)

**Expenditure Categories:**
1. Infrastructure (validators, nodes)
2. Development (core team, grants)
3. Community programs (education, events)
4. Emergency fund (disasters, crises)

**Approval Process:**
- Proposals submitted to Welati parliament
- Review period: 7 days minimum
- Voting period: 14 days
- Execution: Automatic upon approval

---

## Pre-sale & Initial Distribution

### Overview

PezkuwiChain will conduct a **smart contract-based pre-sale** to distribute 500,000,000 PEZ (10% of total supply) to early supporters, genesis validators, and strategic investors. This pre-sale funds initial development, infrastructure, and community growth initiatives.

**Key Details:**
- **Total Pre-sale Allocation:** 500,000,000 PEZ (10% of 5B total supply)
- **Implementation:** ink! Smart Contract (NOT custom pallet)
- **Development Timeline:** 2 weeks
- **Platform:** pallet-contracts (Runtime Index: 104)
- **Vesting:** Built into smart contract logic

### Why Smart Contract (Not Custom Pallet)?

**Decision Rationale:**

| Factor | Custom Pallet | ink! Smart Contract | Winner |
|--------|--------------|-------------------|---------|
| **Development Time** | 6 weeks | 2 weeks | ✅ Contract |
| **Runtime Upgrade** | Required | Not required | ✅ Contract |
| **Audit Complexity** | 2000+ lines | 500 lines | ✅ Contract |
| **Upgradeability** | Hard fork needed | Easy redeployment | ✅ Contract |
| **Security Isolation** | Affects runtime | Isolated execution | ✅ Contract |
| **One-time Use Case** | Overkill | Perfect fit | ✅ Contract |

**Technical Advantage:**
PezkuwiChain already has `pallet-contracts` deployed (Runtime Index 104), enabling immediate smart contract deployment without any runtime modifications. Custom pallets are reserved for **core protocol features** used continuously (e.g., staking, governance, trust scores). Pre-sale is a **one-time event** that perfectly suits smart contract implementation.

### Pre-sale Structure

#### Phase 1: Genesis Validators (COMPLETED)
**Allocation:** 100,000,000 PEZ (20%)
**Status:** ✅ Distributed at genesis
**Recipients:** 8 genesis validators who bootstrapped the network

**Distribution:**
```
Total: 100M PEZ
├── Validator 1-8: 12.5M PEZ each
└── Vesting: 12 months linear unlock
```

#### Phase 2: Strategic Pre-sale (UPCOMING)
**Allocation:** 200,000,000 PEZ (40%)
**Target:** Strategic investors, ecosystem partners
**Price Tiers:** 3 tiers with volume discounts

**Tier Structure:**

| Tier | Min Purchase | Max Purchase | Price per PEZ | Bonus | Total PEZ |
|------|-------------|-------------|--------------|-------|-----------|
| **Seed** | $25,000 | $100,000 | $0.0010 | 25% | 1.25 per USD |
| **Private** | $10,000 | $50,000 | $0.0015 | 15% | 1.15 per USD |
| **Public** | $100 | $10,000 | $0.0020 | 0% | 1.00 per USD |

**Example Calculations:**
```
Seed Tier Purchase: $50,000
├── Base PEZ: 50,000,000 PEZ ($0.0010 per PEZ)
├── Bonus: 12,500,000 PEZ (25% bonus)
└── Total: 62,500,000 PEZ

Public Tier Purchase: $1,000
├── Base PEZ: 500,000 PEZ ($0.0020 per PEZ)
├── Bonus: 0 PEZ (no bonus)
└── Total: 500,000 PEZ
```

**Vesting Schedule:**
- **Seed Tier:** 18 months linear vesting (5.56% per month)
- **Private Tier:** 12 months linear vesting (8.33% per month)
- **Public Tier:** 6 months linear vesting (16.67% per month)

#### Phase 3: Community Distribution (FUTURE)
**Allocation:** 200,000,000 PEZ (40%)
**Target:** Community members, early adopters, airdrops
**Mechanism:** Decentralized distribution via governance proposals

**Potential Use Cases:**
- Airdrop to active community members
- Reward for education platform completion (pallet-perwerde integration)
- Referral incentives (pallet-referral integration)
- Early citizenship NFT holders (pallet-tiki integration)

### Smart Contract Architecture

#### Contract Functions

```rust
// Core pre-sale functions (ink! pseudocode)

#[ink(message)]
pub fn purchase(&mut self, amount: Balance) -> Result<(), Error> {
    // 1. Calculate tier and bonus
    // 2. Transfer payment (stablecoin/native token)
    // 3. Mint/allocate PEZ tokens
    // 4. Record vesting schedule
    // 5. Emit PurchaseEvent
}

#[ink(message)]
pub fn claim(&mut self) -> Result<Balance, Error> {
    // 1. Calculate vested amount (current block - purchase block)
    // 2. Check claimable balance
    // 3. Transfer unlocked PEZ
    // 4. Update vesting record
    // 5. Emit ClaimEvent
}

#[ink(message)]
pub fn get_vesting_schedule(&self, account: AccountId) -> VestingInfo {
    // Return: total_purchased, claimed, claimable, locked, unlock_per_block
}

#[ink(message)]
pub fn emergency_pause(&mut self) -> Result<(), Error> {
    // Only contract owner can pause (security measure)
}
```

#### Vesting Calculation

**Linear Vesting Formula:**
```
Unlocked Amount = (Total PEZ × Elapsed Blocks) / Total Vesting Blocks

Where:
- Elapsed Blocks = Current Block - Purchase Block
- Total Vesting Blocks = Vesting Period Months × (30 days × 24h × 60min × 60s / 6s block time)
- 1 Month ≈ 432,000 blocks (30 days × 14,400 blocks/day)

Example (Seed Tier, 18 months):
- Total Vesting Blocks: 18 × 432,000 = 7,776,000 blocks
- After 3 months (1,296,000 blocks): 16.67% unlocked
- After 9 months (3,888,000 blocks): 50% unlocked
- After 18 months (7,776,000 blocks): 100% unlocked
```

#### Security Features

**Smart Contract Safeguards:**
1. **Reentrancy Protection:** Check-Effects-Interactions pattern
2. **Overflow Protection:** Saturating math operations
3. **Access Control:** Owner-only admin functions
4. **Pausability:** Emergency stop mechanism
5. **Immutable Parameters:** Tier prices locked at deployment
6. **Transfer Validation:** Verify payment before allocation

**Audit Requirements:**
- Third-party security audit before mainnet deployment
- Formal verification of vesting calculations
- Bug bounty program (50,000 HEZ reward pool)

### Payment Methods

**Accepted Tokens:**

| Token | Network | Why Accepted? |
|-------|---------|--------------|
| **USDT** | Polkadot/Kusama | Industry standard stablecoin |
| **USDC** | Polkadot/Kusama | Regulated stablecoin |
| **DOT** | Polkadot | Native parachain token |
| **KSM** | Kusama | Native parachain token |
| **wHEZ** | PezkuwiChain | Wrapped native token (via pallet-token-wrapper) |

**Cross-chain Purchase Flow:**
```
User on Polkadot/Kusama
        ↓
1. Send payment (USDT/DOT) via XCM
        ↓
2. Payment received on PezkuwiChain sovereign account
        ↓
3. Smart contract notified (oracle/bridge)
        ↓
4. PEZ allocation + vesting record created
        ↓
5. User can claim unlocked PEZ monthly
```

### Fund Allocation

**Pre-sale Revenue Target:** $500,000 - $1,000,000 USD

**Planned Use of Funds:**

| Category | Allocation | Purpose |
|----------|-----------|---------|
| **Development** | 40% ($200k-$400k) | Core team salaries, contractor payments |
| **Infrastructure** | 25% ($125k-$250k) | Validator nodes, RPC servers, cloud services |
| **Marketing** | 20% ($100k-$200k) | Community growth, social media, partnerships |
| **Legal & Compliance** | 10% ($50k-$100k) | Entity formation, regulatory consultation |
| **Reserve** | 5% ($25k-$50k) | Emergency fund, unexpected costs |

### Integration with Existing Pallets

**PEZ Token Transfer:**
Pre-sale contract interacts with existing token infrastructure:

```rust
// Transfer PEZ from treasury to purchaser (via contract)
pallet_pez_treasury::transfer_from_treasury(
    contract_account,
    purchaser_account,
    pez_amount
)?;

// Record vesting in contract storage
VestingSchedule {
    total: pez_amount,
    claimed: 0,
    start_block: current_block,
    end_block: current_block + vesting_blocks,
}
```

**Trust Score Bonus:**
Early PEZ purchasers receive trust score boost (via pallet-trust):
- Seed tier: +50 trust points (long-term commitment)
- Private tier: +30 trust points
- Public tier: +10 trust points

**Citizenship Eligibility:**
Pre-sale participants who complete KYC (pallet-identity-kyc) receive priority:
- Fast-track citizenship application review
- Reduced NFT minting fee (50% discount)
- Early access to governance proposals

### Timeline

**Pre-sale Schedule:**

| Phase | Duration | Activities |
|-------|----------|-----------|
| **Week 1-2** | Contract Development | Write ink! smart contract, unit tests |
| **Week 3** | Security Audit | Third-party audit, bug fixes |
| **Week 4** | Testnet Deployment | Deploy to beta testnet, community testing |
| **Week 5-6** | Seed Round | Private sale to strategic investors |
| **Week 7-8** | Private Round | Early community members |
| **Month 3** | Public Round | Open to all participants |
| **Ongoing** | Vesting & Claims | Monthly unlock, user claims |

**Current Status (Nov 17, 2025):** ✅ pallet-contracts deployed (index 104), ready for smart contract development.

### Transparency & Reporting

**On-chain Verification:**
All pre-sale transactions are publicly verifiable:
- Purchase events: `PreSale::Purchase(buyer, tier, amount, timestamp)`
- Claim events: `PreSale::Claim(user, claimed_amount, remaining)`
- Total raised: `PreSale::TotalRaised()` (read-only query)
- Remaining allocation: `PreSale::RemainingPEZ()` (read-only query)

**Monthly Reports:**
Treasury publishes monthly transparency reports:
1. Total funds raised (USD equivalent)
2. Number of unique participants
3. Tier distribution breakdown
4. Fund expenditure by category
5. Remaining PEZ allocation

**Community Oversight:**
- Welati parliament reviews fund usage quarterly
- Any expenditure > 100,000 PEZ requires governance vote
- Multi-signature treasury wallet (5-of-8 validators)

---

## Technical Specifications

### Blockchain Specifications

| Parameter | Value |
|-----------|-------|
| **Consensus** | Hybrid (BABE + GRANDPA) |
| **Block Time** | 6 seconds |
| **Finality** | ~30 seconds |
| **Target TPS** | 100+ transactions/second |
| **Max Block Size** | 5 MB |
| **State Pruning** | Configurable (archive/pruned) |
| **Wasm Runtime** | Yes (forkless upgrades) |

### Network Topology

**Node Types:**
1. **Validator Nodes** (8+ at genesis)
   - Produce and validate blocks
   - Run in secure environments
   - Stake-based selection

2. **Full Nodes** (Unlimited)
   - Store complete blockchain history
   - Serve RPC requests
   - Enable network resilience

3. **Light Clients** (Mobile/Web)
   - Sync headers only
   - Query full nodes for data
   - Low resource requirements

### Performance Targets

| Metric | Target | Critical Threshold |
|--------|--------|-------------------|
| Block Time | 6 seconds | < 10 seconds |
| Transaction Throughput | 100 tx/s | > 50 tx/s |
| Block Finalization | < 30 seconds | < 60 seconds |
| Peer Connections | > 50 | > 25 |
| Memory Usage | < 8GB | < 12GB |
| CPU Usage | < 60% | < 80% |

### Cross-Chain Infrastructure (XCM)

PezkuwiChain implements **full XCM (Cross-Consensus Messaging)** protocol, enabling seamless cross-chain interoperability.

**XCM Implementation Status:**
```rust
// File: runtime/pezkuwichain/src/xcm_config.rs (306 lines)

impl pallet_xcm::Config for Runtime {
    type XcmExecutor = XcmExecutor<XcmConfig>;      // ✅ Fully configured
    type XcmTeleportFilter = Everything;             // ✅ Teleport enabled
    type XcmReserveTransferFilter = Everything;      // ✅ Reserve transfers enabled
    type Currency = Balances;                        // ✅ HEZ token integrated
    type XcmRouter = XcmRouter;                      // ✅ Message routing ready
    // ... 20+ additional config parameters
}
```

**Capabilities:**

| Feature | Status | Description |
|---------|--------|-------------|
| **XCM Protocol** | ✅ Implemented | Full v3 XCM support (306 lines config) |
| **Asset Transactor** | ✅ Ready | HEZ cross-chain transfers |
| **Location Converter** | ✅ Ready | AccountId ↔ Location mapping |
| **Reserve Transfers** | ✅ Active | Cross-chain asset transfers |
| **Teleport** | ✅ Active | Fast cross-chain messaging |
| **XCM Executor** | ✅ Ready | Execute XCM instructions |
| **Relay Chain Support** | 🔄 Pending | Rococo/Westend/Polkadot ready |

**Parachain Readiness:**
```
├── Rococo Runtime (Testnet): ✅ Implemented
├── Westend Runtime (Testnet): ✅ Implemented
├── Parachain Primitives: ✅ Integrated
└── Polkadot Deployment: ⏳ Planned Q1 2027
```

**Bridge Capability:**
```
Standalone Mode (Current):
├── Custom bridges required
└── Manual XCM configuration

Parachain Mode (Future):
├── Ethereum Bridge: Via Snowbridge (automatic)
├── Bitcoin Bridge: Via parachain connectors
├── Polkadot Ecosystem: Direct XCM messaging
└── 50+ Parachains: Automatic interoperability
```

---

### Security Model

**Consensus Security:**
- Byzantine Fault Tolerant (BFT)
- 66% supermajority required for finality
- Slashing for malicious behavior

**XCM Security:**
- Asset reserve verification
- Origin authentication (signed/unsigned)
- Execution barriers for untrusted messages
- Weight-based fee model to prevent spam

**Smart Contract Security:**
- Audited pallet code
- Formal verification (future)
- Bug bounty program (planned)

**Network Security:**
- TLS encryption for RPC
- DDoS protection via sentry nodes
- Rate limiting on public endpoints

---

## Roadmap

### Phase 1: Foundation ✅ COMPLETED (2024-2025)
**Status:** Production Ready - Beta Testnet Operational

**Blockchain Infrastructure:**
- ✅ 11 custom pallets deployed (65% of planned features)
- ✅ 77 total runtime pallets (11 custom + 66 standard Substrate)
- ✅ Beta testnet: 8 validators running (VPS: 37.60.230.9)
- ✅ Network health: 100% uptime, 6-second blocks, ~30-second finality
- ✅ Block production: Stable (Block #5722+ as of Nov 17, 2025)
- ✅ Automated systems: Monthly treasury releases, epoch finalization, trust score updates

**Frontend Applications:**
- ✅ Web Application: 90% complete
  - Citizenship application & KYC
  - Elections & governance participation
  - Staking & validator pool
  - Education platform (Perwerde)
  - DEX (swap, pools, liquidity)
  - Multi-language: 6 languages (Kurmanji, Sorani, Arabic, Persian, Turkish, English)
- ✅ Mobile Application: 95% complete
  - Wallet (Polkadot.js integration)
  - Biometric authentication (Face ID, Touch ID, PIN)
  - Staking & governance
  - NFT gallery (citizenship NFTs)
  - Referral system

**Deployment:**
- ✅ Domain: https://pezkuwichain.io
- ✅ WebSocket RPC: wss://ws.pezkuwichain.io
- ✅ SSL certificates (Let's Encrypt)
- ✅ Nginx reverse proxy
- ✅ 7 validators on VPS (24/7), 1 optional local validator

### Phase 2: Interoperability & Parachain (Q2-Q3 2026)
**Status:** XCM Infrastructure Ready

**Q2 2026: Rococo Testnet Parachain**
- 🔄 Deploy to Rococo testnet as parachain
- 🔄 Activate XCM messaging
- 🔄 Test cross-chain transfers
- 🔄 Ethereum bridge testing (Snowbridge)
- 🔄 Oracle pallet integration (Chainlink/Acurast)
- 🔄 Community XCM education

**Q3 2026: Westend & Stress Testing**
- 📋 Deploy to Westend testnet
- 📋 Load testing with cross-chain traffic
- 📋 Multi-parachain interaction tests
- 📋 Security audit for XCM implementation
- 📋 Bridge optimization

**Q4 2026: Mainnet Preparation**
- 📋 Finalize bridge configurations
- 📋 Community vote on Polkadot deployment
- 📋 Crowdloan preparation (if parachain route chosen)
- 📋 Mainnet stability audit

### Phase 3: Cultural Infrastructure (Q2-Q4 2026)
**Status:** Parallel Development

**Q2 2026:**
- 📋 Pallet Free-Media implementation
- 📋 Enhanced mobile app (media features)
- 📋 Content moderation DAO

**Q3 2026:**
- 📋 Pallet Heritage implementation
- 📋 Multi-language support (4 dialects)
- 📋 Cultural NFT marketplace

**Q4 2026:**
- 📋 Pallet Cooperatives implementation
- 📋 Economic tools expansion

### Phase 4: Economic Expansion (2027)
**Status:** Research Phase

- 📋 Polkadot parachain slot auction (if community approves)
- 📋 Production Ethereum bridge activation
- 📋 Remittance channels (low-fee transfers via XCM)
- 📋 Micro-credit systems
- 📋 DEX integration with Polkadot ecosystem
- 📋 Stablecoin pegging mechanisms
- 📋 Bitcoin bridge via parachains

### Phase 5: Advanced Features (2027-2028)
**Status:** Exploratory

- 📋 Pallet Privacy (ZK-SNARKs)
- 📋 Full Polkadot ecosystem integration
- 📋 Advanced identity (biometrics, DID)
- 📋 AI-powered governance tools
- 📋 Quantum-resistant cryptography
- 📋 Multi-chain liquidity pools

---

## Community & Governance

### Decision-Making Process

**1. Proposal Submission**
- Any citizen can submit a proposal
- Minimum deposit required (refundable)
- Clear description and expected outcomes

**2. Discussion Period**
- Community forum discussion (7 days)
- Technical review by core team
- Impact assessment

**3. Voting Period**
- On-chain voting (14 days)
- Weighted by citizenship tier
- Transparent tallying

**4. Execution**
- Automatic execution upon approval
- Post-implementation review
- Continuous monitoring

### Community Participation

**Ways to Contribute:**
1. **Validators:** Run network infrastructure
2. **Developers:** Contribute code, pallets, dApps
3. **Governance:** Vote, propose, debate
4. **Content:** Create cultural/educational content
5. **Translation:** Localize to Kurdish dialects
6. **Education:** Teach blockchain to community

---

## Legal & Compliance

### Regulatory Considerations

**Current Status:**
- Decentralized network (no central authority)
- Open-source software (Apache 2.0 license)
- Self-sovereign identity model
- Pseudo-anonymous transactions

**Future Considerations:**
- AML/KYC compliance (opt-in for services)
- Data protection (GDPR-compatible)
- Cross-border regulations
- Securities law compliance (token classification)

### Intellectual Property

**Codebase:**
- Apache 2.0 license
- All contributions open-source
- No patent restrictions

**Trademarks:**
- "PezkuwiChain" trademark (registered)
- Logo and branding assets (CC BY-SA)

---

## Environmental Sustainability

### Energy Efficiency

PezkuwiChain's Proof-of-Stake consensus is inherently energy-efficient compared to Proof-of-Work systems.

### Carbon Footprint Analysis

| Blockchain | Consensus | Annual Energy (TWh) | Carbon Footprint (tCO₂/year) | Transactions/kWh |
|-----------|-----------|-------------------|----------------------------|-----------------|
| **PezkuwiChain** | TNPoS (PoS) | ~0.0001 | ~0.001 | ~100,000 |
| Polkadot | NPoS (PoS) | ~0.0002 | ~0.001 | ~85,000 |
| Cardano | Ouroboros PoS | ~0.0003 | ~0.001 | ~75,000 |
| Ethereum | PoS (post-merge) | ~0.01 | ~0.01 | ~10,000 |
| Bitcoin | PoW | ~150 | ~75,000,000 | ~5 |

### Environmental Impact Comparison

**PezkuwiChain vs Bitcoin:**
- **Energy Use:** 99.9999% less energy consumption
- **Carbon Emissions:** ~75 billion times lower carbon footprint
- **Efficiency:** 20,000x more transactions per kWh

**Validator Energy Consumption:**
```
Single Validator Node:
├── Hardware: Standard server (250W average)
├── Daily consumption: 6 kWh
├── Annual consumption: 2,190 kWh
├── Carbon footprint: ~1.1 tCO₂/year
└── Equivalent to: ~2,750 miles driven in average car

Full Network (16 validators):
├── Total annual energy: ~35,040 kWh
├── Total carbon footprint: ~17.5 tCO₂/year
└── Equivalent to: ~43,750 miles driven
```

### Sustainability Initiatives

**1. Validator Efficiency Standards**
- Encourage use of renewable energy
- Optimize node software for minimal resource usage
- Regular performance audits

**2. Carbon Offset Program (Planned)**
- Treasury allocation for carbon offsets
- Partnership with verified carbon credit programs
- Transparent reporting via governance

**3. Green Validator Incentives**
- Extra rewards for validators using renewable energy
- Certification program for green validators
- Public dashboard showing validator energy sources

### Comparison to Traditional State Infrastructure

**Traditional State:**
- Government offices, military, bureaucracy
- Estimated 1,000+ tCO₂/year per 1M citizens

**PezkuwiChain:**
- Fully digital infrastructure
- ~17.5 tCO₂/year for entire network
- **99.998% reduction** in carbon footprint

---

## Risks & Mitigations

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Consensus failure | Low | High | Validator redundancy, monitoring |
| Smart contract bugs | Medium | High | Audits, formal verification |
| Network attacks | Medium | Medium | DDoS protection, sentry nodes |
| Scalability limits | Medium | Medium | Optimization, future parachain |

### Governance Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Low voter turnout | Medium | Medium | Education, incentives |
| Vote buying | Low | High | Transparency, penalties |
| Centralization | Low | High | Validator diversity, delegation limits |

### Economic Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Token volatility | High | Medium | Stablecoin integration (future) |
| Treasury depletion | Low | High | Conservative spending, reserves |
| Inflation pressure | Low | Medium | Deflationary halving mechanism |

---

## Conclusion

PezkuwiChain represents a bold experiment in **digital state-building** through blockchain technology. By combining democratic governance, economic sovereignty, and cultural preservation, we aim to create a resilient, transparent, and community-driven infrastructure for the Kurdish people worldwide.

**Key Achievements:**
- ✅ Production-ready blockchain infrastructure
- ✅ Democratic governance framework (Welati)
- ✅ Economic management system (Pez-Treasury)
- ✅ Digital citizenship (Tiki)
- ✅ Security-first development approach

**Next Steps:**
- Launch mainnet (Q1 2026)
- Expand cultural infrastructure (2026)
- Build economic tools (2027)
- Achieve widespread adoption (2027-2028)

**Join the Revolution:**
- Website: https://pezkuwichain.org (planned)
- GitHub: https://github.com/pezkuwichain
- Forum: https://forum.pezkuwichain.org (planned)
- Telegram: @pezkuwichain (planned)

---

## Appendix

### A. Glossary

- **Pallet:** A Substrate runtime module (like a smart contract library)
- **Extrinsic:** A transaction submitted to the blockchain
- **Runtime:** The state transition function of the blockchain
- **GRANDPA:** Finality gadget used by Polkadot/Substrate
- **BABE:** Block production mechanism (Blind Assignment for Blockchain Extension)
- **Treasury:** On-chain fund managed by governance
- **Referendum:** Direct vote by all token holders
- **Parliament (Welati):** Elected legislative body

### B. Technical Appendix

#### API Documentation

**RPC Endpoints (Production)**

**Beta Testnet:**
- **HTTP RPC:** `https://rpc.pezkuwichain.io:9944`
- **WebSocket:** `wss://ws.pezkuwichain.io`
- **Network:** Beta Testnet (pezkuwichain_beta_testnet)
- **Chain ID:** pezkuwichain_beta_testnet

**Standard Substrate RPC Methods:**

```json
// System Information
{
  "method": "system_health",
  "description": "Get network health status",
  "response": {
    "peers": 7,
    "isSyncing": false,
    "shouldHavePeers": true
  }
}

{
  "method": "system_chain",
  "description": "Get chain name",
  "response": "PezkuwiChain Beta Testnet"
}

{
  "method": "system_version",
  "description": "Get runtime version",
  "response": "1.18.5-9204f73230c"
}

// Chain State
{
  "method": "chain_getHeader",
  "description": "Get latest block header",
  "response": {
    "number": "0x166a",  // Block #5738 (hex)
    "parentHash": "0x...",
    "stateRoot": "0x...",
    "extrinsicsRoot": "0x..."
  }
}

{
  "method": "chain_getFinalizedHead",
  "description": "Get finalized block hash",
  "response": "0x0f00cd4a15848764d705fb9b48e735d96ecbd30fc3d6a874b320be498545d8e8"
}

{
  "method": "chain_getBlock",
  "params": ["<block_hash>"],
  "description": "Get block by hash",
  "response": {
    "block": {
      "header": {...},
      "extrinsics": [...]
    }
  }
}
```

**Custom Pallet RPC Queries:**

```json
// Pallet Tiki (Citizenship)
{
  "method": "state_call",
  "params": ["TikiApi_get_citizen_nft", "<account_id_hex>"],
  "description": "Get citizenship NFT ID for account"
}

{
  "method": "state_call",
  "params": ["TikiApi_get_user_tikis", "<account_id_hex>"],
  "description": "Get all roles (tikis) for account"
}

// Pallet Trust (Trust Scores)
{
  "method": "state_call",
  "params": ["TrustApi_get_trust_score", "<account_id_hex>"],
  "description": "Get composite trust score"
}

// Pallet Welati (Governance)
{
  "method": "state_call",
  "params": ["WelatiApi_get_active_elections"],
  "description": "Get list of ongoing elections"
}

{
  "method": "state_call",
  "params": ["WelatiApi_get_parliament_members"],
  "description": "Get current 201 parliament members"
}

// Pallet Pez-Rewards (Monthly Distribution)
{
  "method": "state_call",
  "params": ["PezRewardsApi_get_current_epoch"],
  "description": "Get current epoch index and snapshot data"
}

{
  "method": "state_call",
  "params": ["PezRewardsApi_get_user_rewards", "<account_id_hex>", "<epoch_index>"],
  "description": "Get claimable rewards for user in epoch"
}
```

**Transaction Submission:**

```json
// Generic extrinsic submission
{
  "method": "author_submitExtrinsic",
  "params": ["<signed_extrinsic_hex>"],
  "description": "Submit signed transaction to mempool"
}

// Watch extrinsic status
{
  "method": "author_submitAndWatchExtrinsic",
  "params": ["<signed_extrinsic_hex>"],
  "description": "Submit and watch transaction lifecycle (WebSocket only)"
}
```

**Example Extrinsic Calls:**

```javascript
// Apply for citizenship (pallet-identity-kyc)
api.tx.identityKyc.applyForKyc(
  ["QmXXXXX", "QmYYYYY"],  // IPFS CIDs
  "0x1234...",             // Commitment hash
  "Application notes"
).signAndSend(account);

// Vote in election (pallet-welati)
api.tx.welati.castVote(
  electionId,
  [candidateId1, candidateId2],
  districtId
).signAndSend(account);

// Stake HEZ (standard pallet-staking)
api.tx.staking.bond(
  stashAccount,
  value,
  rewardDestination
).signAndSend(account);

// Claim PEZ rewards (pallet-pez-rewards)
api.tx.pezRewards.claimReward(
  epochIndex
).signAndSend(account);

// Wrap HEZ to wHEZ (pallet-token-wrapper)
api.tx.tokenWrapper.wrap(
  amount
).signAndSend(account);
```

---

#### Network Configuration

**Genesis Configuration:**

```rust
// Consensus Parameters
const BABE_GENESIS_EPOCH_CONFIG: BabeEpochConfiguration = {
    c: (1, 4),  // 25% secondary slots
    allowed_slots: PrimaryAndSecondaryVRFSlots,
};

// Block Timing
const MILLISECS_PER_BLOCK: u64 = 6000;  // 6 seconds
const SLOT_DURATION: u64 = 6000;
const EPOCH_DURATION_IN_BLOCKS: u32 = 100;  // 10 minutes
const SESSION_LENGTH: u32 = 100;  // 10 minutes (100 blocks)
const ERA_LENGTH: u32 = 600;  // 1 hour (600 blocks)

// Validator Set
const MIN_VALIDATOR_COUNT: u32 = 4;
const MAX_VALIDATOR_COUNT: u32 = 16;
const DESIRED_VALIDATOR_COUNT: u32 = 8;  // Beta testnet

// Economic Parameters
const EXISTENTIAL_DEPOSIT: Balance = 1_000_000_000_000_000_000;  // 1 HEZ
const HEZ_BLOCK_REWARD: Balance = 100_000_000_000_000_000_000;  // 100 HEZ
```

**Runtime Metadata:**

```json
{
  "runtime": "pezkuwichain",
  "specVersion": 100,
  "transactionVersion": 1,
  "authoringVersion": 1,
  "stateVersion": 1,
  "implName": "pezkuwichain",
  "implVersion": 0
}
```

**Pallet Indexes (for extrinsic encoding):**

| Pallet Name | Index | Call Prefix |
|------------|-------|-------------|
| System | 0 | 0x00 |
| Timestamp | 2 | 0x02 |
| Balances | 4 | 0x04 |
| Staking | 9 | 0x09 |
| Session | 8 | 0x08 |
| Treasury | 18 | 0x12 |
| Assets | 36 | 0x24 |
| AssetConversion | 37 | 0x25 |
| Tiki | 42 | 0x2A |
| IdentityKyc | 46 | 0x2E |
| Referral | 47 | 0x2F |
| Perwerde | 48 | 0x30 |
| StakingScore | 49 | 0x31 |
| Trust | 69 | 0x45 |
| Welati | 75 | 0x4B |
| TokenWrapper | 76 | 0x4C |
| PezTreasury | 101 | 0x65 |
| PezRewards | 102 | 0x66 |
| ValidatorPool | 103 | 0x67 |
| Contracts | 104 | 0x68 |

---

#### Developer Resources

**SDK & Libraries:**

```bash
# Polkadot.js API (JavaScript/TypeScript)
npm install @polkadot/api @polkadot/extension-dapp

# Connect to PezkuwiChain
import { ApiPromise, WsProvider } from '@polkadot/api';

const wsProvider = new WsProvider('wss://ws.pezkuwichain.io');
const api = await ApiPromise.create({ provider: wsProvider });

# Query citizenship NFT
const nftId = await api.query.tiki.citizenNft('<account_id>');
console.log('Citizenship NFT ID:', nftId.toString());
```

**Frontend Integration Examples:**

```typescript
// Check if user has citizenship
const hasCitizenship = async (address: string): Promise<boolean> => {
  const nft = await api.query.tiki.citizenNft(address);
  return !nft.isEmpty;
};

// Get trust score
const getTrustScore = async (address: string): Promise<number> => {
  const score = await api.query.trust.trustScores(address);
  return score.toNumber();
};

// Vote in election
const voteInElection = async (
  electionId: number,
  candidates: number[],
  district: number
) => {
  const tx = api.tx.welati.castVote(electionId, candidates, district);
  await tx.signAndSend(account, ({ status, events }) => {
    if (status.isInBlock) {
      console.log('Vote included in block');
    }
  });
};
```

**Testing Endpoints:**

```bash
# Check network health
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' \
  https://rpc.pezkuwichain.io:9944

# Get latest block number
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  https://rpc.pezkuwichain.io:9944 | jq '.result.number'

# Get chain info
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
  https://rpc.pezkuwichain.io:9944
```

---

#### Security Best Practices

**For Developers:**

1. **Key Management:**
   - Never hardcode private keys
   - Use browser extension wallets (Polkadot.js)
   - Implement secure key storage (SecureStore on mobile)

2. **Transaction Validation:**
   - Always verify transaction parameters
   - Display transaction details before signing
   - Implement gas fee estimation

3. **RPC Endpoint Security:**
   - Use WSS (not WS) for production
   - Implement rate limiting on client side
   - Validate all API responses

4. **Smart Contract Interaction:**
   - Audit contract code before deployment
   - Test on testnet extensively
   - Use version control for contract upgrades

**For Validators:**

1. **Node Security:**
   - Run validators in secure environments (no public SSH)
   - Use firewall rules (allow only P2P ports)
   - Regular security updates

2. **Key Management:**
   - Use separate accounts for stash and controller
   - Secure session keys (rotate periodically)
   - Backup validator data regularly

3. **Monitoring:**
   - Set up Prometheus metrics
   - Configure alerting (downtime, low peers)
   - Monitor block production rate

---

### C. References

1. **Substrate Documentation:** https://docs.substrate.io
2. **Polkadot.js API:** https://polkadot.js.org/docs/api
3. **Polkadot Whitepaper:** https://polkadot.network/whitepaper
4. **GRANDPA Consensus:** https://github.com/w3f/consensus
5. **ink! Smart Contracts:** https://use.ink
6. **Kurdish Population Statistics:** Various demographic sources
7. **Blockchain Governance Research:** Academic papers and industry reports

### C. Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | Oct 21, 2025 | Initial whitepaper release (42 pages PDF) |
| 2.0 | Nov 13, 2025 | Enhanced version with merged content:<br/>- Added Competitive Landscape comparison<br/>- Added TNPoS consensus mechanism details<br/>- Added Parliamentary NFT system (201 NFTs)<br/>- Added Dual-Token economy (HEZ + PEZ)<br/>- Added Environmental Sustainability analysis<br/>- Added Comprehensive pallet overview table<br/>- Added Technical constants from implementation<br/>- Enhanced risk assessment framework<br/>- Updated production readiness status |
| 3.0 | Nov 17, 2025 | **Major Update - Production Deployment:**<br/>- Updated to 11 deployed pallets (was 3)<br/>- Added 8 new pallet descriptions with runtime indexes<br/>- Added detailed pallet integration documentation<br/>- Updated deployment status (8 validators operational)<br/>- Added frontend completion status (Web 90%, Mobile 95%)<br/>- Added 6-language internationalization details<br/>- Updated Phase 1 completion metrics<br/>- Added production deployment info (VPS, domain, SSL)<br/>- Comprehensive pallet feature documentation<br/>- Trust score formula and integration details<br/>- Validator pool categories and selection algorithm |

---

**Document Version:** 3.0
**Last Updated:** November 17, 2025
**Maintained By:** PezkuwiChain Core Team

**License:** Creative Commons Attribution-ShareAlike 4.0 International (CC BY-SA 4.0)

---

*"A nation is not defined by borders, but by the bonds of its people and the strength of its vision."*

**— PezkuwiChain Vision Statement**
