# Multisig Governance Rules - Tiki-Based Authorization

**Document Version:** 1.0
**Last Updated:** 2025-11-21
**Status:** Architecture Design - Ready for Implementation
**Priority:** HIGH for Decentralized Governance

---

## 🎯 Executive Summary

PezkuwiChain multisig signers are determined by **on-chain Tiki NFT ownership**, ensuring democratic legitimacy and automatic role-based permissions. This creates a transparent, auditable, and truly decentralized governance system.

---

## 🏛️ Multisig Signer Rules

### Authorized Tiki Roles

**Only holders of these specific Tiki NFTs can be multisig signers**:

1. **Tiki::Serok** (President)
   - Elected by popular vote
   - Represents democratic will
   - 1 position = 1 multisig signer

2. **Tiki::SerokiMeclise** (Parliamentary Speaker)
   - Elected from parliament members
   - Represents legislative branch
   - 1 position = 1 multisig signer

3. **Tiki::Xezinedar** (Treasurer)
   - Appointed position (economic oversight)
   - Manages treasury operations
   - 1 position = 1 multisig signer

4. **Founder** (Transition Period)
   - Original founder account
   - Technical continuity
   - Hardcoded until removed via governance
   - 1 position = 1 multisig signer

### Additional Recommendations (For Discussion)

**Option A: Add Top Validators**
- Tiki::Validator (top 2 by stake)
- Economic alignment
- Network security stakeholders
- **Pros**: Skin in the game, proven commitment
- **Cons**: Centralization risk if whale validators

**Option B: Add Technical Council**
- Tiki::Serokweziran (Prime Minister) or technical role
- Core developer representative
- Runtime upgrade expertise
- **Pros**: Technical safety net
- **Cons**: May slow emergency response

**Option C: Keep Minimal (Recommended for Start)**
- Just Serok + SerokiMeclise + Xezinedar + Founder
- 4 signers, 3-of-4 threshold
- Simple, democratic, functional
- **Pros**: Clean, democratic, manageable
- **Cons**: Smaller pool (less redundancy)

---

## 🔢 Multisig Configuration

### Phase 1: Genesis to Month 3 (Stabilization Period)

**Signers**: **1 (Founder only)**

**Sudo**: Founder account

**Rationale**:
- Network needs to stabilize first
- Elections take time (Serok, Parliament)
- Governance structures forming
- Technical issues may require fast response
- Founder acts as benevolent dictator (temporary)

### Phase 2: Months 4-6 (Initial Multisig)

**Signers**: 4 total
- ✅ Founder (hardcoded)
- ✅ Serok (elected - first presidential election complete)
- ✅ SerokiMeclise (elected - parliament formed, speaker elected)
- ✅ Xezinedar (appointed by Serok, approved by parliament)

**Threshold**: **3-of-4**

**Rationale**:
- First democratic elections completed
- Governance structures operational
- Founder can't act alone (needs 2 more)
- Democratic signers have majority (2 elected + 1 appointed = 3)
- Simple coordination (only 4 people)

### Phase 3: Year 1-2 (Expanded Multisig)

**Signers**: 7 total
- ✅ Founder (can be removed via parliament vote)
- ✅ Serok (elected)
- ✅ SerokiMeclise (elected)
- ✅ Xezinedar (appointed)
- ✅ Additional Signer #1 (elected by parliament) 🆕
- ✅ Additional Signer #2 (elected by parliament) 🆕
- ✅ Additional Signer #3 (elected by parliament) 🆕

**Threshold**: **5-of-7**

**Rationale**:
- Network matured (1 year operational)
- **Parliament selects additional 3 signers** (democratic process)
- Additional signers can be:
  - Top Validators (economic stake)
  - Technical experts (core developers)
  - Community representatives
  - **Parliament decides via vote**
- Founder influence reduced: 1/7 = 14%
- Democratic control: 6 of 7 signers determined by elected bodies

**Parliamentary Selection Process**:
1. Parliament opens nominations for additional signers
2. Candidates present qualifications
3. Parliament votes (simple majority)
4. Top 3 candidates become multisig signers
5. Term: 2 years, renewable

### Phase 4: Year 3+ (Maximum Decentralization)

