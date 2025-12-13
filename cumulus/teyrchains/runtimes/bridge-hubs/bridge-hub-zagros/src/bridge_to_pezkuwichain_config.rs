// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.
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

//! Bridge definitions used on BridgeHub with the Zagros flavor.

use crate::{
	bridge_common_config::BridgeRelayersInstance, weights, xcm_config::UniversalLocation,
	AccountId, Balance, Balances, BridgePezkuwichainMessages, PezkuwiXcm, Runtime, RuntimeEvent,
	RuntimeHoldReason, XcmOverBridgeHubPezkuwichain, XcmRouter, XcmpQueue,
};
use bp_messages::{
	source_chain::FromBridgedChainMessagesDeliveryProof,
	target_chain::FromBridgedChainMessagesProof, LegacyLaneId,
};
use bp_teyrchains::SingleParaStoredHeaderDataBuilder;
use bridge_hub_common::xcm_version::XcmVersionOfDestAndRemoteBridge;
use pallet_xcm_bridge_hub::{BridgeId, XcmAsPlainPayload};

use frame_support::{
	parameter_types,
	traits::{ConstU32, PalletInfoAccess},
};
use frame_system::{EnsureNever, EnsureRoot};
use pallet_bridge_messages::LaneIdOf;
use pallet_bridge_relayers::extension::{
	BridgeRelayersTransactionExtension, WithMessagesExtensionConfig,
};
use pezkuwi_teyrchain_primitives::primitives::Sibling;
use testnet_teyrchains_constants::zagros::currency::UNITS as ZGR;
use teyrchains_common::xcm_config::{AllSiblingSystemTeyrchains, RelayOrOtherSystemTeyrchains};
use xcm::{
	latest::{prelude::*, PEZKUWICHAIN_GENESIS_HASH},
	prelude::{InteriorLocation, NetworkId},
};
use xcm_builder::{BridgeBlobDispatcher, ParentIsPreset, SiblingTeyrchainConvertsVia};

parameter_types! {
	pub const RelayChainHeadersToKeep: u32 = 1024;
	pub const TeyrchainHeadsToKeep: u32 = 64;

	pub const PezkuwichainBridgeTeyrchainPalletName: &'static str = "Paras";
	pub const MaxPezkuwichainParaHeadDataSize: u32 = bp_pezkuwichain::MAX_NESTED_TEYRCHAIN_HEAD_DATA_SIZE;

	pub BridgeZagrosToPezkuwichainMessagesPalletInstance: InteriorLocation = [PalletInstance(<BridgePezkuwichainMessages as PalletInfoAccess>::index() as u8)].into();
	pub PezkuwichainGlobalConsensusNetwork: NetworkId = NetworkId::ByGenesis(PEZKUWICHAIN_GENESIS_HASH);
	pub PezkuwichainGlobalConsensusNetworkLocation: Location = Location::new(
		2,
		[GlobalConsensus(PezkuwichainGlobalConsensusNetwork::get())]
	);
	// see the `FEE_BOOST_PER_RELAY_HEADER` constant get the meaning of this value
	pub PriorityBoostPerRelayHeader: u64 = 32_007_814_407_814;
	// see the `FEE_BOOST_PER_TEYRCHAIN_HEADER` constant get the meaning of this value
	pub PriorityBoostPerTeyrchainHeader: u64 = 1_396_340_903_540_903;
	// see the `FEE_BOOST_PER_MESSAGE` constant to get the meaning of this value
	pub PriorityBoostPerMessage: u64 = 364_088_888_888_888;

	pub BridgeHubPezkuwichainLocation: Location = Location::new(
		2,
		[
			GlobalConsensus(PezkuwichainGlobalConsensusNetwork::get()),
			Teyrchain(<bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain as bp_runtime::Teyrchain>::TEYRCHAIN_ID)
		]
	);

	pub storage BridgeDeposit: Balance = 10 * ZGR;
	pub storage DeliveryRewardInBalance: u64 = 1_000_000;
}

/// Proof of messages, coming from Pezkuwichain.
pub type FromPezkuwichainBridgeHubMessagesProof<MI> =
	FromBridgedChainMessagesProof<bp_bridge_hub_pezkuwichain::Hash, LaneIdOf<Runtime, MI>>;
/// Messages delivery proof for Pezkuwichain Bridge Hub -> Zagros Bridge Hub messages.
pub type ToPezkuwichainBridgeHubMessagesDeliveryProof<MI> =
	FromBridgedChainMessagesDeliveryProof<bp_bridge_hub_pezkuwichain::Hash, LaneIdOf<Runtime, MI>>;

