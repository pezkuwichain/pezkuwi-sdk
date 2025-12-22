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

//! # Asset Hub Pezkuwichain Runtime genesis config presets
//!
//! This module contains genesis configuration for:
//! - TrustBackedAssets (Instance1): PEZ (ID:1) and wHEZ (ID:2)
//! - ForeignAssets (Instance2): Bridged assets from other chains
//! - Initial token distributions

use crate::{
	xcm_config::{bridging::to_zagros::ZagrosNetwork, UniversalLocation},
	*,
};
use alloc::{vec, vec::Vec};
use hex_literal::hex;
use pezcumulus_primitives_core::ParaId;
use pezframe_support::build_struct_json_patch;
use pezsp_core::crypto::UncheckedInto;
use pezsp_genesis_builder::PresetId;
use pezsp_keyring::Sr25519Keyring;
use testnet_teyrchains_constants::pezkuwichain::{
	currency::UNITS as TYR, xcm_version::SAFE_XCM_VERSION,
};
use teyrchains_common::{AccountId, AssetIdForTrustBackedAssets, AuraId};
use xcm::latest::prelude::*;
use xcm_builder::GlobalConsensusConvertsFor;
use xcm_executor::traits::ConvertLocation;

const ASSET_HUB_PEZKUWICHAIN_ED: Balance = ExistentialDeposit::get();

// ============================================================================
// PEZ TOKEN CONSTANTS
// ============================================================================

/// PEZ Token Asset ID - Governance token with fixed 5B supply
pub const PEZ_ASSET_ID: AssetIdForTrustBackedAssets = 1;

/// Wrapped HEZ (wHEZ) Asset ID - Used by TokenWrapper pezpallet
pub const WHEZ_ASSET_ID: AssetIdForTrustBackedAssets = 2;

/// PEZ Token decimals (same as HEZ)
pub const PEZ_DECIMALS: u8 = 12;

/// Treasury allocation: 20.25% = 1,012,500,000 PEZ
pub const PEZ_TREASURY_ALLOCATION: Balance = 1_012_500_000 * TYR;

/// Founder allocation: 1.875% = 93,750,000 PEZ
pub const PEZ_FOUNDER_ALLOCATION: Balance = 93_750_000 * TYR;

/// Presale allocation: 1.875% = 93,750,000 PEZ
pub const PEZ_PRESALE_ALLOCATION: Balance = 93_750_000 * TYR;

/// Rewards pool: 76% = 3,800,000,000 PEZ (distributed via sentetik halving)
pub const PEZ_REWARDS_POOL: Balance = 3_800_000_000 * TYR;

/// Total PEZ supply: 5 Billion (derived from allocations)
pub const PEZ_TOTAL_SUPPLY: Balance =
	PEZ_TREASURY_ALLOCATION + PEZ_FOUNDER_ALLOCATION + PEZ_PRESALE_ALLOCATION + PEZ_REWARDS_POOL;

// Compile-time verification that total equals expected 5 billion
const _: () = assert!(
	PEZ_TOTAL_SUPPLY == 5_000_000_000 * TYR,
	"PEZ allocations must sum to exactly 5 billion tokens"
);