**Signers**: 9 total
- ⏳ Founder (likely removed by parliament vote)
- ✅ Serok (elected)
- ✅ SerokiMeclise (elected)
- ✅ Xezinedar (appointed)
- ✅ Additional Signer #1 (elected by parliament)
- ✅ Additional Signer #2 (elected by parliament)
- ✅ Additional Signer #3 (elected by parliament)
- ✅ Additional Signer #4 (elected by parliament) 🆕
- ✅ Additional Signer #5 (elected by parliament) 🆕

**Threshold**: **6-of-9**

**Rationale**:
- Fully mature network (3+ years)
- Founder likely removed (pure democratic governance)
- **Parliament controls all additional signers**
- Maximum decentralization
- Redundancy: Can lose 3 signers, still functional

**Founder Removal Process** (Year 2+):
1. Parliament proposes founder removal (2/3 vote required)
2. If passed, multisig updated: 8 signers, 6-of-8 threshold
3. Year 3: Add 9th signer to return to 9 total (6-of-9)
4. Founder no longer has any on-chain authority

---

## 🔐 Technical Implementation

### On-Chain Verification

**Multisig Signer Eligibility Check**:
```rust
pub fn is_eligible_multisig_signer(account: &AccountId) -> bool {
    // Check if account holds required Tiki NFT
    let tiki = Tiki::<T>::get_account_tiki(account);

    match tiki {
        Some(Tiki::Serok) => true,
        Some(Tiki::SerokiMeclise) => true,
        Some(Tiki::Xezinedar) => true,
        _ => {
            // Check if founder (hardcoded during transition)
            account == FOUNDER_ACCOUNT
        }
    }
}
```

### Automatic Multisig Update (Future Enhancement)

**When Tiki ownership changes, multisig updates automatically**:

```rust
// Hook into Tiki transfer/assignment
#[pallet::hooks]
impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn on_finalize(n: BlockNumberFor<T>) {
        // Check if any governance Tiki changed hands
        let serok = CurrentOfficials::<T>::get(GovernmentPosition::Serok);
        let speaker = CurrentOfficials::<T>::get(GovernmentPosition::SerokiMeclise);
        let treasurer = CurrentOfficials::<T>::get(GovernmentPosition::Xezinedar);

        // Compare with current multisig signers
        let current_signers = Multisig::<T>::get_signers(TREASURY_MULTISIG);

        // If mismatch, emit event for multisig update proposal
        if needs_update {
            Self::deposit_event(Event::MultisigUpdateRequired {
                old_signers: current_signers,
                new_signers: vec![founder, serok, speaker, treasurer],
            });
        }
    }
}
```

**Note**: Actual multisig update requires existing multisig approval (chicken-egg solved by requiring threshold approval).

---

## 📋 Signer Selection Process

### Serok (President)

**Selection Method**: Direct democratic election
**Pallet**: `pallet-welati`
**Extrinsic**: `start_election(ElectionType::Presidential)`
**Duration**: 4-year term
**NFT**: Tiki::Serok automatically assigned to winner

**Process**:
1. Nomination period (30 days)
2. Campaign period (60 days)
3. Voting period (14 days)
4. Winner receives Tiki::Serok NFT
5. Multisig signer list updated (requires existing multisig approval)

### SerokiMeclise (Parliamentary Speaker)

**Selection Method**: Parliament votes internally
**Prerequisite**: Tiki::Parlementer (elected to parliament first)
**Process**:
1. Parliamentary elections (100 seats)
2. New parliament elects speaker from members
3. Speaker receives Tiki::SerokiMeclise
4. Multisig signer list updated

### Xezinedar (Treasurer)

**Selection Method**: Appointed by Serok, approved by Parliament
**Process**:
1. Serok nominates candidate
2. Parliament votes (simple majority)
3. If approved, receives Tiki::Xezinedar
4. Multisig signer list updated

**Removal**: Parliament can vote to remove (2/3 majority)

### Founder

**Selection Method**: Hardcoded genesis account
**Duration**: Until community votes to remove (Year 2+)
**Special Status**: No Tiki NFT required (fallback authorization)

**Removal Process** (Future):
```rust
// Governance proposal to remove founder from multisig
Democracy::propose(
    Box::new(RuntimeCall::Multisig(
        MultisigCall::update_signers {
            remove: vec![FOUNDER_ACCOUNT],
            add: vec![],
        }
    ))
)
```

**Timeline**:
- Month 0-12: Founder essential (technical oversight)
- Year 2: Community vote on founder removal
- Year 3+: Fully Tiki-based (no hardcoded accounts)

---

## 🔄 Multisig Update Workflow

### Scenario: Serok Election Completes