/// Dispatches received XCM messages from other bridge
type FromPezkuwichainMessageBlobDispatcher = BridgeBlobDispatcher<
	XcmRouter,
	UniversalLocation,
	BridgeZagrosToPezkuwichainMessagesPalletInstance,
>;

/// Transaction extension that refunds relayers that are delivering messages from the Pezkuwichain
/// teyrchain.
pub type OnBridgeHubZagrosRefundBridgeHubPezkuwichainMessages = BridgeRelayersTransactionExtension<
	Runtime,
	WithMessagesExtensionConfig<
		StrOnBridgeHubZagrosRefundBridgeHubPezkuwichainMessages,
		Runtime,
		WithBridgeHubPezkuwichainMessagesInstance,
		BridgeRelayersInstance,
		PriorityBoostPerMessage,
	>,
>;
bp_runtime::generate_static_str_provider!(OnBridgeHubZagrosRefundBridgeHubPezkuwichainMessages);

/// Add GRANDPA bridge pallet to track Pezkuwichain relay chain.
pub type BridgeGrandpaPezkuwichainInstance = pallet_bridge_grandpa::Instance1;
impl pallet_bridge_grandpa::Config<BridgeGrandpaPezkuwichainInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BridgedChain = bp_pezkuwichain::Pezkuwichain;
	type MaxFreeHeadersPerBlock = ConstU32<4>;
	type FreeHeadersInterval = ConstU32<5>;
	type HeadersToKeep = RelayChainHeadersToKeep;
	type WeightInfo = weights::pallet_bridge_grandpa::WeightInfo<Runtime>;
}

/// Add teyrchain bridge pallet to track Pezkuwichain BridgeHub teyrchain
pub type BridgeTeyrchainPezkuwichainInstance = pallet_bridge_teyrchains::Instance1;
impl pallet_bridge_teyrchains::Config<BridgeTeyrchainPezkuwichainInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_bridge_teyrchains::WeightInfo<Runtime>;
	type BridgesGrandpaPalletInstance = BridgeGrandpaPezkuwichainInstance;
	type ParasPalletName = PezkuwichainBridgeTeyrchainPalletName;
	type ParaStoredHeaderDataBuilder =
		SingleParaStoredHeaderDataBuilder<bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain>;
	type HeadsToKeep = TeyrchainHeadsToKeep;
	type MaxParaHeadDataSize = MaxPezkuwichainParaHeadDataSize;
	type OnNewHead = ();
}

/// Add XCM messages support for BridgeHubZagros to support Zagros->Pezkuwichain XCM messages
pub type WithBridgeHubPezkuwichainMessagesInstance = pallet_bridge_messages::Instance1;
impl pallet_bridge_messages::Config<WithBridgeHubPezkuwichainMessagesInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = weights::pallet_bridge_messages::WeightInfo<Runtime>;

	type ThisChain = bp_bridge_hub_zagros::BridgeHubZagros;
	type BridgedChain = bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain;
	type BridgedHeaderChain = pallet_bridge_teyrchains::TeyrchainHeaders<
		Runtime,
		BridgeTeyrchainPezkuwichainInstance,
		bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain,
	>;

	type OutboundPayload = XcmAsPlainPayload;
	type InboundPayload = XcmAsPlainPayload;
	type LaneId = LegacyLaneId;

	type DeliveryPayments = ();
	type DeliveryConfirmationPayments = pallet_bridge_relayers::DeliveryConfirmationPaymentsAdapter<
		Runtime,
		WithBridgeHubPezkuwichainMessagesInstance,
		BridgeRelayersInstance,
		DeliveryRewardInBalance,
	>;

	type MessageDispatch = XcmOverBridgeHubPezkuwichain;
	type OnMessagesDelivered = XcmOverBridgeHubPezkuwichain;
}

/// Add support for the export and dispatch of XCM programs.
pub type XcmOverBridgeHubPezkuwichainInstance = pallet_xcm_bridge_hub::Instance1;
impl pallet_xcm_bridge_hub::Config<XcmOverBridgeHubPezkuwichainInstance> for Runtime {
	type RuntimeEvent = RuntimeEvent;

	type UniversalLocation = UniversalLocation;
	type BridgedNetwork = PezkuwichainGlobalConsensusNetworkLocation;
	type BridgeMessagesPalletInstance = WithBridgeHubPezkuwichainMessagesInstance;

