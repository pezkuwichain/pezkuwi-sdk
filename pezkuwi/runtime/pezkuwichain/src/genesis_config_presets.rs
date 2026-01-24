// Copyright (C) Parity Technologies (UK) Ltd. and Dijital Kurdistan Tech Institute
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

//! Genesis configs presets for the Pezkuwichain runtime
//!
//! This module contains genesis configuration for:
//! - HEZ token initial distribution (200M genesis supply)
//! - Validator session keys
//! - Initial balance distributions
//!
//! ## HEZ Genesis Distribution (200M Total)
//! - 10% Founder: 20,000,000 HEZ
//! - 50% Presale: 100,000,000 HEZ
//! - 20% Kurdistan Treasury: 40,000,000 HEZ
//! - 20% Airdrop: 40,000,000 HEZ

use crate::{
	BabeConfig, BalancesConfig, ConfigurationConfig, RegistrarConfig, RuntimeGenesisConfig,
	SessionConfig, SessionKeys, SudoConfig, BABE_GENESIS_EPOCH_CONFIG,
};
#[cfg(not(feature = "std"))]
use alloc::format;
use alloc::{vec, vec::Vec};
use pezframe_support::build_struct_json_patch;
use pezkuwi_primitives::{AccountId, AssignmentId, SchedulerParams, ValidatorId};
use pezkuwichain_runtime_constants::currency::UNITS as TYR;
use pezsp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use pezsp_consensus_babe::AuthorityId as BabeId;
use pezsp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use pezsp_consensus_grandpa::AuthorityId as GrandpaId;
use pezsp_core::{crypto::get_public_from_string_or_panic, sr25519};
use pezsp_genesis_builder::PresetId;
use pezsp_keyring::Sr25519Keyring;

// ============================================================================
// HEZ TOKEN GENESIS CONSTANTS (Total Supply: 200 Million HEZ)
// ============================================================================

/// Founder allocation: 10% = 20,000,000 HEZ
pub const HEZ_FOUNDER_ALLOCATION: u128 = 20_000_000 * TYR;

/// Presale allocation: 50% = 100,000,000 HEZ
pub const HEZ_PRESALE_ALLOCATION: u128 = 100_000_000 * TYR;

/// Kurdistan Treasury allocation: 20% = 40,000,000 HEZ
pub const HEZ_TREASURY_ALLOCATION: u128 = 40_000_000 * TYR;

/// Airdrop allocation: 20% = 40,000,000 HEZ
pub const HEZ_AIRDROP_ALLOCATION: u128 = 40_000_000 * TYR;

// ===========================================================================
// COMPILE-TIME VALIDATION: Ensure allocations sum to 200M genesis supply
// ===========================================================================
const _: () = assert!(
	HEZ_FOUNDER_ALLOCATION
		+ HEZ_PRESALE_ALLOCATION
		+ HEZ_TREASURY_ALLOCATION
		+ HEZ_AIRDROP_ALLOCATION
		== 200_000_000 * TYR,
	"HEZ allocations MUST sum to genesis supply (200M)"
);

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

fn default_teyrchains_host_configuration(
) -> pezkuwi_runtime_teyrchains::configuration::HostConfiguration<pezkuwi_primitives::BlockNumber> {
	use pezkuwi_primitives::{
		node_features::FeatureIndex, AsyncBackingParams, MAX_CODE_SIZE, MAX_POV_SIZE,
	};

	pezkuwi_runtime_teyrchains::configuration::HostConfiguration {
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
		hrmp_max_teyrchain_inbound_channels: 4,
		hrmp_channel_max_message_size: 1024 * 1024,
		hrmp_max_teyrchain_outbound_channels: 4,
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
			(1u8 << (FeatureIndex::ElasticScalingMVP as usize))
				| (1u8 << (FeatureIndex::EnableAssignmentsV2 as usize))
				| (1u8 << (FeatureIndex::CandidateReceiptV2 as usize)),
		),
		scheduler_params: SchedulerParams {
			lookahead: 3,
			group_rotation_frequency: 20,
			paras_availability_period: 4,
			// num_cores: 0 olmalı çünkü assign_coretime() genesis'te
			// her teyrchain için otomatik olarak artırır
			num_cores: 0,
			..Default::default()
		},
		..Default::default()
	}
}

#[test]
fn default_teyrchains_host_configuration_is_consistent() {
	default_teyrchains_host_configuration().panic_if_not_consistent();
}

#[test]
fn hez_allocations_sum_to_200m() {
	// Runtime validation that allocations sum to 200M
	let total = HEZ_FOUNDER_ALLOCATION
		+ HEZ_PRESALE_ALLOCATION
		+ HEZ_TREASURY_ALLOCATION
		+ HEZ_AIRDROP_ALLOCATION;
	assert_eq!(total, 200_000_000 * TYR, "HEZ total supply must equal 200M");
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

	const ENDOWMENT: u128 = 1_000_000 * TYR;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect::<Vec<_>>(),
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
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(root_key.clone()) },
		configuration: ConfigurationConfig {
			config: pezkuwi_runtime_teyrchains::configuration::HostConfiguration {
				scheduler_params: SchedulerParams {
					max_validators_per_core: Some(1),
					..default_teyrchains_host_configuration().scheduler_params
				},
				..default_teyrchains_host_configuration()
			},
		},
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
	})
}

