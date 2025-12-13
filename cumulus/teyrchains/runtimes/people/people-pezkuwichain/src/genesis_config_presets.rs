// Copyright (C) Parity Technologies (UK) Ltd.
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
use cumulus_primitives_core::ParaId;
use frame_support::build_struct_json_patch;
use hex_literal::hex;
use sp_core::{crypto::UncheckedInto, H256};
use sp_genesis_builder::PresetId;
use sp_keyring::Sr25519Keyring;
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
			// Founder account - MUST be replaced with real address from
			// Founder_treasury_presale_wallets.json before mainnet launch
			let founder_account: AccountId =
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into();

			people_pezkuwichain_genesis(
				// initial collators.
				vec![
					// E8XC6rTJRsioKCp6KMy6zd24ykj4gWsusZ3AkSeyavpVBAG
					(
						hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608")
							.into(),
						hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608")
							.unchecked_into(),
					),
					// G28iWEybndgGRbhfx83t7Q42YhMPByHpyqWDUgeyoGF94ri
					(
						hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944")
							.into(),
						hex!("9864b85e23aa4506643db9879c3dbbeabaa94d269693a4447f537dd6b5893944")
							.unchecked_into(),
					),
					// G839e2eMiq7UXbConsY6DS1XDAYG2XnQxAmLuRLGGQ3Px9c
					(
						hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948")
							.into(),
						hex!("9ce5741ee2f1ac3bdedbde9f3339048f4da2cb88ddf33a0977fa0b4cf86e2948")
							.unchecked_into(),
					),
					// GLao4ukFUW6qhexuZowdFrKa2NLCfnEjZMftSXXfvGv1vvt
					(
						hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f")
							.into(),
						hex!("a676ed15f5a325eab49ed8d5f8c00f3f814b19bb58cda14ad10894c078dd337f")
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
		sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => people_pezkuwichain_genesis(
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
		sp_genesis_builder::DEV_RUNTIME_PRESET => people_pezkuwichain_genesis(
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
		PresetId::from(sp_genesis_builder::DEV_RUNTIME_PRESET),
		PresetId::from(sp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET),
	]
}