	type MessageExportPrice = ();
	type DestinationVersion =
		XcmVersionOfDestAndRemoteBridge<PezkuwiXcm, BridgeHubPezkuwichainLocation>;

	type ForceOrigin = EnsureRoot<AccountId>;
	// We don't want to allow creating bridges for this instance with `LegacyLaneId`.
	type OpenBridgeOrigin = EnsureNever<Location>;
	// Converter aligned with `OpenBridgeOrigin`.
	type BridgeOriginAccountIdConverter =
		(ParentIsPreset<AccountId>, SiblingTeyrchainConvertsVia<Sibling, AccountId>);

	type BridgeDeposit = BridgeDeposit;
	type Currency = Balances;
	type RuntimeHoldReason = RuntimeHoldReason;
	// Do not require deposit from system teyrchains or relay chain
	type AllowWithoutBridgeDeposit =
		RelayOrOtherSystemTeyrchains<AllSiblingSystemTeyrchains, Runtime>;

	type LocalXcmChannelManager = CongestionManager;
	type BlobDispatcher = FromPezkuwichainMessageBlobDispatcher;
}

/// Implementation of `bp_xcm_bridge_hub::LocalXcmChannelManager` for congestion management.
pub struct CongestionManager;
impl pallet_xcm_bridge_hub::LocalXcmChannelManager for CongestionManager {
	type Error = SendError;

	fn is_congested(with: &Location) -> bool {
		// This is used to check the inbound bridge queue/messages to determine if they can be
		// dispatched and sent to the sibling teyrchain. Therefore, checking outbound `XcmpQueue`
		// is sufficient here.
		use bp_xcm_bridge_hub_router::XcmChannelStatusProvider;
		cumulus_pallet_xcmp_queue::bridging::OutXcmpChannelStatusProvider::<Runtime>::is_congested(
			with,
		)
	}

	fn suspend_bridge(local_origin: &Location, bridge: BridgeId) -> Result<(), Self::Error> {
		// This bridge is intended for AH<>AH communication with a hard-coded/static lane,
		// so `local_origin` is expected to represent only the local AH.
		send_xcm::<XcmpQueue>(
			local_origin.clone(),
			bp_asset_hub_zagros::build_congestion_message(bridge.inner(), true).into(),
		)
		.map(|_| ())
	}

	fn resume_bridge(local_origin: &Location, bridge: BridgeId) -> Result<(), Self::Error> {
		// This bridge is intended for AH<>AH communication with a hard-coded/static lane,
		// so `local_origin` is expected to represent only the local AH.
		send_xcm::<XcmpQueue>(
			local_origin.clone(),
			bp_asset_hub_zagros::build_congestion_message(bridge.inner(), false).into(),
		)
		.map(|_| ())
	}
}

#[cfg(feature = "runtime-benchmarks")]
pub(crate) fn open_bridge_for_benchmarks<R, XBHI, C>(
	with: pallet_xcm_bridge_hub::LaneIdOf<R, XBHI>,
	sibling_para_id: u32,
) -> InteriorLocation
where
	R: pallet_xcm_bridge_hub::Config<XBHI>,
	XBHI: 'static,
	C: xcm_executor::traits::ConvertLocation<
		bp_runtime::AccountIdOf<pallet_xcm_bridge_hub::ThisChainOf<R, XBHI>>,
	>,
{
	use pallet_xcm_bridge_hub::{Bridge, BridgeId, BridgeState};
	use sp_runtime::traits::Zero;
	use xcm::{latest::ZAGROS_GENESIS_HASH, VersionedInteriorLocation};

	// insert bridge metadata
	let lane_id = with;
	let sibling_teyrchain = Location::new(1, [Teyrchain(sibling_para_id)]);
	let universal_source =
		[GlobalConsensus(ByGenesis(ZAGROS_GENESIS_HASH)), Teyrchain(sibling_para_id)].into();
	let universal_destination =
		[GlobalConsensus(ByGenesis(PEZKUWICHAIN_GENESIS_HASH)), Teyrchain(2075)].into();
	let bridge_id = BridgeId::new(&universal_source, &universal_destination);

	// insert only bridge metadata, because the benchmarks create lanes
	pallet_xcm_bridge_hub::Bridges::<R, XBHI>::insert(
		bridge_id,
		Bridge {
			bridge_origin_relative_location: alloc::boxed::Box::new(
				sibling_teyrchain.clone().into(),
			),
			bridge_origin_universal_location: alloc::boxed::Box::new(
				VersionedInteriorLocation::from(universal_source.clone()),
			),
			bridge_destination_universal_location: alloc::boxed::Box::new(
				VersionedInteriorLocation::from(universal_destination),
			),
			state: BridgeState::Opened,
			bridge_owner_account: C::convert_location(&sibling_teyrchain).expect("valid AccountId"),
			deposit: Zero::zero(),
			lane_id,
		},
	);
	pallet_xcm_bridge_hub::LaneToBridge::<R, XBHI>::insert(lane_id, bridge_id);

	universal_source
}