**Current State**:
```
Multisig Signers: [Founder, OldSerok, Speaker, Treasurer]
Threshold: 3-of-4
```

**New Serok Elected**:
```
Winner: Alice (receives Tiki::Serok)
OldSerok: Bob (loses Tiki::Serok)
```

**Update Process**:
1. **Automatic Detection**:
   - `pallet-welati` emits `ElectionCompleted` event
   - Off-chain worker or frontend detects change

2. **Multisig Proposal**:
   - Any current signer proposes update
   - `multisig.as_multi(3, [Founder, Bob, Speaker, Treasurer], updateCall)`

3. **Threshold Approval**:
   - 3 of 4 current signers approve
   - Bob (outgoing Serok) should approve transition (good governance)

4. **Execution**:
   - New multisig: `[Founder, Alice, Speaker, Treasurer]`
   - Same threshold: 3-of-4
   - Bob no longer has signing power

5. **Verification**:
   - Query `multisig.signatories(TREASURY_MULTISIG)`
   - Confirm Alice is included, Bob removed

### Scenario: Founder Removal (Year 2+)

**Governance Referendum**:
```rust
// Community proposes founder removal
Democracy::external_propose(
    Box::new(RuntimeCall::Multisig(
        MultisigCall::remove_signer {
            multisig: TREASURY_MULTISIG,
            signer: FOUNDER_ACCOUNT,
        }
    ))
)

// Referendum passes with >50% approval + >25% turnout
// Founder is removed
// New multisig: [Serok, Speaker, Treasurer]
// New threshold: 2-of-3
```

---

## 🛡️ Security Features

### Checks and Balances

1. **No Single Point of Control**
   - Founder alone: ❌ Can't execute (needs 2 more)
   - Serok alone: ❌ Can't execute (needs 2 more)
   - Any 2 signers: ❌ Can't execute (needs 3)
   - Any 3 signers: ✅ Can execute

2. **Democratic Oversight**
   - 2 elected positions (Serok + Speaker)
   - 1 appointed position (Treasurer, removable)
   - 1 founder position (removable via governance)

3. **Automatic Role Verification**
   - Frontend checks Tiki ownership before showing multisig UI
   - Runtime validates Tiki on proposal creation
   - No manual signer management (Tiki = authority)

4. **Transparent Transitions**
   - All multisig changes on-chain
   - Event logs for auditing
   - Community visibility into signer changes

### Attack Resistance

**Scenario: Malicious Serok**
- Can't act alone (needs 2 more signers)
- Parliament Speaker represents legislative check
- Treasurer represents economic check
- Founder provides technical safety net

**Scenario: Colluding Validators**
- If validators in multisig (Option B), only 2 of 7
- Can't reach 5-of-7 threshold without democratic signers
- Serok + Speaker control (democratic legitimacy)

**Scenario: Founder Goes Rogue**
- Needs 2 more signers (elected officials)
- Community can remove via governance (Year 2+)
- Temporary risk accepted for technical stability

---

## 📊 Comparison: Options A vs B vs C

| Feature | Option A (Minimal) | Option B (Extended) | Option C (Maximum) |
|---------|-------------------|---------------------|-------------------|
| **Total Signers** | 4 | 7 | 9 |
| **Threshold** | 3-of-4 | 5-of-7 | 6-of-9 |
| **Democratic Positions** | 2 (Serok, Speaker) | 2 | 2 |
| **Appointed Positions** | 1 (Treasurer) | 2 (Treasurer, PM) | 1 |
| **Economic Stake** | None | 2 validators | 3 validators |
| **Founder Influence** | 1/4 = 25% | 1/7 = 14% | 1/9 = 11% |
| **Coordination Speed** | ⚡ Fast (4 people) | 🐌 Moderate (7 people) | 🐌 Slow (9 people) |
| **Redundancy** | ⚠️ Low (1 failure = risky) | ✅ Good (2 failures ok) | ✅ High (3 failures ok) |
| **Complexity** | 😊 Simple | 😐 Moderate | 😰 Complex |
| **Best For** | Early Mainnet | Mature network | Long-term (Year 2+) |

**Recommendation Progression**:
```
Year 1: Option A (get started, simple)
Year 2: Option B (add validators, stability)
Year 3+: Option C (maximum decentralization)
```

---

## 🎯 Frontend Integration

### Multisig Signer Detection

