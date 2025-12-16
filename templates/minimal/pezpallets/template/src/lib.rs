//! A shell pezpallet built with [`frame`].
//!
//! To get started with this pezpallet, try implementing the guide in
//! <https://docs.pezkuwichain.io/sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html>

#![cfg_attr(not(feature = "std"), no_std)]

use frame::prelude::*;
use pezkuwi_sdk::pezkuwi_sdk_frame as frame;

// Re-export all pezpallet parts, this is needed to properly import the pezpallet into the runtime.
pub use pezpallet::*;

#[frame::pezpallet]
pub mod pezpallet {
	use super::*;

	#[pezpallet::config]
	pub trait Config: pezkuwi_sdk::pezframe_system::Config {}

	#[pezpallet::pezpallet]
	pub struct Pezpallet<T>(_);

	#[pezpallet::storage]
	pub type Value<T> = StorageValue<Value = u32>;
}
