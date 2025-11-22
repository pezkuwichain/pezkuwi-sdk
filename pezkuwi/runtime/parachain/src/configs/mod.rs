// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

mod xcm_config;

use pezkuwi_sdk::{staging_parachain_info as parachain_info, staging_xcm as xcm, *};
#[cfg(not(feature = "runtime-benchmarks"))]
use pezkuwi_sdk::{staging_xcm_builder as xcm_builder, staging_xcm_executor as xcm_executor};

// Substrate and Pezkuwi dependencies
use cumulus_pallet_parachain_system::RelayNumberMonotonicallyIncreases;
use cumulus_primitives_core::{AggregateMessageOrigin, ParaId};
use frame_support::{
	derive_impl,
	dispatch::DispatchClass,
	parameter_types,
	traits::{
		ConstBool, ConstU32, ConstU64, ConstU8, EitherOfDiverse, TransformOrigin, VariantCountOf,
	},
	weights::{ConstantMultiplier, Weight},
	PalletId,
};
use frame_system::{
	limits::{BlockLength, BlockWeights},
	EnsureRoot,
};
use pallet_xcm::{EnsureXcm, IsVoiceOfBody};
use parachains_common::message_queue::{NarrowOriginToSibling, ParaIdToSibling};
use pezkuwi_runtime_common::{
	xcm_sender::NoPriceForMessageDelivery, BlockHashCount, SlowAdjustingFeeUpdate,
};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::Perbill;
use sp_version::RuntimeVersion;
use xcm::latest::prelude::BodyId;

// Local module imports
use super::{
	weights::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight},
	AccountId, Aura, Balance, Balances, Block, BlockNumber, CollatorSelection, ConsensusHook, Hash,
	MessageQueue, Nonce, PalletInfo, ParachainSystem, Runtime, RuntimeCall, RuntimeEvent,
	RuntimeFreezeReason, RuntimeHoldReason, RuntimeOrigin, RuntimeTask, Session, SessionKeys,
	System, Tiki, WeightToFee, XcmpQueue, AVERAGE_ON_INITIALIZE_RATIO, EXISTENTIAL_DEPOSIT, HOURS,
	MAXIMUM_BLOCK_WEIGHT, MICRO_UNIT, NORMAL_DISPATCH_RATIO, SLOT_DURATION, VERSION,
};
use xcm_config::{RelayLocation, XcmOriginToTransactDispatchOrigin};

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	// This part is copied from Substrate's `bin/node/runtime/src/lib.rs`.
	//  The `RuntimeBlockLength` and `RuntimeBlockWeights` exist here because the
	// `DeletionWeightLimit` and `DeletionQueueDepth` depend on those to parameterize
	// the lazy contract deletion.
	pub RuntimeBlockLength: BlockLength =
		BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
	pub const SS58Prefix: u16 = 42;
}

/// The default types are being injected by [`derive_impl`](`frame_support::derive_impl`) from
/// [`ParaChainDefaultConfig`](`struct@frame_system::config_preludes::ParaChainDefaultConfig`),
/// but overridden as needed.
#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig)]
impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The index type for storing how many extrinsics an account has signed.
	type Nonce = Nonce;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The block type.
	type Block = Block;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = RuntimeBlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

/// Configure the palelt weight reclaim tx.
impl cumulus_pallet_weight_reclaim::Config for Runtime {
	type WeightInfo = ();
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Aura;
	type MinimumPeriod = ConstU64<0>;
	type WeightInfo = ();
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type EventHandler = (CollatorSelection,);
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = ConstU32<50>;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = RuntimeHoldReason;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type FreezeIdentifier = RuntimeFreezeReason;
	type MaxFreezes = VariantCountOf<RuntimeFreezeReason>;
	type DoneSlashHandler = ();
}

parameter_types! {
	/// Relay Chain `TransactionByteFee` / 10
	pub const TransactionByteFee: Balance = 10 * MICRO_UNIT;
}