/// Genesis configuration for Asset Hub Pezkuwichain
///
/// # Parameters
/// - `invulnerables`: Initial collators with their Aura keys
/// - `endowed_accounts`: Accounts to receive initial HEZ balance
/// - `endowment`: HEZ amount for each endowed account
/// - `id`: Parachain ID
/// - `treasury_account`: Account holding Treasury PEZ allocation
/// - `founder_account`: Account holding Founder PEZ allocation
/// - `presale_account`: Account holding Presale PEZ allocation
/// - `foreign_assets`: Foreign assets to create at genesis
/// - `foreign_assets_endowed_accounts`: Initial balances for foreign assets
fn asset_hub_pezkuwichain_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	endowment: Balance,
	id: ParaId,
	treasury_account: AccountId,
	founder_account: AccountId,
	presale_account: AccountId,
	foreign_assets: Vec<(Location, AccountId, Balance)>,
	foreign_assets_endowed_accounts: Vec<(Location, AccountId, Balance)>,
) -> serde_json::Value {
	// Verify total PEZ minted at genesis equals PEZ_TOTAL_SUPPLY (5 billion)
	debug_assert_eq!(
		PEZ_TREASURY_ALLOCATION
			+ PEZ_REWARDS_POOL
			+ PEZ_FOUNDER_ALLOCATION
			+ PEZ_PRESALE_ALLOCATION,
		PEZ_TOTAL_SUPPLY,
		"PEZ genesis allocations must equal total supply"
	);

	build_struct_json_patch!(RuntimeGenesisConfig {
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, endowment)).collect(),
		},
		teyrchain_info: TeyrchainInfoConfig { teyrchain_id: id },
		collator_selection: CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: ASSET_HUB_PEZKUWICHAIN_ED * 16,
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
		// TrustBackedAssets (Instance1) - PEZ and wHEZ tokens
		// ====================================================================
		assets: AssetsConfig {
			// Asset definitions: (id, owner, is_sufficient, min_balance)
			assets: vec![
				// PEZ Token - Governance token with 5B fixed supply
				(PEZ_ASSET_ID, treasury_account.clone(), true, 1),
				// wHEZ Token - Wrapped HEZ for DeFi operations
				(WHEZ_ASSET_ID, treasury_account.clone(), true, 1),
			],
			// Asset metadata: (id, name, symbol, decimals)
			metadata: vec![
				(PEZ_ASSET_ID, b"Pez Token".to_vec(), b"PEZ".to_vec(), PEZ_DECIMALS),
				(WHEZ_ASSET_ID, b"Wrapped HEZ".to_vec(), b"wHEZ".to_vec(), PEZ_DECIMALS),
			],
			// Initial balances: (asset_id, account, balance)
			accounts: vec![
				// Treasury gets: 20.25% + 76% rewards pool = 4,812,500,000 PEZ
				// Rewards will be distributed via pezpallet-pez-treasury with sentetik halving
				(
					PEZ_ASSET_ID,
					treasury_account.clone(),
					PEZ_TREASURY_ALLOCATION + PEZ_REWARDS_POOL
				),
				// Founder allocation: 1.875% = 93,750,000 PEZ (4 year vesting)
				(PEZ_ASSET_ID, founder_account.clone(), PEZ_FOUNDER_ALLOCATION),
				// Presale allocation: 1.875% = 93,750,000 PEZ
				(PEZ_ASSET_ID, presale_account.clone(), PEZ_PRESALE_ALLOCATION),
				// wHEZ starts with 0 balance - only created via TokenWrapper
			],
			// Next asset ID after PEZ and wHEZ
			next_asset_id: Some(3),
			..Default::default()
		},

		// ====================================================================
		// ForeignAssets (Instance2) - Bridged assets from other chains
		// ====================================================================
		foreign_assets: ForeignAssetsConfig {
			assets: foreign_assets
				.into_iter()
				.map(|asset| (asset.0.try_into().unwrap(), asset.1, false, asset.2))
				.collect(),
			accounts: foreign_assets_endowed_accounts
				.into_iter()
				.map(|asset| (asset.0.try_into().unwrap(), asset.1, asset.2))
				.collect(),
			..Default::default()
		},
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
		// Treasury, Founder, Presale accounts should be replaced with real addresses
		// ====================================================================
		PRESET_GENESIS => {
			// Placeholder accounts - MUST be replaced with real addresses from
			// Founder_treasury_presale_wallets.json before mainnet launch
			let treasury_account: AccountId =
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into();
			let founder_account: AccountId =
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into();
			let presale_account: AccountId =
				hex!("44cb62d1d6cdd2fff2a5ef3bb7ef827be5b3e117a394ecaa634d8dd9809d5608").into();

			asset_hub_pezkuwichain_genesis(
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
				ASSET_HUB_PEZKUWICHAIN_ED * 524_288,
				1000.into(),
				treasury_account,
				founder_account,
				presale_account,
				vec![],
				vec![],
			)
		},

		// ====================================================================
		// LOCAL TESTNET PRESET - For local multi-node testing (Alice + Bob)
		// ====================================================================
		pezsp_genesis_builder::LOCAL_TESTNET_RUNTIME_PRESET => {
			// For local testnet, Alice acts as treasury/founder/presale
			let treasury_account = Sr25519Keyring::Alice.to_account_id();
			let founder_account = Sr25519Keyring::Alice.to_account_id();
			let presale_account = Sr25519Keyring::Bob.to_account_id();

			asset_hub_pezkuwichain_genesis(
				// initial collators.
				vec![
					(Sr25519Keyring::Alice.to_account_id(), Sr25519Keyring::Alice.public().into()),
					(Sr25519Keyring::Bob.to_account_id(), Sr25519Keyring::Bob.public().into()),
				],
				Sr25519Keyring::well_known().map(|x| x.to_account_id()).collect(),
				testnet_teyrchains_constants::pezkuwichain::currency::UNITS * 1_000_000,
				1000.into(),
				treasury_account,
				founder_account,
				presale_account,
				vec![
					// bridged ZGR
					(
						Location::new(2, [GlobalConsensus(ZagrosNetwork::get())]),
						GlobalConsensusConvertsFor::<UniversalLocation, AccountId>::convert_location(
							&Location { parents: 2, interior: [GlobalConsensus(ZagrosNetwork::get())].into() },
						)
						.unwrap(),
						10_000_000,
					),
				],
				vec![
					// bridged ZGR to Bob
					(
						Location::new(2, [GlobalConsensus(ZagrosNetwork::get())]),
						Sr25519Keyring::Bob.to_account_id(),
						10_000_000 * 4096 * 4096,
					),
				],
			)
		},

		// ====================================================================
		// DEV PRESET - For single-node development (Alice only)
		// ====================================================================
		pezsp_genesis_builder::DEV_RUNTIME_PRESET => {
			// For dev, Alice acts as all special accounts
			let treasury_account = Sr25519Keyring::Alice.to_account_id();
			let founder_account = Sr25519Keyring::Alice.to_account_id();
			let presale_account = Sr25519Keyring::Alice.to_account_id();

			asset_hub_pezkuwichain_genesis(
				// initial collators.
				vec![(
					Sr25519Keyring::Alice.to_account_id(),
					Sr25519Keyring::Alice.public().into(),
				)],
				vec![
					Sr25519Keyring::Alice.to_account_id(),
					Sr25519Keyring::Bob.to_account_id(),
					Sr25519Keyring::AliceStash.to_account_id(),
					Sr25519Keyring::BobStash.to_account_id(),
				],
				TYR * 1_000_000,
				1000.into(),
				treasury_account,
				founder_account,
				presale_account,
				vec![],
				vec![],
			)
		},

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
