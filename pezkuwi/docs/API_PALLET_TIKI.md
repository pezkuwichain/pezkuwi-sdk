# Tiki Pallet API Documentation

## Overview

The Tiki (Role) Pallet is a comprehensive role management system that uses non-transferable NFTs (soulbound tokens) to represent citizenship status and various roles within the PezkuwiChain ecosystem. Each role grants specific permissions, rights, and social standing.

### Key Features

- **Soulbound NFTs**: All role NFTs are non-transferable, ensuring roles remain tied to their holders
- **Citizenship System**: Automatic citizenship NFT minting upon KYC approval
- **Multiple Assignment Types**: Roles can be automatic, appointed, elected, or earned
- **Role-Based Permissions**: Different roles grant different levels of access and authority
- **Trust Score Integration**: Roles contribute to overall trust scores in the ecosystem

### Dependencies

This pallet integrates with:
- **pallet-nfts**: Underlying NFT infrastructure
- **pallet-identity-kyc**: KYC status verification and citizenship eligibility
- **pallet-trust**: Trust score verification for role eligibility (optional)

---

## Configuration (Config trait)

### Associated Types

| Type | Description |
|------|-------------|
| `RuntimeEvent` | The overarching event type from the runtime |
| `AdminOrigin` | Origin that can perform administrative operations (grant/revoke roles) |
| `WeightInfo` | Weight information for extrinsics |
| `TikiCollectionId` | The collection ID holding all Tiki (Role) NFTs |
| `MaxTikisPerUser` | Maximum number of roles a single user can hold (default: 20) |
| `Tiki` | The Tiki enum type representing all available roles |

### Runtime Configuration Example

```rust
impl pallet_tiki::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AdminOrigin = EnsureRoot<AccountId>;
    type WeightInfo = pallet_tiki::weights::SubstrateWeight<Runtime>;
    type TikiCollectionId = ConstU32<1>; // Tiki collection ID
    type MaxTikisPerUser = ConstU32<20>; // Max 20 roles per user
    type Tiki = pallet_tiki::Tiki;
}
```

---

## Storage Items

### CitizenNft

**Type:** `StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>`

**Description:** Maps each user's account to their citizenship NFT ID.

**Access:** Read via `citizen_nft(account)` getter

### UserTikis

**Type:** `StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Tiki, T::MaxTikisPerUser>, ValueQuery>`

**Description:** Stores the list of all roles (Tikis) held by each user.

**Access:** Read via `user_tikis(account)` getter

### TikiHolder

**Type:** `StorageMap<_, Blake2_128Concat, Tiki, T::AccountId, OptionQuery>`

**Description:** Reverse mapping showing which account holds a specific unique role (e.g., Serok, Xezinedar).

**Access:** Read via `tiki_holder(tiki)` getter

### NextItemId

**Type:** `StorageValue<_, u32, ValueQuery>`

**Description:** Counter for generating unique NFT item IDs.

**Access:** Read via `next_item_id()` getter

---

## Role Types (Tiki Enum)

The pallet defines 41 different roles organized by category:

### Governance Roles
- `Welati` - Citizen (basic role for all KYC-approved users)
- `Parlementer` - Parliament Member
- `SerokiMeclise` - Parliament Speaker
- `Serok` - President
- `SerokWeziran` - Prime Minister

### Ministerial Roles
- `Wezir` - Minister (generic)
- `WezireDarayiye` - Minister of Finance
- `WezireParez` - Minister of Defense
- `WezireDad` - Minister of Justice
- `WezireBelaw` - Minister of Education
- `WezireTend` - Minister of Health
- `WezireAva` - Minister of Water Resources
- `WezireCand` - Minister of Culture
- `EndameDiwane` - Diwan Council Member

### Judicial Roles
- `Dadger` - Judge
- `Dozger` - Prosecutor
- `Hiquqnas` - Lawyer
- `Noter` - Notary

### Administrative Roles
- `Xezinedar` - Treasurer
- `Qeydkar` - Registrar
- `Bacgir` - Tax Collector
- `Berdevk` - Spokesperson
- `Balyoz` - Ambassador
- `OperatoreTore` - Network Operator
- `PisporêEwlehiyaSîber` - Cybersecurity Expert
- `GerinendeyeCavkaniye` - Resource Manager
- `GerinendeyeDaneye` - Data Manager

### Educational/Cultural Roles
- `Mamoste` - Teacher
- `Perwerdekar` - Educator
- `Rewsenbir` - Intellectual
- `Mela` - Religious Scholar
- `Feqi` - Religious Leader
- `ParezvaneCandi` - Cultural Guardian