impl pallet_transaction_payment::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OnChargeTransaction = pallet_transaction_payment::FungibleAdapter<Balances, ()>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = ConstU8<5>;
	type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WeightInfo = ();
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const RelayOrigin: AggregateMessageOrigin = AggregateMessageOrigin::Parent;
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type OutboundXcmpMessageSource = XcmpQueue;
	type DmpQueue = frame_support::traits::EnqueueWithOrigin<MessageQueue, RelayOrigin>;
	type ReservedDmpWeight = ReservedDmpWeight;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberMonotonicallyIncreases;
	type ConsensusHook = ConsensusHook;
	type SelectCore = cumulus_pallet_parachain_system::DefaultCoreSelector<Runtime>;
}

impl parachain_info::Config for Runtime {}

parameter_types! {
	pub MessageQueueServiceWeight: Weight = Perbill::from_percent(35) * RuntimeBlockWeights::get().max_block;
}

impl pallet_message_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	#[cfg(feature = "runtime-benchmarks")]
	type MessageProcessor = pallet_message_queue::mock_helpers::NoopMessageProcessor<
		cumulus_primitives_core::AggregateMessageOrigin,
	>;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type MessageProcessor = xcm_builder::ProcessXcmMessage<
		AggregateMessageOrigin,
		xcm_executor::XcmExecutor<xcm_config::XcmConfig>,
		RuntimeCall,
	>;
	type Size = u32;
	// The XCMP queue pallet is only ever able to handle the `Sibling(ParaId)` origin:
	type QueueChangeHandler = NarrowOriginToSibling<XcmpQueue>;
	type QueuePausedQuery = NarrowOriginToSibling<XcmpQueue>;
	type HeapSize = sp_core::ConstU32<{ 103 * 1024 }>;
	type MaxStale = sp_core::ConstU32<8>;
	type ServiceWeight = MessageQueueServiceWeight;
	type IdleMaxServiceWeight = ();
}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ChannelInfo = ParachainSystem;
	type VersionWrapper = ();
	// Enqueue XCMP messages from siblings for later processing.
	type XcmpQueue = TransformOrigin<MessageQueue, AggregateMessageOrigin, ParaId, ParaIdToSibling>;
	type MaxInboundSuspended = sp_core::ConstU32<1_000>;
	type MaxActiveOutboundChannels = ConstU32<128>;
	type MaxPageSize = ConstU32<{ 1 << 16 }>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = ();
	type PriceForSiblingDelivery = NoPriceForMessageDelivery<ParaId>;
}

parameter_types! {
	pub const Period: u32 = 6 * HOURS;
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
	type SessionManager = CollatorSelection;
	// Essentially just Aura, but let's be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type DisablingStrategy = ();
	type WeightInfo = ();
}

#[docify::export(aura_config)]
impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
	type AllowMultipleBlocksPerSlot = ConstBool<true>;
	type SlotDuration = ConstU64<SLOT_DURATION>;
}

parameter_types! {
	pub const PotId: PalletId = PalletId(*b"PotStake");
	pub const SessionLength: BlockNumber = 6 * HOURS;
	// StakingAdmin pluralistic body.
	pub const StakingAdminBodyId: BodyId = BodyId::Defense;
}

/// We allow root and the StakingAdmin to execute privileged collator selection operations.
pub type CollatorSelectionUpdateOrigin = EitherOfDiverse<
	EnsureRoot<AccountId>,
	EnsureXcm<IsVoiceOfBody<RelayLocation, StakingAdminBodyId>>,
>;

impl pallet_collator_selection::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type UpdateOrigin = CollatorSelectionUpdateOrigin;
	type PotId = PotId;
	type MaxCandidates = ConstU32<100>;
	type MinEligibleCollators = ConstU32<4>;
	type MaxInvulnerables = ConstU32<20>;
	// should be a multiple of session or things will get inconsistent
	type KickThreshold = Period;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_collator_selection::IdentityCollator;
	type ValidatorRegistration = Session;
	type WeightInfo = ();
}

/// Configure the pallet template in pallets/template.
impl pallet_parachain_template::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_parachain_template::weights::SubstrateWeight<Runtime>;
}

// ============================================================================
// PALLET-NFTS (Required for pallet-tiki)
// ============================================================================

use pallet_nfts::PalletFeatures;

