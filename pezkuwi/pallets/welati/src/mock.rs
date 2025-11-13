use crate::{self as pallet_welati, *};
use frame_support::{
    assert_ok, construct_runtime, derive_impl, parameter_types,
    traits::{ConstU32, ConstU64, ConstU128, Everything, Randomness, AsEnsureOriginWithArg},
    BoundedVec,
};
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
type AccountId = u64;
type Balance = u128;

// Runtime with pallet-identity included for pallet-tiki dependency
construct_runtime!(
    pub enum Test {
        System: frame_system,
        Balances: pallet_balances,
        Timestamp: pallet_timestamp,
        Nfts: pallet_nfts,
        Identity: pallet_identity,
        IdentityKyc: pallet_identity_kyc,
        Tiki: pallet_tiki,
        Trust: pallet_trust,
        StakingScore: pallet_staking_score,
        Referral: pallet_referral,
        Welati: pallet_welati,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
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
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type DoneSlashHandler = ();
}

impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = ConstU64<1>;
    type WeightInfo = ();
}

// Mock Randomness - SADECE BİR KEZ TANIMLA
pub struct MockRandomness;
impl Randomness<H256, u64> for MockRandomness {
    fn random(_subject: &[u8]) -> (H256, u64) {
        (H256::default(), 0)
    }
}

// NFTs Configuration
parameter_types! {
    pub const CollectionDeposit: Balance = 0;
    pub const ItemDeposit: Balance = 0;
    pub const StringLimit: u32 = 64;
    pub const KeyLimit: u32 = 32;
    pub const ValueLimit: u32 = 64;
    pub const ApprovalsLimit: u32 = 1;
    pub const ItemAttributesApprovalsLimit: u32 = 1;
    pub const MaxTips: u32 = 1;
    pub const MaxDeadlineDuration: u64 = 1000;
    pub const MaxAttributesPerCall: u32 = 1;
}

impl pallet_nfts::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type CollectionId = u32;
    type ItemId = u32;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type Locker = ();
    type CollectionDeposit = CollectionDeposit;
    type ItemDeposit = ItemDeposit;
    type MetadataDepositBase = ConstU128<0>;
    type AttributeDepositBase = ConstU128<0>;
    type DepositPerByte = ConstU128<0>;
    type StringLimit = StringLimit;
    type KeyLimit = KeyLimit;
    type ValueLimit = ValueLimit;
    type ApprovalsLimit = ApprovalsLimit;
    type ItemAttributesApprovalsLimit = ItemAttributesApprovalsLimit;
    type MaxTips = MaxTips;
    type MaxDeadlineDuration = MaxDeadlineDuration;
    type MaxAttributesPerCall = MaxAttributesPerCall;
    type Features = ();
    type OffchainSignature = sp_runtime::testing::TestSignature;
    type OffchainPublic = sp_runtime::testing::UintAuthorityId;
    type WeightInfo = ();
    type BlockNumberProvider = System;
}

// Identity Configuration - MINIMAL for pallet-tiki dependency
parameter_types! {
    pub const BasicDeposit: Balance = 10;
    pub const ByteDeposit: Balance = 1;
    pub const SubAccountDeposit: Balance = 10;
    pub const MaxSubAccounts: u32 = 2;
    pub const MaxRegistrars: u32 = 2;
    pub const MaxAdditionalFields: u32 = 2;
    pub const UsernameDeposit: Balance = 100;
    pub const MaxUsernameLength: u32 = 32;
    pub const MaxSuffixLength: u32 = 7;
    pub const PendingUsernameExpiration: u64 = 100;
    pub const UsernameGracePeriod: u64 = 100;
}

impl pallet_identity::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Slashed = ();
    type ForceOrigin = frame_system::EnsureRoot<AccountId>;
    type RegistrarOrigin = frame_system::EnsureRoot<AccountId>;
    type WeightInfo = ();
    type BasicDeposit = BasicDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxRegistrars = MaxRegistrars;
    type IdentityInformation = pallet_identity::legacy::IdentityInfo<MaxAdditionalFields>;
    type ByteDeposit = ByteDeposit;
    type UsernameDeposit = UsernameDeposit;
    type MaxUsernameLength = MaxUsernameLength;
    type MaxSuffixLength = MaxSuffixLength;
    type PendingUsernameExpiration = PendingUsernameExpiration;
    type UsernameGracePeriod = UsernameGracePeriod;
    type UsernameAuthorityOrigin = frame_system::EnsureRoot<AccountId>;
    type OffchainSignature = sp_runtime::testing::TestSignature;
    type SigningPublicKey = sp_runtime::testing::UintAuthorityId;
}

