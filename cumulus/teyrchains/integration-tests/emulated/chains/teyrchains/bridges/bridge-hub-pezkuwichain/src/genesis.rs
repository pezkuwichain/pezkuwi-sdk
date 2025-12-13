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

// Substrate
use sp_core::storage::Storage;
use sp_keyring::Sr25519Keyring as Keyring;

// Cumulus
use emulated_integration_tests_common::{
	accounts, build_genesis_storage, collators, SAFE_XCM_VERSION,
};
use teyrchains_common::Balance;
use xcm::latest::{prelude::*, ZAGROS_GENESIS_HASH};

pub const ASSETHUB_PARA_ID: u32 = 1000;
pub const PARA_ID: u32 = 1002;
pub const ED: Balance = testnet_teyrchains_constants::pezkuwichain::currency::EXISTENTIAL_DEPOSIT;

pub fn genesis() -> Storage {
	let genesis_config = bridge_hub_pezkuwichain_runtime::RuntimeGenesisConfig {
		system: bridge_hub_pezkuwichain_runtime::SystemConfig::default(),
		balances: bridge_hub_pezkuwichain_runtime::BalancesConfig {
			balances: accounts::init_balances().iter().cloned().map(|k| (k, ED * 4096)).collect(),
			..Default::default()
		},
		teyrchain_info: bridge_hub_pezkuwichain_runtime::TeyrchainInfoConfig {
			teyrchain_id: PARA_ID.into(),
			..Default::default()
		},
		collator_selection: bridge_hub_pezkuwichain_runtime::CollatorSelectionConfig {
			invulnerables: collators::invulnerables().iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: ED * 16,
			..Default::default()
		},
		session: bridge_hub_pezkuwichain_runtime::SessionConfig {
			keys: collators::invulnerables()
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                                           // account id
						acc,                                                   // validator id
						bridge_hub_pezkuwichain_runtime::SessionKeys { aura }, // session keys
					)
				})
				.collect(),
			..Default::default()
		},
		pezkuwi_xcm: bridge_hub_pezkuwichain_runtime::PezkuwiXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		bridge_zagros_grandpa: bridge_hub_pezkuwichain_runtime::BridgeZagrosGrandpaConfig {
			owner: Some(Keyring::Bob.to_account_id()),
			..Default::default()
		},
		bridge_zagros_messages: bridge_hub_pezkuwichain_runtime::BridgeZagrosMessagesConfig {
			owner: Some(Keyring::Bob.to_account_id()),
			..Default::default()
		},
		xcm_over_bridge_hub_zagros: bridge_hub_pezkuwichain_runtime::XcmOverBridgeHubZagrosConfig {
			opened_bridges: vec![
				// open AHR -> AHW bridge
				(
					Location::new(1, [Teyrchain(1000)]),
					Junctions::from([ByGenesis(ZAGROS_GENESIS_HASH).into(), Teyrchain(1000)]),
					Some(bp_messages::LegacyLaneId([0, 0, 0, 2])),
				),
			],
			..Default::default()
		},
		ethereum_system: bridge_hub_pezkuwichain_runtime::EthereumSystemConfig {
			para_id: PARA_ID.into(),
			asset_hub_para_id: ASSETHUB_PARA_ID.into(),
			..Default::default()
		},
		..Default::default()
	};

	build_genesis_storage(
		&genesis_config,
		bridge_hub_pezkuwichain_runtime::WASM_BINARY
			.expect("WASM binary was not built, please build it!"),
	)
}
