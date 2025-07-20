use crate as pallet_staking_score;
use frame_support::{construct_runtime, parameter_types, traits::{ConstU16, ConstU32, ConstU64, Everything}};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
use pezkuwi_primitives::traits::StakingInfoProvider;

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;
pub type BlockNumber = u64;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		StakingScore: pallet_staking_score,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = Everything;
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
}

// Staking bilgilerini test için manuel olarak ayarlamamızı sağlayan mock provider.
#[derive(Default)]
pub struct MockStakingInfoProvider;

parameter_types! {
    // Testlerde kullanılacak sahte staking verilerini tutmak için bir storage.
    pub static MockStakedAmount: Balance = 0;
    pub static MockStakingStartBlock: BlockNumber = 0;
}

impl StakingInfoProvider<AccountId, Balance> for MockStakingInfoProvider {
    fn get_staking_details(who: &AccountId) -> (Balance, BlockNumber) {
        // Test sırasında ayarladığımız değerleri döndürür.
        (MockStakedAmount::get(), MockStakingStartBlock::get())
    }
}

impl pallet_staking_score::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type StakingInfo = MockStakingInfoProvider;
    type WeightInfo = ();
    type Balance = Balance;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}