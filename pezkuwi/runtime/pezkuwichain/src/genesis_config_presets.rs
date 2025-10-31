// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Pezkuwi.

//! Genesis configs presets for the PezkuwiChain runtime
//!
//! ## 🎯 NETWORK MODES:
//! - **Dev**: 1 validator (Alice seed) - Local development
//! - **Local**: 2 validators (Alice+Bob seeds) - Local testing  
//! - **Alfa**: 4 validators (test seeds) - Early testing
//! - **Beta**: 8 validators (REAL KEYS from JSON) ⭐
//! - **Staging**: 20 validators (REAL KEYS from JSON) ⭐
//! - **Production**: 100 validators (REAL KEYS from JSON) ⭐
//!
//! ## 💎 TRIPLE TOKEN SYSTEM:
//! - **HEZ** (Native Coin) - Pallet Balances - 200M total
//! - **wHEZ** (Wrapped HEZ) - Pallet Assets (ID: 0) - Minted on wrap
//! - **PEZ** (Utility Token) - Pallet Assets (ID: 1) - 5B total
//!
//! ## 📦 TOKENOMICS:
//!
//! ### HEZ Distribution (200M):
//! - Founder: 10M (5%) + Airdrop: 60M (30%) = 70M
//! - Treasury: 60M (30%) + DevReserve: 10M (5%) = 70M  
//! - Presale: 20M (10%)
//! - Incentives: 20M (10%)
//! - Validators: 2.5M each (20M for 8 validators)
//!
//! ### PEZ Distribution (5B):
//! - Treasury: 4,812,500,000 (96.25%)
//! - Presale: 93,750,000 (1.875%)
//! - Founder: 93,750,000 (1.875%)
//!
//! ## 📦 JSON LOADING:
//! Validator keys loaded from JSON files at compile time:
//! - `runtime/validators/beta_testnet_validators.json` (8 validators)
//! - `runtime/validators/staging_validators.json` (20 validators)
//! - `runtime/validators/mainnet_validators.json` (100 validators)

use alloc::string::ToString;
use sp_runtime::traits::AccountIdConversion;
use sp_core::Pair;
use crate::{
	AssetsConfig, BabeConfig, BalancesConfig, ConfigurationConfig,
	RegistrarConfig, RuntimeGenesisConfig, SessionConfig, SessionKeys, SudoConfig,
	StakingConfig, BABE_GENESIS_EPOCH_CONFIG, PezTreasuryConfig, PezRewardsConfig,
	TokenWrapperPalletId,
};

#[cfg(not(feature = "std"))]
use alloc::format;
use alloc::{vec, vec::Vec};
use alloc::collections::BTreeMap;
use frame_support::build_struct_json_patch;
use pezkuwi_primitives::{AccountId, AssignmentId, SchedulerParams, ValidatorId};
use pezkuwichain_constants::currency::UNITS as HEZ;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::get_public_from_string_or_panic, sr25519};
use sp_genesis_builder::PresetId;
use sp_runtime::Perbill;
use crate::CouncilConfig;
use sp_keyring::Sr25519Keyring;
#[cfg(feature = "runtime-benchmarks")]
use frame_benchmarking::whitelisted_caller;

// ============================================================================
// 📦 COMPILE-TIME JSON LOADING
// ============================================================================

/// Validator keys loaded from JSON at compile time
const BETA_VALIDATORS_JSON: &str = include_str!("../../validators/beta_testnet_validators.json");
const STAGING_VALIDATORS_JSON: &str = include_str!("../../validators/staging_validators.json");
const MAINNET_VALIDATORS_JSON: &str = include_str!("../../validators/mainnet_validators.json");

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn get_authority_keys_from_seed(
	seed: &str,
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
) {
	let keys = get_authority_keys_from_seed_no_beefy(seed);
	(keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, get_from_seed::<BeefyId>(seed))
}

fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ValidatorId, AssignmentId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ValidatorId>(seed),
		get_from_seed::<AssignmentId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn get_account_id_from_seed<TPublic: sp_core::Public>(seed: &str) -> AccountId
where
	AccountId: From<<TPublic::Pair as sp_core::Pair>::Public>,
{
	AccountId::from(get_from_seed::<TPublic>(seed))
}

fn get_from_seed<TPublic: sp_core::Public>(seed: &str) -> <TPublic::Pair as sp_core::Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

fn account_id_from_hex(hex: &str) -> AccountId {
	get_public_from_string_or_panic::<sr25519::Public>(hex).into()
}

fn key_from_hex<TPublic: sp_core::Public>(hex: &str) -> <TPublic::Pair as sp_core::Pair>::Public
where
	<TPublic::Pair as sp_core::Pair>::Public: From<TPublic>,
{
	get_public_from_string_or_panic::<TPublic>(hex)
}

fn parse_validators_from_json(
	json_str: &str,
) -> Vec<(
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ValidatorId,
	AssignmentId,
	AuthorityDiscoveryId,
	BeefyId,
)> {
	let validators_json: serde_json::Value = serde_json::from_str(json_str)
		.expect("Validator JSON must be valid");
	
	let validators_array = validators_json
		.get("beta")
		.or_else(|| validators_json.get("staging"))
		.or_else(|| validators_json.get("mainnet"))
		.and_then(|v| v.as_array())
		.expect("JSON must have 'beta', 'staging', or 'mainnet' array");
	
	validators_array
		.iter()
		.map(|v| {
			(
				account_id_from_hex(v["stash"].as_str().expect("stash required")),
				account_id_from_hex(v["controller"].as_str().expect("controller required")),
				key_from_hex::<BabeId>(v["babe"].as_str().expect("babe required")),
				key_from_hex::<GrandpaId>(v["grandpa"].as_str().expect("grandpa required")),
				key_from_hex::<ValidatorId>(v["para_validator"].as_str().expect("para_validator required")),
				key_from_hex::<AssignmentId>(v["para_assignment"].as_str().expect("para_assignment required")),
				key_from_hex::<AuthorityDiscoveryId>(v["authority_discovery"].as_str().expect("authority_discovery required")),
				key_from_hex::<BeefyId>(v["beefy"].as_str().expect("beefy required")),
			)
		})
		.collect()
}

fn pezkuwichain_session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	para_validator: ValidatorId,
	para_assignment: AssignmentId,
	authority_discovery: AuthorityDiscoveryId,
	beefy: BeefyId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, para_validator, para_assignment, authority_discovery, beefy }
}

fn standard_accounts() -> StandardAccounts {
	use hex_literal::hex;
	
	StandardAccounts {
		founder: hex!["d2fe55a51763b6b8e047c1a644bb2b610df40dff4bae4ccd9c8e3a185d4abc1d"].into(),
		presale: hex!["fc260ea19d6d7cd2cae4a7fc6c323c97f0208597427da14c93f66a83da90d352"].into(),
		treasury: hex!["1cbf1dd9a54cbc28aa9f31085e375456130e449ad4a2babc3a3d5e4fbfb4c816"].into(),
		incentives: hex!["6d6f646c70792f696e6374760000000000000000000000000000000000000000"].into(),
	}
}

struct StandardAccounts {
	founder: AccountId,
	presale: AccountId,
	treasury: AccountId,
	incentives: AccountId,
}