#[cfg(test)]
mod tests {
	use super::*;
	use bridge_runtime_common::{
		assert_complete_bridge_types,
		integrity::{
			assert_complete_with_teyrchain_bridge_constants, check_message_lane_weights,
			AssertChainConstants, AssertCompleteBridgeConstants,
		},
	};

	/// Every additional message in the message delivery transaction boosts its priority.
	/// So the priority of transaction with `N+1` messages is larger than priority of
	/// transaction with `N` messages by the `PriorityBoostPerMessage`.
	///
	/// Economically, it is an equivalent of adding tip to the transaction with `N` messages.
	/// The `FEE_BOOST_PER_MESSAGE` constant is the value of this tip.
	///
	/// We want this tip to be large enough (delivery transactions with more messages = less
	/// operational costs and a faster bridge), so this value should be significant.
	const FEE_BOOST_PER_MESSAGE: Balance = 2 * ZGR;

	// see `FEE_BOOST_PER_MESSAGE` comment
	const FEE_BOOST_PER_RELAY_HEADER: Balance = 2 * ZGR;
	// see `FEE_BOOST_PER_MESSAGE` comment
	const FEE_BOOST_PER_TEYRCHAIN_HEADER: Balance = 2 * ZGR;

	#[test]
	fn ensure_bridge_hub_zagros_message_lane_weights_are_correct() {
		check_message_lane_weights::<
			bp_bridge_hub_zagros::BridgeHubZagros,
			Runtime,
			WithBridgeHubPezkuwichainMessagesInstance,
		>(
			bp_bridge_hub_pezkuwichain::EXTRA_STORAGE_PROOF_SIZE,
			bp_bridge_hub_zagros::MAX_UNREWARDED_RELAYERS_IN_CONFIRMATION_TX,
			bp_bridge_hub_zagros::MAX_UNCONFIRMED_MESSAGES_IN_CONFIRMATION_TX,
			true,
		);
	}

	#[test]
	fn ensure_bridge_integrity() {
		assert_complete_bridge_types!(
			runtime: Runtime,
			with_bridged_chain_messages_instance: WithBridgeHubPezkuwichainMessagesInstance,
			this_chain: bp_bridge_hub_zagros::BridgeHubZagros,
			bridged_chain: bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain,
			expected_payload_type: XcmAsPlainPayload,
		);

		assert_complete_with_teyrchain_bridge_constants::<
			Runtime,
			BridgeGrandpaPezkuwichainInstance,
			WithBridgeHubPezkuwichainMessagesInstance,
		>(AssertCompleteBridgeConstants {
			this_chain_constants: AssertChainConstants {
				block_length: bp_bridge_hub_zagros::BlockLength::get(),
				block_weights: bp_bridge_hub_zagros::BlockWeightsForAsyncBacking::get(),
			},
		});

		pallet_bridge_relayers::extension::per_relay_header::ensure_priority_boost_is_sane::<
			Runtime,
			BridgeGrandpaPezkuwichainInstance,
			PriorityBoostPerRelayHeader,
		>(FEE_BOOST_PER_RELAY_HEADER);

		pallet_bridge_relayers::extension::per_teyrchain_header::ensure_priority_boost_is_sane::<
			Runtime,
			WithBridgeHubPezkuwichainMessagesInstance,
			bp_bridge_hub_pezkuwichain::BridgeHubPezkuwichain,
			PriorityBoostPerTeyrchainHeader,
		>(FEE_BOOST_PER_TEYRCHAIN_HEADER);

		pallet_bridge_relayers::extension::per_message::ensure_priority_boost_is_sane::<
			Runtime,
			WithBridgeHubPezkuwichainMessagesInstance,
			PriorityBoostPerMessage,
		>(FEE_BOOST_PER_MESSAGE);

		assert_eq!(
			BridgeZagrosToPezkuwichainMessagesPalletInstance::get(),
			[PalletInstance(
				bp_bridge_hub_zagros::WITH_BRIDGE_ZAGROS_TO_PEZKUWICHAIN_MESSAGES_PALLET_INDEX
			)]
		);
	}
}

