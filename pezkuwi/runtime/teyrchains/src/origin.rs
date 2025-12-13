// Copyright (C) Parity Technologies (UK) Ltd.
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

//! Declaration of the teyrchain specific origin and a pallet that hosts it.

use core::result;
use pezkuwi_primitives::Id as ParaId;
use sp_runtime::traits::BadOrigin;

pub use pallet::*;

/// Ensure that the origin `o` represents a teyrchain.
/// Returns `Ok` with the teyrchain ID that effected the extrinsic or an `Err` otherwise.
pub fn ensure_teyrchain<OuterOrigin>(o: OuterOrigin) -> result::Result<ParaId, BadOrigin>
where
	OuterOrigin: Into<result::Result<Origin, OuterOrigin>>,
{
	match o.into() {
		Ok(Origin::Teyrchain(id)) => Ok(id),
		_ => Err(BadOrigin),
	}
}

/// There is no way to register an origin type in `construct_runtime` without a pallet the origin
/// belongs to.
///
/// This module fulfills only the single purpose of housing the `Origin` in `construct_runtime`.
// ideally, though, the `construct_runtime` should support a free-standing origin.
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	/// Origin for the teyrchains.
	#[pallet::origin]
	#[derive(
		PartialEq,
		Eq,
		Clone,
		Encode,
		Decode,
		DecodeWithMemTracking,
		Debug,
		scale_info::TypeInfo,
		MaxEncodedLen,
	)]
	pub enum Origin {
		/// It comes from a teyrchain.
		Teyrchain(ParaId),
	}
}

impl From<u32> for Origin {
	fn from(id: u32) -> Origin {
		Origin::Teyrchain(id.into())
	}
}
