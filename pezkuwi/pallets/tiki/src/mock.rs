use crate as pallet_tiki;
use frame_support::{
	construct_runtime, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU16, ConstU32, ConstU64},
    assert_ok,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use crate::Tiki as TikiEnum;

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;

// Runtime'ı oluştur - Identity ve IdentityKyc pallet'lerini de ekle
construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Identity: pallet_identity::{Pallet, Call, Storage, Event<T>},
		IdentityKyc: pallet_identity_kyc::{Pallet, Call, Storage, Event<T>},
		Nfts: pallet_nfts::{Pallet, Call, Storage, Event<T>},
		Tiki: pallet_tiki::{Pallet, Call, Storage, Event<T>},
	}
);

impl frame_system::Config for Test {
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
	type MaxConsumers = ConstU32<16>;
	type RuntimeTask = ();
	type SingleBlockMigrations = ();
	type MultiBlockMigrator = ();
	type PreInherents = ();
	type PostInherents = ();
	type PostTransactions = (); // Eksik olan trait
	type ExtensionsWeightInfo = ();
}

impl pallet_balances::Config for Test {
	type Balance = Balance;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
	type DoneSlashHandler = ();
}

// pallet_identity::Config implementasyonu
parameter_types! {
	pub const BasicDeposit: Balance = 1000;
	pub const ByteDeposit: Balance = 10;
	pub const SubAccountDeposit: Balance = 100;
	pub const MaxSubAccounts: u32 = 10;
	pub const MaxRegistrars: u32 = 10;
	pub const UsernameDeposit: Balance = 100;
	pub const PendingUsernameExpiration: u64 = 100;
	pub const UsernameGracePeriod: u64 = 50;
	pub const MaxSuffixLength: u32 = 10;
	pub const MaxUsernameLength: u32 = 32;
}

impl pallet_identity::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type BasicDeposit = BasicDeposit;
	type ByteDeposit = ByteDeposit;
	type SubAccountDeposit = SubAccountDeposit;
	type MaxSubAccounts = MaxSubAccounts;
	type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
	type MaxRegistrars = MaxRegistrars;
	type Slashed = ();
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type RegistrarOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type OffchainSignature = sp_runtime::testing::TestSignature;
	type SigningPublicKey = <sp_runtime::testing::TestSignature as sp_runtime::traits::Verify>::Signer;
	type UsernameAuthorityOrigin = frame_system::EnsureRoot<AccountId>;
	type UsernameDeposit = UsernameDeposit;
	type PendingUsernameExpiration = PendingUsernameExpiration;
	type UsernameGracePeriod = UsernameGracePeriod;
	type MaxSuffixLength = MaxSuffixLength;
	type MaxUsernameLength = MaxUsernameLength;
}

parameter_types! {
	pub const MaxAdditionalFields: u32 = 10;
}

// pallet_identity_kyc::Config parameters
parameter_types! {
	pub const KycApplicationDepositAmount: Balance = 100;
	pub const MaxCidLength: u32 = 100;
}

impl pallet_identity_kyc::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = ();
	type KycApprovalOrigin = frame_system::EnsureRoot<AccountId>;
	type KycApplicationDeposit = KycApplicationDepositAmount;
	type MaxStringLength = ConstU32<50>;
	type MaxCidLength = MaxCidLength;
}

parameter_types! {
	pub Features: pallet_nfts::PalletFeatures = pallet_nfts::PalletFeatures::default();
}

impl pallet_nfts::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Locker = ();
	type CollectionDeposit = ConstU128<0>;
	type ItemDeposit = ConstU128<0>;
	type MetadataDepositBase = ConstU128<0>;
	type AttributeDepositBase = ConstU128<0>;
	type DepositPerByte = ConstU128<0>;
	type StringLimit = ConstU32<256>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<256>;
	type ApprovalsLimit = ConstU32<10>;
	type ItemAttributesApprovalsLimit = ConstU32<20>;
	type MaxTips = ConstU32<10>;
	type MaxDeadlineDuration = ConstU64<10000>;
	type MaxAttributesPerCall = ConstU32<10>;
	type Features = Features;
	type OffchainSignature = sp_runtime::testing::TestSignature;
	type OffchainPublic = <Self::OffchainSignature as sp_runtime::traits::Verify>::Signer;
	type WeightInfo = ();
	type BlockNumberProvider = System;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

parameter_types! {
	pub const TikiCollectionId: u32 = 0;
	pub const MaxTikisPerUser: u32 = 100;
}

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type TikiCollectionId = TikiCollectionId;
	type MaxTikisPerUser = MaxTikisPerUser;
	type Tiki = TikiEnum;
}

// Helper functions for tests
pub fn setup_kyc_for_user(account: AccountId) {
    // Give balance to user for deposit
    let _ = Balances::force_set_balance(RuntimeOrigin::root(), account, 10000);

    // Set identity
    assert_ok!(IdentityKyc::set_identity(
        RuntimeOrigin::signed(account),
        b"Test User".to_vec().try_into().unwrap(),
        b"test@example.com".to_vec().try_into().unwrap()
    ));

    // Approve KYC as root
    assert_ok!(IdentityKyc::approve_kyc(
        RuntimeOrigin::root(),
        account
    ));
}

// Legacy function - kept for backwards compatibility
pub fn setup_identity_for_user(account: AccountId) {
    setup_kyc_for_user(account);
}

pub fn advance_blocks(blocks: u64) {
    for _i in 0..blocks {
        let current_block = System::block_number();
        System::set_block_number(current_block + 1);
        // Trigger hooks for the new block
        <pallet_tiki::Pallet<Test> as frame_support::traits::Hooks<u64>>::on_initialize(current_block + 1);
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default()
		.build_storage()
		.unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(1, 10000), (2, 10000), (3, 10000), (4, 10000), (5, 10000)],
		dev_accounts: Default::default(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);

		// Tiki koleksiyonunu oluştur - mint permissions ile
		assert_ok!(Nfts::force_create(
			RuntimeOrigin::root(),
			1, // owner
			pallet_nfts::CollectionConfig {
				settings: pallet_nfts::CollectionSettings::all_enabled(),
				max_supply: None,
				mint_settings: pallet_nfts::MintSettings {
					mint_type: pallet_nfts::MintType::Public,
					price: None,
					start_block: None,
					end_block: None,
					default_item_settings: pallet_nfts::ItemSettings::all_enabled(),
				},
			}
		));
	});
	ext
}