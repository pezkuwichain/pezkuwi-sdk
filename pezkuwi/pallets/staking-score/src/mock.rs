//! pallet-staking-score için mock runtime.

use crate as pallet_staking_score;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU32, ConstU64, Everything, Hooks},
	weights::constants::RocksDbWeight,
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use sp_staking::{StakerStatus, StakingAccount};
use std::collections::BTreeMap;

// Paletimizdeki sabitleri import ediyoruz.
use crate::{UNITS, MONTH_IN_BLOCKS};

// --- Tip Takma Adları ---
type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u64;
pub type Nonce = u64;
pub type SessionIndex = u32;
pub type EraIndex = u32;

// --- Paletler için Sabitler ---
pub const MAX_NOMINATIONS_CONST: u32 = 16;
parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const ExistentialDeposit: Balance = 1;
	pub static SessionsPerEra: SessionIndex = 3;
	pub const BondingDuration: u32 = 3;
	pub const SlashDeferDuration: EraIndex = 0;
	pub static HistoryDepth: u32 = 80;
	pub const MaxUnlockingChunks: u32 = 32;
	pub static MaxNominations: u32 = 16;
	pub const MinimumPeriod: u64 = 5000;
	pub static BagThresholds: &'static [u64] = &[10, 20, 30, 40, 50, 60, 1_000, 2_000, 10_000];
	pub static MaxWinners: u32 = 100;
	// Yeni eklenenler: pallet_staking::Config için gerekli minimum bond miktarları.
	pub const MinNominatorBond: Balance = 1 * UNITS; // Testler için yeterince küçük bir değer.
	pub const MinValidatorBond: Balance = 1 * UNITS; // Testler için yeterince küçük bir değer.
}

// --- construct_runtime! Makrosu ---
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Staking: pallet_staking,
		Session: pallet_session,
		Timestamp: pallet_timestamp,
		Historical: pallet_session::historical,
		BagsList: pallet_bags_list::<Instance1>,
		// Kendi paletimiz:
		StakingScore: pallet_staking_score,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type DbWeight = RocksDbWeight;
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type MaxLocks = ConstU32<1024>;
	type Balance = Balance;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
}

sp_runtime::impl_opaque_keys! {
	pub struct MockSessionKeys {
		pub dummy: sp_runtime::testing::UintAuthorityId,
	}
}

impl From<sp_runtime::testing::UintAuthorityId> for MockSessionKeys {
	fn from(dummy: sp_runtime::testing::UintAuthorityId) -> Self {
		Self { dummy }
	}
}

pub struct TestSessionHandler;
impl pallet_session::SessionHandler<AccountId> for TestSessionHandler {
	const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[sp_runtime::key_types::DUMMY];
	fn on_genesis_session<T: sp_runtime::traits::OpaqueKeys>(_validators: &[(AccountId, T)]) {}
	fn on_new_session<T: sp_runtime::traits::OpaqueKeys>(
		_changed: bool,
		_validators: &[(AccountId, T)],
		_queued_validators: &[(AccountId, T)],
	) {
	}
	fn on_before_session_ending() {}
	fn on_disabled(_validator_index: u32) {}
}

impl pallet_session::Config for Test {
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, Staking>;
	type Keys = MockSessionKeys;
	type ShouldEndSession = pallet_session::PeriodicSessions<SessionsPerEra, ConstU64<0>>;
	type SessionHandler = TestSessionHandler;
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Test>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionsPerEra, ConstU64<0>>;
	type DisablingStrategy = ();
	type WeightInfo = ();
}

impl pallet_session::historical::Config for Test {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Test>;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ConstU64<5>;
	type WeightInfo = ();
}

type VoterBagsListInstance = pallet_bags_list::Instance1;
impl pallet_bags_list::Config<VoterBagsListInstance> for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type ScoreProvider = Staking;
	type BagThresholds = BagThresholds;
	type Score = sp_npos_elections::VoteWeight;
}

pub struct TestBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for TestBenchmarkingConfig {
	type MaxValidators = ConstU32<1000>;
	type MaxNominators = ConstU32<1000>;
}

#[derive_impl(pallet_staking::config_preludes::TestDefaultConfig)]
impl pallet_staking::Config for Test {
	type Currency = Balances;
	type UnixTime = Timestamp;
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SessionInterface = Self;
	type EraPayout = ();
	type NextNewSession = Session;
	type MaxExposurePageSize = ConstU32<64>;
	type ElectionProvider = frame_election_provider_support::NoElection<(
		AccountId,
		BlockNumber,
		Staking,
		MaxWinners,
	)>;
	type GenesisElectionProvider = Self::ElectionProvider;
	type VoterList = BagsList;
	type TargetList = pallet_staking::UseValidatorsMap<Self>;
	type MaxControllersInDeprecationBatch = ConstU32<100>;
	type AdminOrigin = EnsureRoot<AccountId>;
	type EventListeners = ();
	type HistoryDepth = HistoryDepth;
	type NominationsQuota = pallet_staking::FixedNominationsQuota<MAX_NOMINATIONS_CONST>;
	type MaxUnlockingChunks = MaxUnlockingChunks;
	type BenchmarkingConfig = TestBenchmarkingConfig;
	type OldCurrency = Balances;
	// MinNominatorBond ve MinValidatorBond tipleri bu trait'in parçası değildir.
	// Bu nedenle buradan kaldırılmalıdırlar.
	// Bunlar, pallet_staking::GenesisConfig içinde doğrudan değer olarak kullanılır.
}

