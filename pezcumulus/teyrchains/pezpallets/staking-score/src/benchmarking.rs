//! Benchmarking setup for pezpallet-staking-score

use crate::{
	CachedStakingDetails, Call, Config, Pezpallet, StakingDetails, StakingSource,
	StakingStartBlock, UNITS,
};
use pezframe_benchmarking::v2::*;
use pezframe_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn start_score_tracking() {
		let caller: T::AccountId = whitelisted_caller();

		// Populate CachedStakingDetails with test data
		CachedStakingDetails::<T>::insert(
			&caller,
			StakingSource::RelayChain,
			StakingDetails {
				staked_amount: (1000u128 * UNITS).into(),
				nominations_count: 5,
				unlocking_chunks_count: 2,
			},
		);

		StakingStartBlock::<T>::remove(&caller);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()));

		assert!(StakingStartBlock::<T>::get(&caller).is_some());
	}

	#[benchmark]
	fn receive_staking_details() {
		let target: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			target.clone(),
			StakingSource::RelayChain,
			(500u128 * UNITS).into(),
			3u32,
			0u32,
		);

		assert!(CachedStakingDetails::<T>::get(&target, StakingSource::RelayChain).is_some());
	}
}
