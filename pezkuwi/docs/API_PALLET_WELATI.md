# Welati (Governance) Pallet API Documentation

## Overview

The Welati Pallet is a comprehensive governance system implementing democratic elections, voting, and government structure management for PezkuwiChain. It provides complete infrastructure for running a digital democracy with executive, legislative, and advisory branches.

### Key Features

- **Presidential Elections**: Direct democratic election with runoff mechanism
- **Parliamentary Elections**: District-based representation system
- **Cabinet Formation**: Prime Minister selection and ministerial appointments
- **Diwan Council**: Constitutional advisory council
- **Proposal System**: Legislative proposals with weighted voting
- **Official Appointments**: Nomination and confirmation process for government positions
- **Multi-Phase Elections**: Candidacy, campaign, and voting periods
- **Trust-Score Weighted Voting**: Enhanced voting power based on trust scores

### Dependencies

This pallet integrates with:
- **pallet-tiki**: Automatic role assignment for election winners
- **pallet-trust**: Trust score verification for candidates and weighted voting
- **pallet-identity-kyc**: KYC verification for voting eligibility

---

## Configuration (Config trait)

### Associated Types

| Type | Description |
|------|-------------|
| `RuntimeEvent` | The overarching event type |
| `WeightInfo` | Weight information for extrinsics |
| `Randomness` | Randomness source for tie-breaking |
| `RuntimeCall` | Call type for proposal execution |
| `TrustScoreSource` | Provider for trust score queries |
| `TikiSource` | Provider for role score queries |
| `CitizenSource` | Provider for citizen count |
| `KycSource` | Provider for KYC status |

### Runtime Constants

| Constant | Description |
|----------|-------------|
| `ParliamentSize` | Number of parliament seats (e.g., 201) |
| `DiwanSize` | Number of Diwan council members (e.g., 50) |
| `ElectionPeriod` | Duration of voting period in blocks (e.g., 1,728,000 = ~4 months) |
| `CandidacyPeriod` | Duration for candidate registration (e.g., 43,200 = ~3 days) |
| `CampaignPeriod` | Duration for campaigning (e.g., 144,000 = ~10 days) |
| `ElectoralDistricts` | Number of electoral districts (e.g., 10) |
| `CandidacyDeposit` | Deposit required to register as candidate (e.g., 100 tokens) |
| `PresidentialEndorsements` | Endorsements needed for presidential candidacy (e.g., 1000) |
| `ParliamentaryEndorsements` | Endorsements needed for parliamentary candidacy (e.g., 100) |

### Runtime Configuration Example

```rust
impl pallet_welati::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_welati::weights::SubstrateWeight<Runtime>;
    type Randomness = RandomnessCollectiveFlip;
    type RuntimeCall = RuntimeCall;
    type TrustScoreSource = Trust;
    type TikiSource = Tiki;
    type CitizenSource = IdentityKyc;
    type KycSource = IdentityKyc;
    type ParliamentSize = ConstU32<201>;
    type DiwanSize = ConstU32<50>;
    type ElectionPeriod = ConstU32<1_728_000>;
    type CandidacyPeriod = ConstU32<43_200>;
    type CampaignPeriod = ConstU32<144_000>;
    type ElectoralDistricts = ConstU32<10>;
    type CandidacyDeposit = ConstU128<100_000_000_000_000>;
    type PresidentialEndorsements = ConstU32<1000>;
    type ParliamentaryEndorsements = ConstU32<100>;
}
```

---

## Storage Items

### Core Governance Storage

#### CurrentOfficials
**Type:** `StorageMap<_, Blake2_128Concat, GovernmentPosition, T::AccountId, OptionQuery>`

**Description:** Maps government positions to their current holders.

**Positions:**
- `Serok` - President
- `MeclisBaskanı` - Parliament Speaker

**Access:** Read via `current_officials(position)` getter

---

#### CurrentMinisters
**Type:** `StorageMap<_, Blake2_128Concat, MinisterRole, T::AccountId, OptionQuery>`

**Description:** Maps ministerial roles to their current holders.

