use super::*;
use crate::{self as pallet_validator_pool, types::*};
use frame_support::{
    construct_runtime, parameter_types,
    traits::{ConstU32, Everything},
};
use frame_system as system;
use pallet_session::{PeriodicSessions, SessionHandler};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u64;

// Configure a mock runtime to test the pallet.
construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        ValidatorPool: pallet_validator_pool,
        Session: pallet_session,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type RuntimeTask = ();
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = frame_system::mocking::MockBlock<Test>;
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
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

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
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ();
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

// Mock Session Pallet
parameter_types! {
    pub const Period: u64 = 10;
    pub const Offset: u64 = 0;
}

// Mock session handler
pub struct MockSessionHandler;
impl SessionHandler<AccountId> for MockSessionHandler {
    const KEY_TYPE_IDS: &'static [sp_runtime::KeyTypeId] = &[];
    
    fn on_genesis_session<T: sp_runtime::traits::OpaqueKeys>(_validators: &[(AccountId, T)]) {}
    
    fn on_new_session<T: sp_runtime::traits::OpaqueKeys>(
        _changed: bool,
        _validators: &[(AccountId, T)],
        _queued_validators: &[(AccountId, T)],
    ) {}
    
    fn on_disabled(_validator_index: u32) {}
}

// Mock opaque keys with all required trait implementations
#[derive(
    codec::Encode, 
    codec::Decode, 
    codec::DecodeWithMemTracking,
    Clone, 
    PartialEq, 
    Eq, 
    sp_runtime::RuntimeDebug,
    scale_info::TypeInfo
)]
#[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
pub struct MockOpaqueKeys;

impl sp_runtime::traits::OpaqueKeys for MockOpaqueKeys {
    type KeyTypeIdProviders = ();
    
    fn key_ids() -> &'static [sp_runtime::KeyTypeId] {
        &[]
    }
    
    fn get_raw(&self, _id: sp_runtime::KeyTypeId) -> &[u8] {
        &[]
    }
}

impl From<()> for MockOpaqueKeys {
    fn from(_: ()) -> Self {
        MockOpaqueKeys
    }
}

impl pallet_session::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type ValidatorId = AccountId;
    type ValidatorIdOf = ();
    type ShouldEndSession = PeriodicSessions<Period, Offset>;
    type NextSessionRotation = PeriodicSessions<Period, Offset>;
    type SessionManager = ValidatorPool;
    type SessionHandler = MockSessionHandler;
    type Keys = MockOpaqueKeys;
    type WeightInfo = ();
    type DisablingStrategy = ();
}

// Mock Randomness
pub struct MockRandomness;
impl Randomness<H256, BlockNumber> for MockRandomness {
    fn random(subject: &[u8]) -> (H256, BlockNumber) {
        let mut hash = H256::zero();
        // Simple deterministic randomness for testing
        if !subject.is_empty() {
            hash.as_mut()[0] = subject[0];
        }
        (hash, 1)
    }
}

// Test implementations for trait dependencies - FIXED
impl TikiScoreProvider<AccountId> for TestTikiProvider {
    fn get_tiki_score(who: &AccountId) -> u32 {
        match who {
            _ => 1, 
        }
    }
}


pub struct TestTikiProvider;
impl TikiScoreProvider<AccountId> for TestTikiProvider {
    fn get_tiki_score(who: &AccountId) -> u32 {
        match who {
            1..=15 => 1, // Tüm test user'ları için tiki var
            _ => 0,
        }
    }
}

pub struct TestReferralProvider;
impl ReferralProvider<AccountId> for TestReferralProvider {
    fn get_referral_count(who: &AccountId) -> u32 {
        match who {
            1..=15 => 1000, // Tüm test user'ları için yüksek community support (threshold: 500)
            _ => 600,       // Diğerleri için de yeterli
        }
    }
}

pub struct TestPerwerdeProvider;
impl PerwerdeProvider<AccountId> for TestPerwerdeProvider {
    fn get_perwerde_score(who: &AccountId) -> u32 {
        match who {
            1..=15 => 100, // Tüm test user'ları için perwerde var
            _ => 50,
        }
    }
}

parameter_types! {
    pub const MaxValidators: u32 = 21;
    pub const MaxPoolSize: u32 = 500;
    pub const MinStakeAmount: u128 = 1000;
}

// Mock WeightInfo implementation
pub struct MockWeightInfo;
impl crate::WeightInfo for MockWeightInfo {
    fn join_validator_pool() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    
    fn leave_validator_pool() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    
    fn update_performance_metrics() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    
    fn force_new_era() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(50_000, 0)
    }
    
    fn update_category() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
    
    fn set_pool_parameters() -> frame_support::weights::Weight {
        frame_support::weights::Weight::from_parts(10_000, 0)
    }
}

impl Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = MockWeightInfo;
    type Randomness = MockRandomness;
    type TrustSource = TestTrustProvider;
    type TikiSource = TestTikiProvider;
    type ReferralSource = TestReferralProvider;
    type PerwerdeSource = TestPerwerdeProvider;
    type PoolManagerOrigin = frame_system::EnsureRoot<AccountId>;
    type MaxValidators = MaxValidators;
    type MaxPoolSize = MaxPoolSize;
    type MinStakeAmount = MinStakeAmount;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut storage = system::GenesisConfig::<Test>::default().build_storage().unwrap();

    // Initialize balances - Fixed genesis config with correct type
    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (1, 10000),
            (2, 8000),
            (3, 6000),
            (4, 4000),
            (5, 2000),
        ],
        dev_accounts: None, // Changed to None instead of empty vec
    }
    .assimilate_storage(&mut storage)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(storage);
    ext.execute_with(|| {
        System::set_block_number(1);
        // Set initial era length
        ValidatorPool::set_pool_parameters(
            RuntimeOrigin::root(),
            100, // Era length: 100 blocks
        ).unwrap();
    });
    ext
}

// Helper functions for tests
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            ValidatorPool::on_finalize(System::block_number());
            System::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        ValidatorPool::on_initialize(System::block_number());
    }
}

pub fn advance_era() {
    let current_era_start = ValidatorPool::era_start();
    let era_length = ValidatorPool::era_length();
    run_to_block(current_era_start + era_length + 1);
}

// Create test categories
pub fn stake_validator_category() -> ValidatorPoolCategory {
    ValidatorPoolCategory::StakeValidator {
        min_stake: 1000,
        trust_threshold: 450, 
    }
}

pub fn parliamentary_validator_category() -> ValidatorPoolCategory {
    ValidatorPoolCategory::ParliamentaryValidator
}

pub fn merit_validator_category() -> ValidatorPoolCategory {
    ValidatorPoolCategory::MeritValidator {
        special_tikis: vec![1u8].try_into().unwrap(), // Mock tiki type
        community_threshold: 500,
    }
}