parameter_types! {
	pub const NftsCollectionDeposit: Balance = 10 * super::UNIT;
	pub const NftsItemDeposit: Balance = super::UNIT;
	pub const NftsMetadataDepositBase: Balance = super::UNIT;
	pub const NftsAttributeDepositBase: Balance = super::UNIT;
	pub const NftsDepositPerByte: Balance = super::CENTS;
	pub Features: PalletFeatures = PalletFeatures::all_enabled();
}

impl pallet_nfts::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type CollectionId = u32;
	type ItemId = u32;
	type Currency = Balances;
	type ForceOrigin = EnsureRoot<AccountId>;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type Locker = ();
	type CollectionDeposit = NftsCollectionDeposit;
	type ItemDeposit = NftsItemDeposit;
	type MetadataDepositBase = NftsMetadataDepositBase;
	type AttributeDepositBase = NftsAttributeDepositBase;
	type DepositPerByte = NftsDepositPerByte;
	type StringLimit = ConstU32<256>;
	type KeyLimit = ConstU32<64>;
	type ValueLimit = ConstU32<256>;
	type ApprovalsLimit = ConstU32<20>;
	type ItemAttributesApprovalsLimit = ConstU32<20>;
	type MaxTips = ConstU32<10>;
	type MaxDeadlineDuration = ConstU32<{ 12 * 30 * super::DAYS }>;
	type MaxAttributesPerCall = ConstU32<10>;
	type Features = Features;
	type OffchainSignature = super::Signature;
	type OffchainPublic = <super::Signature as sp_runtime::traits::Verify>::Signer;
	type WeightInfo = pallet_nfts::weights::SubstrateWeight<Runtime>;
	type BlockNumberProvider = System;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
}

// ============================================================================
// PALLET-ASSETS CONFIGURATION
// ============================================================================

parameter_types! {
	pub const AssetDeposit: Balance = 100 * super::UNIT;
	pub const ApprovalDeposit: Balance = 1 * super::UNIT;
	pub const AssetsStringLimit: u32 = 50;
	pub const AssetMetadataDepositBase: Balance = 10 * super::UNIT;
	pub const AssetMetadataDepositPerByte: Balance = 1 * super::UNIT;
}

/// Configure pallet-assets (Main Assets Instance)
impl pallet_assets::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type AssetId = u32;
	type AssetIdParameter = u32;
	type Currency = Balances;
	type CreateOrigin = frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId>>;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = AssetDeposit;
	type AssetAccountDeposit = frame_support::traits::ConstU128<0>;
	type MetadataDepositBase = AssetMetadataDepositBase;
	type MetadataDepositPerByte = AssetMetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = AssetsStringLimit;
	type Freezer = ();
	type Extra = ();
	type CallbackHandle = ();
	type WeightInfo = ();
	type Holder = ();
	type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}

// ============================================================================
// CUSTOM PEZKUWICHAIN PALLETS - BATCH 1
// ============================================================================

use sp_core::hex2array;

// Parameter types for pallet-identity-kyc
parameter_types! {
	pub const KycApplicationDepositAmount: Balance = 10 * super::UNIT;
}

// Parameter types for pallet-tiki
parameter_types! {
	pub const TikiCollectionIdConstant: u32 = 42;
	pub const MaxTikisPerUser: u32 = 100;
}

// QaziMuhammedAccount for pallet-referral (and others)
// Using hex representation from pezkuwichain source
#[cfg(not(test))]
parameter_types! {
	pub QaziMuhammedAccount: AccountId = AccountId::new(hex2array!("54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"));
}

#[cfg(test)]
parameter_types! {
	// Alice account for testing
	pub QaziMuhammedAccount: AccountId = AccountId::new(hex2array!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"));
}

/// Configure pallet-tiki
impl pallet_tiki::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type AdminOrigin = EnsureRoot<AccountId>;
	type WeightInfo = pallet_tiki::weights::SubstrateWeight<Runtime>;
	type TikiCollectionId = TikiCollectionIdConstant;
	type MaxTikisPerUser = MaxTikisPerUser;
	type Tiki = pallet_tiki::Tiki;
}

/// Configure pallet-identity-kyc
impl pallet_identity_kyc::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightInfo = pallet_identity_kyc::weights::SubstrateWeight<Runtime>;
	type KycApprovalOrigin = EnsureRoot<AccountId>;
	type KycApplicationDeposit = KycApplicationDepositAmount;
	type MaxStringLength = ConstU32<256>;
	type MaxCidLength = ConstU32<128>;
	type OnKycApproved = pallet_referral::Pallet<Runtime>;
	type CitizenNftProvider = Tiki;
}

