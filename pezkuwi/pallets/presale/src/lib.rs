#![cfg_attr(not(feature = "std"), no_std)]

//! # Pallet Presale - Multi-Presale Launchpad Platform
//!
//! ## Overview
//!
//! A comprehensive multi-presale launchpad platform for PezkuwiChain that allows:
//! - Multiple simultaneous presales with independent configurations
//! - Platform fee collection (2%): 50% treasury, 25% burn, 25% stakers
//! - Refund system with grace period (24h low fee, after higher fee)
//! - Contribution limits (min/max per wallet, hard cap)
//! - Whitelist/KYC support for compliance
//! - Vesting schedules for gradual token release
//! - Bonus tier system for larger contributors
//! - Emergency controls and governance integration
//!
//! ## Features
//!
//! - **Multi-Presale**: Unlimited simultaneous presales
//! - **Configurable**: Any asset, rate, duration per presale
//! - **Platform Fee**: 2% split (50% treasury, 25% burn, 25% stakers)
//! - **Refunds**: Grace period with reduced fees
//! - **Limits**: Min/max contribution, hard cap
//! - **Whitelist**: Optional whitelist/KYC for presales
//! - **Vesting**: Linear token release schedules
//! - **Bonus Tiers**: Reward larger contributions
//! - **Emergency**: Pause, cancel, withdrawal controls

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::{
        dispatch::DispatchResult,
        pallet_prelude::*,
        traits::{
            fungibles::Mutate,
            tokens::{Preservation, Fortitude, Precision},
        },
        PalletId, BoundedVec,
    };
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Saturating};
    use codec::{Encode, Decode, MaxEncodedLen};

    pub type PresaleId = u32;

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    pub enum PresaleStatus {
        Active,
        Finalized,
        Cancelled,
    }

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    #[codec(dumb_trait_bound)]
    pub enum AccessControl {
        Public,              // Anyone can contribute
        Whitelist,           // Only whitelisted accounts
    }

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    #[codec(dumb_trait_bound)]
    pub struct BonusTier {
        /// Minimum contribution to qualify (in payment asset units)
        pub min_contribution: u128,
        /// Bonus percentage (0-100)
        pub bonus_percentage: u8,
    }

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    #[codec(dumb_trait_bound)]
    pub struct VestingSchedule<BlockNumber> {
        /// Percentage released immediately (0-100)
        pub immediate_release_percent: u8,
        /// Linear vesting over N blocks
        pub vesting_duration_blocks: BlockNumber,
        /// Cliff period before vesting starts
        pub cliff_blocks: BlockNumber,
    }

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    #[codec(dumb_trait_bound)]
    pub struct ContributionLimits {
        /// Minimum contribution per wallet
        pub min_contribution: u128,
        /// Maximum contribution per wallet
        pub max_contribution: u128,
        /// Total hard cap for presale
        pub hard_cap: u128,
    }

    #[derive(Clone, Copy, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
    #[cfg_attr(feature = "std", derive(serde::Serialize, serde::Deserialize))]
    #[codec(dumb_trait_bound)]
    pub struct ContributionInfo<BlockNumber> {
        /// Total amount contributed
        pub amount: u128,
        /// Block number when first contributed (for grace period calculation)
        pub contributed_at: BlockNumber,
        /// Whether this contribution was refunded
        pub refunded: bool,
        /// Block number when refunded
        pub refunded_at: Option<BlockNumber>,
        /// Fee paid for refund
        pub refund_fee_paid: u128,
    }

    #[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T, MaxBonusTiers))]
    #[codec(mel_bound(T: Config, MaxBonusTiers: Get<u32>))]
    pub struct PresaleConfig<T: Config, MaxBonusTiers: Get<u32>> {
        /// Presale creator/owner
        pub owner: T::AccountId,
        /// Payment asset (wUSDT, wUSDC, etc.)
        pub payment_asset: <T as pallet_assets::Config>::AssetId,
        /// Reward token asset
        pub reward_asset: <T as pallet_assets::Config>::AssetId,
        /// Total tokens for sale (with decimals)
        /// Example: 10_000_000 * 10^12 = 10M PEZ with 12 decimals
        pub tokens_for_sale: u128,
        /// Presale start block
        pub start_block: BlockNumberFor<T>,
        /// Presale duration in blocks
        pub duration: BlockNumberFor<T>,
        /// Status
        pub status: PresaleStatus,
        /// Access control
        pub access_control: AccessControl,
        /// Contribution limits
        pub limits: ContributionLimits,
        /// Bonus tiers
        pub bonus_tiers: BoundedVec<BonusTier, MaxBonusTiers>,
        /// Optional vesting schedule
        pub vesting: Option<VestingSchedule<BlockNumberFor<T>>>,
        /// Grace period for refunds (blocks) - low fee
        pub grace_period_blocks: BlockNumberFor<T>,
        /// Normal refund fee percentage (0-100)
        pub refund_fee_percent: u8,
        /// Grace period refund fee percentage (0-100)
        pub grace_refund_fee_percent: u8,
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_assets::Config
    where
        <Self as pallet_assets::Config>::AssetId: Clone + MaxEncodedLen,
        <Self as pallet_assets::Config>::Balance: TryFrom<u128>,
    {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// The presale pallet id, used for deriving sub-account treasuries
        #[pallet::constant]
        type PalletId: Get<PalletId>;

        /// Platform treasury account (receives 50% of platform fee)
        #[pallet::constant]
        type PlatformTreasury: Get<Self::AccountId>;

        /// Staking reward pool account (receives 25% of platform fee)
        #[pallet::constant]
        type StakingRewardPool: Get<Self::AccountId>;

        /// Platform fee percentage (e.g., 2 for 2%)
        #[pallet::constant]
        type PlatformFeePercent: Get<u8>;

        /// Maximum number of contributors per presale
        #[pallet::constant]
        type MaxContributors: Get<u32>;

        /// Maximum bonus tiers per presale
        #[pallet::constant]
        type MaxBonusTiers: Get<u32>;

        /// Maximum whitelisted accounts per presale
        #[pallet::constant]
        type MaxWhitelistedAccounts: Get<u32>;

        /// Origin that can create presales
        type CreatePresaleOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Origin for emergency actions
        type EmergencyOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// Weight information
        type PresaleWeightInfo: crate::weights::WeightInfo;
    }

    /// Next presale ID
    #[pallet::storage]
    #[pallet::getter(fn next_presale_id)]
    pub type NextPresaleId<T: Config> = StorageValue<_, PresaleId, ValueQuery>;

    /// Presale configurations
    #[pallet::storage]
    #[pallet::getter(fn presales)]
    pub type Presales<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PresaleId,
        PresaleConfig<T, T::MaxBonusTiers>,
        OptionQuery,
    >;

    /// Contributions: (presale_id, account) => ContributionInfo
    #[pallet::storage]
    #[pallet::getter(fn contributions)]
    pub type Contributions<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, PresaleId,
        Blake2_128Concat, T::AccountId,
        ContributionInfo<BlockNumberFor<T>>,
        OptionQuery,
    >;

    /// Contributors list per presale
    #[pallet::storage]
    #[pallet::getter(fn contributors)]
    pub type Contributors<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PresaleId,
        BoundedVec<T::AccountId, T::MaxContributors>,
        ValueQuery,
    >;

    /// Total raised per presale
    #[pallet::storage]
    #[pallet::getter(fn total_raised)]
    pub type TotalRaised<T: Config> = StorageMap<
        _,
        Blake2_128Concat,
        PresaleId,
        u128,
        ValueQuery,
    >;

    /// Whitelist: (presale_id, account) => is_whitelisted
    #[pallet::storage]
    #[pallet::getter(fn whitelisted)]
    pub type WhitelistedAccounts<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, PresaleId,
        Blake2_128Concat, T::AccountId,
        bool,
        ValueQuery,
    >;

    /// Vesting claims: (presale_id, account) => claimed_amount
    #[pallet::storage]
    #[pallet::getter(fn vesting_claimed)]
    pub type VestingClaimed<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat, PresaleId,
        Blake2_128Concat, T::AccountId,
        u128,
        ValueQuery,
    >;

    /// Platform analytics
    #[pallet::storage]
    #[pallet::getter(fn total_platform_volume)]
    pub type TotalPlatformVolume<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn total_platform_fees)]
    pub type TotalPlatformFees<T: Config> = StorageValue<_, u128, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn successful_presales)]
    pub type SuccessfulPresales<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Presale created [presale_id, owner, payment_asset, reward_asset]
        PresaleCreated {
            presale_id: PresaleId,
            owner: T::AccountId,
            payment_asset: <T as pallet_assets::Config>::AssetId,
            reward_asset: <T as pallet_assets::Config>::AssetId,
        },
        /// Contribution made [presale_id, who, amount, bonus_amount]
        Contributed {
            presale_id: PresaleId,
            who: T::AccountId,
            amount: u128,
            bonus_amount: u128,
        },
        /// Presale finalized [presale_id, total_raised]
        PresaleFinalized {
            presale_id: PresaleId,
            total_raised: u128,
        },
        /// Tokens distributed [presale_id, who, amount]
        Distributed {
            presale_id: PresaleId,
            who: T::AccountId,
            amount: u128,
        },
        /// Refund processed [presale_id, who, amount, fee]
        Refunded {
            presale_id: PresaleId,
            who: T::AccountId,
            amount: u128,
            fee: u128,
        },
        /// Presale cancelled [presale_id]
        PresaleCancelled {
            presale_id: PresaleId,
        },
        /// Platform fee distributed [treasury_share, burn_share, staker_share]
        PlatformFeeDistributed {
            treasury_share: u128,
            burn_share: u128,
            staker_share: u128,
        },
        /// Account whitelisted [presale_id, account]
        AccountWhitelisted {
            presale_id: PresaleId,
            account: T::AccountId,
        },
        /// Vesting tokens claimed [presale_id, who, amount]
        VestingClaimed {
            presale_id: PresaleId,
            who: T::AccountId,
            amount: u128,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        PresaleNotFound,
        PresaleNotActive,
        PresaleEnded,
        PresaleNotEnded,
        AlreadyFinalized,
        ZeroContribution,
        BelowMinContribution,
        AboveMaxContribution,
        HardCapReached,
        NotWhitelisted,
        TooManyContributors,
        ArithmeticOverflow,
        InvalidTokensForSale,
        InvalidFeePercent,
        NoContribution,
        RefundNotAllowed,
        SoftCapReached,
        InsufficientBalance,
        VestingNotEnabled,
        NothingToClaim,
        NotPresaleOwner,
        TooManyBonusTiers,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create a new presale
        #[pallet::call_index(0)]
        #[pallet::weight(T::PresaleWeightInfo::start_presale())]
        pub fn create_presale(
            origin: OriginFor<T>,
            payment_asset: <T as pallet_assets::Config>::AssetId,
            reward_asset: <T as pallet_assets::Config>::AssetId,
            tokens_for_sale: u128,
            duration: BlockNumberFor<T>,
            is_whitelist: bool,
            min_contribution: u128,
            max_contribution: u128,
            hard_cap: u128,
            enable_vesting: bool,
            vesting_immediate_percent: u8,
            vesting_duration_blocks: BlockNumberFor<T>,
            vesting_cliff_blocks: BlockNumberFor<T>,
            grace_period_blocks: BlockNumberFor<T>,
            refund_fee_percent: u8,
            grace_refund_fee_percent: u8,
        ) -> DispatchResult {
            let owner = ensure_signed(origin)?;

            ensure!(tokens_for_sale > 0, Error::<T>::InvalidTokensForSale);
            ensure!(refund_fee_percent <= 100, Error::<T>::InvalidFeePercent);
            ensure!(grace_refund_fee_percent <= 100, Error::<T>::InvalidFeePercent);

            let presale_id = NextPresaleId::<T>::get();
            let start_block = <frame_system::Pallet<T>>::block_number();

            // Start with empty bonus tiers - can be added later
            let bounded_bonus_tiers = BoundedVec::<BonusTier, T::MaxBonusTiers>::default();

            let access_control = if is_whitelist {
                AccessControl::Whitelist
            } else {
                AccessControl::Public
            };

            let limits = ContributionLimits {
                min_contribution,
                max_contribution,
                hard_cap,
            };

            let vesting = if enable_vesting {
                Some(VestingSchedule {
                    immediate_release_percent: vesting_immediate_percent,
                    vesting_duration_blocks,
                    cliff_blocks: vesting_cliff_blocks,
                })
            } else {
                None
            };

            let config = PresaleConfig {
                owner: owner.clone(),
                payment_asset: payment_asset.clone(),
                reward_asset: reward_asset.clone(),
                tokens_for_sale,
                start_block,
                duration,
                status: PresaleStatus::Active,
                access_control,
                limits,
                bonus_tiers: bounded_bonus_tiers,
                vesting,
                grace_period_blocks,
                refund_fee_percent,
                grace_refund_fee_percent,
            };

            Presales::<T>::insert(presale_id, config);
            NextPresaleId::<T>::put(presale_id.saturating_add(1));

            Self::deposit_event(Event::PresaleCreated {
                presale_id,
                owner,
                payment_asset,
                reward_asset,
            });

            Ok(())
        }

        /// Contribute to a presale
        #[pallet::call_index(1)]
        #[pallet::weight(T::PresaleWeightInfo::contribute())]
        pub fn contribute(
            origin: OriginFor<T>,
            presale_id: PresaleId,
            amount: u128,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            // Checks
            ensure!(presale.status == PresaleStatus::Active, Error::<T>::PresaleNotActive);
            ensure!(amount > 0, Error::<T>::ZeroContribution);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = presale.start_block + presale.duration;
            ensure!(current_block < end_block, Error::<T>::PresaleEnded);

            // Check whitelist
            if presale.access_control == AccessControl::Whitelist {
                ensure!(
                    WhitelistedAccounts::<T>::get(presale_id, &who),
                    Error::<T>::NotWhitelisted
                );
            }

            // Check limits
            let existing_contribution = Contributions::<T>::get(presale_id, &who);
            let current_amount = existing_contribution.as_ref().map(|c| c.amount).unwrap_or(0);
            let new_total = current_amount.saturating_add(amount);

            ensure!(
                new_total >= presale.limits.min_contribution,
                Error::<T>::BelowMinContribution
            );
            ensure!(
                new_total <= presale.limits.max_contribution,
                Error::<T>::AboveMaxContribution
            );

            // Calculate remaining capacity and accept only what fits
            let total_raised = TotalRaised::<T>::get(presale_id);
            let remaining_capacity = presale.limits.hard_cap.saturating_sub(total_raised);

            // Accept only what fits (better UX than failing entire transaction)
            let accepted_amount = amount.min(remaining_capacity);

            // Ensure we can accept something
            ensure!(accepted_amount > 0, Error::<T>::HardCapReached);

            // Use accepted_amount for the rest of the function
            let amount = accepted_amount;
            let new_raised = total_raised.saturating_add(amount);

            // Calculate platform fee (2%)
            let platform_fee = amount.saturating_mul(T::PlatformFeePercent::get() as u128) / 100;
            let net_amount = amount.saturating_sub(platform_fee);

            // Transfer payment asset from user to presale treasury
            let treasury = Self::presale_account_id(presale_id);
            let net_amount_balance: <T as pallet_assets::Config>::Balance = net_amount.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                presale.payment_asset.clone(),
                &who,
                &treasury,
                net_amount_balance,
                Preservation::Preserve,
            )?;

            // Distribute platform fee
            Self::distribute_platform_fee(presale.payment_asset.clone(), &who, platform_fee)?;

            // Track contribution with timestamp preservation
            let contribution = if let Some(existing) = existing_contribution {
                // Update existing contribution - preserve original timestamp
                ContributionInfo {
                    amount: existing.amount.saturating_add(amount),
                    contributed_at: existing.contributed_at,  // ✅ Keep original timestamp
                    refunded: false,
                    refunded_at: None,
                    refund_fee_paid: 0,
                }
            } else {
                // New contribution - add to contributors list
                Contributors::<T>::try_mutate(presale_id, |contributors| -> DispatchResult {
                    contributors.try_push(who.clone())
                        .map_err(|_| Error::<T>::TooManyContributors)?;
                    Ok(())
                })?;

                // Create new contribution with current timestamp
                ContributionInfo {
                    amount,
                    contributed_at: current_block,  // ✅ Set timestamp for first contribution only
                    refunded: false,
                    refunded_at: None,
                    refund_fee_paid: 0,
                }
            };

            Contributions::<T>::insert(presale_id, &who, contribution);
            TotalRaised::<T>::insert(presale_id, new_raised);

            // Update platform analytics
            TotalPlatformVolume::<T>::mutate(|v| *v = v.saturating_add(amount));
            TotalPlatformFees::<T>::mutate(|f| *f = f.saturating_add(platform_fee));

            // Calculate bonus using total contribution amount
            let bonus_amount = Self::calculate_bonus(&presale, contribution.amount);

            Self::deposit_event(Event::Contributed {
                presale_id,
                who,
                amount,
                bonus_amount,
            });

            Ok(())
        }

        /// Finalize presale and distribute tokens
        #[pallet::call_index(2)]
        #[pallet::weight(T::PresaleWeightInfo::finalize_presale(Contributors::<T>::get(presale_id).len() as u32))]
        pub fn finalize_presale(
            origin: OriginFor<T>,
            presale_id: PresaleId,
        ) -> DispatchResult {
            ensure_root(origin)?;

            let mut presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            ensure!(presale.status == PresaleStatus::Active, Error::<T>::PresaleNotActive);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = presale.start_block + presale.duration;
            ensure!(current_block >= end_block, Error::<T>::PresaleNotEnded);

            let total_raised = TotalRaised::<T>::get(presale_id);
            let treasury = Self::presale_account_id(presale_id);

            // Distribute rewards to all contributors
            for contributor in Contributors::<T>::get(presale_id).iter() {
                let contribution_info = match Contributions::<T>::get(presale_id, contributor) {
                    Some(info) => info,
                    None => continue,
                };

                // Skip if refunded
                if contribution_info.refunded || contribution_info.amount == 0 {
                    continue;
                }

                // Calculate reward tokens using dynamic rate
                let reward_amount = Self::calculate_reward_dynamic(
                    contribution_info.amount,
                    total_raised,
                    presale.tokens_for_sale,
                )?;

                let bonus = Self::calculate_bonus(&presale, contribution_info.amount);
                let total_reward = reward_amount.saturating_add(bonus);

                // Handle vesting
                if let Some(ref vesting) = presale.vesting {
                    let immediate = total_reward.saturating_mul(vesting.immediate_release_percent as u128) / 100;

                    if immediate > 0 {
                        let immediate_balance: <T as pallet_assets::Config>::Balance = immediate.try_into()
                            .map_err(|_| Error::<T>::ArithmeticOverflow)?;
                        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                            presale.reward_asset.clone(),
                            &treasury,
                            contributor,
                            immediate_balance,
                            Preservation::Preserve,
                        )?;
                    }

                    // Store remaining for vesting
                    VestingClaimed::<T>::insert(presale_id, contributor, immediate);
                } else {
                    // No vesting - transfer all
                    let total_reward_balance: <T as pallet_assets::Config>::Balance = total_reward.try_into()
                        .map_err(|_| Error::<T>::ArithmeticOverflow)?;
                    <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                        presale.reward_asset.clone(),
                        &treasury,
                        contributor,
                        total_reward_balance,
                        Preservation::Preserve,
                    )?;
                }

                Self::deposit_event(Event::Distributed {
                    presale_id,
                    who: contributor.clone(),
                    amount: total_reward,
                });
            }

            presale.status = PresaleStatus::Finalized;
            Presales::<T>::insert(presale_id, presale);
            SuccessfulPresales::<T>::mutate(|c| *c = c.saturating_add(1));

            Self::deposit_event(Event::PresaleFinalized {
                presale_id,
                total_raised,
            });

            Ok(())
        }

        /// Refund contribution (before presale ends)
        #[pallet::call_index(3)]
        #[pallet::weight(T::PresaleWeightInfo::contribute())]
        pub fn refund(
            origin: OriginFor<T>,
            presale_id: PresaleId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            ensure!(presale.status == PresaleStatus::Active, Error::<T>::RefundNotAllowed);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = presale.start_block + presale.duration;
            ensure!(current_block < end_block, Error::<T>::RefundNotAllowed);

            let mut contribution_info = Contributions::<T>::get(presale_id, &who)
                .ok_or(Error::<T>::NoContribution)?;

            ensure!(!contribution_info.refunded, Error::<T>::RefundNotAllowed);
            ensure!(contribution_info.amount > 0, Error::<T>::NoContribution);

            // Calculate fee based on grace period using ORIGINAL contribution timestamp
            let grace_end = contribution_info.contributed_at.saturating_add(presale.grace_period_blocks);
            let fee_percent = if current_block <= grace_end {
                presale.grace_refund_fee_percent
            } else {
                presale.refund_fee_percent
            };

            let fee = contribution_info.amount.saturating_mul(fee_percent as u128) / 100;
            let refund_amount = contribution_info.amount.saturating_sub(fee);

            // Transfer refund from treasury to user
            let treasury = Self::presale_account_id(presale_id);
            let refund_amount_balance: <T as pallet_assets::Config>::Balance = refund_amount.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                presale.payment_asset.clone(),
                &treasury,
                &who,
                refund_amount_balance,
                Preservation::Preserve,
            )?;

            // Distribute refund fee immediately (50% treasury, 25% burn, 25% stakers)
            if fee > 0 {
                Self::distribute_platform_fee(presale.payment_asset.clone(), &treasury, fee)?;
            }

            // Update contribution info (mark as refunded instead of removing)
            contribution_info.refunded = true;
            contribution_info.refunded_at = Some(current_block);
            contribution_info.refund_fee_paid = fee;
            Contributions::<T>::insert(presale_id, &who, contribution_info);

            TotalRaised::<T>::mutate(presale_id, |r| *r = r.saturating_sub(contribution_info.amount));

            Self::deposit_event(Event::Refunded {
                presale_id,
                who,
                amount: refund_amount,
                fee,
            });

            Ok(())
        }

        /// Claim vested tokens
        #[pallet::call_index(4)]
        #[pallet::weight(T::PresaleWeightInfo::contribute())]
        pub fn claim_vested(
            origin: OriginFor<T>,
            presale_id: PresaleId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            let vesting = presale.vesting.ok_or(Error::<T>::VestingNotEnabled)?;

            ensure!(presale.status == PresaleStatus::Finalized, Error::<T>::PresaleNotActive);

            let contribution_info = Contributions::<T>::get(presale_id, &who)
                .ok_or(Error::<T>::NoContribution)?;
            ensure!(contribution_info.amount > 0, Error::<T>::NoContribution);
            ensure!(!contribution_info.refunded, Error::<T>::NoContribution);

            let current_block = <frame_system::Pallet<T>>::block_number();
            let end_block = presale.start_block + presale.duration;
            let vesting_start = end_block + vesting.cliff_blocks;

            ensure!(current_block >= vesting_start, Error::<T>::NothingToClaim);

            // Get total raised for dynamic calculation
            let total_raised = TotalRaised::<T>::get(presale_id);

            // Calculate total reward using dynamic rate
            let total_reward = Self::calculate_reward_dynamic(
                contribution_info.amount,
                total_raised,
                presale.tokens_for_sale,
            )?;
            let bonus = Self::calculate_bonus(&presale, contribution_info.amount);
            let total_with_bonus = total_reward.saturating_add(bonus);

            // Calculate vested amount
            let already_claimed = VestingClaimed::<T>::get(presale_id, &who);
            let vesting_end = vesting_start + vesting.vesting_duration_blocks;

            let claimable = if current_block >= vesting_end {
                // All vested
                total_with_bonus.saturating_sub(already_claimed)
            } else {
                // Linear vesting
                use sp_runtime::traits::SaturatedConversion;
                let elapsed = current_block.saturating_sub(vesting_start);
                let elapsed_u128: u128 = elapsed.saturated_into();
                let duration_u128: u128 = vesting.vesting_duration_blocks.saturated_into();
                let vested_percent = elapsed_u128.saturating_mul(100) / duration_u128;
                let immediate_percent = vesting.immediate_release_percent as u128;
                let vesting_percent = 100u128.saturating_sub(immediate_percent);
                let vested_amount = total_with_bonus.saturating_mul(vesting_percent).saturating_mul(vested_percent) / 10000;
                let total_unlocked = vested_amount.saturating_add(already_claimed);
                total_unlocked.saturating_sub(already_claimed)
            };

            ensure!(claimable > 0, Error::<T>::NothingToClaim);

            // Transfer tokens
            let treasury = Self::presale_account_id(presale_id);
            let claimable_balance: <T as pallet_assets::Config>::Balance = claimable.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                presale.reward_asset,
                &treasury,
                &who,
                claimable_balance,
                Preservation::Preserve,
            )?;

            VestingClaimed::<T>::insert(presale_id, &who, already_claimed.saturating_add(claimable));

            Self::deposit_event(Event::VestingClaimed {
                presale_id,
                who,
                amount: claimable,
            });

            Ok(())
        }

        /// Add account to whitelist (presale owner only)
        #[pallet::call_index(5)]
        #[pallet::weight(T::PresaleWeightInfo::emergency_pause())]
        pub fn add_to_whitelist(
            origin: OriginFor<T>,
            presale_id: PresaleId,
            account: T::AccountId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            ensure!(who == presale.owner, Error::<T>::NotPresaleOwner);

            WhitelistedAccounts::<T>::insert(presale_id, &account, true);

            Self::deposit_event(Event::AccountWhitelisted {
                presale_id,
                account,
            });

            Ok(())
        }

        /// Cancel presale (emergency - owner or root)
        #[pallet::call_index(6)]
        #[pallet::weight(T::PresaleWeightInfo::emergency_pause())]
        pub fn cancel_presale(
            origin: OriginFor<T>,
            presale_id: PresaleId,
        ) -> DispatchResult {
            // Either EmergencyOrigin or Root can cancel
            if T::EmergencyOrigin::ensure_origin(origin.clone()).is_err() {
                ensure_root(origin)?;
            }

            let mut presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            presale.status = PresaleStatus::Cancelled;
            Presales::<T>::insert(presale_id, presale);

            Self::deposit_event(Event::PresaleCancelled { presale_id });

            Ok(())
        }

        /// Refund all contributors when presale is cancelled
        /// Auto-refunds everyone with no fees
        #[pallet::call_index(7)]
        #[pallet::weight(T::PresaleWeightInfo::finalize_presale(100))]
        pub fn refund_cancelled_presale(
            origin: OriginFor<T>,
            presale_id: PresaleId,
        ) -> DispatchResult {
            ensure_signed(origin)?;

            let presale = Presales::<T>::get(presale_id)
                .ok_or(Error::<T>::PresaleNotFound)?;

            // Only works on cancelled presales
            ensure!(
                matches!(presale.status, PresaleStatus::Cancelled),
                Error::<T>::PresaleNotFound
            );

            let current_block = <frame_system::Pallet<T>>::block_number();
            let treasury = Self::presale_account_id(presale_id);

            // Refund all contributors (no fees since presale was cancelled)
            let contributors = Contributors::<T>::get(presale_id);
            for contributor in contributors.iter() {
                if let Some(contribution_info) = Contributions::<T>::get(presale_id, contributor) {
                    if !contribution_info.refunded && contribution_info.amount > 0 {
                        // Full refund (no fees on cancelled presale)
                        let refund_amount: <T as pallet_assets::Config>::Balance =
                            contribution_info.amount.try_into()
                                .map_err(|_| Error::<T>::ArithmeticOverflow)?;

                        <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                            presale.payment_asset.clone(),
                            &treasury,
                            contributor,
                            refund_amount,
                            Preservation::Preserve,
                        )?;

                        // Mark as refunded
                        let updated_info = ContributionInfo {
                            refunded: true,
                            refunded_at: Some(current_block),
                            refund_fee_paid: 0, // No fee on cancelled presale
                            ..contribution_info
                        };
                        Contributions::<T>::insert(presale_id, contributor, updated_info);

                        Self::deposit_event(Event::Refunded {
                            presale_id,
                            who: contributor.clone(),
                            amount: contribution_info.amount,
                            fee: 0,
                        });
                    }
                }
            }

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        /// Get presale sub-account treasury
        pub fn presale_account_id(presale_id: PresaleId) -> T::AccountId {
            T::PalletId::get().into_sub_account_truncating(presale_id)
        }

        /// Distribute platform fee: 50% treasury, 25% burn, 25% stakers
        /// Uses remainder method to prevent dust loss from rounding
        fn distribute_platform_fee(
            asset_id: <T as pallet_assets::Config>::AssetId,
            from: &T::AccountId,
            total_fee: u128,
        ) -> DispatchResult {
            // Calculate burn and stakers first, treasury gets the rest (no dust)
            let to_burn = total_fee.saturating_mul(25) / 100;
            let to_stakers = total_fee.saturating_mul(25) / 100;
            let to_treasury = total_fee
                .saturating_sub(to_burn)
                .saturating_sub(to_stakers);  // Treasury gets remainder, prevents dust loss

            let to_treasury_balance: <T as pallet_assets::Config>::Balance = to_treasury.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;
            let to_burn_balance: <T as pallet_assets::Config>::Balance = to_burn.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;
            let to_stakers_balance: <T as pallet_assets::Config>::Balance = to_stakers.try_into()
                .map_err(|_| Error::<T>::ArithmeticOverflow)?;

            // 1. Treasury (50%)
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                asset_id.clone(),
                from,
                &T::PlatformTreasury::get(),
                to_treasury_balance,
                Preservation::Preserve,
            )?;

            // 2. Burn (25%)
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::burn_from(
                asset_id.clone(),
                from,
                to_burn_balance,
                Preservation::Preserve,
                Precision::Exact,
                Fortitude::Force,
            )?;

            // 3. Stakers (25%)
            <pallet_assets::Pallet<T> as Mutate<T::AccountId>>::transfer(
                asset_id,
                from,
                &T::StakingRewardPool::get(),
                to_stakers_balance,
                Preservation::Preserve,
            )?;

            Self::deposit_event(Event::PlatformFeeDistributed {
                treasury_share: to_treasury,
                burn_share: to_burn,
                staker_share: to_stakers,
            });

            Ok(())
        }

        /// Calculate bonus based on tier
        fn calculate_bonus(
            presale: &PresaleConfig<T, T::MaxBonusTiers>,
            contribution: u128,
        ) -> u128 {
            let mut applicable_bonus = 0u8;

            for tier in presale.bonus_tiers.iter() {
                if contribution >= tier.min_contribution {
                    applicable_bonus = tier.bonus_percentage;
                }
            }

            if applicable_bonus == 0 {
                return 0;
            }

            // Bonus calculation no longer uses conversion_rate
            // Instead, calculate bonus as percentage of contribution
            contribution.saturating_mul(applicable_bonus as u128) / 100
        }

        /// Calculate reward based on user's share of total raised
        /// Formula: (user_contribution / total_raised) * tokens_for_sale
        ///
        /// Example:
        /// - tokens_for_sale: 10,000,000 PEZ (10M * 10^12 decimals)
        /// - total_raised: 100,000 wUSDT (100K * 10^6 decimals)
        /// - user_contribution: 1,000 wUSDT (1K * 10^6 decimals)
        /// - Result: (1,000 / 100,000) * 10M = 100,000 PEZ per user
        fn calculate_reward_dynamic(
            user_contribution: u128,
            total_raised: u128,
            tokens_for_sale: u128,
        ) -> Result<u128, Error<T>> {
            ensure!(
                total_raised > 0,
                Error::<T>::ArithmeticOverflow
            );

            // Calculate user's share: (contribution * tokens_for_sale) / total_raised
            let user_share = user_contribution
                .saturating_mul(tokens_for_sale)
                .checked_div(total_raised)
                .ok_or(Error::<T>::ArithmeticOverflow)?;

            Ok(user_share)
        }
    }
}
