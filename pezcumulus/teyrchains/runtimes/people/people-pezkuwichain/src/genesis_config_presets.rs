// Copyright (C) Parity Technologies (UK) Ltd. and Dijital Kurdistan Tech Institute
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # People Pezkuwichain Runtime genesis config presets
//!
//! This module contains genesis configuration for:
//! - IdentityKyc: Founding citizens (founder account starts as Approved citizen)
//! - Collator selection and session keys
//! - Initial balance distributions

use crate::*;
use alloc::{vec, vec::Vec};
use hex_literal::hex;
use pezcumulus_primitives_core::ParaId;
use pezframe_support::build_struct_json_patch;
use pezsp_core::{crypto::UncheckedInto, H256};
use pezsp_genesis_builder::PresetId;
use pezsp_keyring::Sr25519Keyring;
use testnet_teyrchains_constants::pezkuwichain::{
	currency::UNITS as TYR, xcm_version::SAFE_XCM_VERSION,
};
use teyrchains_common::{AccountId, AuraId};

const PEOPLE_PEZKUWICHAIN_ED: Balance = ExistentialDeposit::get();
const PEOPLE_PARA_ID: ParaId = ParaId::new(1004);

// ============================================================================
// FOUNDING CITIZEN IDENTITY HASH
// ============================================================================

/// Default identity hash for founding citizens
/// This is a placeholder hash - real citizens will update their identity through the KYC process
/// Hash format: keccak256(json_identity_data)
fn default_founding_citizen_identity_hash() -> H256 {
	// A default hash representing "Genesis Founding Citizen"
	H256::from(hex!("0000000000000000000000000000000000000000000000000000000000000001"))
}

/// Genesis configuration for People Pezkuwichain
///
/// # Parameters
/// - `invulnerables`: Initial collators with their Aura keys
/// - `endowed_accounts`: Accounts to receive initial HEZ balance
/// - `endowment`: HEZ amount for each endowed account
/// - `id`: Parachain ID
/// - `founding_citizens`: Accounts that start as Approved citizens (can accept referrals)
fn people_pezkuwichain_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	endowment: Balance,
	id: ParaId,
	founding_citizens: Vec<(AccountId, H256)>,
) -> serde_json::Value {
	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, endowment)).collect(),
		},
		teyrchain_info: TeyrchainInfoConfig { teyrchain_id: id },
		collator_selection: CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: PEOPLE_PEZKUWICHAIN_ED * 16,
		},
		session: SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),          // account id
						acc,                  // validator id
						SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		pezkuwi_xcm: PezkuwiXcmConfig { safe_xcm_version: Some(SAFE_XCM_VERSION) },

		// ====================================================================
		// IdentityKyc - Founding Citizens
		// ====================================================================
		// These accounts start with Approved status and can accept referrals immediately
		// This solves the chicken-egg problem: first citizens need to exist for others to join
		identity_kyc: IdentityKycConfig { founding_citizens, _phantom: Default::default() },
	})
}

/// Encapsulates names of predefined presets.
mod preset_names {
	pub const PRESET_GENESIS: &str = "genesis";
}