// --- Bizim Paletimiz ve Adaptörü ---
pub struct StakingDataProvider;
impl crate::StakingInfoProvider<AccountId, Balance> for StakingDataProvider {
	fn get_staking_details(who: &AccountId) -> Option<crate::StakingDetails<Balance>> {
		if let Ok(ledger) = Staking::ledger(StakingAccount::Stash(who.clone())) {
			let nominations_count = Staking::nominators(who).map_or(0, |n| n.targets.len() as u32);
			let unlocking_chunks_count = ledger.unlocking.len() as u32;

			Some(crate::StakingDetails {
				staked_amount: ledger.total,
				nominations_count,
				unlocking_chunks_count,
			})
		} else {
			None
		}
	}
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type WeightInfo = ();
	type StakingInfo = StakingDataProvider;
}

// --- ExtBuilder ve Yardımcı Fonksiyonlar ---
pub struct ExtBuilder {
	stakers: Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			// Benchmarking ve testlerin düzgün çalışması için başlangıç staker'larını
			// testlerde kullanılacak USER_STASH (10) hesabını içermeyecek şekilde ayarlıyoruz.
			// USER_STASH testlerde manuel olarak bond edilecek.
			stakers: vec![
				// Sadece benchmarking için yeterli sayıda validator ve nominator
				(1, 1, 1_000 * UNITS, StakerStatus::Validator),
				(2, 2, 1_000 * UNITS, StakerStatus::Validator),
				(3, 3, 1_000 * UNITS, StakerStatus::Validator),
				(4, 4, 1_000 * UNITS, StakerStatus::Validator),
				(5, 5, 1_000 * UNITS, StakerStatus::Validator),
				(6, 6, 1_000 * UNITS, StakerStatus::Validator),
				(7, 7, 1_000 * UNITS, StakerStatus::Validator),
				(8, 8, 1_000 * UNITS, StakerStatus::Validator),
				(9, 9, 1_000 * UNITS, StakerStatus::Validator),
				(11, 11, 100 * UNITS, StakerStatus::Nominator(vec![1, 2])),
				(12, 12, 100 * UNITS, StakerStatus::Nominator(vec![3, 4])),
			],
		}
	}
}

impl ExtBuilder {
	pub fn add_staker(mut self, stash: AccountId, ctrl: AccountId, stake: Balance, status: StakerStatus<AccountId>) -> Self {
		self.stakers.push((stash, ctrl, stake, status));
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

		let mut balances: Vec<(AccountId, Balance)> = vec![
			(1, 1_000_000 * UNITS),
			(2, 1_000_000 * UNITS),
			// USER_STASH (10) için de başlangıçta yeterli bakiye atıyoruz,
			// çünkü testlerde bond etmesi beklenecek.
			(10, 1_000_000 * UNITS),
			(20, 100_000 * UNITS),
			(101, 2_000 * UNITS),
		];
		// ExtBuilder'daki tüm staker'ların ve diğer test hesaplarının (eğer varsa)
		// yeterli bakiyeye sahip olduğundan emin olun.
		// Her staker'a veya test hesabına minimum bond miktarının çok üzerinde bakiye ekle.
		for (stash, _, _, _) in &self.stakers {
			if !balances.iter().any(|(acc, _)| acc == stash) {
				balances.push((*stash, 1_000_000 * UNITS)); // Staker'lara bol miktarda bakiye
			}
		}

		pallet_balances::GenesisConfig::<Test> { balances, ..Default::default() }
			.assimilate_storage(&mut storage)
			.unwrap();

		pallet_staking::GenesisConfig::<Test> {
			stakers: self.stakers.clone(),
			validator_count: self.stakers.len() as u32, // Staker sayısını dinamik yap
			minimum_validator_count: 0, // En az 0 validator olmasına izin ver
			invulnerables: self.stakers.iter().filter_map(|(stash, _, _, status)| {
				if let StakerStatus::Validator = status {
					Some(stash.clone())
				} else {
					None
				}
			}).collect(),
			force_era: pallet_staking::Forcing::ForceNew, // Yeni era başlatmaya zorla
			min_nominator_bond: MinNominatorBond::get(), // Tanımlanan minimum değerleri kullan
			min_validator_bond: MinValidatorBond::get(), // Tanımlanan minimum değerleri kullan
			..Default::default()
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		pallet_session::GenesisConfig::<Test> {
			keys: self
				.stakers
				.iter()
				.filter_map(|(stash, ctrl, _, status)| {
					if let StakerStatus::Validator = status {
						Some((
							*stash,
							*ctrl,
							MockSessionKeys { dummy: (*stash).into() },
						))
					} else {
						None
					}
				})
				.collect(),
			..Default::default()
		}
		.assimilate_storage(&mut storage)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(storage);
		// run_to_block çağrısını ExtBuilder::build_and_execute içinde veya
		// benchmarking setup'ında yapmak daha doğru. Burada sadece temel storage'ı kuruyoruz.
		ext
	}

	pub fn build_and_execute(self, test: impl FnOnce() -> ()) {
		self.build().execute_with(test);
	}
}

/// Bloğu `n`'e kadar ilerletir.
pub fn run_to_block(n: BlockNumber) {
	while System::block_number() < n {
		if System::block_number() > 1 {
			System::on_finalize(System::block_number());
			Session::on_finalize(System::block_number());
			Staking::on_finalize(System::block_number());
		}
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Session::on_initialize(System::block_number());
		Staking::on_initialize(System::block_number());
	}
}
