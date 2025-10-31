#![cfg_attr(not(feature = "std"), no_std)]

//! # Token Wrapper Pallet
//!
//! A pallet for wrapping native tokens (HEZ) into fungible assets (wHEZ)
//! to enable DEX operations between native and asset tokens.
//!
//! ## Overview
//!
//! This pallet provides:
//! - `wrap`: Convert native HEZ to wHEZ (Asset ID 0)
//! - `unwrap`: Convert wHEZ back to native HEZ
//!
//! The pallet maintains a 1:1 backing between HEZ and wHEZ.

pub use pallet::*;
pub mod weights;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{
        Currency, ExistenceRequirement,
        fungibles::{Inspect, Mutate, Create},
    },
    PalletId,
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::{AccountIdConversion, Zero, Saturating};

/// Weight functions trait for this pallet.
pub trait WeightInfo {
    fn wrap() -> frame_support::weights::Weight;
    fn unwrap() -> frame_support::weights::Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: crate::WeightInfo;

        /// Native currency (HEZ)
        type Currency: Currency<Self::AccountId>;

        /// Fungible assets (for wHEZ)
        type Assets: Inspect<Self::AccountId, AssetId = u32, Balance = BalanceOf<Self>>
            + Mutate<Self::AccountId>
            + Create<Self::AccountId>;

        /// Pallet ID for the wrapper account
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Asset ID for wrapped token (wHEZ)
        #[pallet::constant]
        type WrapperAssetId: Get<u32>;
    }

    // ============================================================================
    // STORAGE ITEMS
    // ============================================================================

    /// Total amount of native tokens locked in wrapper
    #[pallet::storage]
    #[pallet::getter(fn total_locked)]
    pub type TotalLocked<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

    // ============================================================================
    // EVENTS
    // ============================================================================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Native token wrapped into asset token. [who, amount]
        Wrapped { 
            who: T::AccountId, 
            amount: BalanceOf<T> 
        },
        /// Asset token unwrapped back to native. [who, amount]
        Unwrapped { 
            who: T::AccountId, 
            amount: BalanceOf<T> 
        },
    }

    // ============================================================================
    // ERRORS
    // ============================================================================

    #[pallet::error]
    pub enum Error<T> {
        /// Insufficient balance for wrapping
        InsufficientBalance,
        /// Insufficient wrapped tokens for unwrapping
        InsufficientWrappedBalance,
        /// Transfer failed
        TransferFailed,
        /// Mint failed
        MintFailed,
        /// Burn failed
        BurnFailed,
        /// Amount is zero
        ZeroAmount,
    }

    // ============================================================================
    // DISPATCHABLE FUNCTIONS
    // ============================================================================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Wrap native tokens (HEZ) into wrapped asset tokens (wHEZ)
        ///
        /// - `amount`: The amount of native tokens to wrap
        ///
        /// This will:
        /// 1. Transfer native tokens from user to pallet account (lock)
        /// 2. Mint equivalent amount of wrapped tokens to user
        ///
        /// Emits `Wrapped` event.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::wrap())]
        pub fn wrap(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure amount is not zero
            ensure!(!amount.is_zero(), Error::<T>::ZeroAmount);

            // Check balance
            ensure!(
                T::Currency::free_balance(&who) >= amount,
                Error::<T>::InsufficientBalance
            );

            // Transfer native tokens to pallet account (lock them)
            T::Currency::transfer(
                &who,
                &Self::account_id(),
                amount,
                ExistenceRequirement::KeepAlive,
            )
            .map_err(|_| Error::<T>::TransferFailed)?;

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_add(amount);
            });

            // Mint wrapped tokens to user
            T::Assets::mint_into(T::WrapperAssetId::get(), &who, amount)
                .map_err(|_| Error::<T>::MintFailed)?;

            Self::deposit_event(Event::Wrapped { who, amount });
            Ok(())
        }

        /// Unwrap wrapped asset tokens (wHEZ) back to native tokens (HEZ)
        ///
        /// - `amount`: The amount of wrapped tokens to unwrap
        ///
        /// This will:
        /// 1. Burn wrapped tokens from user
        /// 2. Transfer equivalent native tokens back to user (unlock)
        ///
        /// Emits `Unwrapped` event.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::unwrap())]
        pub fn unwrap(
            origin: OriginFor<T>,
            #[pallet::compact] amount: BalanceOf<T>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Ensure amount is not zero
            ensure!(!amount.is_zero(), Error::<T>::ZeroAmount);

            // Check wrapped token balance
            let wrapped_balance = T::Assets::balance(T::WrapperAssetId::get(), &who);
            ensure!(
                wrapped_balance >= amount,
                Error::<T>::InsufficientWrappedBalance
            );

            // Burn wrapped tokens from user
            T::Assets::burn_from(
                T::WrapperAssetId::get(),
                &who,
                amount,
                frame_support::traits::tokens::Preservation::Expendable,
                frame_support::traits::tokens::Precision::Exact,
                frame_support::traits::tokens::Fortitude::Force,
            )
            .map_err(|_| Error::<T>::BurnFailed)?;

            // Update total locked
            TotalLocked::<T>::mutate(|total| {
                *total = total.saturating_sub(amount);
            });

            // Transfer native tokens back to user (unlock)
            T::Currency::transfer(
                &Self::account_id(),
                &who,
                amount,
                ExistenceRequirement::AllowDeath,
            )
            .map_err(|_| Error::<T>::TransferFailed)?;

            Self::deposit_event(Event::Unwrapped { who, amount });
            Ok(())
        }
    }

    // ============================================================================
    // HELPER FUNCTIONS
    // ============================================================================

    impl<T: Config> Pallet<T> {
        /// Get the account ID of the pallet
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        /// Get the total supply of wrapped tokens
        pub fn total_wrapped() -> BalanceOf<T> {
            T::Assets::total_issuance(T::WrapperAssetId::get())
        }
    }
}