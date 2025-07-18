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
use crate::Tiki as TikiEnum; // TikiEnum'u crate'den import ettik

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64; // Testlerde genellikle u64 kullanılır
pub type Balance = u128; // Bakiyeler için u128

// Runtime'ı oluşturuyoruz. Paletlerin Config<T> yerine sadece temel tanımlarını veriyoruz.
// frame_system zaten GenesisConfig'e sahip olduğu için onun için 'Config<T>' kullanıyoruz.
construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Nfts: pallet_nfts::{Pallet, Call, Storage, Event<T>},
		Tiki: pallet_tiki::{Pallet, Call, Storage, Event<T>},
	}
);

// frame_system::Config implementasyonu
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
	type PostTransactions = ();
	type ExtensionsWeightInfo = ();
}

// pallet_balances::Config implementasyonu
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

parameter_types! {
	pub Features: pallet_nfts::PalletFeatures = pallet_nfts::PalletFeatures::default();
}

// pallet_nfts::Config implementasyonu
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
    // Testte oluşturabildiğimiz tek ID `0` olduğu için sabitimizi `0` yapıyoruz.
	pub const TikiCollectionId: u32 = 0;
    // Anlaştığımız üzere, teknik üst sınırı 100 olarak belirliyoruz.
    pub const MaxTikisPerUser: u32 = 100;
}

// crate::Config implementasyonu
impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = frame_system::EnsureRoot<AccountId>;
	type WeightInfo = ();
	type TikiCollectionId = TikiCollectionId;
	type MaxTikisPerUser = MaxTikisPerUser;
	type ItemId = u32;
	type Tiki = TikiEnum;
}

// Test ortamını başlatan fonksiyon
pub fn new_test_ext() -> sp_io::TestExternalities {
    // Balances GenesisConfig'ini ekleyerek test ortamına başlangıç bakiyeleri sağlıyoruz.
    // Özellikle Root hesabı (genellikle 0 veya 1 olarak kabul edilir) için bakiye olması önemlidir.
    let mut t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(1, 1000), (2, 1000), (3, 1000)], // Testlerde kullanılacak hesaplara bakiye veriyoruz.
        dev_accounts: Default::default(), // `dev_accounts` alanı eklendi
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);

        // Nfts::force_create'i RuntimeOrigin::root() ile çağırıyoruz.
        // Bu, testlerin başında Tiki koleksiyonunun oluşturulduğundan emin olur.
        assert_ok!(Nfts::force_create(
            RuntimeOrigin::root(),
            1, // owner (Bu hesap genellikle testlerde root olarak kullanılır)
            pallet_nfts::CollectionConfig {
                settings: Default::default(),
                max_supply: None,
                mint_settings: Default::default(),
            }
        ));
    });
    ext
}