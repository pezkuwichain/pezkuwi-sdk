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

pub use asset_hub_zagros_runtime;

pub mod genesis;

// Substrate
use frame_support::traits::OnInitialize;

// Cumulus
use asset_hub_zagros_runtime::ForeignAssetReserveData;
use emulated_integration_tests_common::{
	impl_accounts_helpers_for_teyrchain, impl_assert_events_helpers_for_teyrchain,
	impl_assets_helpers_for_system_teyrchain, impl_assets_helpers_for_teyrchain,
	impl_bridge_helpers_for_chain, impl_foreign_assets_helpers_for_teyrchain,
	impl_xcm_helpers_for_teyrchain, impls::Teyrchain, xcm_emulator::decl_test_teyrchains,
	AuraDigestProvider,
};
use zagros_emulated_chain::Zagros;

// AssetHubZagros Teyrchain declaration
decl_test_teyrchains! {
	pub struct AssetHubZagros {
		genesis = genesis::genesis(),
		on_init = {
			asset_hub_zagros_runtime::AuraExt::on_initialize(1);
		},
		runtime = asset_hub_zagros_runtime,
		core = {
			XcmpMessageHandler: asset_hub_zagros_runtime::XcmpQueue,
			LocationToAccountId: asset_hub_zagros_runtime::xcm_config::LocationToAccountId,
			TeyrchainInfo: asset_hub_zagros_runtime::TeyrchainInfo,
			MessageOrigin: cumulus_primitives_core::AggregateMessageOrigin,
			DigestProvider: AuraDigestProvider,
			AdditionalInherentCode: (),
		},
		pallets = {
			PezkuwiXcm: asset_hub_zagros_runtime::PezkuwiXcm,
			Balances: asset_hub_zagros_runtime::Balances,
			Assets: asset_hub_zagros_runtime::Assets,
			ForeignAssets: asset_hub_zagros_runtime::ForeignAssets,
			PoolAssets: asset_hub_zagros_runtime::PoolAssets,
			AssetConversion: asset_hub_zagros_runtime::AssetConversion,
			SnowbridgeSystemFrontend: asset_hub_zagros_runtime::SnowbridgeSystemFrontend,
			Revive: asset_hub_zagros_runtime::Revive,
		}
	},
}

// AssetHubZagros implementation
impl_accounts_helpers_for_teyrchain!(AssetHubZagros);
impl_assert_events_helpers_for_teyrchain!(AssetHubZagros);
impl_assets_helpers_for_system_teyrchain!(AssetHubZagros, Zagros);
impl_assets_helpers_for_teyrchain!(AssetHubZagros);
impl_foreign_assets_helpers_for_teyrchain!(
	AssetHubZagros,
	xcm::v5::Location,
	ForeignAssetReserveData
);
impl_xcm_helpers_for_teyrchain!(AssetHubZagros);
impl_bridge_helpers_for_chain!(
	AssetHubZagros,
	ParaPallet,
	PezkuwiXcm,
	bp_bridge_hub_zagros::RuntimeCall::XcmOverBridgeHubPezkuwichain
);