fn default_parachains_host_configuration(
) -> pezkuwi_runtime_parachains::configuration::HostConfiguration<pezkuwi_primitives::BlockNumber>
{
	use pezkuwi_primitives::{
		node_features::FeatureIndex, AsyncBackingParams, MAX_CODE_SIZE, MAX_POV_SIZE,
	};

	pezkuwi_runtime_parachains::configuration::HostConfiguration {
		validation_upgrade_cooldown: 2u32,
		validation_upgrade_delay: 2,
		code_retention_period: 1200,
		max_code_size: MAX_CODE_SIZE,
		max_pov_size: MAX_POV_SIZE,
		max_head_data_size: 32 * 1024,
		max_upward_queue_count: 8,
		max_upward_queue_size: 1024 * 1024,
		max_downward_message_size: 1024 * 1024,
		max_upward_message_size: 50 * 1024,
		max_upward_message_num_per_candidate: 5,
		hrmp_sender_deposit: 0,
		hrmp_recipient_deposit: 0,
		hrmp_channel_max_capacity: 8,
		hrmp_channel_max_total_size: 8 * 1024,
		hrmp_max_parachain_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_parachain_outbound_channels: 4,
		hrmp_max_message_num_per_candidate: 5,
		dispute_period: 6,
		no_show_slots: 2,
		n_delay_tranches: 25,
		needed_approvals: 2,
		relay_vrf_modulo_samples: 2,
		zeroth_delay_tranche_width: 0,
		minimum_validation_upgrade_delay: 5,
		async_backing_params: AsyncBackingParams {
			max_candidate_depth: 3,
			allowed_ancestry_len: 2,
		},
		node_features: bitvec::vec::BitVec::from_element(
			1u8 << (FeatureIndex::ElasticScalingMVP as usize),
		),
		scheduler_params: SchedulerParams {
			lookahead: 2,
			group_rotation_frequency: 20,
			paras_availability_period: 4,
			..Default::default()
		},
		..Default::default()
	}
}

// ============================================================================
// 🎯 MAIN GENESIS FUNCTION - REAL TOKENOMICS
// ============================================================================