// Identity KYC Configuration
parameter_types! {
    pub const KycApplicationDeposit: Balance = 1_000;
    pub const MaxStringLength: u32 = 128;
    pub const MaxCidLength: u32 = 64;
}

pub struct NoOpOnKycApproved;
impl pallet_identity_kyc::types::OnKycApproved<AccountId> for NoOpOnKycApproved {
	fn on_kyc_approved(_who: &AccountId) {}
}

pub struct NoOpCitizenNftProvider;
impl pallet_identity_kyc::types::CitizenNftProvider<AccountId> for NoOpCitizenNftProvider {
	fn mint_citizen_nft(_who: &AccountId) -> Result<(), sp_runtime::DispatchError> {
		Ok(())
	}
}

impl pallet_identity_kyc::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type KycApprovalOrigin = frame_system::EnsureRoot<AccountId>;
    type WeightInfo = ();
	type OnKycApproved = NoOpOnKycApproved;
	type CitizenNftProvider = NoOpCitizenNftProvider;
    type KycApplicationDeposit = KycApplicationDeposit;
    type MaxStringLength = MaxStringLength;
    type MaxCidLength = MaxCidLength;
}

// Mock StakingInfo provider - SADECE BİR KEZ TANIMLA
pub struct MockStakingInfo;
impl pallet_staking_score::StakingInfoProvider<AccountId, Balance> for MockStakingInfo {
    fn get_staking_details(_account: &AccountId) -> Option<pallet_staking_score::StakingDetails<Balance>> {
        Some(pallet_staking_score::StakingDetails {
            staked_amount: 1000u128,
            nominations_count: 0,
            unlocking_chunks_count: 0,
        })
    }
}

// Staking Score Configuration
impl pallet_staking_score::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Balance = Balance;
    type StakingInfo = MockStakingInfo;
}

// Referral Configuration
impl pallet_referral::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// Tiki Configuration
parameter_types! {
    pub const MaxTikisPerUser: u32 = 50;
    pub const TikiCollectionId: u32 = 0;
}

impl pallet_tiki::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AdminOrigin = frame_system::EnsureRoot<AccountId>;
    type WeightInfo = ();
    type MaxTikisPerUser = MaxTikisPerUser;
    type Tiki = pallet_tiki::Tiki;
    type TikiCollectionId = TikiCollectionId;    
}

// Mock implementations for required traits - YÜKSEK SKORLAR VER
pub struct MockStakingScoreProvider;
impl pallet_staking_score::StakingScoreProvider<AccountId, u64> for MockStakingScoreProvider {
    fn get_staking_score(_account: &AccountId) -> (u32, u64) {
        (1000, 0) // Yüksek skor
    }
}

pub struct MockReferralScoreProvider;
impl pallet_trust::ReferralScoreProvider<AccountId> for MockReferralScoreProvider {
    fn get_referral_score(_account: &AccountId) -> u32 {
        500 // Yüksek skor
    }
}

pub struct MockPerwerdeScoreProvider;
impl pallet_trust::PerwerdeScoreProvider<AccountId> for MockPerwerdeScoreProvider {
    fn get_perwerde_score(_account: &AccountId) -> u32 {
        750 // Yüksek skor
    }
}

pub struct MockTikiScoreProvider;

// `pallet_trust` için implementasyon
impl pallet_trust::TikiScoreProvider<AccountId> for MockTikiScoreProvider {
    fn get_tiki_score(_account: &AccountId) -> u32 {
        100
    }
}

// `pallet_welati`'nin ihtiyaç duyduğu `pallet_tiki` için implementasyon
impl pallet_tiki::TikiScoreProvider<AccountId> for MockTikiScoreProvider {
    fn get_tiki_score(_account: &AccountId) -> u32 {
        1000 // Yüksek Tiki score - tüm kontrolleri geçer
    }
}

pub struct MockCitizenshipStatusProvider;
impl pallet_trust::CitizenshipStatusProvider<AccountId> for MockCitizenshipStatusProvider {
    fn is_citizen(_account: &AccountId) -> bool {
        true // Herkes vatandaş
    }
}

// MOCK TRUST PROVIDER - HERKES İÇİN YÜKSEK SKOR
pub struct MockTrustProvider;
impl pallet_trust::TrustScoreProvider<AccountId> for MockTrustProvider {
    fn trust_score_of(_account: &AccountId) -> u128 {
        1000u128 // Herkes için yüksek trust score
    }
}

// CitizenInfo trait implementation for MockTrustProvider
impl CitizenInfo for MockTrustProvider {
    fn citizen_count() -> u32 {
        110
    }
}

