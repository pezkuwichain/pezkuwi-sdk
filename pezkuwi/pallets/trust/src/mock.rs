// === pallet_trust/src/mock.rs ===

use crate as pallet_trust; // This makes pallet_trust items accessible via crate::
use frame_support::{
    parameter_types,
    traits::{
        ConstU32, ConstU64, Everything,
        schedule::{self, Named as ScheduleNamed},
        Get,
    },
    PalletId,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{IdentityLookup, BlakeTwo256},
    DispatchResult,
};
use sp_std::{marker::PhantomData, prelude::*};

use pezkuwi_primitives::{types::{RawScore, TrustScore}, traits::*}; // AccountIdCore not directly used here

pub type AccountId = u64;
pub type BlockNumber = u64;

pub struct MockEgitimScoreProvider;
impl EgitimScoreProvider<AccountId> for MockEgitimScoreProvider {
    type Score = RawScore;
    fn get_egitim_score(_who: &AccountId) -> RawScore { 100 }
}

pub struct MockStakingScoreProvider;
impl StakingScoreProvider<AccountId> for MockStakingScoreProvider {
    type Score = RawScore;
    fn get_staking_score(_who: &AccountId) -> RawScore { 200 }
}

pub struct MockReferralScoreProvider;
impl ReferralScoreProvider<AccountId> for MockReferralScoreProvider {
    type Score = RawScore;
    fn get_referral_score(_who: &AccountId) -> RawScore { 50 }
}

pub struct MockTikiScoreProvider;
impl TikiScoreProvider<AccountId> for MockTikiScoreProvider {
    type Score = RawScore;
    fn get_tiki_score(_who: &AccountId) -> RawScore { 150 }
}

pub struct MockScheduler<Call>(PhantomData<Call>);
impl<Call: 'static + From<crate::pallet::Call<TestRuntime>> + Clone + Eq + PartialEq + sp_std::fmt::Debug> ScheduleNamed<
    BlockNumber,
    Call,
    <TestRuntime as frame_system::Config>::RuntimeOrigin,
> for MockScheduler<Call> {
    type PalletsOrigin = <TestRuntime as frame_system::Config>::RuntimeOrigin;

    fn schedule_named(
        _id: Vec<u8>,
        _when: schedule::DispatchTime<BlockNumber>,
        _maybe_periodic: Option<schedule::SchedulePeriod<BlockNumber>>,
        _priority: schedule::SchedulePriority,
        _origin: Self::PalletsOrigin,
        _call: Call,
    ) -> DispatchResult {
        Ok(())
    }

    fn cancel_named(_id: Vec<u8>) -> DispatchResult { Ok(()) }
}

frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = MockBlock, // Explicitly use MockBlock defined below
        NodeBlock = MockBlock,
        UncheckedExtrinsic = MockUncheckedExtrinsic, // Explicitly use MockUncheckedExtrinsic
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Trust: pallet_trust::{Pallet, Call, Storage, Event<T>}, // pallet_trust is crate
    }
);

pub type MockUncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<TestRuntime>;
pub type MockBlock = frame_system::mocking::MockBlock<TestRuntime>;

impl system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU32<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const TrustPalletIdValue: PalletId = PalletId(*b"pz/trust");
    pub const EgitimCoeffValue: u32 = 100;
    pub const StakingCoeffValue: u32 = 100;
    pub const ReferralCoeffValue: u32 = 100;
    pub const TikiCoeffValue: u32 = 100;
    pub const ActivityCoeffValue: u32 = 100;
    pub const ScoreFactorValue: u32 = 1000;
    pub const MaxTrustScoreValue: TrustScore = 10000;
    pub const ActivityResetPeriodValue: BlockNumber = 10;
    pub const MaxTaskIdLenValue: u32 = 32;
    pub const MaxActivityCounterItemsValue: u32 = 1000;
}

// Use crate::pallet::Config for the pallet's Config trait
impl crate::pallet::Config for TestRuntime {
    type PalletId = TrustPalletIdValue;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type EgitimScoreProvider = MockEgitimScoreProvider;
    type StakingScoreProvider = MockStakingScoreProvider;
    type ReferralScoreProvider = MockReferralScoreProvider;
    type TikiScoreProvider = MockTikiScoreProvider;
    type EgitimCoefficientScaled = EgitimCoeffValue;
    type StakingCoefficientScaled = StakingCoeffValue;
    type ReferralCoefficientScaled = ReferralCoeffValue;
    type TikiCoefficientScaled = TikiCoeffValue;
    type ActivityCoefficientScaled = ActivityCoeffValue;
    type ScoreScaleFactor = ScoreFactorValue;
    type MaxScaledTrustScore = MaxTrustScoreValue;
    type RecalculateOrigin = frame_system::EnsureRoot<AccountId>;
    type PenaltyOrigin = frame_system::EnsureRoot<AccountId>;
    type ActivityResetOrigin = frame_system::EnsureRoot<AccountId>;
    type RecordActivityOrigin = frame_system::EnsureRoot<AccountId>;
    type ActivityResetPeriod = ActivityResetPeriodValue;
    type MaxTaskIdLen = MaxTaskIdLenValue;
    type RuntimeCall = RuntimeCall;
    type Scheduler = MockScheduler<RuntimeCall>;
    // MODIFIED: GetPalletsOriginDefault is now inside crate::pallet
    // The #[pallet::type_value] on GetPalletsOriginDefault<T: Config> creates a ZST named GetPalletsOriginDefault
    // which implements Get.
    type PalletsOrigin = crate::pallet::GetPalletsOriginDefault;
    type MaxActivityCounterItems = MaxActivityCounterItemsValue;
}