/// Provides the JSON representation of predefined genesis config for given `id`.
pub fn get_preset(id: &PresetId) -> Option<Vec<u8>> {
	use preset_names::*;
	let patch = match id.as_ref() {
		// ====================================================================
		// GENESIS PRESET - For mainnet or production use
		// Uses hardcoded hex keys for collators
		// Founder account is the founding citizen
		// ====================================================================
		PRESET_GENESIS => {
			// MAINNET FOUNDER ACCOUNT - NEW SECURE WALLET (2026-01-21)
			// Founder: 5HN6sFM7TbPQazmfhJP1kU8itw7Tb2A9UML8TwSYRwiN9q5Z
			let founder_account: AccountId =
				hex!("ea71cc341e6790988692d8adcd08a26c75d8c813e45e0a25b24b707dc7846677").into();

			people_pezkuwichain_genesis(
				// initial collators - 4 People Chain collators from mainnet_collators
				vec![
					// Erin (5GsXLpqXCsffdiEvsgRGBUnErpEotJt6GYou4fk1KfitqmJA)
					(
						hex!("d4a6129e5e8e5c148fec5e4fd542adc556a64f4693797383d8c709b7a9e26f55")
							.into(),
						hex!("d4a6129e5e8e5c148fec5e4fd542adc556a64f4693797383d8c709b7a9e26f55")
							.unchecked_into(),
					),
					// Firaz (5FppJpr63gyHZ6RkgcZ4T39jpu9gepCdvQTDok1sUFDY24h6)
					(
						hex!("a65936cee8f0a310728475960e506e44a53436a7332f0bd40d965517aa435041")
							.into(),
						hex!("a65936cee8f0a310728475960e506e44a53436a7332f0bd40d965517aa435041")
							.unchecked_into(),
					),
					// Goran (5HKjdsQN1RmrmXCr72kR5cTBJVatvm56DxBhjJGGxFFiYptd)
					(
						hex!("e8a3d68cc51621451aec7f978601f1a200084bebc82ca1cb3abc021c311d5b7c")
							.into(),
						hex!("e8a3d68cc51621451aec7f978601f1a200084bebc82ca1cb3abc021c311d5b7c")
							.unchecked_into(),
					),
					// Hevi (5CcYFisDG1jmwFRJSVYNSHKHxFWS69D4sfhebPNWc7kXvQhw)
					(
						hex!("1847872999ff97fb137e09143bc2dbd2e8c1c3c93bc240cd25ef10dc26ba9b62")
							.into(),
						hex!("1847872999ff97fb137e09143bc2dbd2e8c1c3c93bc240cd25ef10dc26ba9b62")
							.unchecked_into(),
					),
				],
				Vec::new(),
				PEOPLE_PEZKUWICHAIN_ED * 524_288,
				PEOPLE_PARA_ID,
				// Founding citizens: Founder starts as Approved citizen
				vec![(founder_account, default_founding_citizen_identity_hash())],
			)
		},

		// ====================================================================
		// LOCAL TESTNET PRESET - For local multi-node testing (Alice + Bob)
		// ====================================================================
		pezsp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => people_pezkuwichain_genesis(
			// initial collators.
			vec![
				(Sr25519Keyring::Alice.to_account_id(), Sr25519Keyring::Alice.public().into()),
				(Sr25519Keyring::Bob.to_account_id(), Sr25519Keyring::Bob.public().into()),
			],
			Sr25519Keyring::well_known().map(|x| x.to_account_id()).collect(),
			TYR * 1_000_000,
			PEOPLE_PARA_ID,
			// Founding citizens: Alice and Bob are founding citizens for testing
			vec![
				(Sr25519Keyring::Alice.to_account_id(), default_founding_citizen_identity_hash()),
				(Sr25519Keyring::Bob.to_account_id(), default_founding_citizen_identity_hash()),
			],
		),

		// ====================================================================
		// DEV PRESET - For single-node development (Alice only)
		// ====================================================================
		pezsp_genesis_builder::DEV_RUNTIME_PRESET => people_pezkuwichain_genesis(
			// initial collators.
			vec![(Sr25519Keyring::Alice.to_account_id(), Sr25519Keyring::Alice.public().into())],
			vec![
				Sr25519Keyring::Alice.to_account_id(),
				Sr25519Keyring::Bob.to_account_id(),
				Sr25519Keyring::AliceStash.to_account_id(),
				Sr25519Keyring::BobStash.to_account_id(),
			],
			TYR * 1_000_000,
			PEOPLE_PARA_ID,
			// Founding citizen: Alice is the founding citizen for dev
			vec![(Sr25519Keyring::Alice.to_account_id(), default_founding_citizen_identity_hash())],
		),

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
		PresetId::from(pezsp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(pezsp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}
