use crate as pallet_trust;
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU16, ConstU64},
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
		TrustPallet: pallet_trust,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
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
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	pub const ScoreMultiplierBase: u128 = 1000;
	pub const TrustUpdateInterval: u64 = 100; // Test için kısa interval
}

pub struct MockStakingScoreProvider;
impl pallet_trust::StakingScoreProvider<u64, u64> for MockStakingScoreProvider {
	fn get_staking_score(_who: &u64) -> (u32, u64) {
		(100, 0)
	}
}

pub struct MockReferralScoreProvider;
impl pallet_trust::ReferralScoreProvider<u64> for MockReferralScoreProvider {
	fn get_referral_score(_who: &u64) -> u32 {
		50
	}
}

pub struct MockPerwerdeScoreProvider;
impl pallet_trust::PerwerdeScoreProvider<u64> for MockPerwerdeScoreProvider {
	fn get_perwerde_score(_who: &u64) -> u32 {
		30
	}
}

pub struct MockTikiScoreProvider;
impl pallet_trust::TikiScoreProvider<u64> for MockTikiScoreProvider {
	fn get_tiki_score(_who: &u64) -> u32 {
		20
	}
}

pub struct MockCitizenshipStatusProvider;
impl pallet_trust::CitizenshipStatusProvider<u64> for MockCitizenshipStatusProvider {
	fn is_citizen(who: &u64) -> bool {
		// Test için: 1-100 arası hesaplar vatandaş, 999 değil
		*who >= 1 && *who <= 100 && *who != 999
	}
}

impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Score = u128;
	type ScoreMultiplierBase = ScoreMultiplierBase;
	type UpdateInterval = TrustUpdateInterval;
	type StakingScoreSource = MockStakingScoreProvider;
	type ReferralScoreSource = MockReferralScoreProvider;
	type PerwerdeScoreSource = MockPerwerdeScoreProvider;
	type TikiScoreSource = MockTikiScoreProvider;
	type CitizenshipSource = MockCitizenshipStatusProvider;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}