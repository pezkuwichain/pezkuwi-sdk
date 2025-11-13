// pezkuwi/pallets/pez-treasury/src/mock.rs
// VERSION 3: AccountId tipi H256 yapıldı (u64 yerine)

use crate as pallet_pez_treasury;
use frame_support::{
    assert_ok,
    construct_runtime, parameter_types,
    traits::{ConstU32, ConstU128, OnFinalize, OnInitialize},
    PalletId,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        Balances: pallet_balances,
        Assets: pallet_assets,
        PezTreasury: pallet_pez_treasury,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = frame_support::weights::constants::RocksDbWeight;
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = H256; // V3: u64 -> H256 değişti
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u128 = 1;
}

impl pallet_balances::Config for Test {
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
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

parameter_types! {
    pub const AssetDeposit: u128 = 100;
    pub const ApprovalDeposit: u128 = 1;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: u128 = 10;
    pub const MetadataDepositPerByte: u128 = 1;
}

impl pallet_assets::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u128;
    type AssetId = u32;
    type AssetIdParameter = u32;
    type Currency = Balances;
    type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<
        frame_system::EnsureSigned<Self::AccountId>,
    >;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<0>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type CallbackHandle = ();
    type WeightInfo = ();
    type RemoveItemsLimit = ConstU32<1000>;
    type Holder = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}

// CRITICAL: Bu üç PalletId FARKLI olmak ZORUNDA
parameter_types! {
    pub const PezTreasuryPalletId: PalletId = PalletId(*b"py/pztrs");
    pub const PezIncentivePotId: PalletId = PalletId(*b"py/pzinc");
    pub const PezGovernmentPotId: PalletId = PalletId(*b"py/pzgov");
    pub const PezAssetId: u32 = 1;
}

// V3: Test accounts - H256 formatında
use sp_runtime::traits::AccountIdConversion;

pub fn alice() -> H256 {
    H256::from_low_u64_be(1)
}

pub fn bob() -> H256 {
    H256::from_low_u64_be(2)
}

pub fn charlie() -> H256 {
    H256::from_low_u64_be(3)
}

pub fn presale() -> H256 {
    H256::from_low_u64_be(10)
}

pub fn founder() -> H256 {
    H256::from_low_u64_be(11)
}

parameter_types! {
    pub PresaleAccount: H256 = presale();
    pub FounderAccount: H256 = founder();
}

impl pallet_pez_treasury::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Assets = Assets;
    type WeightInfo = ();
    type PezAssetId = PezAssetId;
    type TreasuryPalletId = PezTreasuryPalletId;
    type IncentivePotId = PezIncentivePotId;
    type GovernmentPotId = PezGovernmentPotId;
    type PresaleAccount = PresaleAccount;
    type FounderAccount = FounderAccount;
    type ForceOrigin = frame_system::EnsureRoot<Self::AccountId>;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![
            (alice(), 1_000_000_000_000_000),
            (bob(), 1_000_000_000_000_000),
            (charlie(), 1_000_000_000_000_000),
            (presale(), 1_000_000_000_000_000),
            (founder(), 1_000_000_000_000_000),
        ],
        dev_accounts: None,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
        
        // Create PEZ asset
        assert_ok!(Assets::force_create(
            RuntimeOrigin::root(),
            PezAssetId::get(),
            alice(),
            true,
            1
        ));
    });
    ext
}

// Helper function to run to specific block
pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 1 {
            AllPalletsWithSystem::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        AllPalletsWithSystem::on_initialize(System::block_number());
    }
}

// V3: Helper to assert balance - H256 account ile
pub fn assert_pez_balance(account: H256, expected: u128) {
    assert_eq!(
        Assets::balance(PezAssetId::get(), account),
        expected,
        "PEZ balance mismatch for account {:?}. Expected: {}, Got: {}",
        account,
        expected,
        Assets::balance(PezAssetId::get(), account)
    );
}

// V3: Helper fonksiyonlar - H256 dönüyor
#[allow(dead_code)]
pub fn treasury_account() -> H256 {
    PezTreasuryPalletId::get().into_account_truncating()
}

#[allow(dead_code)]
pub fn incentive_pot_account() -> H256 {
    PezIncentivePotId::get().into_account_truncating()
}

#[allow(dead_code)]
pub fn government_pot_account() -> H256 {
    PezGovernmentPotId::get().into_account_truncating()
}

// V3: Debug helper
#[allow(dead_code)]
pub fn debug_pot_accounts() {
    println!("\n=== PalletId Debug ===");
    println!("Treasury bytes: {:?}", PezTreasuryPalletId::get().0);
    println!("Incentive bytes: {:?}", PezIncentivePotId::get().0);
    println!("Government bytes: {:?}", PezGovernmentPotId::get().0);
    println!("======================\n");
}