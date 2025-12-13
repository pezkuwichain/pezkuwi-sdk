use crate as pallet_identity_kyc;
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

// Founding citizen for genesis tests
pub const FOUNDER: AccountId = 100;
pub const CITIZEN_1: AccountId = 1;
pub const CITIZEN_2: AccountId = 2;
pub const APPLICANT: AccountId = 3;

construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		IdentityKyc: pallet_identity_kyc,
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
}

// Mock implementation for OnKycApproved hook
// UPDATED: Now includes referrer parameter
pub struct MockOnKycApproved;
impl crate::types::OnKycApproved<AccountId> for MockOnKycApproved {
	fn on_kyc_approved(_who: &AccountId, _referrer: &AccountId) {
		// No-op for tests - in real runtime this triggers referral pallet
	}
}

// Mock implementation for OnCitizenshipRevoked hook
pub struct MockOnCitizenshipRevoked;
impl crate::types::OnCitizenshipRevoked<AccountId> for MockOnCitizenshipRevoked {
	fn on_citizenship_revoked(_who: &AccountId) {
		// No-op for tests - in real runtime this triggers penalty system
	}
}

// Mock implementation for CitizenNftProvider
pub struct MockCitizenNftProvider;
impl crate::types::CitizenNftProvider<AccountId> for MockCitizenNftProvider {
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

impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type GovernanceOrigin = EnsureRoot<Self::AccountId>;
	type WeightInfo = ();
	type OnKycApproved = MockOnKycApproved;
	type OnCitizenshipRevoked = MockOnCitizenshipRevoked;
	type CitizenNftProvider = MockCitizenNftProvider;
	type KycApplicationDeposit = KycApplicationDepositAmount;
	type MaxStringLength = MaxStringLen;
	type MaxCidLength = MaxCidLen;
}

/// Build test externalities with founding citizens
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![
			(FOUNDER, 1_000_000),
			(CITIZEN_1, 10_000),
			(CITIZEN_2, 10_000),
			(APPLICANT, 10_000),
		],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	// Add founding citizen via genesis config
	pallet_identity_kyc::GenesisConfig::<Test> {
		founding_citizens: vec![
			(FOUNDER, H256::from_low_u64_be(1)),   // Founder is pre-approved
			(CITIZEN_1, H256::from_low_u64_be(2)), // Citizen 1 is pre-approved
		],
		_phantom: Default::default(),
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

/// Build test externalities without founding citizens (for edge case tests)
pub fn new_test_ext_empty() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

	pallet_balances::GenesisConfig::<Test> {
		balances: vec![(FOUNDER, 1_000_000), (CITIZEN_1, 10_000), (APPLICANT, 10_000)],
		..Default::default()
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
