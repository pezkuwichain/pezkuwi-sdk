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

use crate::{
	BabeConfig, BalancesConfig, ConfigurationConfig, RegistrarConfig, RuntimeGenesisConfig,
	SessionConfig, SessionKeys, SudoConfig, StakingConfig, BABE_GENESIS_EPOCH_CONFIG,
	PezTreasuryConfig, PezRewardsConfig,
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
			max_candidate_depth: 0,
			allowed_ancestry_len: 0,
		},
		node_features: bitvec::vec::BitVec::from_element(
			(1u8 << (FeatureIndex::ElasticScalingMVP as usize)) |
				(1u8 << (FeatureIndex::EnableAssignmentsV2 as usize)) |
				(1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
		),
		scheduler_params: SchedulerParams {
			lookahead: 3,
			group_rotation_frequency: 20,
			paras_availability_period: 4,
			..Default::default()
		},
		..Default::default()
	}
}

#[test]
fn default_parachains_host_configuration_is_consistent() {
	default_parachains_host_configuration().panic_if_not_consistent();
}

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
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
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

// staging_testnet
fn pezkuwichain_staging_testnet_config_genesis() -> serde_json::Value {
	use hex_literal::hex;
	use sp_core::crypto::UncheckedInto;

	let endowed_accounts: Vec<AccountId> = Vec::from([
		hex!["52bc71c1eca5353749542dfdf0af97bf764f9c2f44e860cd485f1cd86400f649"].into(),
	]);

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		BabeId,
		GrandpaId,
		ValidatorId,
		AssignmentId,
		AuthorityDiscoveryId,
		BeefyId,
	)> = Vec::from([
		(
			hex!["62475fe5406a7cb6a64c51d0af9d3ab5c2151bcae982fb812f7a76b706914d6a"].into(),
			hex!["9e6e781a76810fe93187af44c79272c290c2b9e2b8b92ee11466cd79d8023f50"].into(),
			hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"]
				.unchecked_into(),
			hex!["0e6d7d1afbcc6547b92995a394ba0daed07a2420be08220a5a1336c6731f0bfa"]
				.unchecked_into(),
			hex!["0e07a51d3213842f8e9363ce8e444255990a225f87e80a3d651db7841e1a0205"]
				.unchecked_into(),
			hex!["ec60e71fe4a567ef9fef99d4bbf37ffae70564b41aa6f94ef0317c13e0a5477b"]
				.unchecked_into(),
			hex!["f49eae66a0ac9f610316906ec8f1a0928e20d7059d76a5ca53cbcb5a9b50dd3c"]
				.unchecked_into(),
			hex!["034f68c5661a41930c82f26a662276bf89f33467e1c850f2fb8ef687fe43d62276"]
				.unchecked_into(),
		),
		(
			hex!["520b48452969f6ddf263b664de0adb0c729d0e0ad3b0e5f3cb636c541bc9022a"].into(),
			hex!["6618289af7ae8621981ffab34591e7a6486e12745dfa3fd3b0f7e6a3994c7b5b"].into(),
			hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"]
				.unchecked_into(),
			hex!["fcd5f87a6fd5707a25122a01b4dac0a8482259df7d42a9a096606df1320df08d"]
				.unchecked_into(),
			hex!["669a10892119453e9feb4e3f1ee8e028916cc3240022920ad643846fbdbee816"]
				.unchecked_into(),
			hex!["68bf52c482630a8d1511f2edd14f34127a7d7082219cccf7fd4c6ecdb535f80d"]
				.unchecked_into(),
			hex!["f6f8fe475130d21165446a02fb1dbce3a7bf36412e5d98f4f0473aed9252f349"]
				.unchecked_into(),
			hex!["03a90c2bb6d3b7000020f6152fe2e5002fa970fd1f42aafb6c8edda8dacc2ea77e"]
				.unchecked_into(),
		),
		(
			hex!["92ef83665b39d7a565e11bf8d18d41d45a8011601c339e57a8ea88c8ff7bba6f"].into(),
			hex!["b235f57244230589523271c27b8a490922ffd7dccc83b044feaf22273c1dc735"].into(),
			hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"]
				.unchecked_into(),
			hex!["e1b68fbd84333e31486c08e6153d9a1415b2e7e71b413702b7d64e9b631184a1"]
				.unchecked_into(),
			hex!["a8e61ffacafaf546283dc92d14d7cc70ea0151a5dd81fdf73ff5a2951f2b6037"]
				.unchecked_into(),
			hex!["244f3421b310c68646e99cdbf4963e02067601f57756b072a4b19431448c186e"]
				.unchecked_into(),
			hex!["2c57f81fd311c1ab53813c6817fe67f8947f8d39258252663b3384ab4195494d"]
				.unchecked_into(),
			hex!["039d065fe4f9234f0a4f13cc3ae585f2691e9c25afa469618abb6645111f607a53"]
				.unchecked_into(),
		),
		(
			hex!["38f3c2f38f6d47f161e98c697bbe3ca0e47c033460afda0dda314ab4222a0404"].into(),
			hex!["ba0898c1964196474c0be08d364cdf4e9e1d47088287f5235f70b0590dfe1704"].into(),
			hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
				.unchecked_into(),
			hex!["36be9069cdb4a8a07ecd51f257875150f0a8a1be44a10d9d98dabf10a030aef4"]
				.unchecked_into(),
			hex!["882d72965e642677583b333b2d173ac94b5fd6c405c76184bb14293be748a13b"]
				.unchecked_into(),
			hex!["821271c99c958b9220f1771d9f5e29af969edfa865631dba31e1ab7bc0582b75"]
				.unchecked_into(),
			hex!["2496f28d887d84705c6dae98aee8bf90fc5ad10bb5545eca1de6b68425b70f7c"]
				.unchecked_into(),
			hex!["0307d29bbf6a5c4061c2157b44fda33b7bb4ec52a5a0305668c74688cedf288d58"]
				.unchecked_into(),
		),
		(
			hex!["02a2d8cfcf75dda85fafc04ace3bcb73160034ed1964c43098fb1fe831de1b16"].into(),
			hex!["90cab33f0bb501727faa8319f0845faef7d31008f178b65054b6629fe531b772"].into(),
			hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
				.unchecked_into(),
			hex!["6c878e33b83c20324238d22240f735457b6fba544b383e70bb62a27b57380c81"]
				.unchecked_into(),
			hex!["6a8570b9c6408e54bacf123cc2bb1b0f087f9c149147d0005badba63a5a4ac01"]
				.unchecked_into(),
			hex!["16c69ea8d595e80b6736f44be1eaeeef2ac9c04a803cc4fd944364cb0d617a33"]
				.unchecked_into(),
			hex!["306ac5c772fe858942f92b6e28bd82fb7dd8cdd25f9a4626c1b0eee075fcb531"]
				.unchecked_into(),
			hex!["02fb0330356e63a35dd930bc74525edf28b3bf5eb44aab9e9e4962c8309aaba6a6"]
				.unchecked_into(),
		),
		(
			hex!["02ea6bfa8b23b92fe4b5db1063a1f9475e3acd0ab61e6b4f454ed6ba00b5f864"].into(),
			hex!["d4ffc4c05b47d1115ad200f7f86e307b20b46c50e1b72a912ec4f6f7db46b616"].into(),
			hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
				.unchecked_into(),
			hex!["d9c056c98ca0e6b4eb7f5c58c007c1db7be0fe1f3776108f797dd4990d1ccc33"]
				.unchecked_into(),
			hex!["1efc23c0b51ad609ab670ecf45807e31acbd8e7e5cb7c07cf49ee42992d2867c"]
				.unchecked_into(),
			hex!["4c64d3f06d28adeb36a892fdaccecace150bec891f04694448a60b74fa469c22"]
				.unchecked_into(),
			hex!["160ea09c5717270e958a3da42673fa011613a9539b2e4ebcad8626bc117ca04a"]
				.unchecked_into(),
			hex!["020019a8bb188f8145d02fa855e9c36e9914457d37c500e03634b5223aa5702474"]
				.unchecked_into(),
		),
		(
			hex!["fa373e25a1c4fe19c7148acde13bc3db1811cf656dc086820f3dda736b9c4a00"].into(),
			hex!["62145d721967bd88622d08625f0f5681463c0f1b8bcd97eb3c2c53f7660fd513"].into(),
			hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
				.unchecked_into(),
			hex!["4bea0b37e0cce9bddd80835fa2bfd5606f5dcfb8388bbb10b10c483f0856cf14"]
				.unchecked_into(),
			hex!["042f07fc5268f13c026bbe199d63e6ac77a0c2a780f71cda05cee5a6f1b3f11f"]
				.unchecked_into(),
			hex!["fab485e87ed1537d089df521edf983a777c57065a702d7ed2b6a2926f31da74f"]
				.unchecked_into(),
			hex!["64d59feddb3d00316a55906953fb3db8985797472bd2e6c7ea1ab730cc339d7f"]
				.unchecked_into(),
			hex!["033f1a6d47fe86f88934e4b83b9fae903b92b5dcf4fec97d5e3e8bf4f39df03685"]
				.unchecked_into(),
		),
		(
			hex!["8062e9c21f1d92926103119f7e8153cebdb1e5ab3e52d6f395be80bb193eab47"].into(),
			hex!["fa0388fa88f3f0cb43d583e2571fbc0edad57dff3a6fd89775451dd2c2b8ea00"].into(),
			hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
				.unchecked_into(),
			hex!["4ee66173993dd0db5d628c4c9cb61a27b76611ad3c3925947f0d0011ee2c5dcc"]
				.unchecked_into(),
			hex!["d822d4088b20dca29a580a577a97d6f024bb24c9550bebdfd7d2d18e946a1c7d"]
				.unchecked_into(),
			hex!["481538f8c2c011a76d7d57db11c2789a5e83b0f9680dc6d26211d2f9c021ae4c"]
				.unchecked_into(),
			hex!["4e262811acdfe94528bfc3c65036080426a0e1301b9ada8d687a70ffcae99c26"]
				.unchecked_into(),
			hex!["025e84e95ed043e387ddb8668176b42f8e2773ddd84f7f58a6d9bf436a4b527986"]
				.unchecked_into(),
		),
	]);

	const ENDOWMENT: u128 = 1_000_000 * HEZ;
	const STASH: u128 = 100 * HEZ;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: {
				let mut balances = BTreeMap::new();
				for account in &endowed_accounts {
					balances.insert(account.clone(), ENDOWMENT);
				}
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
		sudo: SudoConfig { key: Some(endowed_accounts[0].clone()) },
		configuration: ConfigurationConfig { config: default_parachains_host_configuration() },
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
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

#[cfg(not(feature = "runtime-benchmarks"))]
fn pezkuwichain_development_config_genesis() -> serde_json::Value {
	pezkuwichain_testnet_genesis(
		Vec::from([get_authority_keys_from_seed("Alice")]),
		Sr25519Keyring::Alice.to_account_id(),
		Some(testnet_accounts()),
	)
}

#[cfg(feature = "runtime-benchmarks")]
fn pezkuwichain_development_config_genesis() -> serde_json::Value {
	use frame_benchmarking::account;
	const SEED: u32 = 0;

	let mut endowed_accounts = testnet_accounts();
	endowed_accounts.push(whitelisted_caller());
	endowed_accounts.push(account("admin", 0, SEED));

	for i in 0..50 {
		endowed_accounts.push(account("benchmark", i, SEED));
	}

	let initial_authorities: Vec<_> = (0..4)
		.map(|i| get_authority_keys_from_seed(&format!("//Validator{}", i)))
		.collect();

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
		    members: vec![],
		    ..Default::default()
		},
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(endowed_accounts[0].clone()) },
		configuration: ConfigurationConfig {
			config: default_parachains_host_configuration(),
		},
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
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

    let endowed_accounts: Vec<AccountId> = Vec::from([
        hex!["54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"].into(), // Founder account
        hex!["74d407c722c5a94659400d6258be308e0ec7a131425d347b3426acce4790e914"].into(), // Presale account
        hex!["54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"].into(), // Treasury admin
        hex!["54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"].into(), // DAO multisig
    ]);

    const FOUNDER_ENDOWMENT: u128 = 93_750_000 * HEZ;
    const PRESALE_ENDOWMENT: u128 = 93_750_000 * HEZ;
    const VALIDATOR_STASH: u128 = 10_000 * HEZ;
    const ADMIN_ENDOWMENT: u128 = 1_000 * HEZ;

    build_struct_json_patch!(RuntimeGenesisConfig {
        balances: BalancesConfig {
            balances: vec![
                (endowed_accounts[0].clone(), FOUNDER_ENDOWMENT),
                (endowed_accounts[1].clone(), PRESALE_ENDOWMENT),
                (endowed_accounts[2].clone(), ADMIN_ENDOWMENT),
                (endowed_accounts[3].clone(), ADMIN_ENDOWMENT),
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
                        pezkuwichain_session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone(), x.6.clone(), x.7.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        council: CouncilConfig {
            members: vec![endowed_accounts[0].clone()],
            ..Default::default()
        },
        babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
        sudo: SudoConfig { key: Some(endowed_accounts[0].clone()) },
        configuration: ConfigurationConfig {
            config: default_parachains_host_configuration(),
        },
        registrar: RegistrarConfig {
            next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID
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

/// Validator Pool Genesis Configuration
#[derive(Default, Clone)]
pub struct ValidatorPoolConfig {
    pub max_pool_size: u32,
    pub era_length_blocks: u32,
    pub initial_era: u32,
}

/// Real PezkuwiChain Testnet Genesis Configuration
/// Token economics: 200M HEZ initial supply + infinite inflation
fn pezkuwichain_real_testnet_genesis() -> serde_json::Value {
    use hex_literal::hex;
    use sp_core::crypto::UncheckedInto;

    const FOUNDER_AMOUNT: u128 = 10_000_000 * HEZ;      // 5% - 4 year vesting
    const TREASURY_AMOUNT: u128 = 60_000_000 * HEZ;     // 30% - Government pot 
    const AIRDROP_AMOUNT: u128 = 60_000_000 * HEZ;      // 30% - Trust score + Hemwelatî holders
    const PRESALE_AMOUNT: u128 = 20_000_000 * HEZ;      // 10% - Early investors
    const INCENTIVES_AMOUNT: u128 = 20_000_000 * HEZ;   // 10% - Ecosystem incentives
    const DEV_RESERVE: u128 = 10_000_000 * HEZ;         // 5% - Development reserve

    let founder_account: AccountId = hex!["54581177449f8ab246e300fc76bd9ce21bdab84f23fdc98a9b06a46979318d50"].into();
    let presale_account: AccountId = hex!["74d407c722c5a94659400d6258be308e0ec7a131425d347b3426acce4790e914"].into();
	let treasury_account: AccountId = hex!["6d6f646c70792f74727372790000000000000000000000000000000000000000"].into();
	let airdrop_account: AccountId = hex!["6d6f646c70792f61697264727000000000000000000000000000000000000000"].into();
    let incentives_account: AccountId = hex!["6d6f646c70792f696e6374760000000000000000000000000000000000000000"].into();
    let dev_reserve_account: AccountId = hex!["6d6f646c70792f64657672737600000000000000000000000000000000000000"].into();

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

    const VALIDATOR_STAKE: u128 = 2_500_000 * HEZ; // 2.5M HEZ per validator (20M / 8)

    build_struct_json_patch!(RuntimeGenesisConfig {
        balances: BalancesConfig {
            balances: {
                let mut balances = BTreeMap::new();
                
                balances.insert(founder_account.clone(), FOUNDER_AMOUNT);
                balances.insert(presale_account.clone(), PRESALE_AMOUNT);
                balances.insert(treasury_account.clone(), TREASURY_AMOUNT);
                balances.insert(airdrop_account.clone(), AIRDROP_AMOUNT);
                balances.insert(incentives_account.clone(), INCENTIVES_AMOUNT);
                balances.insert(dev_reserve_account.clone(), DEV_RESERVE);
                
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
            members: vec![founder_account.clone()],
            ..Default::default()
        },

        babe: BabeConfig { 
            epoch_config: BABE_GENESIS_EPOCH_CONFIG,
        },

        sudo: SudoConfig { 
            key: Some(founder_account.clone()),
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

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	vec![
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from("staging_testnet"),
		PresetId::from("versi_local_testnet"),
		PresetId::from("production"),
		PresetId::from("real_testnet"),
	]
}