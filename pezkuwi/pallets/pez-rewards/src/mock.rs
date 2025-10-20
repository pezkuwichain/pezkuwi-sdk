// pezkuwi/pallets/pez-rewards/src/mock.rs (v1.0 - dev_accounts FIXED)

use crate as pallet_pez_rewards;
use frame_support::{
	assert_ok, construct_runtime, parameter_types,
	traits::{
		AsEnsureOriginWithArg, ConstU128, ConstU32, ConstU64,
		OnFinalize, OnInitialize,
		fungibles::Mutate,
	},
	PalletId,
};
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

// --- Dummy Trait Implementations for pallet-trust ---
pub struct MockStakingScoreProvider;
impl pallet_trust::StakingScoreProvider<H256, u64> for MockStakingScoreProvider {
	fn get_staking_score(_who: &H256) -> (u32, u64) { (0, 0) }
}
pub struct MockReferralScoreProvider;
impl pallet_trust::ReferralScoreProvider<H256> for MockReferralScoreProvider {
	fn get_referral_score(_who: &H256) -> u32 { 0 }
}
pub struct MockPerwerdeScoreProvider;
impl pallet_trust::PerwerdeScoreProvider<H256> for MockPerwerdeScoreProvider {
	fn get_perwerde_score(_who: &H256) -> u32 { 0 }
}
pub struct MockTikiScoreProvider;
impl pallet_trust::TikiScoreProvider<H256> for MockTikiScoreProvider {
	fn get_tiki_score(_who: &H256) -> u32 { 0 }
}
pub struct MockCitizenshipStatusProvider;
impl pallet_trust::CitizenshipStatusProvider<H256> for MockCitizenshipStatusProvider {
	fn is_citizen(_who: &H256) -> bool { false }
}

type Block = frame_system::mocking::MockBlock<Test>;
type Balance = u128;
type BlockNumber = u64;
type Weight = frame_support::weights::Weight;

// Runtime'ı yapılandır
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Trust: pallet_trust,
		PezRewards: pallet_pez_rewards,
	}
);

// --- frame_system::Config ---
parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const SS58Prefix: u8 = 42;
}
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = H256;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
	type RuntimeTask = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
	type ExtensionsWeightInfo = ();
}

// --- pallet_balances::Config ---
parameter_types! {
	pub const ExistentialDeposit: Balance = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}
impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type DoneSlashHandler = ();
}

// --- pallet_assets::Config ---
parameter_types! {
	pub const AssetDeposit: Balance = 100;
	pub const ApprovalDeposit: Balance = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 10;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const AssetAccountDeposit: Balance = 1;
}
impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<Self::AccountId>>;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = AssetAccountDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	type Holder = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

// --- pallet_trust::Config ---
pub struct MockTrustScore;
impl pallet_trust::TrustScoreProvider<H256> for MockTrustScore {
	fn trust_score_of(account: &H256) -> u128 {
		if *account == alice() { 100 }
		else if *account == bob() { 50 }
		else if *account == charlie() { 75 }
		else { 0 }
	}
}
impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Score = u128;
	type ScoreMultiplierBase = ConstU128<1>;
	type UpdateInterval = ConstU64<100>;
	type StakingScoreSource = MockStakingScoreProvider;
	type ReferralScoreSource = MockReferralScoreProvider;
	type PerwerdeScoreSource = MockPerwerdeScoreProvider;
	type TikiScoreSource = MockTikiScoreProvider;
	type CitizenshipSource = MockCitizenshipStatusProvider;
}

// --- pallet_pez_rewards::Config ---
parameter_types! {
	pub const IncentivePotId: PalletId = PalletId(*b"pez/rpot");
	pub const PezAssetId: u32 = 1;
	pub ClawbackRecipient: H256 = H256::from_low_u64_be(999);
}
pub struct MockWeightInfo;
impl crate::weights::WeightInfo for MockWeightInfo {
	fn initialize_rewards_system() -> Weight { Weight::zero() }
	fn record_trust_score() -> Weight { Weight::zero() }
	fn finalize_epoch() -> Weight { Weight::zero() }
	fn claim_reward() -> Weight { Weight::zero() }
	fn close_epoch() -> Weight { Weight::zero() }
	fn register_parliamentary_nft_owner() -> Weight { Weight::zero() }
}
impl pallet_pez_rewards::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Assets = Assets;
	type TrustScoreSource = MockTrustScore;
	type IncentivePotId = IncentivePotId;
	type PezAssetId = PezAssetId;
	type ClawbackRecipient = ClawbackRecipient;
	type WeightInfo = MockWeightInfo;
	type ForceOrigin = EnsureRoot<Self::AccountId>;
	type CollectionId = u32;
	type ItemId = u32;
}

// --- Helper Fonksiyonlar ---
pub fn alice() -> H256 { H256::from_low_u64_be(1) }
pub fn bob() -> H256 { H256::from_low_u64_be(2) }
pub fn charlie() -> H256 { H256::from_low_u64_be(3) }
pub fn dave() -> H256 { H256::from_low_u64_be(4) }

// --- new_test_ext ---
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	// HATA DÜZELTMESİ: dev_accounts field'ı eklendi (Option tipi)
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(alice(), 1_000_000_000_000_000),
			(bob(), 1_000_000_000_000_000),
			(charlie(), 1_000_000_000_000_000),
			(dave(), 1_000_000_000_000_000),
			(ClawbackRecipient::get(), 1_000_000_000_000_000),
		],
		dev_accounts: None, // Test ortamı için dev account'a gerek yok
	}
	.assimilate_storage(&mut t)
	.unwrap();

	pallet_assets::GenesisConfig::<Test> {
		assets: vec![(
			PezAssetId::get(), alice(), true, 1,
		)],
		metadata: vec![(
			PezAssetId::get(), b"Pez Token".to_vec(), b"PEZ".to_vec(), 12,
		)],
		accounts: vec![(
			PezAssetId::get(),
			PezRewards::incentive_pot_account_id(),
			1_000_000_000_000_000,
		)],
		next_asset_id: Some(PezAssetId::get() + 1),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PezRewards::initialize_rewards_system(RuntimeOrigin::root()));
	});
	ext
}

// --- Blok İlerletme Helper ---
pub fn advance_blocks(n: BlockNumber) {
	let target = System::block_number() + n;
	while System::block_number() < target {
		if System::block_number() > 0 {
			AllPalletsWithSystem::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		AllPalletsWithSystem::on_initialize(System::block_number());
	}
}

// --- Diğer Helper Fonksiyonlar ---
pub fn pez_balance(account: &H256) -> Balance {
	Assets::balance(PezAssetId::get(), account)
}

pub fn fund_incentive_pot(amount: Balance) {
	let pot = PezRewards::incentive_pot_account_id();
	assert_ok!(Assets::mint_into(PezAssetId::get(), &pot, amount));
}

pub fn register_nft_owner(nft_id: u32, owner: H256) {
	PezRewards::do_register_parliamentary_nft_owner(nft_id, owner);
}