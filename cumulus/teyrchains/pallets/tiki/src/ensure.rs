//! Custom Origin verification mechanisms based on Tiki ownership.
//!
//! This module provides `EnsureOrigin` implementations that verify
//! the caller holds a specific Tiki role (Serok, Wezir, or Parlementer).
//!
//! # Feature Unification Note
//!
//! Due to Cargo's feature unification behavior, this pallet must be excluded
//! from `cargo check --benches` operations when its `runtime-benchmarks` feature
//! is not explicitly enabled. The CI workflow handles this appropriately.
//!
//! The issue: When building with `--benches`, other packages may enable
//! `frame-support/runtime-benchmarks`, which makes `EnsureOrigin::try_successful_origin`
//! a required trait method. However, if `pallet-tiki/runtime-benchmarks` is not enabled,
//! our cfg-gated method won't be compiled, causing E0046 errors.

use crate::{Config, Pallet as TikiPallet};
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use sp_std::marker::PhantomData;

// --- Marker Trait for Tiki Roles ---

/// A trait to return a specific `Tiki` enum variant.
///
/// This trait is implemented by marker structs to identify which
/// Tiki role is required for origin verification.
pub trait GetTiki {
	/// Returns the specific Tiki variant this marker represents.
	fn tiki() -> crate::Tiki;
}

// --- Marker Structs for Each Role ---

/// Marker struct representing the `Serok` (President/Leader) role.
///
/// Use with `EnsureTiki` to require the caller holds the Serok Tiki:
/// ```ignore
/// type SerokOrigin = EnsureTiki<Runtime, SerokRole>;
/// ```
pub struct SerokRole;

impl GetTiki for SerokRole {
	fn tiki() -> crate::Tiki {
		crate::Tiki::Serok
	}
}

/// Marker struct representing the `Wezir` (Minister/Advisor) role.
///
/// Use with `EnsureTiki` to require the caller holds the Wezir Tiki:
/// ```ignore
/// type WezirOrigin = EnsureTiki<Runtime, WezirRole>;
/// ```
pub struct WezirRole;

impl GetTiki for WezirRole {
	fn tiki() -> crate::Tiki {
		crate::Tiki::Wezir
	}
}

/// Marker struct representing the `Parlementer` (Parliamentarian) role.
///
/// Use with `EnsureTiki` to require the caller holds the Parlementer Tiki:
/// ```ignore
/// type ParlementerOrigin = EnsureTiki<Runtime, ParlementerRole>;
/// ```
pub struct ParlementerRole;

impl GetTiki for ParlementerRole {
	fn tiki() -> crate::Tiki {
		crate::Tiki::Parlementer
	}
}

// --- EnsureOrigin Implementation ---

/// An `EnsureOrigin` implementation that requires ownership of a specific Tiki.
///
/// This struct verifies that the origin is a signed account that currently
/// holds the Tiki role specified by the `I: GetTiki` type parameter.
///
/// # Type Parameters
///
/// * `T` - The runtime configuration type implementing `Config`
/// * `I` - A marker type implementing `GetTiki` to specify which Tiki role is required
///
/// # Example
///
/// ```ignore
/// // Require the caller to hold the Serok Tiki
/// type SerokOrigin = EnsureTiki<Runtime, SerokRole>;
///
/// // Use in a pallet's dispatchable
/// #[pallet::call]
/// impl<T: Config> Pallet<T> {
///     pub fn privileged_action(origin: OriginFor<T>) -> DispatchResult {
///         let who = T::SerokOrigin::ensure_origin(origin)?;
///         // ... action requiring Serok authority
///     }
/// }
/// ```
pub struct EnsureTiki<T, I>(PhantomData<(T, I)>);

impl<T, I> EnsureOrigin<T::RuntimeOrigin> for EnsureTiki<T, I>
where
	T: Config,
	I: GetTiki,
{
	type Success = T::AccountId;

	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		// First, verify the origin is a signed account
		let who = match ensure_signed(o.clone()) {
			Ok(account) => account,
			Err(_) => return Err(o),
		};

		// Get the required Tiki role from the marker type
		let required_tiki = I::tiki();

		// Check if the caller currently holds this Tiki
		match TikiPallet::<T>::tiki_holder(required_tiki) {
			Some(holder) if holder == who => Ok(who),
			_ => Err(o),
		}
	}

	/// Returns a successful origin for benchmarking purposes.
	///
	/// This method generates a deterministic account that can be used
	/// when running runtime benchmarks. It does NOT grant actual Tiki
	/// ownership - benchmarks must separately set up the Tiki holder state.
	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		use codec::Decode;
		use sp_runtime::traits::TrailingZeroInput;

		// Generate a deterministic zero-filled account for benchmarking
		let zero_account = T::AccountId::decode(&mut TrailingZeroInput::zeroes())
			.expect("infinite length input; no invalid inputs for type; qed");

		Ok(T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(zero_account)))
	}
}

// Convenience type aliases
pub type EnsureSerok<T> = EnsureTiki<T, SerokRole>;
pub type EnsureWezir<T> = EnsureTiki<T, WezirRole>;
pub type EnsureParlementer<T> = EnsureTiki<T, ParlementerRole>;