```typescript
// Check if connected account is eligible multisig signer
async function isMultisigSigner(api: ApiPromise, account: string): Promise<boolean> {
  // Check Tiki ownership
  const tiki = await api.query.tiki.accountTikis(account);

  if (tiki.isSome) {
    const tikiType = tiki.unwrap();

    // Check if Tiki is governance role
    if (
      tikiType.isSerok ||
      tikiType.isSerokiMeclise ||
      tikiType.isXezinedar
    ) {
      return true;
    }
  }

  // Check if founder (fallback)
  const sudoKey = await api.query.sudo.key();
  if (account === sudoKey.toString()) {
    return true;
  }

  return false;
}
```

### Admin Panel UI

```typescript
// Display multisig signer status
const MultisigSignerBadge: React.FC = () => {
  const { account } = useWallet();
  const { api } = usePolkadot();
  const [signerStatus, setSignerStatus] = useState<SignerStatus | null>(null);

  useEffect(() => {
    if (account && api) {
      checkSignerStatus(api, account).then(status => {
        setSignerStatus(status);
      });
    }
  }, [account, api]);

  if (!signerStatus?.isEligible) return null;

  return (
    <Badge variant="gradient" className="bg-gradient-to-r from-green-600 to-yellow-600">
      <Shield className="w-3 h-3 mr-1" />
      {signerStatus.role === 'Serok' && 'Multisig Signer (Serok)'}
      {signerStatus.role === 'Speaker' && 'Multisig Signer (Speaker)'}
      {signerStatus.role === 'Treasurer' && 'Multisig Signer (Treasurer)'}
      {signerStatus.role === 'Founder' && 'Multisig Signer (Founder)'}
    </Badge>
  );
};
```

### Multisig Proposal UI

```typescript
// Show pending multisig proposals
const MultisigProposals: React.FC = () => {
  const { api } = usePolkadot();
  const [proposals, setProposals] = useState<MultisigProposal[]>([]);

  useEffect(() => {
    // Fetch pending multisig proposals
    api.query.multisig.multisigs.entries().then(entries => {
      const pendingProposals = entries
        .filter(([key, value]) => value.isSome)
        .map(([key, value]) => parseMultisigProposal(key, value));

      setProposals(pendingProposals);
    });
  }, [api]);

  return (
    <div className="space-y-4">
      <h3>Pending Multisig Proposals</h3>
      {proposals.map(proposal => (
        <MultisigProposalCard
          key={proposal.hash}
          proposal={proposal}
          onApprove={handleApprove}
        />
      ))}
    </div>
  );
};
```

---

## 📚 Appendix A: Tiki Role Definitions

### Governance Tikis (Multisig Eligible)

| Tiki | Kurdish | English | Selection | Term | Multisig? |
|------|---------|---------|-----------|------|-----------|
| `Serok` | سەرۆك | President | Popular Vote | 4 years | ✅ YES |
| `SerokiMeclise` | سەرۆکی مەجلیس | Speaker | Parliament Vote | 4 years | ✅ YES |
| `Xezinedar` | خەزینەدار | Treasurer | Appointment | 4 years | ✅ YES |

### Other Government Tikis (Not Multisig)

| Tiki | Kurdish | English | Multisig? |
|------|---------|---------|-----------|
| `Serokweziran` | سەرۆکوەزیران | Prime Minister | ❌ NO |
| `Wezir` | وەزیر | Minister | ❌ NO |
| `Parlementer` | پەرلەمەنتەر | Parliamentarian | ❌ NO |
| `Validator` | ڤالیدێتەر | Validator | ❌ NO (unless added per Option B) |

**Rationale**: Only top executive/legislative/economic positions in multisig. Prevents multisig bloat while maintaining legitimacy.

---

## 📚 Appendix B: Emergency Scenarios

### Scenario: Serok Becomes Unresponsive

**Problem**: Serok holder loses private key or becomes inactive

**Solution**:
1. Parliament initiates impeachment vote
2. If passed (2/3 majority), Serok Tiki is revoked
3. New presidential election held (emergency, 30-day process)
4. New Serok receives Tiki, joins multisig
5. During transition: 3 remaining signers can still operate (2-of-3)

**Mitigation**: Treasury operations continue with remaining signers

### Scenario: Coordinated Attack (3 Signers Compromised)

**Problem**: Attackers gain control of 3-of-4 signers

**Prevention**:
- Signers use hardware wallets (Ledger, Trezor)
- Geographic distribution (different jurisdictions)
- Operational security training