/// Configure pallet-referral
impl pallet_referral::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_referral::weights::SubstrateWeight<Runtime>;
	type DefaultReferrer = QaziMuhammedAccount;
}

// ============================================================================
// CUSTOM PEZKUWICHAIN PALLETS - BATCH 2
// ============================================================================

parameter_types! {
	pub const MaxCourseNameLength: u32 = 100;
	pub const MaxCourseDescLength: u32 = 500;
	pub const MaxCourseLinkLength: u32 = 200;
	pub const MaxStudentsPerCourse: u32 = 1000;
}

/// Configure pallet-perwerde (Education Platform)
impl pallet_perwerde::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// TODO: Change to proper governance origin when pallet-collective is added
	type AdminOrigin = frame_system::EnsureSigned<AccountId>;
	type WeightInfo = pallet_perwerde::weights::SubstrateWeight<Runtime>;
	type MaxCourseNameLength = MaxCourseNameLength;
	type MaxCourseDescLength = MaxCourseDescLength;
	type MaxCourseLinkLength = MaxCourseLinkLength;
	type MaxStudentsPerCourse = MaxStudentsPerCourse;
}

// ============================================================================
// CUSTOM PEZKUWICHAIN PALLETS - BATCH 3
// ============================================================================

parameter_types! {
	pub const PresalePalletId: frame_support::PalletId = frame_support::PalletId(*b"py/prsal");
	pub const PlatformFeePercent: u8 = 2;
	pub const MaxContributors: u32 = 10000;
	pub const MaxBonusTiers: u32 = 5;
	pub const MaxWhitelistedAccounts: u32 = 10000;
	// Platform treasury - sudo account for now
	pub PlatformTreasuryAccount: AccountId = QaziMuhammedAccount::get();
	// Staking reward pool account - use sudo for now
	pub StakingRewardPoolAccount: AccountId = QaziMuhammedAccount::get();
}

/// Configure pallet-presale (Multi-Presale Launchpad)
impl pallet_presale::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PalletId = PresalePalletId;
	type PlatformTreasury = PlatformTreasuryAccount;
	type StakingRewardPool = StakingRewardPoolAccount;
	type PlatformFeePercent = PlatformFeePercent;
	type MaxContributors = MaxContributors;
	type MaxBonusTiers = MaxBonusTiers;
	type MaxWhitelistedAccounts = MaxWhitelistedAccounts;
	type CreatePresaleOrigin = frame_system::EnsureSigned<AccountId>;
	type EmergencyOrigin = frame_system::EnsureRoot<AccountId>;
	type PresaleWeightInfo = pallet_presale::weights::SubstrateWeight<Runtime>;
}

// ============================================================================
// CUSTOM PEZKUWICHAIN PALLETS - BATCH 4
// ============================================================================

// pallet-token-wrapper configuration
parameter_types! {
	pub const TokenWrapperPalletId: frame_support::PalletId = frame_support::PalletId(*b"py/wrper");
	pub const WrapperAssetId: u32 = 0;  // wHEZ = Asset ID 0
}

/// Configure pallet-token-wrapper (Native HEZ ↔ wHEZ wrapper)
impl pallet_token_wrapper::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_token_wrapper::weights::SubstrateWeight<Runtime>;
	type Currency = Balances;
	type Assets = crate::Assets;
	type PalletId = TokenWrapperPalletId;
	type WrapperAssetId = WrapperAssetId;
}

// pallet-welati configuration
parameter_types! {
	// Constants for pallet-welati (governance elections)
	pub const ParliamentSize: u32 = 201;
	pub const DiwanSize: u32 = 11;
	pub const ElectionPeriod: BlockNumber = 432_000;  // ~30 days (6s block time)
	pub const CandidacyPeriod: BlockNumber = 86_400;  // ~6 days
	pub const CampaignPeriod: BlockNumber = 259_200;  // ~18 days
	pub const ElectoralDistricts: u32 = 10;
	pub const CandidacyDeposit: Balance = 100 * super::UNIT;
	pub const PresidentialEndorsements: u32 = 100;
	pub const ParliamentaryEndorsements: u32 = 50;
}

