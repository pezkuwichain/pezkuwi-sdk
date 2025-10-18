// pezkuwi/pallets/pez-treasury/src/mock.rs

use crate as pallet_pez_treasury;
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, ConstU128, AsEnsureOriginWithArg},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};
use frame_system as system;
use frame_system::EnsureRoot;

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets, // EKLENDİ
		PezTreasury: pallet_pez_treasury,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
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
	type AccountId = AccountId;
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
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
}

// DÜZELTİLDİ: pallet_assets için Config
parameter_types! {
	pub const AssetDeposit: u128 = 1;
	pub const ApprovalDeposit: u128 = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: u128 = 1;
	pub const MetadataDepositPerByte: u128 = 1;
}
impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128<0>;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
	type RemoveItemsLimit = ConstU32<1000>;
	type CallbackHandle = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}


parameter_types! {
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const IncentivePotId: PalletId = PalletId(*b"py/incnt");
	pub const GovernmentPotId: PalletId = PalletId(*b"py/govmt");
	pub const PresaleAccount: u64 = 100;
	pub const FounderAccount: u64 = 200;
	pub const PezAssetId: u32 = 1; // EKLENDİ
}

// DÜZELTİLDİ: PezTreasury için Config
impl pallet_pez_treasury::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Assets = Assets; // `Currency = Balances`'dan değiştirildi
	type WeightInfo = ();
	type PezAssetId = PezAssetId; // EKLENDİ
	type TreasuryPalletId = TreasuryPalletId;
	type IncentivePotId = IncentivePotId;
	type GovernmentPotId = GovernmentPotId;
	type PresaleAccount = PresaleAccount;
	type FounderAccount = FounderAccount;
	type ForceOrigin = frame_system::EnsureRoot<u64>;
}

// DÜZELTİLDİ: Genesis bloğu
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::<Test>::default().build_storage().unwrap();

	// PEZ token'ını genesis'te yarat
	let assets_genesis = pallet_assets::GenesisConfig::<Test> {
		assets: vec![(
			PezAssetId::get(), // id
			0, // owner (root/alice)
			true, // is_sufficient
			1, // min_balance
		)],
		metadata: vec![(PezAssetId::get(), "Pez".into(), "PEZ".into(), 12)],
		accounts: vec![],
		next_asset_id: Some(2),
	};
	assets_genesis.assimilate_storage(&mut storage).unwrap();

	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}