// pezkuwi/pallets/referral/src/mock.rs (Yeniden İnşa Edilmiş)

use crate as pallet_referral;
use frame_support::traits::{ConstU128, ConstU16, ConstU32, ConstU64, Everything};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Test runtime'ımızı yapılandıralım
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		IdentityKyc: pallet_identity_kyc,
		Referral: pallet_referral,
	}
);

// frame_system için modern konfigürasyon
impl system::Config for Test {
	type BaseCallFilter = Everything;
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
	type MaxConsumers = ConstU32<16>;
    type RuntimeTask = ();
    type ExtensionsWeightInfo = ();
    type SingleBlockMigrations = ();
    type MultiBlockMigrator = ();
    type PreInherents = ();
    type PostInherents = ();
    type PostTransactions = ();
}

// pallet_balances için konfigürasyon
impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ConstU32<1>;
	type DoneSlashHandler = ();
}

// pallet_identity_kyc için konfigürasyon
impl pallet_identity_kyc::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxStringLength = ConstU32<50>;
	type MaxCidLength = ConstU32<100>;
	type KycApprovalOrigin = frame_system::EnsureRoot<u64>;
	type Currency = Balances;
	type KycApplicationDeposit = ConstU128<100>;
}

// Nihayet, pallet_referral için konfigürasyon
impl pallet_referral::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

// Testler için başlangıç durumu oluşturan fonksiyon
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::<Test>::default().build_storage().unwrap();

	// Test için başlangıç bakiyeleri
	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10_000), (2, 10_000), (3, 10_000)],
		dev_accounts: None,
	}
	.assimilate_storage(&mut storage)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(storage);
	ext.execute_with(|| System::set_block_number(1));
	ext
}