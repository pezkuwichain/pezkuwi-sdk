use crate as pallet_presale;
use frame_support::{
	parameter_types,
	traits::{ConstU128, ConstU32, ConstU64, ConstU8},
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
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
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
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureRoot<u64>>;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
	type AssetDeposit = ConstU128<1>;
	type AssetAccountDeposit = ConstU128<10>;
	type MetadataDepositBase = ConstU128<1>;
	type MetadataDepositPerByte = ConstU128<1>;
	type ApprovalDeposit = ConstU128<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
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
	type PresaleWeightInfo = ();
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
		1.into(),
		1, // Alice as admin
		true,
		1
	));

	// Create wUSDT asset (ID: 2)
	assert_ok!(Assets::force_create(
		RuntimeOrigin::root(),
		2.into(),
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
	use sp_runtime::traits::AccountIdConversion;
	PresalePalletId::get().into_sub_account_truncating(presale_id)
}