**Recovery**:
1. Remaining honest signer(s) raise alarm
2. Emergency governance proposal (fast-track)
3. Community votes to replace compromised signers
4. New Tiki holders assigned, multisig reconstituted

**Note**: This is catastrophic scenario, requires governance intervention

### Scenario: Founder Refuses to Sign Critical Upgrade

**Problem**: Founder blocks emergency runtime upgrade

**Solution**:
1. Serok + Speaker + Treasurer = 3-of-4 threshold ✅
2. Can execute without founder approval
3. Founder influence limited (can't block democratic will)

**Long-term**: Community removes founder via governance (Year 2+)

---

## 🔄 Migration Path

### From Current Single-Account Treasury

**Step 1: Create Multisig (Month 3-6)**
```rust
// Founder creates multisig on-chain
Multisig::as_multi_threshold_1(
    signatories: [Founder, Serok, Speaker, Treasurer].sorted(),
    call: System::remark("Multisig treasury created")
)

// Get multisig address
let multisig_addr = Multisig::multi_account_id(&signatories, 3);
```

**Step 2: Fund Multisig**
```rust
// Transfer treasury funds to multisig
Balances::transfer(multisig_addr, 70_000_000 * HEZ);
Assets::transfer(PEZ_ASSET_ID, multisig_addr, 4_812_500_000 * HEZ);
```

**Step 3: Verify**
```rust
// Confirm multisig has funds
let balance = Balances::free_balance(multisig_addr);
assert!(balance >= 70_000_000 * HEZ);

// Confirm old treasury is empty
let old_balance = Balances::free_balance(OLD_TREASURY);
assert_eq!(old_balance, 0);
```

**Step 4: Update Sudo (Month 6)**
```rust
// Founder transfers sudo to multisig
Sudo::set_key(multisig_addr);
```

**Complete!** Decentralized treasury + decentralized sudo ✅

---

## 📞 Communication Plan

### Announce Tiki-Based Multisig Rules

**Pre-Mainnet**:
- Documentation: This document
- Blog post: "How Tiki NFTs Power PezkuwiChain Governance"
- Community AMA: Explain multisig rules

**Post-Mainnet (Month 1)**:
- Governance portal: Show current multisig signers with Tiki badges
- Transparency dashboard: Live multisig proposals
- Tutorial: "How to Become a Multisig Signer (Get Elected!)"

**Ongoing**:
- Monthly reports: Multisig activity (# proposals, approvals, rejections)
- Annual review: Evaluate if expansion needed (Option A → B → C)

---

## ✅ Summary: Why Tiki-Based Multisig?

### Advantages

✅ **Democratic Legitimacy**: Signers are elected, not appointed by founder
✅ **Transparent**: Tiki ownership = multisig power (visible on-chain)
✅ **Automatic**: Role changes trigger multisig updates
✅ **Auditable**: All signer changes logged as events
✅ **Secure**: Threshold prevents single point of failure
✅ **Flexible**: Can expand signers as network matures (A → B → C)
✅ **Fair**: Founder is 1-of-4, not dominant (can be removed later)

### Unique Features vs Other Chains

| Feature | PezkuwiChain | Polkadot | Kusama | Substrate Default |
|---------|--------------|----------|--------|-------------------|
| **Multisig Authority** | Tiki NFT | Council Election | Council Election | Manual Assignment |
| **Automatic Updates** | Yes (Tiki transfer) | No | No | No |
| **Democratic Input** | Direct (Serok election) | Indirect (council) | Indirect (council) | None |
| **Transparency** | NFT visible | Council list | Council list | Address list |
| **Cultural Alignment** | Kurdish governance titles | Generic | Generic | Generic |

---

**Next Steps**:
1. ✅ Document created (this file)
2. ⏳ Community feedback period (2 weeks)
3. ⏳ Finalize Option A vs B vs C (recommend A for start)
4. ⏳ Implement automatic Tiki verification in runtime
5. ⏳ Build frontend multisig UI
6. ⏳ Test on testnet (Month 3-6)
7. ⏳ Deploy on Mainnet (Month 6)

---

**Last Updated:** 2025-11-21
**Status:** Awaiting Community Feedback
**Forum Discussion**: https://forum.pezkuwichain.io/t/multisig-governance-rules

---

*"Tiki NFTs represent on-chain authority. Multisig signers must earn their role through democratic legitimacy, not centralized appointment."*
