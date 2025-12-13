// pezkuwi/pallets/referral/src/mock.rs (Updated for new trustless model)

use crate as pallet_referral;
use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{ConstU128, ConstU32},
};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::BuildStorage;

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type Balance = u128;

// Test accounts
pub const FOUNDER: AccountId = 100;
pub const REFERRER: AccountId = 1;
pub const REFERRED: AccountId = 2;
pub const USER_3: AccountId = 3;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		IdentityKyc: pallet_identity_kyc,
		Referral: pallet_referral,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type Balance = Balance;
	type ExistentialDeposit = ConstU128<1>;
	type AccountStore = System;
}

parameter_types! {
	pub const KycApplicationDepositAmount: Balance = 100;
	pub const MaxStringLen: u32 = 50;
	pub const MaxCidLen: u32 = 128;
	pub const PenaltyPerRevocationAmount: u32 = 3;
}

// Mock implementation for CitizenNftProvider
pub struct MockCitizenNftProvider;
impl pallet_identity_kyc::types::CitizenNftProvider<AccountId> for MockCitizenNftProvider {
	fn mint_citizen_nft(_who: &AccountId) -> sp_runtime::DispatchResult {
		Ok(())
	}

	fn mint_citizen_nft_confirmed(_who: &AccountId) -> sp_runtime::DispatchResult {
		Ok(())
	}

	fn burn_citizen_nft(_who: &AccountId) -> sp_runtime::DispatchResult {
		Ok(())
	}
}

impl pallet_identity_kyc::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type GovernanceOrigin = EnsureRoot<AccountId>;
	type WeightInfo = ();
	type OnKycApproved = Referral; // Referral pallet handles KYC approval hook
	type OnCitizenshipRevoked = Referral; // Referral pallet handles revocation penalty
	type CitizenNftProvider = MockCitizenNftProvider;
	type KycApplicationDeposit = KycApplicationDepositAmount;
	type MaxStringLength = MaxStringLen;
	type MaxCidLength = MaxCidLen;
}

// Default referrer for testing (founder account)
pub struct DefaultReferrerAccount;
impl frame_support::traits::Get<AccountId> for DefaultReferrerAccount {
	fn get() -> AccountId {
		FOUNDER
	}
}

impl pallet_referral::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type DefaultReferrer = DefaultReferrerAccount;
	type PenaltyPerRevocation = PenaltyPerRevocationAmount;
}

/// Build test externalities with founding citizens
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(FOUNDER, 1_000_000),
			(REFERRER, 10_000),
			(REFERRED, 10_000),
			(USER_3, 10_000),
		],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Add founding citizens via genesis config
	pallet_identity_kyc::GenesisConfig::<Test> {
		founding_citizens: vec![
			(FOUNDER, H256::from_low_u64_be(1)),
			(REFERRER, H256::from_low_u64_be(2)),
		],
		_phantom: Default::default(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Build test externalities for penalty tests (needs revoked citizens)
pub fn new_test_ext_with_citizens() -> sp_io::TestExternalities {
	new_test_ext()
}
