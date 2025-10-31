// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Pezkuwi.

// Pezkuwi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Pezkuwi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Pezkuwi.  If not, see <http://www.gnu.org/licenses/>.

//! Genesis configs presets for the PezkuwiChain runtime

use alloc::string::ToString;
use sp_runtime::traits::AccountIdConversion;
use crate::{
	AssetsConfig, BabeConfig, BalancesConfig, ConfigurationConfig, PezTreasuryPalletId,
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

/// Helper function to generate stash, controller and session key from seed
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
	(
		keys.0,
		keys.1,
		keys.2,
		keys.3,
		keys.4,
		keys.5,
		keys.6,
		get_public_from_string_or_panic::<BeefyId>(seed),
	)
}

/// Helper function to generate stash, controller and session key from seed
fn get_authority_keys_from_seed_no_beefy(
	seed: &str,
) -> (AccountId, AccountId, BabeId, GrandpaId, ValidatorId, AssignmentId, AuthorityDiscoveryId) {
	(
		get_public_from_string_or_panic::<sr25519::Public>(&format!("{}//stash", seed)).into(),
		get_public_from_string_or_panic::<sr25519::Public>(seed).into(),
		get_public_from_string_or_panic::<BabeId>(seed),
		get_public_from_string_or_panic::<GrandpaId>(seed),
		get_public_from_string_or_panic::<ValidatorId>(seed),
		get_public_from_string_or_panic::<AssignmentId>(seed),
		get_public_from_string_or_panic::<AuthorityDiscoveryId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
	Sr25519Keyring::well_known().map(|x| x.to_account_id()).collect()
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

/// Standard wallet addresses used across ALL networks
/// This ensures consistency and prevents seed phrase loss
fn standard_accounts() -> StandardAccounts {
	use hex_literal::hex;
	
	StandardAccounts {
		// Founder: Satoshi Qazi Muhammed
		founder: hex!["54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"].into(),
		
		// Presale account
		presale: hex!["74d407c722c5a94659400d6258be308e0ec7a131425d347b3426acce4790e914"].into(),
		
		// Treasury account (real wallet, not pallet account)
		treasury: hex!["1cbf1dd9a54cbc28aa9f31085e375456130e449ad4a2babc3a3d5e4fbfb4c816"].into(),
		
		// Incentives pallet account (modlpy/inctv)
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

/// PezkuwiChain testnet genesis (used by local_testnet and development)
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
	endowed_accounts: Option<Vec<AccountId>>,
) -> serde_json::Value {
	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);
	let accounts = standard_accounts();

	const ENDOWMENT: u128 = 10_000_000_000 * HEZ;
	const STAKER_ENDOWMENT: u128 = 1_000_000_000 * HEZ;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();
				for account in &endowed_accounts {
					balances.insert(account.clone(), ENDOWMENT);
				}
				for authority in &initial_authorities {
					balances.entry(authority.0.clone()).or_insert(STAKER_ENDOWMENT);
				}
				balances.into_iter().collect::<Vec<_>>()
			},
		},

		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 1,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.0.clone(), STAKER_ENDOWMENT / 100, pallet_staking::StakerStatus::Validator))
				.collect(),
			invulnerables: vec![],
			slash_reward_fraction: Perbill::from_percent(10),
			force_era: pallet_staking::Forcing::NotForcing,
			min_nominator_bond: 1000 * HEZ,
			min_validator_bond: 1000 * HEZ,
			max_validator_count: Some(100),
			max_nominator_count: Some(1000),
			..Default::default()
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						pezkuwichain_session_keys(
							x.2.clone(),
							x.3.clone(),
							x.4.clone(),
							x.5.clone(),
							x.6.clone(),
							x.7.clone(),
						),
					)
				})
				.collect::<Vec<_>>(),
		},
		council: CouncilConfig {
			members: vec![root_key.clone()],
			..Default::default()
		},
		babe: BabeConfig {
			authorities: initial_authorities
				.iter()
				.map(|x| (x.2.clone(), 1))
				.collect(),
			epoch_config: BABE_GENESIS_EPOCH_CONFIG,
			_config: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key.clone()) },
		configuration: ConfigurationConfig {
			config: pezkuwi_runtime_parachains::configuration::HostConfiguration {
				scheduler_params: SchedulerParams {
					max_validators_per_core: Some(1),
					..default_parachains_host_configuration().scheduler_params
				},
				..default_parachains_host_configuration()
			},
		},
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },

		// PEZ Token Assets Configuration - Triple Token System (HEZ + wHEZ + PEZ)
		assets: AssetsConfig {
			assets: vec![
				(0, {
					let wrapper_pallet_id: sp_runtime::AccountId32 = TokenWrapperPalletId::get().into_account_truncating();
					wrapper_pallet_id
				}, true, 1),  // wHEZ - Asset ID 0
				(1, root_key.clone(), true, 1),  // PEZ - Asset ID 1
			],
			metadata: vec![
				(0, "Wrapped HEZ".into(), "wHEZ".into(), 12),
				(1, "Pez".into(), "PEZ".into(), 12),
			],
			accounts: {
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

fn pezkuwichain_staging_testnet_config_genesis() -> serde_json::Value {
	let accounts = standard_accounts();
	
	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)> = Vec::from([get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")]);

	const ENDOWMENT: u128 = 1_000_000 * HEZ;
	const STASH: u128 = 100 * HEZ;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();
				// Standard accounts get endowment
				balances.insert(accounts.founder.clone(), ENDOWMENT);
				balances.insert(accounts.presale.clone(), ENDOWMENT);
				balances.insert(accounts.treasury.clone(), ENDOWMENT);
				
				// Validators get stash
				for authority in &initial_authorities {
					balances.entry(authority.0.clone()).or_insert(STASH);
				}
				balances.into_iter().collect::<Vec<_>>()
			},
		},
		session: SessionConfig {
			keys: initial_authorities
				.into_iter()
				.map(|x| (x.0.clone(), x.0, pezkuwichain_session_keys(x.2, x.3, x.4, x.5, x.6, x.7)))
				.collect::<Vec<_>>(),
		},
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(accounts.founder.clone()) },
		configuration: ConfigurationConfig { config: default_parachains_host_configuration() },
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
	})
}