// pezstaging_testnet
fn pezkuwichain_staging_testnet_config_genesis() -> serde_json::Value {
	use hex_literal::hex;
	use pezsp_core::crypto::UncheckedInto;

	// pez_subkey inspect "$SECRET"
	let endowed_accounts = Vec::from([
		// 5DwBmEFPXRESyEam5SsQF1zbWSCn2kCjyLW51hJHXe9vW4xs
		hex!["52bc71c1eca5353749542dfdf0af97bf764f9c2f44e860cd485f1cd86400f649"].into(),
	]);

	// ./scripts/prepare-test-net.sh 8
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
			//5EHZkbp22djdbuMFH9qt1DVzSCvqi3zWpj6DAYfANa828oei
			hex!["62475fe5406a7cb6a64c51d0af9d3ab5c2151bcae982fb812f7a76b706914d6a"].into(),
			//5FeSEpi9UYYaWwXXb3tV88qtZkmSdB3mvgj3pXkxKyYLGhcd
			hex!["9e6e781a76810fe93187af44c79272c290c2b9e2b8b92ee11466cd79d8023f50"].into(),
			//5Fh6rDpMDhM363o1Z3Y9twtaCPfizGQWCi55BSykTQjGbP7H
			hex!["a076ef1280d768051f21d060623da3ab5b56944d681d303ed2d4bf658c5bed35"]
				.unchecked_into(),
			//5CPd3zoV9Aaah4xWucuDivMHJ2nEEmpdi864nPTiyRZp4t87
			hex!["0e6d7d1afbcc6547b92995a394ba0daed07a2420be08220a5a1336c6731f0bfa"]
				.unchecked_into(),
			//5CP6oGfwqbEfML8efqm1tCZsUgRsJztp9L8ZkEUxA16W8PPz
			hex!["0e07a51d3213842f8e9363ce8e444255990a225f87e80a3d651db7841e1a0205"]
				.unchecked_into(),
			//5HQdwiDh8Qtd5dSNWajNYpwDvoyNWWA16Y43aEkCNactFc2b
			hex!["ec60e71fe4a567ef9fef99d4bbf37ffae70564b41aa6f94ef0317c13e0a5477b"]
				.unchecked_into(),
			//5HbSgM72xVuscsopsdeG3sCSCYdAeM1Tay9p79N6ky6vwDGq
			hex!["f49eae66a0ac9f610316906ec8f1a0928e20d7059d76a5ca53cbcb5a9b50dd3c"]
				.unchecked_into(),
			//5DPSWdgw38Spu315r6LSvYCggeeieBAJtP5A1qzuzKhqmjVu
			hex!["034f68c5661a41930c82f26a662276bf89f33467e1c850f2fb8ef687fe43d62276"]
				.unchecked_into(),
		),
		(
			//5DvH8oEjQPYhzCoQVo7WDU91qmQfLZvxe9wJcrojmJKebCmG
			hex!["520b48452969f6ddf263b664de0adb0c729d0e0ad3b0e5f3cb636c541bc9022a"].into(),
			//5ENZvCRzyXJJYup8bM6yEzb2kQHEb1NDpY2ZEyVGBkCfRdj3
			hex!["6618289af7ae8621981ffab34591e7a6486e12745dfa3fd3b0f7e6a3994c7b5b"].into(),
			//5DLjSUfqZVNAADbwYLgRvHvdzXypiV1DAEaDMjcESKTcqMoM
			hex!["38757d0de00a0c739e7d7984ef4bc01161bd61e198b7c01b618425c16bb5bd5f"]
				.unchecked_into(),
			//5HnDVBN9mD6mXyx8oryhDbJtezwNSj1VRXgLoYCBA6uEkiao
			hex!["fcd5f87a6fd5707a25122a01b4dac0a8482259df7d42a9a096606df1320df08d"]
				.unchecked_into(),
			//5EPEWRecy2ApL5n18n3aHyU1956zXTRqaJpzDa9DoqiggNwF
			hex!["669a10892119453e9feb4e3f1ee8e028916cc3240022920ad643846fbdbee816"]
				.unchecked_into(),
			//5ES3fw5X4bndSgLNmtPfSbM2J1kLqApVB2CCLS4CBpM1UxUZ
			hex!["68bf52c482630a8d1511f2edd14f34127a7d7082219cccf7fd4c6ecdb535f80d"]
				.unchecked_into(),
			//5HeXbwb5PxtcRoopPZTp5CQun38atn2UudQ8p2AxR5BzoaXw
			hex!["f6f8fe475130d21165446a02fb1dbce3a7bf36412e5d98f4f0473aed9252f349"]
				.unchecked_into(),
			//5F7nTtN8MyJV4UsXpjg7tHSnfANXZ5KRPJmkASc1ZSH2Xoa5
			hex!["03a90c2bb6d3b7000020f6152fe2e5002fa970fd1f42aafb6c8edda8dacc2ea77e"]
				.unchecked_into(),
		),
		(
			//5FPMzsezo1PRxYbVpJMWK7HNbR2kUxidsAAxH4BosHa4wd6S
			hex!["92ef83665b39d7a565e11bf8d18d41d45a8011601c339e57a8ea88c8ff7bba6f"].into(),
			//5G6NQidFG7YiXsvV7hQTLGArir9tsYqD4JDxByhgxKvSKwRx
			hex!["b235f57244230589523271c27b8a490922ffd7dccc83b044feaf22273c1dc735"].into(),
			//5GpZhzAVg7SAtzLvaAC777pjquPEcNy1FbNUAG2nZvhmd6eY
			hex!["d2644c1ab2c63a3ad8d40ad70d4b260969e3abfe6d7e6665f50dc9f6365c9d2a"]
				.unchecked_into(),
			//5HAes2RQYPbYKbLBfKb88f4zoXv6pPA6Ke8CjN7dob3GpmSP
			hex!["e1b68fbd84333e31486c08e6153d9a1415b2e7e71b413702b7d64e9b631184a1"]
				.unchecked_into(),
			//5FtAGDZYJKXkhVhAxCQrXmaP7EE2mGbBMfmKDHjfYDgq2BiU
			hex!["a8e61ffacafaf546283dc92d14d7cc70ea0151a5dd81fdf73ff5a2951f2b6037"]
				.unchecked_into(),
			//5CtK7JHv3h6UQZ44y54skxdwSVBRtuxwPE1FYm7UZVhg8rJV
			hex!["244f3421b310c68646e99cdbf4963e02067601f57756b072a4b19431448c186e"]
				.unchecked_into(),
			//5D4r6YaB6F7A7nvMRHNFNF6zrR9g39bqDJFenrcaFmTCRwfa
			hex!["2c57f81fd311c1ab53813c6817fe67f8947f8d39258252663b3384ab4195494d"]
				.unchecked_into(),
			//5EPoHj8uV4fFKQHYThc6Z9fDkU7B6ih2ncVzQuDdNFb8UyhF
			hex!["039d065fe4f9234f0a4f13cc3ae585f2691e9c25afa469618abb6645111f607a53"]
				.unchecked_into(),
		),
		(
			//5DMNx7RoX6d7JQ38NEM7DWRcW2THu92LBYZEWvBRhJeqcWgR
			hex!["38f3c2f38f6d47f161e98c697bbe3ca0e47c033460afda0dda314ab4222a0404"].into(),
			//5GGdKNDr9P47dpVnmtq3m8Tvowwf1ot1abw6tPsTYYFoKm2v
			hex!["ba0898c1964196474c0be08d364cdf4e9e1d47088287f5235f70b0590dfe1704"].into(),
			//5EjkyPCzR2SjhDZq8f7ufsw6TfkvgNRepjCRQFc4TcdXdaB1
			hex!["764186bc30fd5a02477f19948dc723d6d57ab174debd4f80ed6038ec960bfe21"]
				.unchecked_into(),
			//5DJV3zCBTJBLGNDCcdWrYxWDacSz84goGTa4pFeKVvehEBte
			hex!["36be9069cdb4a8a07ecd51f257875150f0a8a1be44a10d9d98dabf10a030aef4"]
				.unchecked_into(),
			//5F9FsRjpecP9GonktmtFL3kjqNAMKjHVFjyjRdTPa4hbQRZA
			hex!["882d72965e642677583b333b2d173ac94b5fd6c405c76184bb14293be748a13b"]
				.unchecked_into(),
			//5F1FZWZSj3JyTLs8sRBxU6QWyGLSL9BMRtmSKDmVEoiKFxSP
			hex!["821271c99c958b9220f1771d9f5e29af969edfa865631dba31e1ab7bc0582b75"]
				.unchecked_into(),
			//5CtgRR74VypK4h154s369abs78hDUxZSJqcbWsfXvsjcHJNA
			hex!["2496f28d887d84705c6dae98aee8bf90fc5ad10bb5545eca1de6b68425b70f7c"]
				.unchecked_into(),
			//5CPx6dsr11SCJHKFkcAQ9jpparS7FwXQBrrMznRo4Hqv1PXz
			hex!["0307d29bbf6a5c4061c2157b44fda33b7bb4ec52a5a0305668c74688cedf288d58"]
				.unchecked_into(),
		),
		(
			//5C8AL1Zb4bVazgT3EgDxFgcow1L4SJjVu44XcLC9CrYqFN4N
			hex!["02a2d8cfcf75dda85fafc04ace3bcb73160034ed1964c43098fb1fe831de1b16"].into(),
			//5FLYy3YKsAnooqE4hCudttAsoGKbVG3hYYBtVzwMjJQrevPa
			hex!["90cab33f0bb501727faa8319f0845faef7d31008f178b65054b6629fe531b772"].into(),
			//5Et3tfbVf1ByFThNAuUq5pBssdaPPskip5yob5GNyUFojXC7
			hex!["7c94715e5dd8ab54221b1b6b2bfa5666f593f28a92a18e28052531de1bd80813"]
				.unchecked_into(),
			//5EX1JBghGbQqWohTPU6msR9qZ2nYPhK9r3RTQ2oD1K8TCxaG
			hex!["6c878e33b83c20324238d22240f735457b6fba544b383e70bb62a27b57380c81"]
				.unchecked_into(),
			//5EUNaBpX9mJgcmLQHyG5Pkms6tbDiKuLbeTEJS924Js9cA1N
			hex!["6a8570b9c6408e54bacf123cc2bb1b0f087f9c149147d0005badba63a5a4ac01"]
				.unchecked_into(),
			//5CaZuueRVpMATZG4hkcrgDoF4WGixuz7zu83jeBdY3bgWGaG
			hex!["16c69ea8d595e80b6736f44be1eaeeef2ac9c04a803cc4fd944364cb0d617a33"]
				.unchecked_into(),
			//5DABsdQCDUGuhzVGWe5xXzYQ9rtrVxRygW7RXf9Tsjsw1aGJ
			hex!["306ac5c772fe858942f92b6e28bd82fb7dd8cdd25f9a4626c1b0eee075fcb531"]
				.unchecked_into(),
			//5H91T5mHhoCw9JJG4NjghDdQyhC6L7XcSuBWKD3q3TAhEVvQ
			hex!["02fb0330356e63a35dd930bc74525edf28b3bf5eb44aab9e9e4962c8309aaba6a6"]
				.unchecked_into(),
		),
		(
			//5C8XbDXdMNKJrZSrQURwVCxdNdk8AzG6xgLggbzuA399bBBF
			hex!["02ea6bfa8b23b92fe4b5db1063a1f9475e3acd0ab61e6b4f454ed6ba00b5f864"].into(),
			//5GsyzFP8qtF8tXPSsjhjxAeU1v7D1PZofuQKN9TdCc7Dp1JM
			hex!["d4ffc4c05b47d1115ad200f7f86e307b20b46c50e1b72a912ec4f6f7db46b616"].into(),
			//5GHWB8ZDzegLcMW7Gdd1BS6WHVwDdStfkkE4G7KjPjZNJBtD
			hex!["bab3cccdcc34401e9b3971b96a662686cf755aa869a5c4b762199ce531b12c5b"]
				.unchecked_into(),
			//5GzDPGbUM9uH52ZEwydasTj8edokGUJ7vEpoFWp9FE1YNuFB
			hex!["d9c056c98ca0e6b4eb7f5c58c007c1db7be0fe1f3776108f797dd4990d1ccc33"]
				.unchecked_into(),
			//5CmLCFeSurRXXtwMmLcVo7sdJ9EqDguvJbuCYDcHkr3cpqyE
			hex!["1efc23c0b51ad609ab670ecf45807e31acbd8e7e5cb7c07cf49ee42992d2867c"]
				.unchecked_into(),
			//5DnsSy8a8pfE2aFjKBDtKw7WM1V4nfE5sLzP15MNTka53GqS
			hex!["4c64d3f06d28adeb36a892fdaccecace150bec891f04694448a60b74fa469c22"]
				.unchecked_into(),
			//5CZdFnyzZvKetZTeUwj5APAYskVJe4QFiTezo5dQNsrnehGd
			hex!["160ea09c5717270e958a3da42673fa011613a9539b2e4ebcad8626bc117ca04a"]
				.unchecked_into(),
			//5HgoR9JJkdBusxKrrs3zgd3ToppgNoGj1rDyAJp4e7eZiYyT
			hex!["020019a8bb188f8145d02fa855e9c36e9914457d37c500e03634b5223aa5702474"]
				.unchecked_into(),
		),
		(
			//5HinEonzr8MywkqedcpsmwpxKje2jqr9miEwuzyFXEBCvVXM
			hex!["fa373e25a1c4fe19c7148acde13bc3db1811cf656dc086820f3dda736b9c4a00"].into(),
			//5EHJbj6Td6ks5HDnyfN4ttTSi57osxcQsQexm7XpazdeqtV7
			hex!["62145d721967bd88622d08625f0f5681463c0f1b8bcd97eb3c2c53f7660fd513"].into(),
			//5EeCsC58XgJ1DFaoYA1WktEpP27jvwGpKdxPMFjicpLeYu96
			hex!["720537e2c1c554654d73b3889c3ef4c3c2f95a65dd3f7c185ebe4afebed78372"]
				.unchecked_into(),
			//5DnEySxbnppWEyN8cCLqvGjAorGdLRg2VmkY96dbJ1LHFK8N
			hex!["4bea0b37e0cce9bddd80835fa2bfd5606f5dcfb8388bbb10b10c483f0856cf14"]
				.unchecked_into(),
			//5CAC278tFCHAeHYqE51FTWYxHmeLcENSS1RG77EFRTvPZMJT
			hex!["042f07fc5268f13c026bbe199d63e6ac77a0c2a780f71cda05cee5a6f1b3f11f"]
				.unchecked_into(),
			//5HjRTLWcQjZzN3JDvaj1UzjNSayg5ZD9ZGWMstaL7Ab2jjAa
			hex!["fab485e87ed1537d089df521edf983a777c57065a702d7ed2b6a2926f31da74f"]
				.unchecked_into(),
			//5ELv74v7QcsS6FdzvG4vL2NnYDGWmRnJUSMKYwdyJD7Xcdi7
			hex!["64d59feddb3d00316a55906953fb3db8985797472bd2e6c7ea1ab730cc339d7f"]
				.unchecked_into(),
			//5FaUcPt4fPz93vBhcrCJqmDkjYZ7jCbzAF56QJoCmvPaKrmx
			hex!["033f1a6d47fe86f88934e4b83b9fae903b92b5dcf4fec97d5e3e8bf4f39df03685"]
				.unchecked_into(),
		),
		(
			//5Ey3NQ3dfabaDc16NUv7wRLsFCMDFJSqZFzKVycAsWuUC6Di
			hex!["8062e9c21f1d92926103119f7e8153cebdb1e5ab3e52d6f395be80bb193eab47"].into(),
			//5HiWsuSBqt8nS9pnggexXuHageUifVPKPHDE2arTKqhTp1dV
			hex!["fa0388fa88f3f0cb43d583e2571fbc0edad57dff3a6fd89775451dd2c2b8ea00"].into(),
			//5H168nKX2Yrfo3bxj7rkcg25326Uv3CCCnKUGK6uHdKMdPt8
			hex!["da6b2df18f0f9001a6dcf1d301b92534fe9b1f3ccfa10c49449fee93adaa8349"]
				.unchecked_into(),
			//5DrA2fZdzmNqT5j6DXNwVxPBjDV9jhkAqvjt6Us3bQHKy3cF
			hex!["4ee66173993dd0db5d628c4c9cb61a27b76611ad3c3925947f0d0011ee2c5dcc"]
				.unchecked_into(),
			//5Gx6YeNhynqn8qkda9QKpc9S7oDr4sBrfAu516d3sPpEt26F
			hex!["d822d4088b20dca29a580a577a97d6f024bb24c9550bebdfd7d2d18e946a1c7d"]
				.unchecked_into(),
			//5DhDcHqwxoes5s89AyudGMjtZXx1nEgrk5P45X88oSTR3iyx
			hex!["481538f8c2c011a76d7d57db11c2789a5e83b0f9680dc6d26211d2f9c021ae4c"]
				.unchecked_into(),
			//5DqAvikdpfRdk5rR35ZobZhqaC5bJXZcEuvzGtexAZP1hU3T
			hex!["4e262811acdfe94528bfc3c65036080426a0e1301b9ada8d687a70ffcae99c26"]
				.unchecked_into(),
			//5E41Znrr2YtZu8bZp3nvRuLVHg3jFksfQ3tXuviLku4wsao7
			hex!["025e84e95ed043e387ddb8668176b42f8e2773ddd84f7f58a6d9bf436a4b527986"]
				.unchecked_into(),
		),
	]);

	const ENDOWMENT: u128 = 1_000_000 * TYR;
	const STASH: u128 = 100 * TYR;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|k: &AccountId| (k.clone(), ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.into_iter()
				.map(|x| (
					x.0.clone(),
					x.0,
					pezkuwichain_session_keys(x.2, x.3, x.4, x.5, x.6, x.7)
				))
				.collect::<Vec<_>>(),
		},
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(endowed_accounts[0].clone()) },
		configuration: ConfigurationConfig { config: default_teyrchains_host_configuration() },
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
	})
}

