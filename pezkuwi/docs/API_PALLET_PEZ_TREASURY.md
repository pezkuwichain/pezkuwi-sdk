# PEZ Treasury Pallet API Documentation

## Overview

The PEZ Treasury Pallet manages the complete lifecycle of PEZ token distribution including genesis allocation, automated halving mechanics, and scheduled monthly releases. It implements a deflationary tokenomics model similar to Bitcoin's halving mechanism but with monthly distribution periods.

### Key Features

- **One-Time Genesis Distribution**: Initial allocation to treasury, presale, and founder accounts
- **Automated Halving**: Monthly release amount halves every 48 months (4 years)
- **Multi-Pot System**: Separate accounts for treasury reserves, incentive rewards, and government operations
- **Scheduled Releases**: Block-based deterministic monthly fund distribution
- **Deflationary Model**: Decreasing supply over time through halving
- **Security**: One-time genesis protection prevents duplicate minting

### Token Economics

| Parameter | Value | Percentage |
|-----------|-------|------------|
| **Total Supply** | 5,000,000,000 PEZ | 100% |
| **Treasury Allocation** | 4,812,500,000 PEZ | 96.25% |
| **Presale Allocation** | 93,750,000 PEZ | 1.875% |
| **Founder Allocation** | 93,750,000 PEZ | 1.875% |

### Halving Schedule

- **Period Duration**: 48 months (4 years = 20,736,000 blocks)
- **Blocks per Month**: 432,000 blocks (~30 days at 10 blocks/minute)
- **Distribution Split**: 75% to Incentive Pot, 25% to Government Pot
- **Initial Monthly Release**: ~50,130,208.333 PEZ (first period)
- **Halving Effect**: Monthly amount halves each period (50M -> 25M -> 12.5M -> ...)

### Dependencies

This pallet requires:
- **pallet-assets**: For fungible token operations (PEZ is implemented as an asset)

---

## Configuration (Config trait)

### Associated Types

| Type | Description |
|------|-------------|
| `RuntimeEvent` | The overarching event type |
| `Assets` | Asset pallet implementing fungible token operations |
| `WeightInfo` | Weight information for extrinsics |

### Runtime Constants

| Constant | Description |
|----------|-------------|
| `PezAssetId` | Asset ID for PEZ token (e.g., 1) |
| `TreasuryPalletId` | Pallet ID for main treasury account |
| `IncentivePotId` | Pallet ID for incentive rewards account |
| `GovernmentPotId` | Pallet ID for government operations account |
| `PresaleAccount` | Account receiving presale allocation |
| `FounderAccount` | Account receiving founder allocation |
| `ForceOrigin` | Privileged origin (typically root or governance) |

### Runtime Configuration Example

```rust
parameter_types! {
    pub const PezAssetId: u32 = 1;
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const IncentivePotId: PalletId = PalletId(*b"py/incen");
    pub const GovernmentPotId: PalletId = PalletId(*b"py/govmt");
    pub PresaleAccount: AccountId = hex!["..."].into();
    pub FounderAccount: AccountId = hex!["..."].into();
}

impl pallet_pez_treasury::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Assets = Assets;
    type WeightInfo = pallet_pez_treasury::weights::SubstrateWeight<Runtime>;
    type PezAssetId = PezAssetId;
    type TreasuryPalletId = TreasuryPalletId;
    type IncentivePotId = IncentivePotId;
    type GovernmentPotId = GovernmentPotId;
    type PresaleAccount = PresaleAccount;
    type FounderAccount = FounderAccount;
    type ForceOrigin = EnsureRoot<AccountId>;
}
```

---

## Storage Items

### HalvingInfo

**Type:** `StorageValue<_, HalvingData<T>, ValueQuery>`

**Description:** Stores current halving period information and monthly release parameters.

**Structure:**
```rust
pub struct HalvingData<T: Config> {
    pub current_period: u32,              // Current halving period (0, 1, 2, ...)
    pub period_start_block: BlockNumberFor<T>,  // Block when period started
    pub monthly_amount: BalanceOf<T>,     // Current monthly release amount
    pub total_released: BalanceOf<T>,     // Total released so far
}
```

**Access:** Read via `halving_info()` getter

**Example:**
```rust
let info = PezTreasury::halving_info();
println!("Period: {}, Monthly: {}", info.current_period, info.monthly_amount);
```

---

### MonthlyReleases

**Type:** `StorageMap<_, Blake2_128Concat, u32, MonthlyRelease<T>, OptionQuery>`