/// Contains the migration for the AssetHubZagros<>AssetHubPezkuwichain bridge.
pub mod migration {
	use super::*;
	use bp_messages::LegacyLaneId;

	parameter_types! {
		pub AssetHubZagrosToAssetHubPezkuwichainMessagesLane: LegacyLaneId = LegacyLaneId([0, 0, 0, 2]);
		pub AssetHubZagrosLocation: Location = Location::new(1, [Teyrchain(bp_asset_hub_zagros::ASSET_HUB_ZAGROS_TEYRCHAIN_ID)]);
		pub AssetHubPezkuwichainUniversalLocation: InteriorLocation = [GlobalConsensus(PezkuwichainGlobalConsensusNetwork::get()), Teyrchain(bp_asset_hub_pezkuwichain::ASSET_HUB_PEZKUWICHAIN_TEYRCHAIN_ID)].into();
	}

	mod v1_wrong {
		use bp_messages::{LaneState, MessageNonce, UnrewardedRelayer};
		use bp_runtime::AccountIdOf;
		use codec::{Decode, Encode};
		use pallet_bridge_messages::BridgedChainOf;
		use sp_std::collections::vec_deque::VecDeque;

		#[derive(Encode, Decode, Clone, PartialEq, Eq)]
		pub(crate) struct StoredInboundLaneData<T: pallet_bridge_messages::Config<I>, I: 'static>(
			pub(crate) InboundLaneData<AccountIdOf<BridgedChainOf<T, I>>>,
		);
		#[derive(Encode, Decode, Clone, PartialEq, Eq)]
		pub(crate) struct InboundLaneData<RelayerId> {
			pub state: LaneState,
			pub(crate) relayers: VecDeque<UnrewardedRelayer<RelayerId>>,
			pub(crate) last_confirmed_nonce: MessageNonce,
		}
		#[derive(Encode, Decode, Clone, PartialEq, Eq)]
		pub(crate) struct OutboundLaneData {
			pub state: LaneState,
			pub(crate) oldest_unpruned_nonce: MessageNonce,
			pub(crate) latest_received_nonce: MessageNonce,
			pub(crate) latest_generated_nonce: MessageNonce,
		}
	}

	mod v1 {
		pub use bp_messages::{InboundLaneData, LaneState, OutboundLaneData};
		pub use pallet_bridge_messages::{InboundLanes, OutboundLanes, StoredInboundLaneData};
	}

	/// Fix for v1 migration - corrects data for OutboundLaneData/InboundLaneData (it is needed only
	/// for Pezkuwichain/Zagros).
	pub struct FixMessagesV1Migration<T, I>(sp_std::marker::PhantomData<(T, I)>);

	impl<T: pallet_bridge_messages::Config<I>, I: 'static> frame_support::traits::OnRuntimeUpgrade
		for FixMessagesV1Migration<T, I>
	{
		fn on_runtime_upgrade() -> Weight {
			use sp_core::Get;
			let mut weight = T::DbWeight::get().reads(1);

			// `InboundLanes` - add state to the old structs
			let translate_inbound =
				|pre: v1_wrong::StoredInboundLaneData<T, I>| -> Option<v1::StoredInboundLaneData<T, I>> {
					weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
					Some(v1::StoredInboundLaneData(v1::InboundLaneData {
						state: v1::LaneState::Opened,
						relayers: pre.0.relayers,
						last_confirmed_nonce: pre.0.last_confirmed_nonce,
					}))
				};
			v1::InboundLanes::<T, I>::translate_values(translate_inbound);

			// `OutboundLanes` - add state to the old structs
			let translate_outbound =
				|pre: v1_wrong::OutboundLaneData| -> Option<v1::OutboundLaneData> {
					weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
					Some(v1::OutboundLaneData {
						state: v1::LaneState::Opened,
						oldest_unpruned_nonce: pre.oldest_unpruned_nonce,
						latest_received_nonce: pre.latest_received_nonce,
						latest_generated_nonce: pre.latest_generated_nonce,
					})
				};
			v1::OutboundLanes::<T, I>::translate_values(translate_outbound);

			weight
		}
	}
}
