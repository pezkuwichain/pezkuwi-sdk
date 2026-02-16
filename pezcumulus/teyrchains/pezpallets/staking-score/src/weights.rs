// This file is part of Bizinikiwi.

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

//! Manually estimated weights for `pezpallet_staking_score`
//!
//! These weights are conservative overestimates pending proper benchmark runs.
//! They account for the `OnStakingUpdate` callback cost (trust pallet update).
//!
//! DATE: 2026-02-16
//! TODO: Run proper benchmarks to replace these estimates.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]
#![allow(dead_code)]

use pezframe_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pezpallet_staking_score`.
pub trait WeightInfo {
	fn start_score_tracking() -> Weight;
	fn receive_staking_details() -> Weight;
}

/// Weights for `pezpallet_staking_score` using the Bizinikiwi node and recommended hardware.
pub struct BizinikiwiWeight<T>(PhantomData<T>);
impl<T: pezframe_system::Config> WeightInfo for BizinikiwiWeight<T> {
	/// Storage: `StakingScore::StakingStartBlock` (r:1 w:1)
	/// Storage: `StakingScore::CachedStakingDetails` (r:2 w:0) -- iter_prefix worst case 2 sources
	/// Storage: `Trust::TrustScores` (r:1 w:1) -- OnStakingUpdate callback
	/// Storage: `Trust::TotalActiveTrustScore` (r:1 w:1) -- OnStakingUpdate callback
	/// Storage: `IdentityKyc::KycStatuses` (r:1 w:0) -- citizenship check
	/// Storage: `StakingScore` reads for score calc (r:2 w:0)
	///
	/// Total: r:8 w:3
	fn start_score_tracking() -> Weight {
		// Conservative estimate: ~35 microseconds execution + 8 reads + 3 writes
		// Proof size: StakingStartBlock(52) + CachedStakingDetails(77*2) + TrustScores(48) +
		//             TotalActiveTrustScore(16) + KycStatuses(34) + overhead = ~8000
		Weight::from_parts(35_000_000, 8_000)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}

	/// Storage: `StakingScore::CachedStakingDetails` (r:1 w:1) -- DoubleMap insert
	/// Storage: `Trust::TrustScores` (r:1 w:1) -- OnStakingUpdate callback
	/// Storage: `Trust::TotalActiveTrustScore` (r:1 w:1) -- OnStakingUpdate callback
	/// Storage: `IdentityKyc::KycStatuses` (r:1 w:0) -- citizenship check
	/// Storage: `StakingScore` reads for score calc (r:2 w:0)
	///
	/// Total: r:6 w:3
	fn receive_staking_details() -> Weight {
		// Conservative estimate: ~30 microseconds execution + 6 reads + 3 writes
		// Proof size: CachedStakingDetails(77) + TrustScores(48) +
		//             TotalActiveTrustScore(16) + KycStatuses(34) + overhead = ~7000
		Weight::from_parts(30_000_000, 7_000)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	fn start_score_tracking() -> Weight {
		Weight::from_parts(35_000_000, 8_000)
			.saturating_add(RocksDbWeight::get().reads(8_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}

	fn receive_staking_details() -> Weight {
		Weight::from_parts(30_000_000, 7_000)
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