fn pezkuwichain_development_config_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([get_authority_keys_from_seed("Alice")]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

//local_testnet
fn pezkuwichain_local_testnet_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([get_authority_keys_from_seed("Alice"), get_authority_keys_from_seed("Bob")]),
		Sr25519Keyring::Alice.to_account_id(),
		None,
	)
}

/// `Versi` is a temporary testnet that uses the same runtime as pezkuwichain.
// versi_local_testnet
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

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	let patch = match id.as_ref() {
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => pezkuwichain_local_testnet_genesis(),
		sp_genesis_builder::DEV_RUNTIME_PRESET => pezkuwichain_development_config_genesis(),
		"staging_testnet" => pezkuwichain_staging_testnet_config_genesis(),
		"versi_local_testnet" => versi_local_testnet_genesis(),
		"production" => pezkuwichain_production_config_genesis(),
		"real_testnet" => pezkuwichain_real_testnet_genesis(),
		"beta_testnet" => pezkuwichain_beta_testnet_genesis(),
		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// Production genesis configuration
fn pezkuwichain_production_config_genesis() -> serde_json::Value {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	let accounts = standard_accounts();

	let initial_authorities: Vec<(
		AccountId, AccountId, BabeId, GrandpaId, ValidatorId,
		AssignmentId, AuthorityDiscoveryId, BeefyId,
	)> = Vec::from([
		(
			hex!["d2fe55a51763b6b8e047c1a644bb2b610df40dff4bae4ccd9c8e3a185d4abc1d"].into(), // Stash
			hex!["1cbf1dd9a54cbc28aa9f31085e375456130e449ad4a2babc3a3d5e4fbfb4c816"].into(), // Controller
			hex!["149d6e00fd8c26c3b69802feb63a70e1cb52bd5127fff4c6547092c408cba679"].unchecked_into(), // Babe
			hex!["02c4b8be50387755b0430e0c1056f8fa9db11e6a9ea9416c36a8941a767c896a"].unchecked_into(), // Grandpa
			hex!["da77ad045773f9a9c71e2c081c96e2b8c7785d2920033135d5fe641e8c28ff0f"].unchecked_into(), // Para Validator
			hex!["3c522a83afc977a71da9407504f828521c03667d0e73c398353064c31134e47d"].unchecked_into(), // Para Assignment
			hex!["367916884f71cf35b502d071a7d037be4f141510f450e27189fa24b9a954fe18"].unchecked_into(), // Authority Discovery
			hex!["030b3676b133e8162eb266ba24362ebcdc08cd3abe90e74da1e69f51392f9b2d6d"].unchecked_into(), // Beefy
		),
		(
			hex!["fc260ea19d6d7cd2cae4a7fc6c323c97f0208597427da14c93f66a83da90d352"].into(), // Stash
			hex!["cc63a2dffad73936d41d98b68511ac9b963e2b605cd0f72e11c0bde3da304326"].into(), // Controller
			hex!["84acb9d8e22fe6ee32e2ea92919e6ee7355c9fc4112bc2ca4b29942ebd986e69"].unchecked_into(), // Babe
			hex!["3f50dd6e1cb2f47d355e6160f63f11cf42660e0fadd8126785314943294b3678"].unchecked_into(), // Grandpa
			hex!["9252b826f5d0d45774c463d91ea5d877b950014473024425bb44d460d3e00470"].unchecked_into(), // Para Validator
			hex!["f25790ae9338110f46945f070aed4353fee5636bc0f2d6c8552ddee3135e014c"].unchecked_into(), // Para Assignment
			hex!["9eb5e8d43c9a5110ed4f80a2c876aadf5892062036bfc4d999e151c58086bb13"].unchecked_into(), // Authority Discovery
			hex!["02536f28c11da1b6c6f195939a9a024ef811635ff04e1a9786c96a37842d051f6e"].unchecked_into(), // Beefy
		),
	]);

	const FOUNDER_ENDOWMENT: u128 = 93_750_000 * HEZ;
	const PRESALE_ENDOWMENT: u128 = 93_750_000 * HEZ;
	const VALIDATOR_STASH: u128 = 10_000 * HEZ;
	const ADMIN_ENDOWMENT: u128 = 1_000 * HEZ;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: vec![
				(accounts.founder.clone(), FOUNDER_ENDOWMENT),
				(accounts.presale.clone(), PRESALE_ENDOWMENT),
				(accounts.treasury.clone(), ADMIN_ENDOWMENT),
			]
			.into_iter()
			.chain(initial_authorities.iter().map(|x| (x.0.clone(), VALIDATOR_STASH)))
			.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 2,
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), VALIDATOR_STASH / 2, pallet_staking::StakerStatus::Validator))
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
				.map(|x| {
					(
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
					)
				})
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
		assets: AssetsConfig {
			assets: vec![
				(0, {
					let wrapper_pallet_id: sp_runtime::AccountId32 = TokenWrapperPalletId::get().into_account_truncating();
					wrapper_pallet_id
				}, true, 1),  // wHEZ - Asset ID 0
				(1, accounts.founder.clone(), true, 1), // PEZ - Asset ID 1
			],
			metadata: vec![
				(0, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), 12),
				(1, b"Pez Token".to_vec(), b"PEZ".to_vec(), 12),
			],
			accounts: vec![
				// wHEZ starts at 0 (minted when wrapped)
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

/// Real testnet genesis configuration - Production-ready incentivized testnet
fn pezkuwichain_real_testnet_genesis() -> serde_json::Value {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	let accounts = standard_accounts();

	// HEZ Token Allocation (200M total)
	const FOUNDER_AMOUNT: u128 = 10_000_000 * HEZ;      // 5% - Founder (Satoshi Qazi Muhammed)
	const TREASURY_AMOUNT: u128 = 60_000_000 * HEZ;     // 30% - Treasury wallet
	const AIRDROP_AMOUNT: u128 = 60_000_000 * HEZ;      // 30% - Airdrop (Phase 1: Founder controlled)
	const PRESALE_AMOUNT: u128 = 20_000_000 * HEZ;      // 10% - Presale
	const INCENTIVES_AMOUNT: u128 = 20_000_000 * HEZ;   // 10% - Ecosystem incentives
	const DEV_RESERVE: u128 = 10_000_000 * HEZ;         // 5% - Development reserve
	const VALIDATOR_STAKE: u128 = 2_500_000 * HEZ;      // 2.5M each (20M total for 8 validators)

	// Phase 1: Airdrop under founder control (temporary)
	// Phase 2: Will migrate to airdrop smart contract
	let airdrop_account = accounts.founder.clone();
	let dev_reserve_account = accounts.treasury.clone(); // Dev reserve = treasury for Phase 1

	let initial_authorities: Vec<(
		AccountId, AccountId, BabeId, GrandpaId, ValidatorId,
		AssignmentId, AuthorityDiscoveryId, BeefyId,
	)> = vec![
		get_authority_keys_from_seed("Alice"),
		get_authority_keys_from_seed("Bob"),
		get_authority_keys_from_seed("Charlie"),
		get_authority_keys_from_seed("Dave"),
		get_authority_keys_from_seed("Eve"),
		get_authority_keys_from_seed("Ferdie"),
		get_authority_keys_from_seed("One"),
		get_authority_keys_from_seed("Two"),
	];

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();

				balances.insert(accounts.founder.clone(), FOUNDER_AMOUNT + AIRDROP_AMOUNT); // Founder + Airdrop (Phase 1)
				balances.insert(accounts.presale.clone(), PRESALE_AMOUNT);
				balances.insert(accounts.treasury.clone(), TREASURY_AMOUNT + DEV_RESERVE); // Treasury + Dev Reserve
				balances.insert(accounts.incentives.clone(), INCENTIVES_AMOUNT);

				for authority in &initial_authorities {
					balances.insert(authority.0.clone(), VALIDATOR_STAKE);
				}

				balances.into_iter().collect::<Vec<_>>()
			},
		},

		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: 4,
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
				.map(|x| {
					(
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
					)
				})
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

		// PEZ Token Configuration - Triple Token System (HEZ + wHEZ + PEZ)
		assets: AssetsConfig {
			assets: vec![
				(0, {
					let wrapper_pallet_id: sp_runtime::AccountId32 = TokenWrapperPalletId::get().into_account_truncating();
					wrapper_pallet_id
				}, true, 1),  // wHEZ - Asset ID 0
				(1, accounts.founder.clone(), true, 1), // PEZ - Asset ID 1
			],
			metadata: vec![
				(0, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), 12),
				(1, b"Pez Token".to_vec(), b"PEZ".to_vec(), 12),
			],
			accounts: vec![
				// wHEZ starts at 0 (minted when wrapped)
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

/// Beta PezkuwiChain Testnet Genesis Configuration  
fn pezkuwichain_beta_testnet_genesis() -> serde_json::Value {
	// Beta testnet uses same config as real testnet
	pezkuwichain_real_testnet_genesis()
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from("staging_testnet"),
		PresetId::from("versi_local_testnet"),
		PresetId::from("production"),
		PresetId::from("real_testnet"),
		PresetId::from("beta_testnet"),
	]
}