// Mock CitizenInfo provider for pallet-welati
pub struct MockCitizenInfo;
impl pallet_welati::CitizenInfo for MockCitizenInfo {
	fn citizen_count() -> u32 {
		// TODO: Integrate with pallet-identity-kyc to get real count
		100_000u32
	}
}

/// Configure pallet-welati (Governance & Elections)
impl pallet_welati::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = (); // TODO: Generate weights via benchmarking
	type Randomness = MockRandomness;
	type RuntimeCall = RuntimeCall;
	type TrustScoreSource = crate::Trust;
	type TikiSource = crate::Tiki;
	type CitizenSource = MockCitizenInfo;
	type KycSource = crate::IdentityKyc;
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

// pallet-staking-score configuration
// Note: Parachain doesn't have pallet-staking, so we'll use a mock provider
pub struct MockStakingInfoProvider;
impl pallet_staking_score::StakingInfoProvider<AccountId, Balance> for MockStakingInfoProvider {
	fn get_staking_details(_who: &AccountId) -> Option<pallet_staking_score::StakingDetails<Balance>> {
		// In parachain, staking is done on relay chain, not locally
		// This is a placeholder - real implementation would query relay chain state
		None
	}
}

/// Configure pallet-staking-score (Staking reputation scoring)
impl pallet_staking_score::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type WeightInfo = pallet_staking_score::weights::SubstrateWeight<Runtime>;
	type StakingInfo = MockStakingInfoProvider;
}

// ============================================================================
// CUSTOM PEZKUWICHAIN PALLETS - BATCH 5
// ============================================================================

// pallet-trust configuration
parameter_types! {
	// Constants for Trust pallet
	pub const TrustScoreMultiplierBase: u128 = 1000;
	pub const TrustUpdateInterval: BlockNumber = 432_000;  // ~30 days (6s block time)
}

// Adapter provider structs for pallet-trust
pub struct CitizenshipProvider;
impl pallet_trust::CitizenshipStatusProvider<AccountId> for CitizenshipProvider {
	fn is_citizen(who: &AccountId) -> bool {
		use pallet_identity_kyc::types::KycLevel;
		pallet_identity_kyc::KycStatuses::<Runtime>::get(who) == KycLevel::Approved
	}
}

pub struct ReferralProvider;
impl pallet_trust::ReferralScoreProvider<AccountId> for ReferralProvider {
	fn get_referral_score(who: &AccountId) -> u32 {
		crate::Referral::referral_count(who)
	}
}

pub struct PerwerdeProvider;
impl pallet_trust::PerwerdeScoreProvider<AccountId> for PerwerdeProvider {
	fn get_perwerde_score(_who: &AccountId) -> u32 {
		// TODO: Implement actual perwerde score getter
		// For now return 0 as placeholder
		0
	}
}

pub struct TikiProvider;
impl pallet_trust::TikiScoreProvider<AccountId> for TikiProvider {
	fn get_tiki_score(_who: &AccountId) -> u32 {
		// TODO: Implement actual tiki score getter
		// For now return 0 as placeholder
		0
	}
}

/// Configure pallet-trust (Unified trust scoring system)
impl pallet_trust::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_trust::weights::SubstrateWeight<Runtime>;
	type Score = u128;
	type ScoreMultiplierBase = TrustScoreMultiplierBase;
	type UpdateInterval = TrustUpdateInterval;
	// Bind Providers
	type StakingScoreSource = crate::StakingScore;
	type ReferralScoreSource = ReferralProvider;
	type PerwerdeScoreSource = PerwerdeProvider;
	type TikiScoreSource = TikiProvider;
	type CitizenshipSource = CitizenshipProvider;
}

// pallet-pez-treasury and pallet-pez-rewards configuration
parameter_types! {
	pub const PezAssetId: u32 = 1; // PEZ Token ID will be 1
	pub const PezTreasuryPalletId: frame_support::PalletId = frame_support::PalletId(*b"py/pztrs");
	pub const PezIncentivePotId: frame_support::PalletId = frame_support::PalletId(*b"py/pzinc");
	pub const PezGovernmentPotId: frame_support::PalletId = frame_support::PalletId(*b"py/pzgov");
	pub PresaleAccount: AccountId = QaziMuhammedAccount::get();
}

