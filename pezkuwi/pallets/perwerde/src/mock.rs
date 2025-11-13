use crate as pallet_perwerde;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, ConstU128, Everything, SortedMembers},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

// Temel tipleri tanımlıyoruz.
pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u64;
pub type Block = frame_system::mocking::MockBlock<Test>;

// Test runtime'ımızı kuruyoruz.
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Perwerde: pallet_perwerde,
		Council: pallet_collective::<Instance1>,
	}
);

// frame_system için implementasyon.
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
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// pallet_balances için implementasyon.
impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<1>;
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

parameter_types! {
	pub const MaxCourseNameLength: u32 = 100;
	pub const MaxCourseDescLength: u32 = 500;
	pub const MaxCourseLinkLength: u32 = 200;
	pub const MaxStudentsPerCourse: u32 = 100; // Reduced for test performance
}

// --- KESİN ÇÖZÜM BURADA BAŞLIYOR ---

// AdminOrigin'i test etmek için kendi özel yetki sağlayıcımızı oluşturuyoruz.
use frame_system::EnsureSignedBy;

// Bu struct, derleyicinin talep ettiği `SortedMembers` trait'ini manuel olarak uygular.
// Bu, harici ve versiyona bağımlı araçlara olan ihtiyacı ortadan kaldırır.
pub struct TestAdminProvider;
impl SortedMembers<AccountId> for TestAdminProvider {
	fn sorted_members() -> Vec<AccountId> {
		// Test için admin olarak sadece 0 ID'li hesabı yetkili kılıyoruz.
		vec![0]
	}
}

impl pallet_perwerde::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	// AdminOrigin'i, kendi yazdığımız ve sadece 0'ı admin kabul eden sağlayıcıya bağlıyoruz.
	type AdminOrigin = EnsureSignedBy<TestAdminProvider, AccountId>;
	type WeightInfo = ();
	type MaxCourseNameLength = MaxCourseNameLength;
	type MaxCourseDescLength = MaxCourseDescLength;
	type MaxCourseLinkLength = MaxCourseLinkLength;
	type MaxStudentsPerCourse = MaxStudentsPerCourse;
}

// Council Paletinin Mock Kurulumu (construct_runtime'da gerekli olduğu için kalıyor)
use pallet_collective::Instance1;
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * 60; // 5 minutes
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
	pub MaxProposalWeight: frame_support::weights::Weight = frame_support::weights::Weight::from_parts(1_000_000_000, 0);
}
impl pallet_collective::Config<Instance1> for Test {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = ();
	type SetMembersOrigin = EnsureRoot<AccountId>;
	type MaxProposalWeight = MaxProposalWeight;
	type DisapproveOrigin = EnsureRoot<AccountId>;
	type KillOrigin = EnsureRoot<AccountId>;
	type Consideration = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	// `pallet-collective`'in genesis'ini de kurmamıza gerek kalmadı çünkü artık testimiz ona bağlı değil.
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}