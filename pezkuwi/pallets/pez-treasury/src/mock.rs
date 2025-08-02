use crate as pallet_pez_treasury;
use frame_support::{
	derive_impl, parameter_types, assert_ok,
	traits::{ConstU16, ConstU64, ConstU128, Hooks},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};
use frame_system as system;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		PezTreasury: pallet_pez_treasury,
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

parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const IncentivePotId: PalletId = PalletId(*b"py/incnt");
	pub const GovernmentPotId: PalletId = PalletId(*b"py/govmt");
	pub const PresaleAccount: u64 = 100;
	pub const FounderAccount: u64 = 200;
}

impl pallet_pez_treasury::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = ();
	type TreasuryPalletId = TreasuryPalletId;
	type IncentivePotId = IncentivePotId;
	type GovernmentPotId = GovernmentPotId;
	type PresaleAccount = PresaleAccount;
	type FounderAccount = FounderAccount;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext: sp_io::TestExternalities = system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap()
		.into();
	
	ext.execute_with(|| {
		// Genesis distribution ve treasury initialization
		assert_ok!(PezTreasury::force_genesis_distribution(RuntimeOrigin::root()));
		assert_ok!(PezTreasury::initialize_treasury(RuntimeOrigin::root()));
	});
	
	ext
}

// Test helper functions
pub fn run_to_block(n: u64) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
	}
}

pub fn advance_blocks(num_blocks: u64) {
	let target = System::block_number() + num_blocks;
	run_to_block(target);
}