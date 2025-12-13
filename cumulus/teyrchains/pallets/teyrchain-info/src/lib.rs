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

//! Minimal Pallet that injects a TeyrchainId into Runtime storage from

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use cumulus_primitives_core::ParaId;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		#[serde(skip)]
		pub _config: core::marker::PhantomData<T>,
		pub teyrchain_id: ParaId,
	}

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { teyrchain_id: 100.into(), _config: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			TeyrchainId::<T>::put(self.teyrchain_id);
		}
	}

	#[pallet::type_value]
	pub(super) fn DefaultForTeyrchainId() -> ParaId {
		100.into()
	}

	#[pallet::storage]
	pub(super) type TeyrchainId<T: Config> =
		StorageValue<_, ParaId, ValueQuery, DefaultForTeyrchainId>;

	impl<T: Config> Get<ParaId> for Pallet<T> {
		fn get() -> ParaId {
			TeyrchainId::<T>::get()
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn teyrchain_id() -> ParaId {
			TeyrchainId::<T>::get()
		}
	}
}