/// Real testnet/mainnet genesis with PROPER tokenomics
/// Used for Beta, Staging, Production
fn pezkuwichain_real_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)>,
) -> serde_json::Value {
	let accounts = standard_accounts();
	
	// ============================================================================
	// 💰 HEZ COIN DISTRIBUTION (200M Total)
	// ============================================================================
	const FOUNDER_AMOUNT: u128 = 10_000_000 * HEZ;      // 10M - 5%
	const TREASURY_AMOUNT: u128 = 60_000_000 * HEZ;     // 60M - 30%
	const AIRDROP_AMOUNT: u128 = 60_000_000 * HEZ;      // 60M - 30% (Phase 1: Founder holds)
	const PRESALE_AMOUNT: u128 = 20_000_000 * HEZ;      // 20M - 10%
	const INCENTIVES_AMOUNT: u128 = 20_000_000 * HEZ;   // 20M - 10%
	const DEV_RESERVE: u128 = 10_000_000 * HEZ;         // 10M - 5%
	const VALIDATOR_STAKE: u128 = 2_500_000 * HEZ;      // 2.5M per validator

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();
				
				// Founder + Airdrop (Phase 1: Founder controlled)
				balances.insert(accounts.founder.clone(), FOUNDER_AMOUNT + AIRDROP_AMOUNT);
				// Presale
				balances.insert(accounts.presale.clone(), PRESALE_AMOUNT);
				// Treasury + Dev Reserve
				balances.insert(accounts.treasury.clone(), TREASURY_AMOUNT + DEV_RESERVE);
				// Incentives pallet (for PEZ rewards distribution)
				balances.insert(accounts.incentives.clone(), INCENTIVES_AMOUNT);
				
				// Validators
				for authority in &initial_authorities {
					balances.insert(authority.0.clone(), VALIDATOR_STAKE);
				}
				
				balances.into_iter().collect::<Vec<_>>()
			},
		},
		
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: if initial_authorities.len() >= 4 { 4 } else { 1 },
			stakers: initial_authorities
				.iter()
				.map(|x| (
					x.0.clone(),
					x.1.clone(),
					VALIDATOR_STAKE / 2,
					pallet_staking::StakerStatus::Validator
				))
				.collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			force_era: pallet_staking::Forcing::NotForcing,
			min_nominator_bond: 100 * HEZ,
			min_validator_bond: 1000 * HEZ,
			max_validator_count: Some(100),
			max_nominator_count: Some(10000),
			..Default::default()
		},
		
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (
					x.0.clone(),
					x.1.clone(),
					pezkuwichain_session_keys(
						x.2.clone(),
						x.3.clone(),
						x.4.clone(),
						x.5.clone(),
						x.6.clone(),
						x.7.clone(),
					),
				))
				.collect::<Vec<_>>(),
		},
		
		council: CouncilConfig {
			members: vec![accounts.founder.clone()],
			..Default::default()
		},
		
		babe: BabeConfig {
			epoch_config: BABE_GENESIS_EPOCH_CONFIG,
		},
		
		sudo: SudoConfig {
			key: Some(accounts.founder.clone()),
		},
		
		configuration: ConfigurationConfig {
			config: pezkuwi_runtime_parachains::configuration::HostConfiguration {
				scheduler_params: SchedulerParams {
					max_validators_per_core: Some(1),
					..default_parachains_host_configuration().scheduler_params
				},
				..default_parachains_host_configuration()
			},
		},
		
		registrar: RegistrarConfig {
			next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID
		},
		
		// ============================================================================
		// 💎 TRIPLE TOKEN SYSTEM
		// ============================================================================
		assets: AssetsConfig {
			assets: vec![
				// wHEZ - Asset ID 0
				(0, {
					let wrapper_pallet_id: sp_runtime::AccountId32 = TokenWrapperPalletId::get().into_account_truncating();
					wrapper_pallet_id
				}, true, 1),
				// PEZ - Asset ID 1
				(1, accounts.founder.clone(), true, 1),
			],
			metadata: vec![
				(0, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), 12),
				(1, b"Pez Token".to_vec(), b"PEZ".to_vec(), 12),
			],
			accounts: vec![
				// wHEZ starts at 0 (minted on wrap)
				
				// ============================================================================
				// 💰 PEZ TOKEN DISTRIBUTION (5B Total)
				// ============================================================================
				// Treasury: 96.25% of 5B = 4,812,500,000 PEZ
				(1, accounts.treasury.clone(), 4_812_500_000 * HEZ),
				// Presale: 1.875% of 5B = 93,750,000 PEZ
				(1, accounts.presale.clone(), 93_750_000 * HEZ),
				// Founder: 1.875% of 5B = 93,750,000 PEZ
				(1, accounts.founder.clone(), 93_750_000 * HEZ),
			],
			next_asset_id: Some(2),
		},
		
		pez_treasury: PezTreasuryConfig {
			initialize_treasury: true,
			_phantom: Default::default(),
		},
		
		pez_rewards: PezRewardsConfig {
			start_rewards_system: true,
			_phantom: Default::default(),
		},
	})
}

// ============================================================================
// SIMPLE TESTNET GENESIS (Dev, Local, Alfa)
// ============================================================================

