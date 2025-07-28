//! Mock runtime for pallet-trust.

use crate as pallet_trust;
use frame_support::{construct_runtime, derive_impl, parameter_types, traits::ConstU64};
use sp_core::H256;
use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, BuildStorage};
use sp_std::collections::btree_map::BTreeMap;

// Paletimizin ihtiyaç duyduğu Arayüzleri (trait) import ediyoruz.
use pallet_trust::{
	CitizenshipStatusProvider, PerwerdeScoreProvider, RawScore, ReferralScoreProvider,
	StakingScoreProvider, TikiScoreProvider,
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type BlockNumber = u64;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Trust: pallet_trust,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

// --- Mock Veri Sağlayıcıları ---
#[derive(Default)]
pub struct MockStakingScoreProvider {
	pub scores: BTreeMap<AccountId, (RawScore, BlockNumber)>,
}
impl StakingScoreProvider<AccountId, BlockNumber> for MockStakingScoreProvider {
	fn get_staking_score(who: &AccountId) -> (RawScore, BlockNumber) {
		self.scores.get(who).cloned().unwrap_or((0, 0))
	}
}

#[derive(Default)]
pub struct MockReferralScoreProvider { pub scores: BTreeMap<AccountId, RawScore> }
impl ReferralScoreProvider<AccountId> for MockReferralScoreProvider {
	fn get_referral_score(who: &AccountId) -> RawScore { self.scores.get(who).cloned().unwrap_or(0) }
}

#[derive(Default)]
pub struct MockPerwerdeScoreProvider { pub scores: BTreeMap<AccountId, RawScore> }
impl PerwerdeScoreProvider<AccountId> for MockPerwerdeScoreProvider {
	fn get_perwerde_score(who: &AccountId) -> RawScore { self.scores.get(who).cloned().unwrap_or(0) }
}

#[derive(Default)]
pub struct MockTikiScoreProvider { pub scores: BTreeMap<AccountId, RawScore> }
impl TikiScoreProvider<AccountId> for MockTikiScoreProvider {
	fn get_tiki_score(who: &AccountId) -> RawScore { self.scores.get(who).cloned().unwrap_or(0) }
}

#[derive(Default)]
pub struct MockCitizenshipStatusProvider { pub citizens: BTreeMap<AccountId, bool> }
impl CitizenshipStatusProvider<AccountId> for MockCitizenshipStatusProvider {
	fn is_citizen(who: &AccountId) -> bool { self.citizens.get(who).cloned().unwrap_or(false) }
}

// --- Paletimiz için Config Implementasyonu ---
parameter_types! {
	pub const ScoreMultiplierBase: u128 = 1000;
}

impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Score = u128;
	type ScoreMultiplierBase = ScoreMultiplierBase;

	// Mock sağlayıcıları bağlıyoruz
	type StakingScoreSource = MockStakingScoreProvider;
	type ReferralScoreSource = MockReferralScoreProvider;
	type PerwerdeScoreSource = MockPerwerdeScoreProvider;
	type TikiScoreSource = MockTikiScoreProvider;
	type CitizenshipSource = MockCitizenshipStatusProvider;
}

// --- Test Ortamı Kurulumu ---
pub struct ExtBuilder {
	pub staking_scores: BTreeMap<AccountId, (RawScore, BlockNumber)>,
	pub referral_scores: BTreeMap<AccountId, RawScore>,
	pub perwerde_scores: BTreeMap<AccountId, RawScore>,
	pub tiki_scores: BTreeMap<AccountId, RawScore>,
	pub citizens: BTreeMap<AccountId, bool>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			staking_scores: BTreeMap::new(),
			referral_scores: BTreeMap::new(),
			perwerde_scores: BTreeMap::new(),
			tiki_scores: BTreeMap::new(),
			citizens: BTreeMap::new(),
		}
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		// Mock sağlayıcıların başlangıç verilerini ayarlıyoruz.
		MockStakingScoreProvider { scores: self.staking_scores }.assimilate_storage(&mut t).unwrap();
		MockReferralScoreProvider { scores: self.referral_scores }.assimilate_storage(&mut t).unwrap();
		MockPerwerdeScoreProvider { scores: self.perwerde_scores }.assimilate_storage(&mut t).unwrap();
		MockTikiScoreProvider { scores: self.tiki_scores }.assimilate_storage(&mut t).unwrap();
		MockCitizenshipStatusProvider { citizens: self.citizens }.assimilate_storage(&mut t).unwrap();
		
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}