**Description:** Historical record of all monthly fund releases indexed by month number.

**Structure:**
```rust
pub struct MonthlyRelease<T: Config> {
    pub month_index: u32,                 // Month number (0, 1, 2, ...)
    pub release_block: BlockNumberFor<T>, // Block when released
    pub amount_released: BalanceOf<T>,    // Total amount released
    pub incentive_amount: BalanceOf<T>,   // Amount to incentive pot (75%)
    pub government_amount: BalanceOf<T>,  // Amount to government pot (25%)
}
```

**Access:** Read via `monthly_releases(month_index)` getter

**Example:**
```rust
if let Some(release) = PezTreasury::monthly_releases(0) {
    println!("Month 0 released {} PEZ", release.amount_released);
}
```

---

### NextReleaseMonth

**Type:** `StorageValue<_, u32, ValueQuery>`

**Description:** Counter tracking the next month index to be released.

**Access:** Read via `next_release_month()` getter

**Example:**
```rust
let next_month = PezTreasury::next_release_month();
// Next scheduled release is for month `next_month`
```

---

### TreasuryStartBlock

**Type:** `StorageValue<_, BlockNumberFor<T>, OptionQuery>`

**Description:** Block number when treasury was initialized. Used to calculate elapsed months.

**Access:** Read via `treasury_start_block()` getter

**Example:**
```rust
if let Some(start) = PezTreasury::treasury_start_block() {
    let current = System::block_number();
    let blocks_elapsed = current - start;
}
```

---

### GenesisDistributionDone

**Type:** `StorageValue<_, bool, ValueQuery>`

**Description:** Security flag preventing duplicate genesis distribution.

**Access:** Read via `genesis_distribution_done()` getter

**Example:**
```rust
let already_done = PezTreasury::genesis_distribution_done();
if !already_done {
    // Genesis can still be executed
}
```

---

## Constants

### Token Supply Constants

```rust
pub const TOTAL_SUPPLY: u128 = 5_000_000_000 * 1_000_000_000_000;      // 5 billion PEZ
pub const TREASURY_ALLOCATION: u128 = 4_812_500_000 * 1_000_000_000_000;  // 96.25%
pub const PRESALE_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000;      // 1.875%
pub const FOUNDER_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000;      // 1.875%
```

### Halving Constants

```rust
pub const HALVING_PERIOD_MONTHS: u32 = 48;          // 4 years
pub const BLOCKS_PER_MONTH: u32 = 432_000;          // ~30 days
pub const HALVING_PERIOD_BLOCKS: u32 = 20_736_000;  // 48 * 432,000
```

---

## Extrinsics (Callable Functions)

### 1. force_genesis_distribution

**Description:** Performs one-time initial token distribution to treasury, presale, and founder accounts (Privileged).

**Signature:**
```rust
pub fn force_genesis_distribution(origin: OriginFor<T>) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `ForceOrigin` (typically root or governance)

**Requirements:**
- Must be called with privileged origin
- Can only be executed once (protected by `GenesisDistributionDone` flag)

**Behavior:**
1. Verifies genesis hasn't already been executed
2. Mints tokens to three destinations:
   - **Treasury Account**: 4,812,500,000 PEZ (96.25%)
   - **Presale Account**: 93,750,000 PEZ (1.875%)
   - **Founder Account**: 93,750,000 PEZ (1.875%)
3. Sets `GenesisDistributionDone` flag to prevent re-execution

**Events Emitted:**
- `GenesisDistributionCompleted { treasury_amount, presale_amount, founder_amount }`

**Errors:**
- `GenesisDistributionAlreadyDone` - Genesis already executed
- `InsufficientTreasuryBalance` - Conversion error (should not occur)

**Weight:** `WeightInfo::force_genesis_distribution()`

**Example:**
```rust
// Execute genesis distribution (only works once)
PezTreasury::force_genesis_distribution(Origin::root())?;
```

**Security Note:** This is a critical one-time operation. The storage flag ensures it cannot be called twice, preventing inflation beyond the total supply.

---

### 2. initialize_treasury

**Description:** Initializes the halving mechanism and prepares for monthly releases (Privileged).

**Signature:**
```rust
pub fn initialize_treasury(origin: OriginFor<T>) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `ForceOrigin`

**Requirements:**
- Must be called with privileged origin
- Can only be called once (treasury must not already be initialized)

**Behavior:**
1. Records current block as `TreasuryStartBlock`
2. Calculates initial monthly release amount:
   - First period total: 50% of treasury (2,406,250,000 PEZ)
   - Monthly amount: First period total / 48 months = ~50,130,208.333 PEZ