/// Configure pallet-pez-treasury (PEZ token treasury management)
impl pallet_pez_treasury::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Assets = crate::Assets;
	type WeightInfo = (); // TODO: Generate weights via benchmarking
	type PezAssetId = PezAssetId;
	type TreasuryPalletId = PezTreasuryPalletId;
	type IncentivePotId = PezIncentivePotId;
	type GovernmentPotId = PezGovernmentPotId;
	type PresaleAccount = PresaleAccount;
	type FounderAccount = QaziMuhammedAccount;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
}

/// Configure pallet-pez-rewards (PEZ reward distribution)
impl pallet_pez_rewards::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Assets = crate::Assets;
	type WeightInfo = pallet_pez_rewards::weights::SubstrateWeight<Runtime>;
	type PezAssetId = PezAssetId;
	type TrustScoreSource = crate::Trust;
	type IncentivePotId = PezIncentivePotId;
	type ClawbackRecipient = QaziMuhammedAccount;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type CollectionId = u32;
	type ItemId = u32;
}

// pallet-validator-pool configuration
parameter_types! {
	// Validator Pool Configuration
	pub const MaxValidatorsPerEra: u32 = 21;
	pub const MaxValidatorPoolSize: u32 = 500;
	pub const MinValidatorStakeAmount: Balance = 10_000 * super::UNIT;
}

// Adapter Structs for ValidatorPool Integration
pub struct ValidatorPoolTrustAdapter;
impl pallet_validator_pool::TrustScoreProvider<AccountId> for ValidatorPoolTrustAdapter {
	fn trust_score_of(who: &AccountId) -> u128 {
		crate::Trust::calculate_trust_score(who).unwrap_or(0)
	}
}

pub struct ValidatorPoolTikiAdapter;
impl pallet_validator_pool::TikiScoreProvider<AccountId> for ValidatorPoolTikiAdapter {
	fn get_tiki_score(_who: &AccountId) -> u32 {
		// TODO: Implement actual tiki score getter
		// For now return 0 as placeholder
		0
	}
}

pub struct ValidatorPoolReferralAdapter;
impl pallet_validator_pool::types::ReferralProvider<AccountId> for ValidatorPoolReferralAdapter {
	fn get_referral_count(who: &AccountId) -> u32 {
		crate::Referral::referral_count(who)
	}
}

pub struct ValidatorPoolPerwerdeAdapter;
impl pallet_validator_pool::types::PerwerdeProvider<AccountId> for ValidatorPoolPerwerdeAdapter {
	fn get_perwerde_score(_who: &AccountId) -> u32 {
		// TODO: Implement actual perwerde score getter
		// For now return 0 as placeholder
		0
	}
}

// Mock Randomness for testing (parachains don't have native randomness without VRF)
pub struct MockRandomness;
impl frame_support::traits::Randomness<sp_core::H256, BlockNumber> for MockRandomness {
	fn random(_subject: &[u8]) -> (sp_core::H256, BlockNumber) {
		// Use system block hash as basic randomness source
		// In production, should use relay chain VRF or similar
		(frame_system::Pallet::<Runtime>::block_hash(
			frame_system::Pallet::<Runtime>::block_number()
		), frame_system::Pallet::<Runtime>::block_number())
	}
}

/// Configure pallet-validator-pool (Validator pool management)
impl pallet_validator_pool::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = pallet_validator_pool::weights::SubstrateWeight<Runtime>;
	type Randomness = MockRandomness;
	type TrustSource = ValidatorPoolTrustAdapter;
	type TikiSource = ValidatorPoolTikiAdapter;
	type ReferralSource = ValidatorPoolReferralAdapter;
	type PerwerdeSource = ValidatorPoolPerwerdeAdapter;
	type PoolManagerOrigin = frame_system::EnsureRoot<AccountId>;
	type MaxValidators = MaxValidatorsPerEra;
	type MaxPoolSize = MaxValidatorPoolSize;
	type MinStakeAmount = MinValidatorStakeAmount;
}