fn pezkuwichain_testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)>,
	root_key: AccountId,
	_: Option<Vec<AccountId>>,
) -> serde_json::Value {
	let accounts = standard_accounts();
	
	// Generous amounts for local testing
	const ENDOWMENT: u128 = 10_000_000_000 * HEZ;       // 10B for test accounts
	const STAKER_ENDOWMENT: u128 = 1_000_000_000 * HEZ; // 1B per validator

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();
				balances.insert(accounts.founder.clone(), ENDOWMENT);
				balances.insert(accounts.presale.clone(), ENDOWMENT);
				balances.insert(accounts.treasury.clone(), ENDOWMENT);
				
				for authority in &initial_authorities {
					balances.entry(authority.0.clone()).or_insert(STAKER_ENDOWMENT);
				}
				balances.into_iter().collect::<Vec<_>>()
			},
		},
		
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), pezkuwichain_session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone(), x.6.clone(), x.7.clone())))
				.collect::<Vec<_>>(),
		},
		
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(root_key.clone()) },
		configuration: ConfigurationConfig { config: default_parachains_host_configuration() },
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
		
		// Triple token for testnets too
		assets: AssetsConfig {
			assets: vec![
				(0, {
					let wrapper_pallet_id: sp_runtime::AccountId32 = TokenWrapperPalletId::get().into_account_truncating();
					wrapper_pallet_id
				}, true, 1),
				(1, root_key.clone(), true, 1),
			],
			metadata: vec![
				(0, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), 12),
				(1, b"Pez Token".to_vec(), b"PEZ".to_vec(), 12),
			],
			accounts: {
				// Equal distribution for testing
				const TOTAL_PEZ_SUPPLY: u128 = 5_000_000_000 * HEZ;
				let pez_per_validator = TOTAL_PEZ_SUPPLY / initial_authorities.len() as u128;
				initial_authorities
					.iter()
					.map(|x| (1, x.0.clone(), pez_per_validator))
					.collect()
			},
			next_asset_id: Some(2),
		},
		
		pez_treasury: PezTreasuryConfig {
			initialize_treasury: true,
			_phantom: Default::default(),
		},
		
		pez_rewards: PezRewardsConfig {
			start_rewards_system: true,
			_phantom: Default::default(),
		},
	})
}

// ============================================================================
// NETWORK PRESETS
// ============================================================================

fn pezkuwichain_development_config_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([get_authority_keys_from_seed("Alice")]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

fn pezkuwichain_local_testnet_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

fn pezkuwichain_alfa_testnet_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

/// Beta Testnet - 8 validators (JSON)
fn pezkuwichain_beta_testnet_genesis() -> serde_json::Value {
	let initial_authorities = parse_validators_from_json(BETA_VALIDATORS_JSON);
	pezkuwichain_real_genesis(initial_authorities)
}

/// Staging - 20 validators (JSON)
fn pezkuwichain_staging_genesis() -> serde_json::Value {
	let initial_authorities = parse_validators_from_json(STAGING_VALIDATORS_JSON);
	pezkuwichain_real_genesis(initial_authorities)
}

/// Production Mainnet - 100 validators (JSON)
fn pezkuwichain_production_config_genesis() -> serde_json::Value {
	let initial_authorities = parse_validators_from_json(MAINNET_VALIDATORS_JSON);
	pezkuwichain_real_genesis(initial_authorities)
}

/// Deprecated
fn pezkuwichain_real_testnet_genesis() -> serde_json::Value {
	pezkuwichain_beta_testnet_genesis()
}

fn versi_local_testnet_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
			get_authority_keys_from_seed("Charlie"),
			get_authority_keys_from_seed("Dave"),
		]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

fn pezkuwichain_staging_testnet_config_genesis() -> serde_json::Value {
	pezkuwichain_staging_genesis()
}

// ============================================================================
// PUBLIC API
// ============================================================================

pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from("dev"),
		PresetId::from("alfa_testnet"),
		PresetId::from("beta_testnet"),
		PresetId::from("staging"),
		PresetId::from("staging_testnet"),
		PresetId::from("versi_local_testnet"),
		PresetId::from("production"),
		PresetId::from("real_testnet"),
	]
}

pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => pezkuwichain_local_testnet_genesis(),
		sp_genesis_builder::DEV_RUNTIME_PRESET | "dev" => pezkuwichain_development_config_genesis(),
		"alfa_testnet" => pezkuwichain_alfa_testnet_genesis(),
		"beta_testnet" => pezkuwichain_beta_testnet_genesis(),
		"staging" => pezkuwichain_staging_genesis(),
		"staging_testnet" => pezkuwichain_staging_testnet_config_genesis(),
		"versi_local_testnet" => versi_local_testnet_genesis(),
		"production" => pezkuwichain_production_config_genesis(),
		"real_testnet" => pezkuwichain_real_testnet_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

#[cfg(feature = "runtime-benchmarks")]
pub fn get_default_benchmark_config() -> serde_json::Value {
	pezkuwichain_development_config_genesis()
}