### Expert/Community Roles
- `Axa` - Elder/Expert
- `Pêseng` - Pioneer
- `Sêwirmend` - Counselor
- `Hekem` - Wise Person
- `SerokêKomele` - Community Leader
- `ModeratorêCivakê` - Community Moderator
- `RêveberêProjeyê` - Project Manager

### Economic Roles
- `Bazargan` - Merchant
- `Navbeynkar` - Mediator

### Quality Control
- `Mufetis` - Inspector
- `KaliteKontrolker` - Quality Controller

---

## Role Assignment Types

```rust
pub enum RoleAssignmentType {
    Automatic,  // System-assigned (e.g., Welati after KYC)
    Appointed,  // Admin-assigned (e.g., Ministers, Judges)
    Elected,    // Community-voted (e.g., Parliament, Serok)
    Earned,     // Achievement-based (e.g., Axa, Mamoste)
}
```

---

## Extrinsics (Callable Functions)

### 1. grant_tiki

**Description:** Grants a role to a user (Admin-only, for appointed roles).

**Signature:**
```rust
pub fn grant_tiki(
    origin: OriginFor<T>,
    dest: <T::Lookup as StaticLookup>::Source,
    tiki: Tiki,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `AdminOrigin` (typically root)
- `dest`: The account to receive the role
- `tiki`: The role to grant

**Requirements:**
- Caller must have `AdminOrigin` permission
- Role must be appointable (not automatic, elected, or earned)
- Target account must have citizenship NFT
- Target account must not already have the role
- For unique roles, no one else can hold it
- Target account must not exceed `MaxTikisPerUser`

**Events Emitted:**
- `TikiGranted { who: AccountId, tiki: Tiki }`

**Errors:**
- `InvalidRoleAssignmentMethod` - Role cannot be assigned via appointment
- `CitizenNftNotFound` - Target account is not a citizen
- `RoleAlreadyTaken` - Unique role already assigned to someone else
- `UserAlreadyHasRole` - User already has this role
- `ExceedsMaxRolesPerUser` - Would exceed maximum role limit

**Weight:** Defined by `WeightInfo::grant_tiki()`

**Example:**
```rust
// Grant Minister role to Alice
TikiPallet::grant_tiki(
    Origin::root(),
    MultiAddress::Id(alice.clone()),
    Tiki::Wezir
)?;
```

---

### 2. revoke_tiki

**Description:** Revokes a role from a user (Admin-only).

**Signature:**
```rust
pub fn revoke_tiki(
    origin: OriginFor<T>,
    target: <T::Lookup as StaticLookup>::Source,
    tiki: Tiki,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `AdminOrigin`
- `target`: The account to revoke the role from
- `tiki`: The role to revoke

**Requirements:**
- Caller must have `AdminOrigin` permission
- Target account must have the specified role
- Cannot revoke `Welati` (citizenship) role

**Events Emitted:**
- `TikiRevoked { who: AccountId, tiki: Tiki }`

**Errors:**
- `RoleNotAssigned` - User doesn't have this role or attempting to revoke Welati

**Weight:** Defined by `WeightInfo::revoke_tiki()`

**Example:**
```rust
// Revoke Minister role from Alice
TikiPallet::revoke_tiki(
    Origin::root(),
    MultiAddress::Id(alice.clone()),
    Tiki::Wezir
)?;
```

---

### 3. force_mint_citizen_nft

**Description:** Manually mints citizenship NFT for testing or emergency purposes (Admin-only).

**Signature:**
```rust
pub fn force_mint_citizen_nft(
    origin: OriginFor<T>,
    dest: <T::Lookup as StaticLookup>::Source,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `AdminOrigin`
- `dest`: The account to receive citizenship NFT

**Requirements:**
- Caller must have `AdminOrigin` permission
- Target account must not already have citizenship NFT

**Events Emitted:**
- `CitizenNftMinted { who: AccountId, nft_id: u32 }`
- `TikiGranted { who: AccountId, tiki: Tiki::Welati }`

**Errors:**
- `CitizenNftAlreadyExists` - Account already has citizenship NFT

**Weight:** Defined by `WeightInfo::grant_tiki()`

**Example:**
```rust
// Manually mint citizenship NFT for Bob
TikiPallet::force_mint_citizen_nft(
    Origin::root(),
    MultiAddress::Id(bob.clone())
)?;
```

---

### 4. grant_elected_role

**Description:** Grants a role through the election system (called by pallet-welati).

**Signature:**
```rust
pub fn grant_elected_role(
    origin: OriginFor<T>,
    dest: <T::Lookup as StaticLookup>::Source,
    tiki: Tiki,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `AdminOrigin` (typically called by governance pallet)
- `dest`: The account to receive the role
- `tiki`: The role to grant

**Requirements:**
- Role must be electable (Parlementer, SerokiMeclise, Serok)
- Target account must have citizenship NFT

**Events Emitted:**
- `TikiGranted { who: AccountId, tiki: Tiki }`

**Errors:**
- `InvalidRoleAssignmentMethod` - Role cannot be granted via election
- `CitizenNftNotFound` - Target account is not a citizen
- `RoleAlreadyTaken` - Unique role already assigned
- `UserAlreadyHasRole` - User already has this role

**Weight:** Defined by `WeightInfo::grant_tiki()`

---

### 5. grant_earned_role

**Description:** Grants a role through exam/achievement system.

**Signature:**
```rust
pub fn grant_earned_role(
    origin: OriginFor<T>,
    dest: <T::Lookup as StaticLookup>::Source,
    tiki: Tiki,
) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `AdminOrigin` (typically called by exam/achievement pallet)
- `dest`: The account to receive the role
- `tiki`: The role to grant

**Requirements:**
- Role must be earnable (Axa, Mamoste, Rewsenbir, SerokêKomele, ModeratorêCivakê)
- Target account must have citizenship NFT

**Events Emitted:**
- `TikiGranted { who: AccountId, tiki: Tiki }`

**Errors:**
- `InvalidRoleAssignmentMethod` - Role cannot be earned
- Other errors same as `grant_tiki`

**Weight:** Defined by `WeightInfo::grant_tiki()`

---

### 6. apply_for_citizenship

**Description:** Users can apply for citizenship after completing KYC.

**Signature:**
```rust
pub fn apply_for_citizenship(origin: OriginFor<T>) -> DispatchResult
```

**Parameters:**
- `origin`: Signed by the user applying for citizenship

**Requirements:**
- Caller's KYC status must be `Approved`
- Caller must not already have citizenship NFT

**Events Emitted:**
- `CitizenNftMinted { who: AccountId, nft_id: u32 }`
- `TikiGranted { who: AccountId, tiki: Tiki::Welati }`

**Errors:**
- `KycNotCompleted` - KYC not approved
- `CitizenNftAlreadyExists` - Already a citizen

**Weight:** Defined by `WeightInfo::grant_tiki()`

**Example:**
```rust
// Alice applies for citizenship after KYC approval
TikiPallet::apply_for_citizenship(Origin::signed(alice.clone()))?;
```

---

### 7. check_transfer_permission

**Description:** Internal check to block transfers of Tiki NFTs.

**Signature:**
```rust
pub fn check_transfer_permission(
    _origin: OriginFor<T>,
    collection_id: T::CollectionId,
    item_id: u32,
    from: T::AccountId,
    to: T::AccountId,
) -> DispatchResult
```

**Parameters:**
- `collection_id`: NFT collection ID
- `item_id`: NFT item ID
- `from`: Source account
- `to`: Destination account

**Behavior:** Blocks all transfers of Tiki NFTs (soulbound).

**Events Emitted:**
- `TransferBlocked { collection_id, item_id, from, to }`

**Errors:**
- Returns `DispatchError::Other("Citizen NFTs are non-transferable")` for Tiki NFTs

**Weight:** Defined by `WeightInfo::grant_tiki()`

---

## Events

### CitizenNftMinted
```rust
CitizenNftMinted { who: T::AccountId, nft_id: u32 }
```
Emitted when a new citizenship NFT is minted.

### TikiGranted
```rust
TikiGranted { who: T::AccountId, tiki: Tiki }
```
Emitted when a role is granted to a user.

### TikiRevoked
```rust
TikiRevoked { who: T::AccountId, tiki: Tiki }
```
Emitted when a role is revoked from a user.

### TransferBlocked
```rust
TransferBlocked {
    collection_id: T::CollectionId,
    item_id: u32,
    from: T::AccountId,
    to: T::AccountId
}
```
Emitted when an attempt to transfer a Tiki NFT is blocked.

---

## Errors

| Error | Description |
|-------|-------------|
| `RoleAlreadyTaken` | Unique role is already assigned to another user |
| `NotTheHolder` | Specified person is not the holder of this role |
| `RoleNotAssigned` | Role is not assigned to the user |
| `ExceedsMaxRolesPerUser` | User has reached maximum role count |
| `KycNotCompleted` | KYC approval required |
| `CitizenNftAlreadyExists` | User already has citizenship NFT |
| `CitizenNftNotFound` | User doesn't have citizenship NFT |
| `UserAlreadyHasRole` | User already has this role |
| `InsufficientTrustScore` | User's trust score is too low |
| `InvalidRoleAssignmentMethod` | Role cannot be assigned via this method |

---

## Role Assignment Types by Role

### Automatic Roles
- `Welati` - Automatically granted after KYC approval

### Elected Roles
- `Parlementer` - Parliament Member
- `SerokiMeclise` - Parliament Speaker
- `Serok` - President

### Earned Roles
- `Axa` - Elder/Expert
- `Mamoste` - Teacher
- `Rewsenbir` - Intellectual
- `SerokêKomele` - Community Leader
- `ModeratorêCivakê` - Community Moderator

### Appointed Roles (All Others)
All other roles not listed above are appointed by administrators.

---

## Unique Roles

Some roles can only be held by one person at a time:
- `Serok` - President
- `SerokiMeclise` - Parliament Speaker
- `Xezinedar` - Treasurer
- `Balyoz` - Ambassador

---

## Role Trust Score Bonuses

Each role contributes to the holder's trust score:

| Role | Score Bonus |
|------|-------------|
| Axa | 250 |
| RêveberêProjeyê | 250 |
| Serok | 200 |
| ModeratorêCivakê | 200 |
| EndameDiwane | 175 |
| Dadger, SerokiMeclise | 150 |
| SerokWeziran | 125 |
| Dozger | 120 |
| Parlementer, Wezir, Ministers, Xezinedar, PisporêEwlehiyaSîber | 100 |
| Mufetis | 90 |
| Balyoz | 80 |
| Hiquqnas | 75 |
| Berdevk, Mamoste | 70 |
| Bazargan, OperatorêTorê | 60 |
| Mela, Feqi, Noter, Bacgir | 50 |
| Perwerdekar, Rewsenbir, GerinendeyeCavkaniye, GerinendeyeDaneye | 40 |
| Navbeynkar, KaliteKontrolker, Hekem | 30 |
| Qeydkar, ParêzvaneÇandî | 25 |
| Sêwirmend | 20 |
| Welati | 10 |
| Others | 5 |

---

## Public Helper Functions

### has_tiki
```rust
pub fn has_tiki(who: &T::AccountId, tiki: &Tiki) -> bool
```
Checks if a user has a specific role.

### is_citizen
```rust
pub fn is_citizen(who: &T::AccountId) -> bool
```
Checks if a user has citizenship (has CitizenNft).

### get_tiki_score
```rust
pub fn get_tiki_score(who: &T::AccountId) -> u32
```
Calculates total trust score bonus from all user's roles.

### is_unique_role
```rust
pub fn is_unique_role(tiki: &Tiki) -> bool
```
Returns whether a role can only be held by one person.

### get_role_assignment_type
```rust
pub fn get_role_assignment_type(tiki: &Tiki) -> RoleAssignmentType
```
Returns the assignment type for a specific role.

---

## Hooks

### on_initialize
```rust
fn on_initialize(_block_number: BlockNumberFor<T>) -> Weight
```
Automatically checks for newly KYC-approved users and mints citizenship NFTs.

**Behavior:** Iterates through KYC-approved accounts and mints citizenship NFTs for those who don't have one yet.

---

## Integration Traits

### TikiScoreProvider
```rust
pub trait TikiScoreProvider<AccountId> {
    fn get_tiki_score(who: &AccountId) -> u32;
}
```
Allows other pallets to query role-based trust scores.

### TikiProvider
```rust
pub trait TikiProvider<AccountId> {
    fn has_tiki(who: &AccountId, tiki: &Tiki) -> bool;
    fn get_user_tikis(who: &AccountId) -> Vec<Tiki>;
    fn is_citizen(who: &AccountId) -> bool;
}
```
Allows other pallets to query role ownership and citizenship status.

### CitizenNftProvider
```rust
impl CitizenNftProvider<T::AccountId> for Pallet<T> {
    fn mint_citizen_nft(who: &T::AccountId) -> DispatchResult;
}
```
Allows pallet-identity-kyc to trigger citizenship NFT minting.

---

## Usage Examples

### Check if user has a role
```rust
let is_judge = pallet_tiki::Pallet::<Runtime>::has_tiki(&alice, &Tiki::Dadger);
```

### Get all roles for a user
```rust
let roles = pallet_tiki::Pallet::<Runtime>::user_tikis(&alice);
for role in roles {
    println!("Role: {:?}", role);
}
```

### Calculate trust score from roles
```rust
let tiki_score = pallet_tiki::Pallet::<Runtime>::get_tiki_score(&alice);
```

### Grant a role via governance
```rust
// From another pallet or extrinsic
pallet_tiki::Pallet::<Runtime>::grant_tiki(
    RuntimeOrigin::root(),
    MultiAddress::Id(bob),
    Tiki::Wezir
)?;
```
