// === pallet_hemwelati_odul/src/mock.rs ===

use crate as pallet_hemwelati_odul;
use frame_support::{
    parameter_types,
    traits::{ConstU32, ConstU64, Everything, AsEnsureOriginWithArg,ConstBool}, // Added ConstU64, ConstBool
    PalletId
};
use frame_system as system;
use frame_system::EnsureRoot; // For Balances admin operations if needed
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{IdentityLookup, BlakeTwo256}, // Added BlakeTwo256
};

// Define AccountId and Balance types for the test runtime
pub type AccountId = u64;
pub type Balance = u64;


frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>}, // Added Balances
        HemwelatiOdul: pallet_hemwelati_odul::{Pallet, Call, Storage, Event<T>},
    }
);

impl system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin; // Changed from Origin
    type RuntimeCall = RuntimeCall;     // Changed from Call
    type Index = u64;                // Added Index
    type BlockNumber = u64;          // Changed from RuntimeBlockNumber
    type Hash = H256;
    type Hashing = BlakeTwo256;      // Changed from sp_runtime::traits::BlakeTwo256
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;   // Changed from Event
    type BlockHashCount = ConstU32<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>; // Changed from ()
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU32<42>;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>; // Added MaxConsumers
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1; // Minimum balance for an account to exist
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for TestRuntime {
    type Balance = Balance;
    type DustRemoval = ();
    type RuntimeEvent = RuntimeEvent;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<TestRuntime>; // Or a custom one
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type FreezeIdentifier = (); // Added FreezeIdentifier
    type MaxFreezes = ConstU32; // Added MaxFreezes
    type HoldIdentifier = (); // Added HoldIdentifier
    type MaxHolds = ConstU32; // Added MaxHolds
}


parameter_types! {
    pub const HemwelatiPalletId: PalletId = PalletId(*b"pz/rewrd"); // Matched PalletId from lib.rs
}

impl pallet_hemwelati_odul::Config for TestRuntime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances; // Use pallet-balances for Currency operations
    // Use the test WeightInfo for tests, or SubstrateWeight for more realistic benchmarking setup
    type WeightInfo = crate::weights::SubstrateWeight<TestRuntime>;
    // PalletId is not configured here as it's a const in lib.rs
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = system::GenesisConfig::default().build_storage::<TestRuntime>().unwrap();
    // Configure balances for genesis accounts if needed
    pallet_balances::GenesisConfig::<TestRuntime> {
        balances: vec![
            (1, 1000), // Example: Alice (1) has 1000 balance
            (2, 500),  // Example: Bob (2) has 500 balance
            (HemwelatiPalletId::get().into_account_truncating(), 2000) // Fund the pallet account
        ],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1)); // Progress to block 1 to emit events
    ext
}