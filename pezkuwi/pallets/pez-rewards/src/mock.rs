// pezkuwi/pallets/pez-rewards/src/mock.rs

use crate as pallet_pez_rewards;
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU16, ConstU32, ConstU64, ConstU128, AsEnsureOriginWithArg, Hooks},
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
type Balance = u128;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Assets: pallet_assets,
		Trust: pallet_trust,
		PezRewards: pallet_pez_rewards,
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
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
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

parameter_types! {
	pub const AssetDeposit: Balance = 1;
	pub const ApprovalDeposit: Balance = 1;
	pub const StringLimit: u32 = 50;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
}
impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = ConstU128;
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

pub struct MockTrustScoreProvider;
impl pallet_trust::TrustScoreProvider<u64> for MockTrustScoreProvider {
    // HATA GİDERİLDİ: Her zaman 100 döndür, 0 kenar durumunu kaldır.
    fn trust_score_of(_who: &u64) -> u128 { 100 }
}
impl pallet_trust::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}


parameter_types! {
	pub const PezAssetId: u32 = 1;
	pub const IncentivePotId: PalletId = PalletId(*b"py/incnt");
	pub const ClawbackRecipient: AccountId = 99;
}

impl pallet_pez_rewards::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Assets = Assets;
	type PezAssetId = PezAssetId;
	type WeightInfo = ();
	type TrustScoreSource = MockTrustScoreProvider;
	type IncentivePotId = IncentivePotId;
	type ClawbackRecipient = ClawbackRecipient;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CollectionId = u32;
	type ItemId = u32;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::<Test>::default().build_storage().unwrap();

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