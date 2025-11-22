# PezkuwiChain Future Pallets Roadmap
**Version:** 1.0
**Date:** November 13, 2025
**Status:** Planning Document

---

## Table of Contents

1. [Overview](#overview)
2. [Pallet Free-Media](#1-pallet-free-media)
3. [Pallet Heritage](#2-pallet-heritage)
4. [Pallet Cooperatives](#3-pallet-cooperatives)
5. [Pallet Privacy](#4-pallet-privacy)
6. [Pallet Diaspora](#5-pallet-diaspora)
7. [Pallet Education](#6-pallet-education)
8. [Pallet Remittance](#7-pallet-remittance)
9. [Implementation Timeline](#implementation-timeline)
10. [Technical Dependencies](#technical-dependencies)

---

## Overview

This document outlines the **future pallet development roadmap** for PezkuwiChain. These pallets will be implemented in subsequent runtime upgrades after the initial mainnet launch.

**Design Philosophy:**
- Build on existing core infrastructure (Welati, Pez-Treasury, Tiki)
- Maintain backward compatibility
- Prioritize user needs and community feedback
- Security-first development approach
- Gradual rollout with thorough testing

**Implementation Approach:**
- Each pallet will be developed independently
- Runtime upgrades will add pallets incrementally
- Community governance will approve each upgrade
- Beta testing on testnet before mainnet deployment

---

## 1. Pallet Free-Media

### Purpose
Provide censorship-resistant media publishing and journalism protection for Kurdish communities.

### Problem Statement
- Journalists face persecution for reporting truth
- Content gets censored by centralized platforms
- Fake news spreads without accountability
- No protection for whistleblowers

### Solution
Decentralized media platform with community-driven verification and permanent content storage.

---

### Technical Specification

#### Storage Items

```rust
/// Media content metadata
#[pallet::storage]
pub type MediaContent<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    ContentHash,  // IPFS hash
    ContentMetadata<T>,
>;

pub struct ContentMetadata<T: Config> {
    /// Content creator
    pub author: T::AccountId,
    /// IPFS content hash
    pub content_hash: Hash,
    /// Content type (article, video, image, audio)
    pub content_type: ContentType,
    /// Publication timestamp
    pub published_at: BlockNumberFor<T>,
    /// Content category (news, opinion, investigation, etc.)
    pub category: Category,
    /// Verification status
    pub verified: VerificationStatus,
    /// Number of endorsements
    pub endorsements: u32,
    /// Number of flags
    pub flags: u32,
}

/// Journalist reputation scores
#[pallet::storage]
pub type JournalistReputation<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    T::AccountId,
    ReputationScore,
>;

pub struct ReputationScore {
    /// Total articles published
    pub articles_published: u32,
    /// Verified content count
    pub verified_count: u32,
    /// Community trust score (0-100)
    pub trust_score: u8,
    /// Penalty points (reduces trust)
    pub penalties: u32,
}

/// Content verification votes
#[pallet::storage]
pub type VerificationVotes<T: Config> = StorageDoubleMap<
    _,
    Blake2_128Concat, ContentHash,
    Blake2_128Concat, T::AccountId,
    VerificationVote,
>;

pub enum VerificationVote {
    Verified,
    Disputed,
    Abstain,
}

pub enum VerificationStatus {
    Pending,
    CommunityVerified,
    Disputed,
    Removed,  // Governance decision
}

pub enum ContentType {
    Article,
    Video,
    Image,
    Audio,
    Document,
}

pub enum Category {
    News,
    Opinion,
    Investigation,
    Analysis,
    Culture,
    Education,
}
```

#### Extrinsics (Functions)

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Publish new content
    #[pallet::weight(T::WeightInfo::publish_content())]
    pub fn publish_content(
        origin: OriginFor<T>,
        content_hash: Hash,  // IPFS hash
        content_type: ContentType,
        category: Category,
    ) -> DispatchResult {
        let author = ensure_signed(origin)?;

        // Verify author is verified citizen
        ensure!(
            pallet_tiki::Pallet::<T>::is_citizen(&author),
            Error::<T>::NotCitizen
        );

        // Store content metadata
        let metadata = ContentMetadata {
            author: author.clone(),
            content_hash,
            content_type,
            published_at: <frame_system::Pallet<T>>::block_number(),
            category,
            verified: VerificationStatus::Pending,
            endorsements: 0,
            flags: 0,
        };

        MediaContent::<T>::insert(content_hash, metadata);

        // Update journalist stats
        JournalistReputation::<T>::mutate(&author, |score| {
            score.articles_published = score.articles_published.saturating_add(1);
        });

        Self::deposit_event(Event::ContentPublished {
            author,
            content_hash,
            content_type,
        });

        Ok(())
    }

    /// Verify content (community fact-checking)
    #[pallet::weight(T::WeightInfo::verify_content())]
    pub fn verify_content(
        origin: OriginFor<T>,
        content_hash: Hash,
        vote: VerificationVote,
    ) -> DispatchResult {
        let voter = ensure_signed(origin)?;

        // Only citizens can verify
        ensure!(
            pallet_tiki::Pallet::<T>::is_citizen(&voter),
            Error::<T>::NotCitizen
        );

        // Record vote
        VerificationVotes::<T>::insert(content_hash, &voter, vote);

        // Update verification status based on votes
        Self::update_verification_status(content_hash)?;

        Ok(())
    }

    /// Flag content as problematic
    #[pallet::weight(T::WeightInfo::flag_content())]
    pub fn flag_content(
        origin: OriginFor<T>,
        content_hash: Hash,
        reason: FlagReason,
    ) -> DispatchResult {
        let flagger = ensure_signed(origin)?;

        MediaContent::<T>::mutate(content_hash, |metadata| {
            metadata.flags = metadata.flags.saturating_add(1);
        });

        // If too many flags, escalate to Welati governance
        let flags = MediaContent::<T>::get(content_hash)
            .map(|m| m.flags)
            .unwrap_or(0);

        if flags > T::FlagThreshold::get() {
            Self::create_governance_review(content_hash)?;
        }

        Ok(())
    }

    /// Endorse content (support journalist)
    #[pallet::weight(T::WeightInfo::endorse_content())]
    pub fn endorse_content(
        origin: OriginFor<T>,
        content_hash: Hash,
    ) -> DispatchResult {
        let endorser = ensure_signed(origin)?;

        MediaContent::<T>::mutate(content_hash, |metadata| {
            metadata.endorsements = metadata.endorsements.saturating_add(1);
        });

        // Increase journalist reputation
        let author = MediaContent::<T>::get(content_hash)
            .map(|m| m.author)
            .ok_or(Error::<T>::ContentNotFound)?;

        JournalistReputation::<T>::mutate(&author, |score| {
            score.trust_score = score.trust_score.saturating_add(1).min(100);
        });

        Ok(())
    }

    /// Reward journalist (from treasury)
    #[pallet::weight(T::WeightInfo::reward_journalist())]
    pub fn reward_journalist(
        origin: OriginFor<T>,
        journalist: T::AccountId,
        amount: BalanceOf<T>,
    ) -> DispatchResult {
        // Only Welati governance can reward
        T::GovernanceOrigin::ensure_origin(origin)?;

        // Transfer from treasury
        pallet_pez_treasury::Pallet::<T>::spend(journalist.clone(), amount)?;

        Self::deposit_event(Event::JournalistRewarded {
            journalist,
            amount,
        });

        Ok(())
    }
}
```

#### Events

```rust
#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
    /// Content published
    ContentPublished {
        author: T::AccountId,
        content_hash: Hash,
        content_type: ContentType,
    },
    /// Content verified by community
    ContentVerified {
        content_hash: Hash,
        verifiers: u32,
    },
    /// Content flagged
    ContentFlagged {
        content_hash: Hash,
        flags: u32,
    },
    /// Journalist rewarded
    JournalistRewarded {
        journalist: T::AccountId,
        amount: BalanceOf<T>,
    },
}
```

---

### Integration Points

**With Pallet Tiki:**
- Verify citizenship before content publication
- Use reputation scores for trust calculation

**With Pallet Welati:**
- Governance review for disputed content
- Journalist reward proposals

**With Pallet Pez-Treasury:**
- Funding for journalism grants
- Reward distribution

---

### Frontend Integration

**Mobile App "KurdMedia" Section:**
```typescript
// Example: Publish article
const publishArticle = async (ipfsHash: string, category: string) => {
  const api = await ApiPromise.create({ provider });

  const tx = api.tx.freeMedia.publishContent(
    ipfsHash,
    'Article',
    category
  );

  await tx.signAndSend(userAccount);
};

// Example: Verify content
const verifyContent = async (contentHash: string, vote: 'Verified' | 'Disputed') => {
  const tx = api.tx.freeMedia.verifyContent(contentHash, vote);
  await tx.signAndSend(userAccount);
};
```

---

### Implementation Timeline

**Phase 1 (Q2 2026):**
- Basic content publishing
- IPFS integration
- Simple verification system

**Phase 2 (Q3 2026):**
- Reputation system
- Advanced verification (fact-checking)
- Journalist rewards

**Phase 3 (Q4 2026):**
- Content monetization
- Subscription models
- Advanced analytics

---

## 2. Pallet Heritage

### Purpose
Preserve Kurdish cultural heritage, language content, and historical records on-chain.

### Problem Statement
- Cultural artifacts at risk of destruction
- Oral traditions being lost
- Language suppression threatens preservation
- No permanent archive for historical documents

### Solution
Blockchain-based cultural preservation with NFT tokenization and community curation.

---

### Technical Specification

#### Storage Items

```rust
/// Cultural assets
#[pallet::storage]
pub type CulturalAssets<T: Config> = StorageMap<
    _,
    Blake2_128Concat,
    AssetId,
    CulturalMetadata<T>,
>;

pub struct CulturalMetadata<T: Config> {
    /// Asset type
    pub asset_type: AssetType,
    /// Language/dialect
    pub language: Language,
    /// IPFS content hash
    pub content_hash: Hash,
    /// Submitter
    pub curator: T::AccountId,
    /// Verification status
    pub verified: bool,
    /// NFT ID (if minted)
    pub nft_id: Option<NftId>,
    /// Historical period
    pub era: Era,
    /// Geographic origin
    pub region: Region,
}

pub enum AssetType {
    Document,         // Historical documents
    Artifact,         // Physical artifacts (photos)
    OralHistory,      // Recorded stories
    Music,            // Traditional music
    Literature,       // Books, poetry
    Art,              // Visual art
    Language,         // Language lessons
}

pub enum Language {
    Kurmanji,
    Sorani,
    Zazaki,
    Feyli,
}

pub enum Era {
    Ancient,          // Pre-Islamic
    Medieval,         // 7th-15th century
    Modern,           // 16th-20th century
    Contemporary,     // 21st century
}

pub enum Region {
    NorthernKurdistan,  // Turkey
    SouthernKurdistan,  // Iraq
    EasternKurdistan,   // Iran
    WesternKurdistan,   // Syria
    Diaspora,
}
```

#### Extrinsics

```rust
#[pallet::call]
impl<T: Config> Pallet<T> {
    /// Submit cultural heritage item
    #[pallet::weight(T::WeightInfo::submit_heritage())]
    pub fn submit_heritage(
        origin: OriginFor<T>,
        asset_type: AssetType,
        language: Language,
        content_hash: Hash,
        era: Era,
        region: Region,
    ) -> DispatchResult {
        let curator = ensure_signed(origin)?;

        let asset_id = Self::next_asset_id();

        let metadata = CulturalMetadata {
            asset_type,
            language,
            content_hash,
            curator: curator.clone(),
            verified: false,  // Requires governance approval
            nft_id: None,
            era,
            region,
        };

        CulturalAssets::<T>::insert(asset_id, metadata);

        Self::deposit_event(Event::HeritageSubmitted {
            asset_id,
            curator,
            asset_type,
        });

        Ok(())
    }

    /// Verify heritage item (governance)
    #[pallet::weight(T::WeightInfo::verify_heritage())]
    pub fn verify_heritage(
        origin: OriginFor<T>,
        asset_id: AssetId,
    ) -> DispatchResult {
        T::GovernanceOrigin::ensure_origin(origin)?;

        CulturalAssets::<T>::mutate(asset_id, |metadata| {
            metadata.verified = true;
        });

        Self::deposit_event(Event::HeritageVerified { asset_id });

        Ok(())
    }

    /// Mint heritage NFT
    #[pallet::weight(T::WeightInfo::mint_heritage_nft())]
    pub fn mint_heritage_nft(
        origin: OriginFor<T>,
        asset_id: AssetId,
    ) -> DispatchResult {
        let owner = ensure_signed(origin)?;

        // Only verified assets can be minted
        let metadata = CulturalAssets::<T>::get(asset_id)
            .ok_or(Error::<T>::AssetNotFound)?;
        ensure!(metadata.verified, Error::<T>::NotVerified);

        // Mint NFT via Tiki pallet
        let nft_id = pallet_tiki::Pallet::<T>::mint_nft(
            owner.clone(),
            metadata.content_hash,
        )?;

        // Update metadata
        CulturalAssets::<T>::mutate(asset_id, |meta| {
            meta.nft_id = Some(nft_id);
        });

        Self::deposit_event(Event::HeritageNftMinted {
            asset_id,
            nft_id,
            owner,
        });

        Ok(())
    }
}
```

---

### Integration with Frontend

**Heritage Archive Interface:**
- Browse by language/dialect
- Search by era/region
- View verified cultural items
- Submit new heritage content
- Mint NFTs for preservation

---

### Implementation Timeline

**Q3 2026:** Basic heritage submission and storage
**Q4 2026:** NFT minting, advanced search
**2027:** AI-powered translation, 3D artifact scanning

---

## 3. Pallet Cooperatives

### Purpose
Enable on-chain management of worker cooperatives and collective economics.

### Technical Specification

**Status:** Detailed specification to be developed after treasury stabilization.

**Core Features (Planned):**
- Cooperative registration
- Member management
- Profit-sharing mechanisms
- Democratic decision-making
- Treasury management
- Contract enforcement

**Implementation:** Q4 2026 - Q1 2027

---

## 4. Pallet Privacy

### Purpose
Provide zero-knowledge privacy features for sensitive transactions and voting.

### Technical Specification

**Status:** Research phase. Likely to be separate parachain.

**Core Features (Planned):**
- ZK-SNARK integration
- Private voting
- Anonymous transactions (opt-in)
- Selective disclosure identity

**Implementation:** 2027-2028 (or separate privacy parachain)

---

## 5. Pallet Diaspora

### Purpose
Connect diaspora Kurdish communities with homeland through verifiable cross-border identity.

**Features:**
- Diaspora profile management
- Cross-border voting rights
- Heritage verification
- Contribution tracking

**Implementation:** 2027

---

## 6. Pallet Education

### Purpose
Decentralized education credentialing and scholarship management.

**Features:**
- Credential verification
- Scholarship distribution
- Course certification
- University diploma verification

**Implementation:** 2027

---

## 7. Pallet Remittance

### Purpose
Low-cost international money transfers for diaspora communities.

**Features:**
- Remittance channels
- Fiat on/off ramps
- Low-fee transfers
- Instant settlement

**Implementation:** 2027-2028

---

## Implementation Timeline

### 2026 Roadmap

**Q1 2026:**
- ✅ Mainnet launch (current pallets)
- Design Free-Media pallet

**Q2 2026:**
- Implement pallet-free-media
- Beta testing on testnet
- Frontend integration

**Q3 2026:**
- Implement pallet-heritage
- Multi-language support
- Cultural NFT marketplace

**Q4 2026:**
- Implement pallet-cooperatives
- Advanced economic tools
- Community feedback integration

### 2027 Roadmap

**Q1-Q2 2027:**
- Pallet-diaspora
- Pallet-education
- Cross-border features

**Q3-Q4 2027:**
- Pallet-remittance
- DEX integration
- Stablecoin mechanisms

### 2028+ Roadmap

**Long-term:**
- Pallet-privacy (or parachain)
- Parachain deployment
- Advanced AI tools
- Quantum-resistant crypto

---

## Technical Dependencies

### Infrastructure Requirements

**IPFS Integration:**
- Required for: Free-Media, Heritage
- Implementation: Pinning service, local nodes
- Cost: Storage fees, bandwidth

**ZK-Proof Libraries:**
- Required for: Privacy
- Implementation: ZK-SNARKs (Groth16 or Plonk)
- Cost: High computational overhead

**Oracle Integration:**
- Required for: Remittance, Cooperatives
- Implementation: Price feeds, fiat rates
- Cost: Oracle service fees

**NFT Standards:**
- Required for: Heritage, Education
- Implementation: Substrate NFT framework
- Cost: Minimal

---

## Governance & Approval Process

### Runtime Upgrade Process

1. **Proposal:** Core team or community submits proposal
2. **Review:** Technical review (7 days)
3. **Discussion:** Community discussion (14 days)
4. **Vote:** Welati parliament vote (14 days)
5. **Testing:** Testnet deployment (30 days)
6. **Deployment:** Mainnet runtime upgrade
7. **Monitoring:** Post-upgrade monitoring (30 days)

### Approval Criteria

**Required for Approval:**
- Technical audit passed
- Security review completed
- Community support (>66% approval)
- Testnet validation successful
- Documentation complete

---

## Risk Assessment

### Technical Risks

| Pallet | Risk Level | Mitigation |
|--------|------------|------------|
| Free-Media | Medium | Thorough testing, gradual rollout |
| Heritage | Low | Simple implementation, proven patterns |
| Cooperatives | High | Complex economics, extensive testing |
| Privacy | Very High | Separate parachain, expert audit |

### Economic Risks

- **Treasury depletion:** Conservative reward mechanisms
- **Spam content:** Fees + reputation system
- **NFT market manipulation:** Governance oversight

---

## Community Involvement

### Open Development

- All pallets developed open-source
- Community feedback encouraged
- Bounties for contributions
- Regular progress updates

### Testing & Feedback

- Public testnet access
- Bug bounty program
- Community testing rewards
- Governance participation

---

## Conclusion

This roadmap provides a clear path for expanding PezkuwiChain's capabilities beyond the core infrastructure. Each pallet addresses specific community needs while maintaining the security and decentralization of the network.

**Key Principles:**
- Community-driven development
- Security-first approach
- Incremental deployment
- Transparent governance

**Next Steps:**
1. Community review of roadmap
2. Prioritization voting
3. Technical specification refinement
4. Development kickoff (Q2 2026)

---

**Document Version:** 1.0
**Last Updated:** November 13, 2025
**Maintained By:** PezkuwiChain Core Team
**Status:** Living Document (subject to community input)

---

*"Building the future, one pallet at a time."*

**— PezkuwiChain Development Philosophy**
