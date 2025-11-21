use crate as pallet_presale;
use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU16, ConstU32, ConstU64},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Presale: pallet_presale,
	}
);

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
	type MaxConsumers = ConstU32<16>;
	type RuntimeTask = ();
	type ExtensionsWeightInfo = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = ();
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<u64>>;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<0>; // No deposit required for test environment
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
	type Holder = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

parameter_types! {
	pub const PresalePalletId: PalletId = PalletId(*b"py/prsal");
	pub const PlatformFeePercent: u8 = 2;
	pub const MaxContributors: u32 = 10000;
	pub const MaxBonusTiers: u32 = 5;
	pub const MaxWhitelistedAccounts: u32 = 10000;
	pub PlatformTreasuryAccount: u64 = 999;
	pub StakingRewardPoolAccount: u64 = 998;
}

impl pallet_presale::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = PresalePalletId;
	type PlatformTreasury = PlatformTreasuryAccount;
	type StakingRewardPool = StakingRewardPoolAccount;
	type PlatformFeePercent = PlatformFeePercent;
	type MaxContributors = MaxContributors;
	type MaxBonusTiers = MaxBonusTiers;
	type MaxWhitelistedAccounts = MaxWhitelistedAccounts;
	type CreatePresaleOrigin = frame_system::EnsureSigned<u64>;
	type EmergencyOrigin = frame_system::EnsureRoot<u64>;
	type PresaleWeightInfo = crate::weights::SubstrateWeight<Test>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(1, 1_000_000_000_000_000), // Alice
			(2, 1_000_000_000_000_000), // Bob
			(3, 1_000_000_000_000_000), // Charlie
			(999, 1_000_000_000_000_000), // Platform Treasury
			(998, 1_000_000_000_000_000), // Staking Pool
		],
		dev_accounts: None,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// Helper to create assets
pub fn create_assets() {
	use frame_support::assert_ok;

	// Create PEZ asset (ID: 1)
	assert_ok!(Assets::force_create(
		RuntimeOrigin::root(),
		1u32,
		1, // Alice as admin
		true,
		1
	));

	// Create wUSDT asset (ID: 2)
	assert_ok!(Assets::force_create(
		RuntimeOrigin::root(),
		2u32,
		1, // Alice as admin
		true,
		1
	));
}

// Helper to mint assets to accounts
pub fn mint_assets(asset_id: u32, account: u64, amount: u128) {
	use frame_support::assert_ok;

	assert_ok!(Assets::mint(
		RuntimeOrigin::signed(1),
		asset_id.into(),
		account,
		amount
	));
}

// Helper to get presale sub-account treasury for a specific presale ID
pub fn presale_treasury(presale_id: u32) -> u64 {
	use sp_io::hashing::blake2_256;

	// Create a unique account ID for each presale by hashing pallet_id + presale_id
	// This matches the logic in pallet_presale::Pallet::presale_account_id
	let pallet_id = PresalePalletId::get();
	let mut buf = Vec::new();
	buf.extend_from_slice(&pallet_id.0[..]);
	buf.extend_from_slice(&presale_id.to_le_bytes());
	let hash = blake2_256(&buf);

	// Convert hash to u64 (since Test uses u64 as AccountId)
	// Take first 8 bytes and convert to u64
	u64::from_le_bytes([
		hash[0], hash[1], hash[2], hash[3],
		hash[4], hash[5], hash[6], hash[7],
	])
}