3. Initializes `HalvingData` with period 0
4. Sets `NextReleaseMonth` to 0

**Events Emitted:**
- `TreasuryInitialized { start_block, initial_monthly_amount }`

**Errors:**
- `TreasuryAlreadyInitialized` - Already initialized
- `InvalidHalvingPeriod` - Calculation error

**Weight:** `WeightInfo::initialize_treasury()`

**Example:**
```rust
// Initialize treasury (after genesis distribution)
PezTreasury::initialize_treasury(Origin::root())?;
```

**Calculation Details:**
```
Period 0: 2,406,250,000 PEZ / 48 months = 50,130,208.333 PEZ/month
Period 1: 1,203,125,000 PEZ / 48 months = 25,065,104.167 PEZ/month
Period 2: 601,562,500 PEZ / 48 months = 12,532,552.083 PEZ/month
...and so on
```

---

### 3. release_monthly_funds

**Description:** Releases scheduled monthly funds to incentive and government pots (Privileged).

**Signature:**
```rust
pub fn release_monthly_funds(origin: OriginFor<T>) -> DispatchResult
```

**Parameters:**
- `origin`: Must be authorized via `ForceOrigin`

**Requirements:**
- Treasury must be initialized
- Sufficient blocks must have passed (at least 1 month since last release)
- Monthly release for current month must not already be done

**Behavior:**
1. Calculates months elapsed since treasury start
2. Checks if enough time has passed for next release
3. If new halving period started (48 months passed):
   - Increments period number
   - Halves monthly amount
   - Updates period start block
   - Emits `NewHalvingPeriod` event
4. Calculates distribution:
   - **Incentive Pot**: 75% of monthly amount
   - **Government Pot**: 25% of monthly amount
5. Transfers from treasury to respective pots
6. Records release in `MonthlyReleases` storage
7. Increments `NextReleaseMonth`

**Events Emitted:**
- `MonthlyFundsReleased { month_index, total_amount, incentive_amount, government_amount }`
- `NewHalvingPeriod { period, new_monthly_amount }` (if period changed)

**Errors:**
- `TreasuryNotInitialized` - Treasury not initialized
- `MonthlyReleaseAlreadyDone` - Already released for this month
- `ReleaseTooEarly` - Not enough blocks passed
- `InsufficientTreasuryBalance` - Treasury doesn't have enough funds

**Weight:** `WeightInfo::release_monthly_funds()`

**Example:**
```rust
// Release monthly funds (after 1 month has passed)
PezTreasury::release_monthly_funds(Origin::root())?;
```

**Timing Logic:**
```rust
// Month 0 can be released after block: start_block + 432,000 blocks
// Month 1 can be released after block: start_block + 864,000 blocks
// Month N can be released after block: start_block + (N+1) * 432,000 blocks
```

**Distribution Split:**
```
Monthly Amount: 50,130,208.333 PEZ (period 0)
├─ Incentive Pot: 37,597,656.25 PEZ (75%)
└─ Government Pot: 12,532,552.083 PEZ (25%)
```

---

## Events

### GenesisDistributionCompleted
```rust
GenesisDistributionCompleted {
    treasury_amount: BalanceOf<T>,
    presale_amount: BalanceOf<T>,
    founder_amount: BalanceOf<T>,
}
```
Emitted when initial token distribution is completed.

**Fields:**
- `treasury_amount`: Amount minted to treasury (4,812,500,000 PEZ)
- `presale_amount`: Amount minted to presale account (93,750,000 PEZ)
- `founder_amount`: Amount minted to founder account (93,750,000 PEZ)

---

### TreasuryInitialized
```rust
TreasuryInitialized {
    start_block: BlockNumberFor<T>,
    initial_monthly_amount: BalanceOf<T>,
}
```
Emitted when treasury halving system is initialized.

**Fields:**
- `start_block`: Block number when initialized
- `initial_monthly_amount`: First period monthly release amount

---

### MonthlyFundsReleased
```rust
MonthlyFundsReleased {
    month_index: u32,
    total_amount: BalanceOf<T>,
    incentive_amount: BalanceOf<T>,
    government_amount: BalanceOf<T>,
}
```
Emitted when monthly funds are distributed.

**Fields:**
- `month_index`: Sequential month number (0, 1, 2, ...)
- `total_amount`: Total released this month
- `incentive_amount`: Amount sent to incentive pot (75%)
- `government_amount`: Amount sent to government pot (25%)