**Roles:**
- `SerokWeziran` - Prime Minister
- `WezireDarayiye` - Minister of Finance
- `WezireParez` - Minister of Defense
- `WezireDad` - Minister of Justice
- `WezireBelaw` - Minister of Education
- `WezireTend` - Minister of Health
- `WezireAva` - Minister of Water Resources
- `WezireCand` - Minister of Culture

**Access:** Read via `current_ministers(role)` getter

---

#### ParliamentMembers
**Type:** `StorageValue<_, BoundedVec<ParliamentMember<T>, T::ParliamentSize>, ValueQuery>`

**Description:** List of all current parliament members with their metadata.

**ParliamentMember Structure:**
```rust
pub struct ParliamentMember<T: Config> {
    pub account: T::AccountId,
    pub elected_at: BlockNumberFor<T>,
    pub term_ends_at: BlockNumberFor<T>,
    pub votes_participated: u32,
    pub total_votes_eligible: u32,
    pub participation_rate: u8,
    pub committees: BoundedVec<Committee, ConstU32<10>>,
}
```

**Access:** Read via `parliament_members()` getter

---

#### DiwanMembers
**Type:** `StorageValue<_, BoundedVec<DiwanMember<T>, T::DiwanSize>, ValueQuery>`

**Description:** List of all Diwan (constitutional council) members.

**DiwanMember Structure:**
```rust
pub struct DiwanMember<T: Config> {
    pub account: T::AccountId,
    pub appointed_at: BlockNumberFor<T>,
    pub term_ends_at: BlockNumberFor<T>,
    pub appointed_by: AppointmentAuthority<T>,
    pub specialization: ConstitutionalSpecialization,
    pub decisions_made: u32,
}
```

**Access:** Read via `diwan_members()` getter

---

#### AppointedOfficials
**Type:** `StorageMap<_, Blake2_128Concat, OfficialRole, T::AccountId, OptionQuery>`

**Description:** Maps appointed government positions to their holders.

**Access:** Read via `appointed_officials(role)` getter

---

### Election System Storage

#### ActiveElections
**Type:** `StorageMap<_, Blake2_128Concat, u32, ElectionInfo<T>, OptionQuery>`

**Description:** Stores information about ongoing elections.

**ElectionInfo Structure:**
```rust
pub struct ElectionInfo<T: Config> {
    pub election_id: u32,
    pub election_type: ElectionType,
    pub start_block: BlockNumberFor<T>,
    pub candidacy_deadline: BlockNumberFor<T>,
    pub campaign_start: BlockNumberFor<T>,
    pub voting_start: BlockNumberFor<T>,
    pub end_block: BlockNumberFor<T>,
    pub candidates: BoundedVec<T::AccountId, ConstU32<500>>,
    pub total_votes: u32,
    pub status: ElectionStatus,
    pub districts: BoundedVec<ElectoralDistrict, ConstU32<50>>,
    pub minimum_turnout: u8,
}
```

---

#### NextElectionId
**Type:** `StorageValue<_, u32, ValueQuery>`

**Description:** Counter for generating unique election IDs.

**Access:** Read via `next_election_id()` getter

---

#### ElectionCandidates
**Type:** `StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, CandidateInfo<T>, OptionQuery>`

**Description:** Stores candidate information for each election.

**CandidateInfo Structure:**
```rust
pub struct CandidateInfo<T: Config> {
    pub account: T::AccountId,
    pub district_id: Option<u32>,
    pub registered_at: BlockNumberFor<T>,
    pub endorsers: BoundedVec<T::AccountId, ConstU32<1000>>,
    pub vote_count: u32,
    pub deposit_paid: u128,
    pub campaign_data: BoundedVec<u8, ConstU32<1000>>,
}
```

---

#### ElectionVotes
**Type:** `StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, ElectionVoteInfo<T>, OptionQuery>`

**Description:** Records votes cast in elections.