//development
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

/// Encapsulates names of predefined presets.
mod preset_names {
	pub const PRESET_GENESIS: &str = "genesis";
}

/// Genesis configuration for mainnet with HEZ distribution
/// Accounts from Founder_treasury_presale_wallets.json
fn pezkuwichain_genesis_config() -> serde_json::Value {
	use hex_literal::hex;
	use pezsp_core::crypto::UncheckedInto;

	// ==========================================================================
	// MAINNET ACCOUNTS - Real addresses from founder_governance.json & presale_airdrop_wallets.json
	// ==========================================================================

	// Founder account - receives 10% (20M HEZ)
	// NEW SECURE WALLET - 2026-01-21
	// SS58: 5HN6sFM7TbPQazmfhJP1kU8itw7Tb2A9UML8TwSYRwiN9q5Z
	let founder_account: AccountId =
		hex!("ea71cc341e6790988692d8adcd08a26c75d8c813e45e0a25b24b707dc7846677").into();

	// Presale account - receives 50% (100M HEZ)
	// NEW SECURE WALLET - 2026-01-21
	// SS58: 5GsFKogGuxr9ToPuZ2XPxksZWTWdCGUnd8hmqSyssfsvprtA
	let presale_account: AccountId =
		hex!("d47027192dd48b2c48606012a8bb7410cd92fed091e4896e4dc4c67772974606").into();

	// Kurdistan Treasury account - receives 20% (40M HEZ)
	// NEW SECURE WALLET - 2026-01-21
	// SS58: 5D7guUmrk2xap2xuCwDJgJB5JDtxy439Dx2vaQ5JkvgtNjb4
	let treasury_account: AccountId =
		hex!("2e82c43a0a7edc05a179901d18bdfac86d52953c1b7ca5e8e3ceeb3a83044b4f").into();

	// Airdrop account - receives 20% (40M HEZ)
	// NEW SECURE WALLET - 2026-01-21
	// SS58: 5CZqFpRXHHR6VHk1P3E1jJfoz6ngB5Nc8akfGre6yAXqz5TF
	let airdrop_account: AccountId =
		hex!("16370640e359b026c6a4e5bd4352e475fa3748bc7436b4f737dfe38fadf5d20e").into();

	// ==========================================================================
	// INITIAL VALIDATORS - 21 validators from mainnet_validators JSON
	// ==========================================================================
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
			// Validator 1: Satoshi-Qazi-Mohammed (5FTWGbYvNKXJDWFdRrHTY3jDW8jwxJ8HpccTAYfu91omNAcc)
			hex!("9618567b32d46c19596b30f26d047bfee7507fb93aba7d2d655bee9bedf29e58").into(),
			hex!("9618567b32d46c19596b30f26d047bfee7507fb93aba7d2d655bee9bedf29e58").into(),
			hex!("803572349e718b779e70433206c820703407bf3379738c6f0f16bae2d9d1eb33")
				.unchecked_into(),
			hex!("ab894aadbd1bd4a057e290881b003f01c00f83dbfcbbe05d45bcd4a4703a51d6")
				.unchecked_into(),
			hex!("8298e1387a1c9c9a417cbe329a5f2e6cb10872033b8ad55a359e62074fcd9969")
				.unchecked_into(),
			hex!("8ee2365cbf5d0d85950843527fba15f175c0d9c106f0411657777abf29dfbe35")
				.unchecked_into(),
			hex!("8ccbb9145c324cc20d9d6da86467f7b1e8b363be1cd28be0c7c4548460b2b410")
				.unchecked_into(),
			hex!("032f6af3c0b1beed03758754d02b228549fd14d8264b83a8cc9b40da0498c11a94")
				.unchecked_into(),
		),
		(
			// Validator 2: presale_airdrop_wallet_1 (5E2WhTJyboMX69nkXzFomYNSKjDbSmwYKxY1gx8kD2VTcz3s)
			hex!("56cc6978e0b14e16061778b8a5c1e645195e9bba3db8898dbc4319acf6817a26").into(),
			hex!("56cc6978e0b14e16061778b8a5c1e645195e9bba3db8898dbc4319acf6817a26").into(),
			hex!("d8f44eeeac7349e33b50adbb3c87df7208c5308015b98a42dc9cfb92ce27f605")
				.unchecked_into(),
			hex!("5030c54bbb6b3ce2ce9d066720099bf393ba502d57d5735a3a159b31c2a637e7")
				.unchecked_into(),
			hex!("2ecdbe40537b9ea173ed183c9a0741fc5a4e4da66f98237c7ff2953db38cf160")
				.unchecked_into(),
			hex!("2c71424c2a39ef2f98312477091946bfc866d67e85b54e4fea42ec7f7322372b")
				.unchecked_into(),
			hex!("ca3044fac8499a8a129339461782f0f7d8d28195f3412b20b2920abe6f1cc950")
				.unchecked_into(),
			hex!("03a81ac2d9a8d163d0bfd3c66e6e2a1dcd9077035791991a80e02577bb341374a4")
				.unchecked_into(),
		),
		(
			// Validator 3: presale_airdrop_wallet_2 (5ChSoGci7EE55SRUybv8sMLQrBRswUzv7CzUKzfQECLjpqRV)
			hex!("1c0563612287d2a78ac7610c366b03849d7c6511273a9d7b90c76cbbc588245d").into(),
			hex!("1c0563612287d2a78ac7610c366b03849d7c6511273a9d7b90c76cbbc588245d").into(),
			hex!("188180f6e8fa6993505b78608d57c82f2009a327ea1f9b2bc7b49de567884918")
				.unchecked_into(),
			hex!("d0c2e79482190a7df3479fa7956fa933e08f07ebbfbe96c395e3656345c9f911")
				.unchecked_into(),
			hex!("62f2cd6b2a5a674f2fa60b274120f6195fbdb59ed11fbbd6964c9a7a66e7b543")
				.unchecked_into(),
			hex!("a2ce3925d8ab92ad6d3fb114023ad4e3faee7c5d652078437549b00c6984e10f")
				.unchecked_into(),
			hex!("b4956ebceb50efc7c1a2b85ff32dd688d06e7b5994ddf4d54be57b9ad23c8f6c")
				.unchecked_into(),
			hex!("02d7360f09a0060f0435375fdaa11a461049bb0abda5e7513256c417ddcf9cbf6b")
				.unchecked_into(),
		),
		(
			// Validator 4: presale_airdrop_wallet_3 (5EWUZcdDEc7Dqox5UwvDMvRa5pHVahXoViksZDWdxN37XJuN)
			hex!("6c20188182f663bc47697e753b98f42ff5fb6b9a226ea477c5e5494d052c4b66").into(),
			hex!("6c20188182f663bc47697e753b98f42ff5fb6b9a226ea477c5e5494d052c4b66").into(),
			hex!("2c6cd0d79027685dce9ffbaf44b3d9a2742c241479703ac36c48a8e7a2aaf730")
				.unchecked_into(),
			hex!("cf0cd6fa1fc07e980963eadba842f1b9de0e8d03b3a23047cef124cb6240125b")
				.unchecked_into(),
			hex!("668d616c4221a1261dcb080fe07dea0e8875fa49d2cb6f9bd8809383aef3301f")
				.unchecked_into(),
			hex!("52598f11206bd9402d56ebaf01417a3102d49e3d01266e69b3d9ca198472e540")
				.unchecked_into(),
			hex!("64af89aa204a62068a6271e86b43a2f6cf5dc833210bda0a6c1f4f362acfec75")
				.unchecked_into(),
			hex!("03742a58081b384a3726ed4f9e73b8f9507e49617e2d19c05606d70e2039463f92")
				.unchecked_into(),
		),
		(
			// Validator 5: Treasury_1 (5H9NDZ6uiiRb7tw81anLZG29Nt9CHX1Xyh7qEG45X5QdSPTy)
			hex!("e0bb47cd37410827ac16c17ae7db0d6ca05c0e63372e904cb98aeb20c2c5e772").into(),
			hex!("e0bb47cd37410827ac16c17ae7db0d6ca05c0e63372e904cb98aeb20c2c5e772").into(),
			hex!("803502616fa8376f0199d5006ae0bff19e95bc9ebdd18f9e68d1eea34e7c8f43")
				.unchecked_into(),
			hex!("37e1e52d952a0685175fea80ba619c05e60872517e05b35a2f085b3ec07a5510")
				.unchecked_into(),
			hex!("8c18ed6bbd184cc37f743df51d571db3c38331a8fc3eba660964b72a377a4d4b")
				.unchecked_into(),
			hex!("60d5d88a646fd30805896e5f40a6840464f7226f65ff0dfa06206b26b879a66c")
				.unchecked_into(),
			hex!("c8c3eba3a4c9c2e1aed7f6f7649437ad3d2aec2ca9408f82f1e4f60f2020de3e")
				.unchecked_into(),
			hex!("021e12c87d8b450bd64c3d4b13b87cfd62d9e3b0041eec5c4296621183554f423a")
				.unchecked_into(),
		),
		(
			// Validator 6: Treasury_2 (5FL1bTPvxLWQ5rZvuJj1k6RPDr4kHM6D3bM5fU3DjHTUBSCC)
			hex!("9061178549731d50002daaf990be40f3aabb0c82b5bfdc2aa1e2828970e4692e").into(),
			hex!("9061178549731d50002daaf990be40f3aabb0c82b5bfdc2aa1e2828970e4692e").into(),
			hex!("0ce12f58a53d16af3d2b53c25575dee7244c7f06df8f1d5d83992fa33d5b3102")
				.unchecked_into(),
			hex!("6cb478a75dedbd168b87c8025e56f1e1b656053cd2e7df13be0882c5986cd61f")
				.unchecked_into(),
			hex!("46ac0e3febbf9991d48f360f8b64fd7044dcdf8e0103b0f92a29d0723a586e4d")
				.unchecked_into(),
			hex!("483a6618f7ce843f80d427b356de69c3c3b7a8728f7a2f2ba6747afff5ff7b41")
				.unchecked_into(),
			hex!("0e131be861ec41745345b39ff917440733a4fcc370b0fdd0e4f5b43136890b6f")
				.unchecked_into(),
			hex!("0211ddd0d8465db59783ba416678070209dc5781bba2c1e6c4f3c55088b02a99ef")
				.unchecked_into(),
		),
		(
			// Validator 7: Tresury_3 (5FqeuNdg2McKuyu4N5z4WiFCnFF6kGaVtsTv8yNsjQ5HmfMg)
			hex!("a6fccd3937ae5c8591baf40f107887e4807a92e4d7c04ee272a0259cd9febf24").into(),
			hex!("a6fccd3937ae5c8591baf40f107887e4807a92e4d7c04ee272a0259cd9febf24").into(),
			hex!("5865bda854c717c3d6946db494b927b12742ad0148afa2f373f1125d4781de6b")
				.unchecked_into(),
			hex!("512e5fc8f632a15619619fed1e6fca2c71e88e974a1223faf9f72b4ce1703cab")
				.unchecked_into(),
			hex!("267523f76d57d56aff43fe6df734c52fd3f466858cf505776ff27c979f42647d")
				.unchecked_into(),
			hex!("88b212c925fbeab3402dd1f1a38d4bbf9f1b518804ec7689a243871d271b615d")
				.unchecked_into(),
			hex!("b25177091ca52a8f97253ded1fd50c3730e3385e2613e49192258bc21ce78406")
				.unchecked_into(),
			hex!("02ad0452e1828d0366978f36dd6198d665ee55ac7db56d222ce831e1ec35c2c138")
				.unchecked_into(),
		),
		(
			// Validator 8: Validator-mainnet-1 (5FNJ1BK1nBNVuw43ZNsLMwDtpA9ptkZD8ruZRBRQxLEGnJzH)
			hex!("921ed2020765bc0ac2c5d2be3fee95b0eb20a92ea4a9ec265279213aa3d3eb32").into(),
			hex!("921ed2020765bc0ac2c5d2be3fee95b0eb20a92ea4a9ec265279213aa3d3eb32").into(),
			hex!("0a1fb6dde04f68e1b6f61d4025a81393cad303cc97617e2b4b13738fa519e210")
				.unchecked_into(),
			hex!("b74e1263643a7669162a2287fee61152ffac7c04fb8501ce47abd96b7c878be4")
				.unchecked_into(),
			hex!("e610e51a1cebccf1a07c39cee084ba2ef6ac801948462c7ff61868522e4a8d10")
				.unchecked_into(),
			hex!("5889339c47d4fecd66ab8c0a63d40cc15e577727b8a7d0867764daa4638c5108")
				.unchecked_into(),
			hex!("c89b9337c67689b5641ba88ddc494e69e6b4323936534c6ca5b7065d91b02178")
				.unchecked_into(),
			hex!("02a45f882347f7bab8ebd59ca1086692bd8007375ae30ac81ef8bdf4d6e26ce11a")
				.unchecked_into(),
		),
		(
			// Validator 9: Validator-mainnet-2 (5FX8fysMNwMj6dTkSCXs4bymf8aZUNbAypBm9cM3dNoySgyq)
			hex!("98dc9e3a60fe0980e27ce16f591818259c9c57866a8205f2e19accad7cc61224").into(),
			hex!("98dc9e3a60fe0980e27ce16f591818259c9c57866a8205f2e19accad7cc61224").into(),
			hex!("985471c2cff625dd54b686be102c59717ae0116eaa43779ae67f8b8165b8ce77")
				.unchecked_into(),
			hex!("91920ad3857d9739028479a4059941ffc52a040526cdc25672d0501b5d08c34a")
				.unchecked_into(),
			hex!("2a026da96949a4f3bf02e68c055a204d702e80eed8b45e983413b813eecb3c1b")
				.unchecked_into(),
			hex!("d0128f36d5b48672c4feceb5b65a23078180b431af2beab931c2d34522c8cf35")
				.unchecked_into(),
			hex!("5e69b64bfd5368a1bdb46b890fd30fc0da505ad1ed89b78681dbfcd94916727d")
				.unchecked_into(),
			hex!("02669ac19842efeb35a06e7627323756c4f7efbead39c18450a0a4f527cef6d553")
				.unchecked_into(),
		),
		(
			// Validator 10: Validator-mainnet-3 (5EPZBLw8V9oX5ZRqvAb99vzrD1ab3HCmpRJHkhHUGfeyjH35)
			hex!("66d8eab4d54e2074e733b268fa2e07e916f1e9eee835becba5cd9936db7d1874").into(),
			hex!("66d8eab4d54e2074e733b268fa2e07e916f1e9eee835becba5cd9936db7d1874").into(),
			hex!("082b78eb3c90948060a2590f437fe19064f12f6461155dc448b8f8ac4cef4932")
				.unchecked_into(),
			hex!("9178e52724ea162c628ebb33256030c13fd32c6913e66e65d7aeb8a53a17b2ee")
				.unchecked_into(),
			hex!("1cc6b985e4a2e970f8c7ef024f23f5b4f91828e748a144a7b9989a5e55559943")
				.unchecked_into(),
			hex!("724e81bbf0e72e3813553b72723af6298884d0ab63493a2850f76fe934293b04")
				.unchecked_into(),
			hex!("faa7902f18d95a4f4718b44c70d32516d3b3208c4540bb4cc1d1e608ebad8b51")
				.unchecked_into(),
			hex!("024abebfd2519feffd4bab7b8d1b7551992a973bccc9880bc7dfddf1546c6ed489")
				.unchecked_into(),
		),
		(
			// Validator 11: Validator-mainnet-4 (5HawXVx3L26t75ctDdXbDmV5yRW1GGos9LQ7DHui342kb9TK)
			hex!("f43c8b8817e61a123b6f504280d3125fbfcc65eb5f4d63f5f121554e1e09c742").into(),
			hex!("f43c8b8817e61a123b6f504280d3125fbfcc65eb5f4d63f5f121554e1e09c742").into(),
			hex!("1205b7b70dce92f31b5d6d0177a5cb2b774f63d77c975b15b924cbace51a586d")
				.unchecked_into(),
			hex!("d8758bf8fa32170bbbbb31761dc6e1f736fd9ea9da42258a9a5baf3a310a421e")
				.unchecked_into(),
			hex!("4c2e65de4477d273d72ee6d0b776c82c9595886de1c6602f0fad00a14db2107b")
				.unchecked_into(),
			hex!("fa51d247f0021da822af0f0cf3752771ae106e586230efecf88fab5f6aeb0c52")
				.unchecked_into(),
			hex!("30b6faabeb1cab5b2e320190a24ab22df3d5f866b500c92c089be42921c77905")
				.unchecked_into(),
			hex!("03b5dd0ea8385cbf016b62c31351f8d3260cc1e1789742bf504d307ad2d4fba3b3")
				.unchecked_into(),
		),
		(
			// Validator 12: Validator-mainnet-5 (5DJdwqVauM2KDPA2Ge5x9jQg2Gq4gEBtw6ggPm7WzAuChuFh)
			hex!("36dc80fdb5982e6419a2135dc9f4ea50fae9f599e66c470b59d3cf0852b28d5e").into(),
			hex!("36dc80fdb5982e6419a2135dc9f4ea50fae9f599e66c470b59d3cf0852b28d5e").into(),
			hex!("3896b67b7208ccfbb305fcbdcd067a13fd0346ddff5fc4ea4873eeeedb20486f")
				.unchecked_into(),
			hex!("73f9a554725554b239b55fff62eac7fb4e5da557489a3a7f9ba8971ee5ae2b92")
				.unchecked_into(),
			hex!("3688922cfae0274c8727f5802bee380a35a41b184c356b8f577b0d20560ca545")
				.unchecked_into(),
			hex!("a046a781a8731590d73b4b3e051a56d485ecdc8ef4d73c0399c3897913da1466")
				.unchecked_into(),
			hex!("6080d82ebbd4fbc1e4f388392f3422f18d62be8774493ad34e04137d05a36b4e")
				.unchecked_into(),
			hex!("032b857006af4f6e0a07663dc17a5563ef82d43f1fe134d85c2b6d758c1424bfd0")
				.unchecked_into(),
		),
		(
			// Validator 13: Validator-mainnet-6 (5Fvuq9UQ6KEfCVKA7387oSTDShURchzQkJnzuwAfCMqaWjyc)
			hex!("aaff472128d740715f83b13bc7bbf8f01106ae942d137b868fc38b21ac373c76").into(),
			hex!("aaff472128d740715f83b13bc7bbf8f01106ae942d137b868fc38b21ac373c76").into(),
			hex!("6e055519696432354fde572621d297a4ceba0b907f54273afc17cb09a1469d17")
				.unchecked_into(),
			hex!("74910e73fe0f9c1f963595458885d2f715978b765c17e94f5c6f9e738962c47b")
				.unchecked_into(),
			hex!("427561df4d5db581afbda93f5070dfa181a8bb733d7e0bf7135fc466d7666875")
				.unchecked_into(),
			hex!("ba173f3d333d16ae1b8aef2b0a59034d3d9eb3fb9bbf48e4d98f9484ffa81f46")
				.unchecked_into(),
			hex!("6c8a74bc055d1b7569d3a2d6902ae01c759d17e12d1f00b38fc667e05bc66463")
				.unchecked_into(),
			hex!("0286a1add16864903e17e49a6b5cdde8521158265373b72dd82231230f9ebd5d7b")
				.unchecked_into(),
		),
		(
			// Validator 14: Validator-mainnet-7 (5DvnWeGLVvyeVdhsS29yubL3SWEDgGhqsfKQnd44utYs5MSU)
			hex!("526e2c43459a26631948b49dc9b72ccb8cfc04dc83025d7aa16cd0833339001b").into(),
			hex!("526e2c43459a26631948b49dc9b72ccb8cfc04dc83025d7aa16cd0833339001b").into(),
			hex!("8ea8e67af05252d3cf5c7c4399a887152459c3f677d4dfcd356da50666926f26")
				.unchecked_into(),
			hex!("f94870bdb3ca4538c144e8fcb4c301c16c93e19493dff99eb9246c167bb187c2")
				.unchecked_into(),
			hex!("362e6e4d7b326bf6938f22443fad5b916673e4a8a29cef7363ea87716eb9c83b")
				.unchecked_into(),
			hex!("022be2dc1a8a29f114efc15d18158fa6e0105a96a66349da8791c7bdefaed251")
				.unchecked_into(),
			hex!("b2da7aabe6972fa114bfa138f4e38c8ffbf805f50b14c76cf03a56372dbd3a4a")
				.unchecked_into(),
			hex!("02e83fcf6d01adcfafce0ff0d186003e79074a2a6532606e9221c2fc091c607e9e")
				.unchecked_into(),
		),
		(
			// Validator 15: Validator-mainnet-8 (5DaS75pj8cxatgcqeyjVH2RuYCWaVNXPQTkgF6B1gUrmAxeP)
			hex!("42e89249b0a7844e1b0d663ba042083aba831f97146789b0b7a6b2c403f9bb33").into(),
			hex!("42e89249b0a7844e1b0d663ba042083aba831f97146789b0b7a6b2c403f9bb33").into(),
			hex!("dca30ab8cc4ab46ee47c16d446f5d15056eb07aa5470f945dba29190035b7c2a")
				.unchecked_into(),
			hex!("7694a03177cacc8cfa50cc284d571825919fe1ca8df4104359f6d3ec6d74f05a")
				.unchecked_into(),
			hex!("ea89ea61580cab1680f277963e5bd671aa10d68a861bee21679657f44017946e")
				.unchecked_into(),
			hex!("d8d76388a1c4997856ba41eae8bdae73fb414dccd1586414f24cd8b38e93a569")
				.unchecked_into(),
			hex!("ec93bce67ace84ed8898ea7e526f5dc940afaad37bcc15f13c04534e129ec429")
				.unchecked_into(),
			hex!("02f1c1ac09613119da13a70ff038127417813cad80c490d4ab85c9f2442708dfe3")
				.unchecked_into(),
		),
		(
			// Validator 16: Validator-mainnet-9 (5G6e3QujHaeVRgNm4ohvZSH6YzCzkBcoLQS2KACBs2FTnbfJ)
			hex!("b26a95407dccbe75464b78b7e67ace5bda18de6dc6df6d02c100de31f059ed2d").into(),
			hex!("b26a95407dccbe75464b78b7e67ace5bda18de6dc6df6d02c100de31f059ed2d").into(),
			hex!("4634d9e1c3df96c04c0abdf9eb37e5f8ab254d856694fb93370d3d2855942e2d")
				.unchecked_into(),
			hex!("010e6ecf62c5f0054a58502d5c1e31a8c72263b1526a27d7ccf73b9a69f8e36b")
				.unchecked_into(),
			hex!("98294a235afff8784c56403c8fb6ac55a408a26701962ecbf1ede905608bb648")
				.unchecked_into(),
			hex!("aa304e4f44cceea29ed47243a2e7800403f0d57dd9247db726dc8ea9f8e36570")
				.unchecked_into(),
			hex!("10e5cb05ac527507e61c24886c72947e82230e63a76f32a375279a71e7abad5a")
				.unchecked_into(),
			hex!("03ffeaeec68bf546459eeb0cb487e0603334ae474d6c4254426c1aac0d21eb373b")
				.unchecked_into(),
		),
		(
			// Validator 17: Validator-mainnet-10 (5Guzbqg6FKv32dco331abaSacJZBbMW4KPSz8PvBcsMcJQ7n)
			hex!("d6885337f55673109a7f65c57ecf403b4498a8755cde394cedcab69148667a36").into(),
			hex!("d6885337f55673109a7f65c57ecf403b4498a8755cde394cedcab69148667a36").into(),
			hex!("d065e08dcea10ccf6dcb8142747e404b0a55a072f3ef3dc5d9b6d66d4ce6b57b")
				.unchecked_into(),
			hex!("9ea7408bbb51249588052113d15d640ad88fd3efed16dfe849fe5e6badce57bf")
				.unchecked_into(),
			hex!("0e7ed104e41d47cdc33f197083c8990035ef5e7e54d185a3be3286bcf8fdf678")
				.unchecked_into(),
			hex!("68a8e27089c8e684726f0308b273bf8b730b87311bab1b072442b912a67b7458")
				.unchecked_into(),
			hex!("fa2b2e43f95e029098750303646e291ffd5b145ba71309cf16f5233e47af7d59")
				.unchecked_into(),
			hex!("020c32b46c68a870b8dd98caae6caaab599c35229fd8b46d5ae41c13dedd0a90e1")
				.unchecked_into(),
		),
		(
			// Validator 18: Validator-mainnet-11 (5CqSaXs4f29E2cqimjFSUcVdhwDH53Tm9xKTuMQfzspWtgYp)
			hex!("221e9cb7b59bbe76a96a78f1f778760e13f38468edd1767ca6cadee44bd43d46").into(),
			hex!("221e9cb7b59bbe76a96a78f1f778760e13f38468edd1767ca6cadee44bd43d46").into(),
			hex!("8acea7a76305a373c7884802ff17908e5d765fdd1a10ad3f5eb0ea39cf8d7b7c")
				.unchecked_into(),
			hex!("77e4dbf8fdaef2e03cc2379dab6b70553d720e295e9a499e6db4442d452765f7")
				.unchecked_into(),
			hex!("3efff77a44f8055ebaa575db85d6555ede7898fa6e637bee99353866f4481763")
				.unchecked_into(),
			hex!("288834ec426b1963c75b57d3941e19beceed2674354d2f6bbd8f49483f20e735")
				.unchecked_into(),
			hex!("c65e11df6ed27a989615cdc8c271c09a991af0857372dbe25c6da4169f99104e")
				.unchecked_into(),
			hex!("0343112f981bd96993ca9ea9ab83e7a2bf3962b3bee9eee4a81239b65487e10e3f")
				.unchecked_into(),
		),
		(
			// Validator 19: Validator-mainnet-12 (5G3nvAPYndCLYx2NeYtA1GN126F67ZxvSCxUqkUFCf7LfJ6Y)
			hex!("b03eb9028a658b16ee8285d9b140c3f1df7ec22d37f0db5454588f7fb28e343f").into(),
			hex!("b03eb9028a658b16ee8285d9b140c3f1df7ec22d37f0db5454588f7fb28e343f").into(),
			hex!("ac7f833756e63807bddbeba8bb3d1c607438311c172a505eb0af3b182e725345")
				.unchecked_into(),
			hex!("3062aca55a74f0afc87fbdd074096e3f4ee709ba8b60f942cb23c01c53ab72ae")
				.unchecked_into(),
			hex!("42a0d37fb74ab46319cae9e420c55bc6830b4c30a1b2cf2434c6f96a0b49c607")
				.unchecked_into(),
			hex!("00dd476dc6b3fc99fddf377faf99544fbd95a19c6d0f20261a6af388804e6f1b")
				.unchecked_into(),
			hex!("6cb1077493b7dab811023318592d008dafa4957b9ba9502ab6d0653180584247")
				.unchecked_into(),
			hex!("03f5e25f119274d13bff4e4b39a9204daeab0e50186ead66fc23fcda849fadeae7")
				.unchecked_into(),
		),
		(
			// Validator 20: Validator-mainnet-13 (5DNpfwoMt21oQrYJYoHucfhA5uqQmL3eMWQ5jo6GcwqdvAjp)
			hex!("3a0d97abd7993674827e51c8ed7030cc2799da66738a8d9713010943ebccd967").into(),
			hex!("3a0d97abd7993674827e51c8ed7030cc2799da66738a8d9713010943ebccd967").into(),
			hex!("4e4186adb82edc0dec0beed4aeaa1c169cb103f80602b382f6bb70977c291234")
				.unchecked_into(),
			hex!("074bc005b43cac20cc158c4459cefad74bf75d88ebc1ef2fe547200781ea79a9")
				.unchecked_into(),
			hex!("d2ed25f260af952786478a51a381da7fec82698d11d34d92aee87356c02be76c")
				.unchecked_into(),
			hex!("a64fc46537c8b241812ab93580a75fb38cfe777ce53de8ec4b4753d8aaf8c277")
				.unchecked_into(),
			hex!("765d1c24ec942166bb859c71fee66f60746673c46086c09d4bcd417831b63d10")
				.unchecked_into(),
			hex!("0355929e5a58da13c084981946c928bdbcccc30357f1b8a96e8a7023bc99da9cbc")
				.unchecked_into(),
		),
		(
			// Validator 21: Validator-mainnet-14 (5C5JBiXex7uHV5YnguWrSd3kYE5L9eV2e5aUvRpJG82jQqe4)
			hex!("00738f4b8759072e575cb374d6372ad0f79ba75cda95eb0ea882a95d18208a1c").into(),
			hex!("00738f4b8759072e575cb374d6372ad0f79ba75cda95eb0ea882a95d18208a1c").into(),
			hex!("5cc983fd2896f87b4f909bd3088403523095dcf1e6912d7892388ff5a2812c15")
				.unchecked_into(),
			hex!("e84b5b55b4a826bda4fc8642bfb4fd9bed646666b5e9e25f32d4f5176f28d99c")
				.unchecked_into(),
			hex!("a6ea2d2131daf470169e04df583724a0c0f1908a7b09ffce2da14db2ad05fd07")
				.unchecked_into(),
			hex!("fe03d447901b719254607cbf3fcace39e536c0fee4d771a4c88d82b9bbbad26e")
				.unchecked_into(),
			hex!("b8d447af78698a5c2bcddf8b0227990ac83b0f044030972fc5c6e8fa62ac2e5c")
				.unchecked_into(),
			hex!("02f91d0db40c05a3d057e41e996e333e6cd0165e0cf9ac0af7a95d1c2592998dec")
				.unchecked_into(),
		),
	]);

	// Validator stash amount
	const STASH: u128 = 100 * TYR;

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: vec![
				// HEZ Genesis Distribution (200M Total)
				(founder_account.clone(), HEZ_FOUNDER_ALLOCATION), // 10% = 20M HEZ
				(presale_account.clone(), HEZ_PRESALE_ALLOCATION), // 50% = 100M HEZ
				(treasury_account.clone(), HEZ_TREASURY_ALLOCATION), // 20% = 40M HEZ
				(airdrop_account.clone(), HEZ_AIRDROP_ALLOCATION), // 20% = 40M HEZ
			]
			.into_iter()
			// Add validator stash balances
			.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
			.collect::<Vec<_>>(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.into_iter()
				.map(|x| (
					x.0.clone(),
					x.0,
					pezkuwichain_session_keys(x.2, x.3, x.4, x.5, x.6, x.7)
				))
				.collect::<Vec<_>>(),
		},
		babe: BabeConfig { epoch_config: BABE_GENESIS_EPOCH_CONFIG },
		sudo: SudoConfig { key: Some(founder_account) },
		configuration: ConfigurationConfig { config: default_teyrchains_host_configuration() },
		registrar: RegistrarConfig { next_free_para_id: pezkuwi_primitives::LOWEST_PUBLIC_ID },
	})
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	use preset_names::*;
	let patch = match id.as_ref() {
		// ====================================================================
		// GENESIS PRESET - For mainnet with HEZ distribution
		// ====================================================================
		PRESET_GENESIS => pezkuwichain_genesis_config(),

		// ====================================================================
		// LOCAL TESTNET PRESET - For local multi-node testing
		// ====================================================================
		pezsp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => pezkuwichain_local_testnet_genesis(),

		// ====================================================================
		// DEV PRESET - For single-node development
		// ====================================================================
		pezsp_genesis_builder::DEV_RUNTIME_PRESET => pezkuwichain_development_config_genesis(),

		// ====================================================================
		// STAGING TESTNET - For pre-production testing
		// ====================================================================
		"pezstaging_testnet" => pezkuwichain_staging_testnet_config_genesis(),

		// ====================================================================
		// VERSI LOCAL TESTNET - Extended local testing
		// ====================================================================
		"versi_local_testnet" => versi_local_testnet_genesis(),

		_ => return None,
	};
	Some(
		serde_json::to_string(&patch)
			.expect("serialization to json is expected to work. qed.")
			.into_bytes(),
	)
}

/// List of supported presets.
pub fn preset_names() -> Vec<PresetId> {
	use preset_names::*;
	vec![
		PresetId::from(PRESET_GENESIS),
		PresetId::from(pezsp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
		PresetId::from(pezsp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from("pezstaging_testnet"),
		PresetId::from("versi_local_testnet"),
	]
}