---

### NewHalvingPeriod
```rust
NewHalvingPeriod {
    period: u32,
    new_monthly_amount: BalanceOf<T>,
}
```
Emitted when a new halving period begins (every 48 months).

**Fields:**
- `period`: New period number (1, 2, 3, ...)
- `new_monthly_amount`: New halved monthly amount

---

## Errors

| Error | Description |
|-------|-------------|
| `TreasuryAlreadyInitialized` | Treasury has already been initialized |
| `TreasuryNotInitialized` | Treasury must be initialized before releasing funds |
| `MonthlyReleaseAlreadyDone` | Funds for this month have already been released |
| `InsufficientTreasuryBalance` | Treasury doesn't have enough balance for transfer |
| `InvalidHalvingPeriod` | Error calculating halving period parameters |
| `ReleaseTooEarly` | Not enough time has passed for next release |
| `GenesisDistributionAlreadyDone` | Genesis distribution already executed |

---

## Public Helper Functions

### treasury_account_id
```rust
pub fn treasury_account_id() -> T::AccountId
```
Returns the treasury account ID derived from `TreasuryPalletId`.

**Example:**
```rust
let treasury = PezTreasury::treasury_account_id();
let balance = Assets::balance(PEZ_ASSET_ID, &treasury);
```

---

### incentive_pot_account_id
```rust
pub fn incentive_pot_account_id() -> T::AccountId
```
Returns the incentive pot account ID derived from `IncentivePotId`.

**Example:**
```rust
let incentive_pot = PezTreasury::incentive_pot_account_id();
```

---

### government_pot_account_id
```rust
pub fn government_pot_account_id() -> T::AccountId
```
Returns the government pot account ID derived from `GovernmentPotId`.

**Example:**
```rust
let gov_pot = PezTreasury::government_pot_account_id();
```

---

### get_current_halving_info
```rust
pub fn get_current_halving_info() -> HalvingData<T>
```
Returns current halving period information.

**Example:**
```rust
let info = PezTreasury::get_current_halving_info();
println!("Period: {}", info.current_period);
println!("Monthly: {}", info.monthly_amount);
println!("Released: {}", info.total_released);
```

---

### get_incentive_pot_balance
```rust
pub fn get_incentive_pot_balance() -> BalanceOf<T>
```
Returns current balance of the incentive pot.

**Example:**
```rust
let balance = PezTreasury::get_incentive_pot_balance();
```

---

### get_government_pot_balance
```rust
pub fn get_government_pot_balance() -> BalanceOf<T>
```
Returns current balance of the government pot.

**Example:**
```rust
let balance = PezTreasury::get_government_pot_balance();
```

---

## Genesis Configuration

The pallet supports genesis configuration for automatic initialization:

```rust
#[pallet::genesis_config]
pub struct GenesisConfig<T: Config> {
    pub initialize_treasury: bool,
}

#[pallet::genesis_build]
impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
    fn build(&self) {
        if self.initialize_treasury {
            let _ = Pallet::<T>::do_initialize_treasury();
        }
    }
}
```

**Usage in chain spec:**
```rust
pez_treasury: PezTreasuryConfig {
    initialize_treasury: true,
},
```

---

## Usage Examples

### Complete Setup Flow

```rust
// 1. Execute genesis distribution (once)
PezTreasury::force_genesis_distribution(Origin::root())?;
// Result:
// - Treasury: 4,812,500,000 PEZ
// - Presale: 93,750,000 PEZ
// - Founder: 93,750,000 PEZ

// 2. Initialize treasury system
PezTreasury::initialize_treasury(Origin::root())?;
// Result: Halving system activated, month counter starts

// 3. Wait for 432,000 blocks (~30 days)
// ...

// 4. Release month 0 funds
PezTreasury::release_monthly_funds(Origin::root())?;
// Result:
// - Incentive Pot: +37,597,656.25 PEZ
// - Government Pot: +12,532,552.083 PEZ

// 5. Continue monthly releases
// Each call releases the next month's allocation
```

---

### Querying Treasury State

```rust
// Check if genesis is done
let genesis_done = PezTreasury::genesis_distribution_done();

// Get current halving info
let halving_info = PezTreasury::get_current_halving_info();
println!("Current Period: {}", halving_info.current_period);
println!("Monthly Amount: {}", halving_info.monthly_amount);
println!("Total Released: {}", halving_info.total_released);

// Check pot balances
let incentive_balance = PezTreasury::get_incentive_pot_balance();
let gov_balance = PezTreasury::get_government_pot_balance();

// Check if next release is ready
let next_month = PezTreasury::next_release_month();
let start_block = PezTreasury::treasury_start_block().unwrap();
let current_block = System::block_number();
let blocks_passed = current_block - start_block;
let months_passed = blocks_passed / 432_000;

if months_passed > next_month {
    // Can release next month
    PezTreasury::release_monthly_funds(Origin::root())?;
}
```