**ElectionVoteInfo Structure:**
```rust
pub struct ElectionVoteInfo<T: Config> {
    pub voter: T::AccountId,
    pub candidates: BoundedVec<T::AccountId, ConstU32<20>>,
    pub vote_block: BlockNumberFor<T>,
    pub vote_weight: u32,
    pub vote_type: VoteType,
    pub district_id: Option<u32>,
}
```

---

#### ElectionResults
**Type:** `StorageMap<_, Blake2_128Concat, u32, ElectionResult<T>, OptionQuery>`

**Description:** Stores finalized election results.

**ElectionResult Structure:**
```rust
pub struct ElectionResult<T: Config> {
    pub election_id: u32,
    pub winners: BoundedVec<T::AccountId, T::ParliamentSize>,
    pub total_votes: u32,
    pub turnout_percentage: u8,
    pub finalized_at: BlockNumberFor<T>,
}
```

---

### Appointment System Storage

#### PendingNominations
**Type:** `StorageDoubleMap<_, Blake2_128Concat, OfficialRole, Blake2_128Concat, T::AccountId, NominationInfo<T>, OptionQuery>`

**Description:** Tracks pending official nominations.

---

#### AppointmentProcesses
**Type:** `StorageMap<_, Blake2_128Concat, u32, AppointmentProcess<T>, OptionQuery>`

**Description:** Stores ongoing appointment processes.

---

#### NextAppointmentId
**Type:** `StorageValue<_, u32, ValueQuery>`

**Description:** Counter for appointment process IDs.

---

### Collective Decision Storage

#### ActiveProposals
**Type:** `StorageMap<_, Blake2_128Concat, u32, CollectiveProposal<T>, OptionQuery>`

**Description:** Stores active legislative proposals.

**CollectiveProposal Structure:**
```rust
pub struct CollectiveProposal<T: Config> {
    pub proposal_id: u32,
    pub proposer: T::AccountId,
    pub title: BoundedVec<u8, ConstU32<100>>,
    pub description: BoundedVec<u8, ConstU32<1000>>,
    pub proposed_at: BlockNumberFor<T>,
    pub voting_starts_at: BlockNumberFor<T>,
    pub expires_at: BlockNumberFor<T>,
    pub decision_type: CollectiveDecisionType,
    pub status: ProposalStatus,
    pub aye_votes: u32,
    pub nay_votes: u32,
    pub abstain_votes: u32,
    pub threshold: u32,
    pub votes_cast: u32,
    pub priority: ProposalPriority,
    pub call: Option<Box<<T as frame_system::Config>::RuntimeCall>>,
}
```

---

#### NextProposalId
**Type:** `StorageValue<_, u32, ValueQuery>`

**Description:** Counter for proposal IDs.

---

#### CollectiveVotes
**Type:** `StorageDoubleMap<_, Blake2_128Concat, u32, Blake2_128Concat, T::AccountId, CollectiveVote<T>, OptionQuery>`

**Description:** Records votes on proposals.

---

## Types and Enums

### ElectionType
```rust
pub enum ElectionType {
    Presidential,           // Presidential election
    Parliamentary,          // Parliament member election
    SpeakerElection,       // Parliament speaker election
    ConstitutionalCourt,   // Diwan council election
}
```

### ElectionStatus
```rust
pub enum ElectionStatus {
    CandidacyPeriod,   // Accepting candidate registrations
    CampaignPeriod,    // Candidates campaigning
    VotingPeriod,      // Active voting
    Completed,         // Election finalized
}
```

### CollectiveDecisionType
```rust
pub enum CollectiveDecisionType {
    ParliamentSimpleMajority,      // >50% of parliament
    ParliamentSuperMajority,       // >66% of parliament
    ParliamentAbsoluteMajority,    // >75% of parliament
    ConstitutionalReview,          // Diwan review
    ConstitutionalUnanimous,       // Unanimous Diwan decision
    ExecutiveDecision,             // Presidential decision
}
```

### ProposalStatus
```rust
pub enum ProposalStatus {
    Active,      // Currently accepting votes
    Passed,      // Proposal passed
    Rejected,    // Proposal rejected
    Executed,    // Proposal executed
    Expired,     // Voting period expired
}
```

