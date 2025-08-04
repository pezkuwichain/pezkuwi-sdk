use frame_support::{assert_ok, traits::{Currency, Hooks}};
use crate as pallet_pez_rewards;
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU16, ConstU64, ConstU128, ConstU32},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};
use frame_system::{self as system, EnsureSigned};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Nfts: pallet_nfts,
		PezRewards: pallet_pez_rewards,
		Trust: pallet_trust,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type DoneSlashHandler = ();
}

// Add NFT pallet configuration
parameter_types! {
	pub const CollectionDeposit: u128 = 100;
	pub const ItemDeposit: u128 = 1;
	pub const StringLimit: u32 = 256;
	pub const MetadataDepositBase: u128 = 10;
	pub const AttributeDepositBase: u128 = 10;
	pub const DepositPerByte: u128 = 1;
	pub const MaxTips: u32 = 10;
	pub const MaxDeadlineDuration: u64 = 12345;
	pub const MaxAttributesPerCall: u32 = 10;
}

impl pallet_nfts::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<EnsureSigned<u64>>;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type Locker = ();
	type CollectionDeposit = CollectionDeposit;
	type ItemDeposit = ItemDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type AttributeDepositBase = AttributeDepositBase;
	type DepositPerByte = DepositPerByte;
	type StringLimit = StringLimit;
	type KeyLimit = ConstU32<32>;
	type ValueLimit = ConstU32<256>;
	type ApprovalsLimit = ConstU32<20>;
	type ItemAttributesApprovalsLimit = ConstU32<20>;
	type MaxTips = MaxTips;
	type MaxDeadlineDuration = MaxDeadlineDuration;
	type MaxAttributesPerCall = MaxAttributesPerCall;
	type Features = ();
	type OffchainSignature = sp_runtime::testing::TestSignature;
	type OffchainPublic = sp_runtime::testing::UintAuthorityId;
	type BlockNumberProvider = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const TrustScoreMultiplierBase: u128 = 1000;
	pub const TrustUpdateInterval: u64 = 100;
}

// Mock Trust Score Provider
pub struct MockTrustScoreProvider;
impl pallet_trust::TrustScoreProvider<u64> for MockTrustScoreProvider {
	fn trust_score_of(who: &u64) -> u128 {
		// Simple mock: account 1 = 100 points, account 2 = 50 points, etc.
		match who {
			1 => 100,
			2 => 50,
			3 => 75,
			_ => 0,
		}
	}
}

impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Score = u128;
	type ScoreMultiplierBase = TrustScoreMultiplierBase;
	type UpdateInterval = TrustUpdateInterval;
	type StakingScoreSource = MockStakingScoreProvider;
	type ReferralScoreSource = MockReferralScoreProvider;
	type PerwerdeScoreSource = MockPerwerdeScoreProvider;
	type TikiScoreSource = MockTikiScoreProvider;
	type CitizenshipSource = MockCitizenshipProvider;
}

// Mock providers for Trust pallet dependencies
pub struct MockStakingScoreProvider;
impl pallet_trust::StakingScoreProvider<u64, u64> for MockStakingScoreProvider {
	fn get_staking_score(_who: &u64) -> (u32, u64) {
		(50, 0)
	}
}

pub struct MockReferralScoreProvider;
impl pallet_trust::ReferralScoreProvider<u64> for MockReferralScoreProvider {
	fn get_referral_score(_who: &u64) -> u32 {
		25
	}
}

pub struct MockPerwerdeScoreProvider;
impl pallet_trust::PerwerdeScoreProvider<u64> for MockPerwerdeScoreProvider {
	fn get_perwerde_score(_who: &u64) -> u32 {
		15
	}
}

pub struct MockTikiScoreProvider;
impl pallet_trust::TikiScoreProvider<u64> for MockTikiScoreProvider {
	fn get_tiki_score(_who: &u64) -> u32 {
		10
	}
}

pub struct MockCitizenshipProvider;
impl pallet_trust::CitizenshipStatusProvider<u64> for MockCitizenshipProvider {
	fn is_citizen(_who: &u64) -> bool {
		true
	}
}

parameter_types! {
	pub const IncentivePotId: PalletId = PalletId(*b"py/incnt");
	pub const ClawbackRecipient: u64 = 999; // Qazi Muhammed test account
}

impl pallet_pez_rewards::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = crate::weights::WeightInfo<Test>;
	type TrustScoreSource = MockTrustScoreProvider;
	type IncentivePotId = IncentivePotId;
	type ClawbackRecipient = ClawbackRecipient;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type CollectionId = u32;
	type ItemId = u32;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into();
	
	ext.execute_with(|| {
		// Initialize rewards system
		assert_ok!(PezRewards::initialize_rewards_system(RuntimeOrigin::root()));
		
		// Setup incentive pot with some balance for testing
		let incentive_pot = PezRewards::incentive_pot_account_id();
		<Balances as Currency<u64>>::deposit_creating(&incentive_pot, 100_000_000_000); // Daha fazla bakiye
		
		// Give some users balances for testing
		<Balances as Currency<u64>>::deposit_creating(&1, 1_000_000);
		<Balances as Currency<u64>>::deposit_creating(&2, 1_000_000);
		<Balances as Currency<u64>>::deposit_creating(&3, 1_000_000);
		
		// Note: NFT creation is simplified for testing
		// In actual runtime, parliamentary NFTs would be properly created
	});
	
	ext
}

// Test helper functions
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			<System as Hooks<u64>>::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		<System as Hooks<u64>>::on_initialize(System::block_number());
	}
}

pub fn advance_blocks(num_blocks: u64) {
	let target = System::block_number() + num_blocks;
	run_to_block(target);
}

pub fn advance_to_epoch_end() {
	// Advance to end of current epoch (432,000 blocks)
	advance_blocks(pallet_pez_rewards::BLOCKS_PER_EPOCH as u64);
}

pub fn advance_to_claim_period_end() {
	// Advance past claim period (100,800 blocks)
	advance_blocks(pallet_pez_rewards::CLAIM_PERIOD_BLOCKS as u64);
}