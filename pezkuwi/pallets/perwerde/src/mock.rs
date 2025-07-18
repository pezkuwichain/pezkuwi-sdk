use crate as pallet_egitim;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, Everything},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;
type Balance = u128;
type BlockNumber = u64;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Egitim: pallet_egitim,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}

parameter_types! {
	pub const MaxCourseNameLength: u32 = 100;
	pub const MaxCourseDescLength: u32 = 500;
	pub const MaxCourseLinkLength: u32 = 200;
	pub const MaxStudentsPerCourse: u32 = 1000;
}

impl pallet_egitim::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type MaxCourseNameLength = MaxCourseNameLength;
	type MaxCourseDescLength = MaxCourseDescLength;
	type MaxCourseLinkLength = MaxCourseLinkLength;
	type MaxStudentsPerCourse = MaxStudentsPerCourse;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	// Test hesaplarına başlangıç bakiyesi verelim
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 1000), (2, 1000), (3, 1000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}