// Trust Configuration
parameter_types! {
    pub const ScoreMultiplierBase: u128 = 100;
    pub const UpdateInterval: u64 = 1000;
}

impl pallet_trust::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type Score = u128;
    type ScoreMultiplierBase = ScoreMultiplierBase;
    type UpdateInterval = UpdateInterval;
    type StakingScoreSource = MockStakingScoreProvider;
    type ReferralScoreSource = MockReferralScoreProvider;
    type PerwerdeScoreSource = MockPerwerdeScoreProvider;
    type TikiScoreSource = MockTikiScoreProvider;
    type CitizenshipSource = MockCitizenshipStatusProvider;
}

// Welati Configuration - SADECE BİR KEZ TANIMLA
parameter_types! {
    pub const ParliamentSize: u32 = 201;
    pub const DiwanSize: u32 = 11;
    pub const ElectionPeriod: u64 = 432_000;
    pub const CandidacyPeriod: u64 = 86_400;
    pub const CampaignPeriod: u64 = 259_200;
    pub const ElectoralDistricts: u32 = 10;
    pub const CandidacyDeposit: u128 = 10_000;
    pub const PresidentialEndorsements: u32 = 100;
    pub const ParliamentaryEndorsements: u32 = 50;
}

impl pallet_welati::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = crate::weights::WeightInfo<Test>;
    type Randomness = MockRandomness;
    type RuntimeCall = RuntimeCall;
    type TrustScoreSource = MockTrustProvider; // Mock provider kullan
    type TikiSource = MockTikiScoreProvider; // Mock Tiki provider kullan
    type CitizenSource = MockTrustProvider; // Mock provider kullan
    type KycSource = IdentityKyc;
    type ParliamentSize = ParliamentSize;
    type DiwanSize = DiwanSize;
    type ElectionPeriod = ElectionPeriod;
    type CandidacyPeriod = CandidacyPeriod;
    type CampaignPeriod = CampaignPeriod;
    type ElectoralDistricts = ElectoralDistricts;
    type CandidacyDeposit = CandidacyDeposit;
    type PresidentialEndorsements = PresidentialEndorsements;
    type ParliamentaryEndorsements = ParliamentaryEndorsements;
}

// CRITICAL: CitizenInfo trait implementation - SADECE BİR KEZ TANIMLA
impl CitizenInfo for Trust {
    fn citizen_count() -> u32 {
        110
    }
}

// Test externalities builder
pub struct ExtBuilder {
    balances: Vec<(AccountId, Balance)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        Self {
            balances: (1..=110)
                .map(|i| (i as AccountId, 100_000_000_000_000))
                .collect(),
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.balances,
            dev_accounts: None,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| {
            System::set_block_number(1);

            assert_ok!(Nfts::create(
                RuntimeOrigin::signed(1),
                1,
                Default::default()
            ));

            setup_test_users();
        });
        ext
    }
}

// SIMPLIFIED TEST USER SETUP - BOŞ BIRAK, MOCK PROVIDERS YETERLI
pub fn setup_test_users() {
    // Mock provider'lar zaten herkesin yüksek trust score'u olmasını sağlıyor
    // ve TikiScoreProvider da herkesin Tiki'ye sahip olduğunu söylüyor
    // Bu sayede pallet-tiki ile uğraşmak zorunda kalmıyoruz
    
    // Sadece NFTs collection'ı oluşturuldu, bu yeterli
    // Testlerde KYC kontrolü zaten bypass ediliyor
}

// CRITICAL HELPER FUNCTION FOR TESTS
pub fn add_parliament_member(account: AccountId) {
    let member = ParliamentMember {
        account,
        elected_at: System::block_number(),
        term_ends_at: System::block_number() + 100_000,
        votes_participated: 0,
        total_votes_eligible: 0,
        participation_rate: 100,
        committees: BoundedVec::default(),
    };

    let mut members = ParliamentMembers::<Test>::get();
    if members.try_push(member).is_ok() {
        ParliamentMembers::<Test>::put(members);
    }
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        if System::block_number() > 0 {
            System::on_finalize(System::block_number());
            Welati::on_finalize(System::block_number());
        }
        System::set_block_number(System::block_number() + 1);
        Welati::on_initialize(System::block_number());
        System::on_initialize(System::block_number());
    }
}

pub fn last_event() -> RuntimeEvent {
    System::events().pop().expect("Event expected").event
}

pub fn events() -> Vec<RuntimeEvent> {
    let evt = System::events().into_iter().map(|evt| evt.event).collect::<Vec<_>>();
    System::reset_events();
    evt
}