---

### Monthly Release History

```rust
// Query specific month's release
if let Some(release) = PezTreasury::monthly_releases(5) {
    println!("Month 5:");
    println!("  Block: {}", release.release_block);
    println!("  Total: {}", release.amount_released);
    println!("  Incentive: {}", release.incentive_amount);
    println!("  Government: {}", release.government_amount);
}

// Iterate through all releases
let next_month = PezTreasury::next_release_month();
for month in 0..next_month {
    if let Some(release) = PezTreasury::monthly_releases(month) {
        println!("Month {}: {} PEZ released", month, release.amount_released);
    }
}
```

---

## Halving Schedule Reference

| Period | Months | Monthly Amount (PEZ) | Period Total (PEZ) | Start Block |
|--------|--------|----------------------|--------------------|-------------|
| 0 | 0-47 | 50,130,208.333 | 2,406,250,000 | 0 |
| 1 | 48-95 | 25,065,104.167 | 1,203,125,000 | 20,736,000 |
| 2 | 96-143 | 12,532,552.083 | 601,562,500 | 41,472,000 |
| 3 | 144-191 | 6,266,276.042 | 300,781,250 | 62,208,000 |
| 4 | 192-239 | 3,133,138.021 | 150,390,625 | 82,944,000 |
| 5+ | ... | Continues halving | ... | ... |

---

## Security Considerations

### One-Time Genesis Protection
- `GenesisDistributionDone` flag prevents duplicate minting
- Total supply is strictly enforced at 5 billion PEZ
- No inflation beyond initial distribution

### Privileged Operations
- All extrinsics require `ForceOrigin` (typically root)
- Prevents unauthorized fund releases
- Ensures controlled distribution schedule

### Deterministic Scheduling
- Block-based timing eliminates timing manipulation
- Predictable release schedule based on block numbers
- Automatic halving without manual intervention

### Transfer Safety
- Uses `Preservation::Preserve` to prevent account removal
- Ensures pallet accounts always exist
- Treasury balance validated before transfers

---

## Integration Notes

### Other Pallets Can Query Balances

```rust
// Check available incentive funds
let available = PezTreasury::get_incentive_pot_balance();

// Access pot accounts directly
let incentive_pot = PezTreasury::incentive_pot_account_id();
Assets::transfer(origin, PEZ_ASSET_ID, recipient, amount)?;
```

### Automated Release Scheduling

Consider implementing an offchain worker or scheduler to automatically call `release_monthly_funds`:

```rust
// In runtime hooks
fn on_initialize(block: BlockNumber) -> Weight {
    if should_release_funds(block) {
        let _ = PezTreasury::release_monthly_funds(Origin::root());
    }
    Weight::zero()
}
```

### Governance Integration

The treasury can be controlled by governance:

```rust
impl pallet_pez_treasury::Config for Runtime {
    // ...
    type ForceOrigin = EnsureSerok<Runtime>; // President controls releases
    // or
    type ForceOrigin = EnsureParlementer<Runtime>; // Parliament controls releases
}
```

---

## Mathematical Formulas

### Monthly Amount Calculation
```
monthly_amount = (treasury_allocation / 2^period) / 48
```

### Release Schedule
```
month_N_release_block = start_block + (N + 1) * 432_000
```

### Period Transition
```
new_period = floor((current_block - start_block) / 20_736_000)
```

### Distribution Split
```
incentive_amount = monthly_amount * 0.75
government_amount = monthly_amount * 0.25
```

---

## Testing Utilities

For testing, you can advance blocks to trigger releases:

```rust
#[test]
fn test_monthly_release() {
    new_test_ext().execute_with(|| {
        // Setup
        assert_ok!(PezTreasury::force_genesis_distribution(Origin::root()));
        assert_ok!(PezTreasury::initialize_treasury(Origin::root()));

        // Advance 1 month
        run_to_block(432_000);

        // Should be able to release
        assert_ok!(PezTreasury::release_monthly_funds(Origin::root()));

        // Check balances
        let incentive = PezTreasury::get_incentive_pot_balance();
        assert!(incentive > 0);
    });
}
```