### VoteChoice
```rust
pub enum VoteChoice {
    Aye,      // Vote in favor
    Nay,      // Vote against
    Abstain,  // Abstain from voting
}
```

---

## Extrinsics (Callable Functions)

### 1. initiate_election

**Description:** Starts a new election process (Root only).

**Signature:**
```rust
pub fn initiate_election(
    origin: OriginFor<T>,
    election_type: ElectionType,
    districts: Option<Vec<ElectoralDistrict>>,
    initial_candidates: Option<BoundedVec<T::AccountId, ConstU32<2>>>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be root
- `election_type`: Type of election to initiate
- `districts`: Electoral districts (for parliamentary elections)
- `initial_candidates`: Pre-selected candidates (for runoff elections only)

**Requirements:**
- Must be called with root origin
- For runoff elections, must provide exactly 2 initial candidates

**Events Emitted:**
- `ElectionStarted { election_id, election_type, start_block, end_block }`

**Errors:**
- `InvalidElectionType` - Invalid election type for runoff
- `InvalidInitialCandidates` - Wrong number of runoff candidates
- `InvalidDistrict` - Invalid district configuration

**Weight:** `WeightInfo::initiate_election()`

**Example:**
```rust
// Start presidential election
Welati::initiate_election(
    Origin::root(),
    ElectionType::Presidential,
    None,
    None
)?;
```

---

### 2. register_candidate

**Description:** Register as a candidate for an election.

**Signature:**
```rust
pub fn register_candidate(
    origin: OriginFor<T>,
    election_id: u32,
    district_id: Option<u32>,
    endorsers: Vec<T::AccountId>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by the candidate
- `election_id`: ID of the election
- `district_id`: Electoral district (for parliamentary elections)
- `endorsers`: List of endorsing citizens

**Requirements:**
- KYC must be approved
- Must be within candidacy period
- Must have required trust score:
  - Presidential: 600
  - Parliamentary: 300
  - Speaker: 400
  - Constitutional Court: 750
- Must have required endorsements:
  - Presidential: 1000 citizens
  - Parliamentary: 100 citizens
- Must pay candidacy deposit
- Must not already be registered

**Events Emitted:**
- `CandidateRegistered { election_id, candidate, deposit_paid }`

**Errors:**
- `ElectionNotFound` - Election doesn't exist
- `CandidacyPeriodExpired` - Registration period ended
- `NotACitizen` - KYC not approved
- `InsufficientTrustScore` - Trust score too low
- `InsufficientEndorsements` - Not enough endorsers
- `AlreadyCandidate` - Already registered

**Weight:** `WeightInfo::register_candidate()`

**Example:**
```rust
// Register for presidential election
Welati::register_candidate(
    Origin::signed(alice.clone()),
    election_id,
    None,
    endorsers_list
)?;
```

---

### 3. cast_vote

**Description:** Cast vote in an active election.

**Signature:**
```rust
pub fn cast_vote(
    origin: OriginFor<T>,
    election_id: u32,
    candidates: Vec<T::AccountId>,
    district_id: Option<u32>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by the voter
- `election_id`: ID of the election
- `candidates`: List of candidates to vote for
- `district_id`: Electoral district (for parliamentary elections)

**Requirements:**
- KYC must be approved
- Must be within voting period
- Must not have already voted
- All candidates must be registered

**Vote Weight:**
- Presidential/Parliamentary: 1 vote per citizen
- Other elections: Trust-score weighted (trust_score / 100, capped 1-10)

**Events Emitted:**
- `VoteCast { election_id, voter, candidates, district_id }`

**Errors:**
- `ElectionNotFound` - Election doesn't exist
- `VotingPeriodNotStarted` - Voting hasn't started
- `NotACitizen` - KYC not approved
- `AlreadyVoted` - Already voted in this election

**Weight:** `WeightInfo::cast_vote()`

**Example:**
```rust
// Vote for presidential candidate
Welati::cast_vote(
    Origin::signed(bob.clone()),
    election_id,
    vec![alice.clone()],
    None
)?;
```

---

### 4. finalize_election

**Description:** Finalizes election and determines winners (Root only).

**Signature:**
```rust
pub fn finalize_election(
    origin: OriginFor<T>,
    election_id: u32,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be root
- `election_id`: ID of the election to finalize

**Requirements:**
- Must be called with root origin
- Voting period must be ended
- Election must not already be finalized

**Behavior:**
- **Presidential**: Winner needs >50% of votes, otherwise triggers runoff
- **Parliamentary**: Top N candidates win (N = ParliamentSize)
- **Speaker**: Top candidate wins
- **Diwan**: Top N candidates win (N = DiwanSize)
- Automatically assigns Tiki roles to winners

**Events Emitted:**
- `ElectionFinalized { election_id, winners, total_votes, turnout_percentage }`
- For presidential runoff: `ElectionStarted` for runoff election

**Errors:**
- `ElectionNotFound` - Election doesn't exist
- `ElectionNotActive` - Voting period not ended
- `ElectionAlreadyFinalized` - Already finalized
- `ParliamentFull` - Too many winners for parliament
- `DiwanFull` - Too many winners for Diwan

**Weight:** `WeightInfo::finalize_election()`

**Example:**
```rust
// Finalize election
Welati::finalize_election(Origin::root(), election_id)?;
```

---

### 5. nominate_official

**Description:** Nominate someone for a government position.

**Signature:**
```rust
pub fn nominate_official(
    origin: OriginFor<T>,
    nominee: T::AccountId,
    role: OfficialRole,
    justification: BoundedVec<u8, ConstU32<1000>>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by nominator (must be Serok or Minister)
- `nominee`: Account to nominate
- `role`: Government role to assign
- `justification`: Reasoning for nomination

**Requirements:**
- Nominator must be Serok or a Minister
- Role must not already be filled
- No pending nomination for this role and nominee

**Events Emitted:**
- `OfficialNominated { process_id, nominator, nominee, role }`

**Errors:**
- `NotAuthorizedToNominate` - Not authorized to nominate
- `RoleAlreadyFilled` - Position already occupied

**Weight:** `WeightInfo::nominate_official()`

**Example:**
```rust
// President nominates a treasurer
Welati::nominate_official(
    Origin::signed(serok.clone()),
    alice.clone(),
    OfficialRole::Xezinedar,
    b"Highly qualified candidate".to_vec().try_into().unwrap()
)?;
```

---

### 6. approve_appointment

**Description:** Approve a pending nomination (Serok only).

**Signature:**
```rust
pub fn approve_appointment(
    origin: OriginFor<T>,
    process_id: u32,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by Serok
- `process_id`: ID of the appointment process

**Requirements:**
- Caller must be current Serok
- Appointment process must exist
- Status must be `WaitingPresidentialApproval`

**Events Emitted:**
- `AppointmentApproved { process_id, approver, appointee, role }`

**Errors:**
- `NotAuthorizedToApprove` - Not authorized
- `AppointmentProcessNotFound` - Process doesn't exist
- `AppointmentAlreadyProcessed` - Already processed

**Weight:** `WeightInfo::approve_appointment()`

**Example:**
```rust
// Serok approves appointment
Welati::approve_appointment(
    Origin::signed(serok.clone()),
    process_id
)?;
```

---

### 7. submit_proposal

**Description:** Submit a legislative proposal.

**Signature:**
```rust
pub fn submit_proposal(
    origin: OriginFor<T>,
    title: BoundedVec<u8, ConstU32<100>>,
    description: BoundedVec<u8, ConstU32<1000>>,
    decision_type: CollectiveDecisionType,
    priority: ProposalPriority,
    call: Option<Box<<T as frame_system::Config>::RuntimeCall>>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by proposer
- `title`: Proposal title (max 100 bytes)
- `description`: Proposal description (max 1000 bytes)
- `decision_type`: Type of collective decision
- `priority`: Proposal priority level
- `call`: Optional runtime call to execute if passed

**Requirements:**
- For `ExecutiveDecision`: Must be Serok
- For other types: Must be Parliament member or Serok

**Voting Thresholds:**
- `ParliamentSimpleMajority`: >50% of parliament
- `ParliamentSuperMajority`: >66% of parliament
- `ParliamentAbsoluteMajority`: >75% of parliament
- `ConstitutionalReview`: >66% of Diwan
- `ConstitutionalUnanimous`: 100% of Diwan

**Events Emitted:**
- `ProposalSubmitted { proposal_id, proposer, decision_type, voting_deadline }`

**Errors:**
- `NotAuthorizedToPropose` - Not authorized to submit proposals

**Weight:** `WeightInfo::submit_proposal()`

**Example:**
```rust
// Submit a simple majority proposal
Welati::submit_proposal(
    Origin::signed(parlementer.clone()),
    b"Budget Amendment".to_vec().try_into().unwrap(),
    b"Increase education budget by 10%".to_vec().try_into().unwrap(),
    CollectiveDecisionType::ParliamentSimpleMajority,
    ProposalPriority::Normal,
    None
)?;
```

---

### 8. vote_on_proposal

**Description:** Vote on an active proposal.

**Signature:**
```rust
pub fn vote_on_proposal(
    origin: OriginFor<T>,
    proposal_id: u32,
    vote: VoteChoice,
    rationale: Option<BoundedVec<u8, ConstU32<500>>>,
) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by voter
- `proposal_id`: ID of the proposal
- `vote`: Vote choice (Aye/Nay/Abstain)
- `rationale`: Optional explanation for vote

**Requirements:**
- Must be Parliament member (for Parliament decisions)
- Must be Diwan member (for Constitutional decisions)
- Must not have already voted on this proposal

**Events Emitted:**
- `CollectiveVoteCast { proposal_id, voter, vote }`

**Errors:**
- `ProposalNotFound` - Proposal doesn't exist
- `NotAuthorizedToVote` - Not authorized to vote
- `ProposalAlreadyVoted` - Already voted

**Weight:** `WeightInfo::vote_on_proposal()`

**Example:**
```rust
// Vote in favor of proposal
Welati::vote_on_proposal(
    Origin::signed(parlementer.clone()),
    proposal_id,
    VoteChoice::Aye,
    Some(b"Strong support for this initiative".to_vec().try_into().unwrap())
)?;
```

---

## Events

### Election Events

#### ElectionStarted
```rust
ElectionStarted {
    election_id: u32,
    election_type: ElectionType,
    start_block: BlockNumberFor<T>,
    end_block: BlockNumberFor<T>,
}
```
Emitted when a new election is initiated.

---

#### CandidateRegistered
```rust
CandidateRegistered {
    election_id: u32,
    candidate: T::AccountId,
    deposit_paid: u128,
}
```
Emitted when a candidate successfully registers.

---

#### VoteCast
```rust
VoteCast {
    election_id: u32,
    voter: T::AccountId,
    candidates: Vec<T::AccountId>,
    district_id: Option<u32>,
}
```
Emitted when a vote is cast.

---

#### ElectionFinalized
```rust
ElectionFinalized {
    election_id: u32,
    winners: Vec<T::AccountId>,
    total_votes: u32,
    turnout_percentage: u8,
}
```
Emitted when an election is finalized.

---

### Appointment Events

#### OfficialNominated
```rust
OfficialNominated {
    process_id: u32,
    nominator: T::AccountId,
    nominee: T::AccountId,
    role: OfficialRole,
}
```
Emitted when an official is nominated.

---

#### AppointmentApproved
```rust
AppointmentApproved {
    process_id: u32,
    approver: T::AccountId,
    appointee: T::AccountId,
    role: OfficialRole,
}
```
Emitted when an appointment is approved.

---

#### AppointmentRejected
```rust
AppointmentRejected {
    process_id: u32,
    rejector: T::AccountId,
    nominee: T::AccountId,
    role: OfficialRole,
    reason: BoundedVec<u8, ConstU32<500>>,
}
```
Emitted when an appointment is rejected.

---

### Collective Decision Events

#### ProposalSubmitted
```rust
ProposalSubmitted {
    proposal_id: u32,
    proposer: T::AccountId,
    decision_type: CollectiveDecisionType,
    voting_deadline: BlockNumberFor<T>,
}
```
Emitted when a proposal is submitted.

---

#### CollectiveVoteCast
```rust
CollectiveVoteCast {
    proposal_id: u32,
    voter: T::AccountId,
    vote: VoteChoice,
}
```
Emitted when a vote is cast on a proposal.

---

#### ProposalFinalized
```rust
ProposalFinalized {
    proposal_id: u32,
    result: ProposalStatus,
    aye_votes: u32,
    nay_votes: u32,
    abstain_votes: u32,
}
```
Emitted when a proposal is finalized.

---

### Governance Events

#### ParliamentUpdated
```rust
ParliamentUpdated {
    new_members: Vec<T::AccountId>,
    term_start: BlockNumberFor<T>,
}
```
Emitted when parliament composition changes.

---

#### DiwanMemberAppointed
```rust
DiwanMemberAppointed {
    member: T::AccountId,
    appointed_by: AppointmentAuthority<T>,
    specialization: ConstitutionalSpecialization,
}
```
Emitted when a Diwan member is appointed.

---

#### VetoApplied
```rust
VetoApplied {
    proposal_id: u32,
    vetoed_by: T::AccountId,
    reason: BoundedVec<u8, ConstU32<1000>>,
}
```
Emitted when a veto is applied to a proposal.

---

## Errors

| Error | Description |
|-------|-------------|
| `InsufficientTrustScore` | Candidate's trust score too low |
| `MissingRequiredTiki` | Required role not held |
| `NotACitizen` | KYC not approved |
| `ElectionNotFound` | Election doesn't exist |
| `ElectionNotActive` | Election period ended/not started |
| `ElectionAlreadyStarted` | Election already initiated |
| `ElectionAlreadyFinalized` | Election already finalized |
| `CandidacyPeriodExpired` | Registration period ended |
| `CampaignPeriodNotStarted` | Campaign hasn't begun |
| `VotingPeriodNotStarted` | Voting hasn't started |
| `VotingPeriodExpired` | Voting period ended |
| `AlreadyCandidate` | Already registered as candidate |
| `AlreadyVoted` | Already voted in this election |
| `InvalidDistrict` | Invalid electoral district |
| `InsufficientEndorsements` | Not enough endorsements |
| `DepositRequired` | Candidacy deposit not paid |
| `TooManyCandidates` | Too many candidates registered |
| `InvalidInitialCandidates` | Wrong number of runoff candidates |
| `NotAuthorizedToNominate` | Not authorized to nominate |
| `NotAuthorizedToApprove` | Not authorized to approve |
| `AppointmentProcessNotFound` | Process doesn't exist |
| `NominationNotFound` | Nomination not found |
| `AppointmentAlreadyProcessed` | Already processed |
| `RoleAlreadyFilled` | Position already occupied |
| `ProposalNotFound` | Proposal doesn't exist |
| `ProposalNotActive` | Proposal not active |
| `NotAuthorizedToPropose` | Not authorized to propose |
| `NotAuthorizedToVote` | Not authorized to vote |
| `ProposalAlreadyVoted` | Already voted on proposal |
| `QuorumNotMet` | Quorum not reached |
| `ProposalExecutionFailed` | Proposal execution failed |
| `ParliamentFull` | Parliament at capacity |
| `DiwanFull` | Diwan at capacity |
| `InvalidElectionType` | Invalid election type |
| `CalculationOverflow` | Arithmetic overflow |
| `RunoffElectionFailed` | Runoff initiation failed |

---

## Helper Functions

### Public Query Functions

#### ensure_serok
```rust
pub fn ensure_serok(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError>
```
Verifies that the caller is the current Serok.

---

#### ensure_parliament_member
```rust
pub fn ensure_parliament_member(origin: OriginFor<T>) -> Result<T::AccountId, DispatchError>
```
Verifies that the caller is a parliament member.

---

#### get_required_trust_score
```rust
pub fn get_required_trust_score(election_type: &ElectionType) -> u128
```
Returns minimum trust score for candidacy.

---

#### get_required_tiki
```rust
pub fn get_required_tiki(election_type: &ElectionType) -> Option<Tiki>
```
Returns required role for candidacy.

---

#### get_required_endorsements
```rust
pub fn get_required_endorsements(election_type: &ElectionType) -> u32
```
Returns number of required endorsements.

---

#### calculate_vote_weight
```rust
pub fn calculate_vote_weight(voter: &T::AccountId, election_type: &ElectionType) -> u32
```
Calculates vote weight based on trust score.

---

## Origin Types

### EnsureSerok
Custom origin type that ensures the caller is the current Serok (President).

```rust
use pallet_welati::EnsureSerok;

// Use in pallet config
type SerokOrigin = EnsureSerok<Runtime>;
```

---

### EnsureParlementer
Custom origin type that ensures the caller is a parliament member.

```rust
use pallet_welati::EnsureParlementer;

// Use in pallet config
type ParliamentOrigin = EnsureParlementer<Runtime>;
```

---

### EnsureDiwan
Custom origin type that ensures the caller is a Diwan member.

```rust
use pallet_welati::EnsureDiwan;

// Use in pallet config
type DiwanOrigin = EnsureDiwan<Runtime>;
```

---

## Usage Examples

### Running a Presidential Election

```rust
// 1. Initiate election
Welati::initiate_election(
    Origin::root(),
    ElectionType::Presidential,
    None,
    None
)?;

// 2. Candidates register (during candidacy period)
Welati::register_candidate(
    Origin::signed(alice.clone()),
    election_id,
    None,
    endorsers_for_alice
)?;

// 3. Citizens vote (during voting period)
Welati::cast_vote(
    Origin::signed(bob.clone()),
    election_id,
    vec![alice.clone()],
    None
)?;

// 4. Finalize election (after voting ends)
Welati::finalize_election(Origin::root(), election_id)?;
// If no candidate got >50%, runoff is automatically initiated
```

---

### Submitting and Voting on Proposals

```rust
// 1. Parliament member submits proposal
Welati::submit_proposal(
    Origin::signed(parlementer.clone()),
    b"Budget Amendment".to_vec().try_into().unwrap(),
    b"Increase education budget".to_vec().try_into().unwrap(),
    CollectiveDecisionType::ParliamentSimpleMajority,
    ProposalPriority::High,
    None
)?;

// 2. Parliament members vote
Welati::vote_on_proposal(
    Origin::signed(member1.clone()),
    proposal_id,
    VoteChoice::Aye,
    None
)?;

Welati::vote_on_proposal(
    Origin::signed(member2.clone()),
    proposal_id,
    VoteChoice::Nay,
    Some(b"Needs more discussion".to_vec().try_into().unwrap())
)?;
```

---

### Appointing Government Officials

```rust
// 1. Minister nominates official
Welati::nominate_official(
    Origin::signed(minister.clone()),
    nominee.clone(),
    OfficialRole::Xezinedar,
    b"Experienced in treasury management".to_vec().try_into().unwrap()
)?;

// 2. Serok approves appointment
Welati::approve_appointment(
    Origin::signed(serok.clone()),
    process_id
)?;
```

---

## Integration Notes

### Automatic Tiki Assignment

When elections are finalized, winners automatically receive appropriate Tiki roles:
- Presidential winner gets `Tiki::Serok`
- Parliament winners get `Tiki::Parlementer`
- Speaker winner gets `Tiki::SerokiMeclise`
- Diwan winners get `Tiki::EndameDiwane`

### Trust Score Integration

Trust scores affect:
- **Candidacy eligibility**: Minimum scores required
- **Vote weighting**: Higher trust = more influence (in non-citizen elections)
- **Proposal submission**: Trust threshold for certain proposal types

### KYC Requirements

All participation requires KYC approval:
- Candidate registration
- Voting
- Endorsing candidates
- Submitting proposals
- Voting on proposals
