// ================================================================================================================================================
// === KK/pallets/society/trust/lib.rs  =========
// ================================================================================================================================================
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    ensure,
    pallet_prelude::*,
    PalletId,
    traits::{
        schedule::{DispatchTime, Named as ScheduleNamed, MaybeHashed},
        Get,
    },
    BoundedVec,
    // REMOVED: Explicit import for MultiRemovalResults
    // storage::types::MultiRemovalResults,
};
use frame_support::dispatch::{DispatchResult, DispatchResultWithPostInfo};
use sp_runtime::{
    Perbill,
    RuntimeDebug,
    traits::{
        AccountIdConversion,
        BlockNumberProvider, IntegerSquareRoot, One, Saturating, Zero,
        CheckedAdd, SaturatedConversion,
    },
};
use frame_system::{
    ensure_root, ensure_signed,
    pallet_prelude::{BlockNumberFor, OriginFor},
    RawOrigin,
};
use log;
use pallet_trust_api::TrustApi as TrustApiTrait;
use pezkuwi_primitives::{
    types::{AccountIdCore, RawScore, TrustScore},
    traits::{
        EgitimScoreProvider, StakingScoreProvider, ReferralScoreProvider, TikiScoreProvider,
        ActivityRecorder as PrimitivesActivityRecorder,
    },
};
use sp_std::{marker::PhantomData, prelude::*};

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
pub use weights::WeightInfo;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::schedule::{DispatchTime, Named as ScheduleNamed};
    use frame_system::pallet_prelude::OriginFor;

    #[pallet::type_value]
    pub fn GetPalletsOriginDefault<T: Config>() -> <T as frame_system::Config>::RuntimeOrigin {
        RawOrigin::None.into()
    }

    #[pallet::pallet]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        #[pallet::constant]
        type PalletId: Get<PalletId>;
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: super::weights::WeightInfo;

        type EgitimScoreProvider: EgitimScoreProvider<Self::AccountId, Score = RawScore>;
        type StakingScoreProvider: StakingScoreProvider<Self::AccountId, Score = RawScore>;
        type ReferralScoreProvider: ReferralScoreProvider<Self::AccountId, Score = RawScore>;
        type TikiScoreProvider: TikiScoreProvider<Self::AccountId, Score = RawScore>;

        #[pallet::constant]
        type EgitimCoefficientScaled: Get<u32>;
        #[pallet::constant]
        type StakingCoefficientScaled: Get<u32>;
        #[pallet::constant]
        type ReferralCoefficientScaled: Get<u32>;
        #[pallet::constant]
        type TikiCoefficientScaled: Get<u32>;
        #[pallet::constant]
        type ActivityCoefficientScaled: Get<u32>;

        #[pallet::constant]
        type ScoreScaleFactor: Get<u32>;
        #[pallet::constant]
        type MaxScaledTrustScore: Get<TrustScore>;

        type RecalculateOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type PenaltyOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type ActivityResetOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type RecordActivityOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        #[pallet::constant]
        type ActivityResetPeriod: Get<BlockNumberFor<Self>>;
        #[pallet::constant]
        type MaxTaskIdLen: Get<u32>;
        #[pallet::constant]
        type MaxActivityCounterItems: Get<u32>;

        type RuntimeCall: From<Call<Self>> + Parameter + IsType<<Self as frame_system::Config>::RuntimeCall>;
        type Scheduler: ScheduleNamed<
            BlockNumberFor<Self>,
            <Self as pallet::Config>::RuntimeCall,
            Self::PalletsOrigin,
        >;
        /// Bu paletin zamanlanmış görevler oluştururken kullanacağı Origin tipi.
        /// Bu, runtime tarafından sağlanmalıdır.
        type PalletsOrigin: From<frame_system::RawOrigin<Self::AccountId>>;
    }

    #[pallet::storage]
    #[pallet::getter(fn final_trust_scores)]
    pub type FinalTrustScores<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, TrustScore, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn account_activity_counters)]
    pub type AccountActivityCounters<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, RawScore, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn scheduled_reset_task)]
    pub type ScheduledResetTask<T: Config> =
        StorageValue<_, BoundedVec<u8, T::MaxTaskIdLen>, OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TrustScoreUpdated {
            who: T::AccountId,
            new_scaled_score: TrustScore,
        },
        TrustScoreCalculationFailed {
            who: T::AccountId,
            error: DispatchError,
        },
        ActivityCountersReset { items_cleared: u32 },
        CounterResetScheduled {
            when: BlockNumberFor<T>,
            id: BoundedVec<u8, T::MaxTaskIdLen>,
        },
        ScheduledTaskCancelled { id: BoundedVec<u8, T::MaxTaskIdLen> },
        TrustPenaltyApplied {
            target: T::AccountId,
            penalty_amount_scaled: TrustScore,
            new_scaled_score: TrustScore,
        },
        ActivityRecorded {
            who: T::AccountId,
            new_activity_count: RawScore,
        },
    }

    #[pallet::error]
    pub enum Error<T> {
        ArithmeticOverflowOrConversion,
        CannotCalculateScore,
        ScheduleError,
        CancelScheduleError,
        NoActivityCounterYet,
        InvalidPenaltyAmount,
        TaskIdTooLong,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(current_block: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();
            let db_read_weight = T::DbWeight::get().reads(1);
            let db_write_weight = T::DbWeight::get().writes(1);

            let reset_period = T::ActivityResetPeriod::get();

            if !reset_period.is_zero() && (current_block % reset_period).is_zero() {
                weight = weight.saturating_add(db_read_weight);

                let task_id_vec = b"pallet_trust::reset_activity_counters_periodic".to_vec();
                let task_id: BoundedVec<u8, T::MaxTaskIdLen> = match task_id_vec.try_into() {
                    Ok(id) => id,
                    Err(_) => {
                        log::error!(
                            target: "runtime::trust",
                            "Failed to convert task_id to BoundedVec. MaxTaskIdLen might be too small."
                        );
                        return weight;
                    }
                };

                let next_reset_dispatch_time = current_block.saturating_add(One::one());

                if let Some(old_id) = ScheduledResetTask::<T>::get() {
                    if T::Scheduler::cancel_named(old_id.clone().into()).is_ok() {
                        Self::deposit_event(Event::ScheduledTaskCancelled { id: old_id });
                        weight = weight.saturating_add(db_write_weight);
                    } else {
                        log::warn!(
                            target: "runtime::trust",
                            "Failed to cancel previously scheduled reset task with id: {:?}. Will attempt to reschedule.",
                            old_id
                        );
                    }
                }

                let call_to_schedule: <T as Config>::RuntimeCall =
                    Call::<T>::reset_activity_counters {}.into();

                // let schedule_origin = <T as Config>::PalletsOrigin::get();
                let schedule_origin = RawOrigin::Root.into();

                if T::Scheduler::schedule_named(
                    task_id.clone().into(),
                    DispatchTime::At(next_reset_dispatch_time),
                    None,
                    0,
                    schedule_origin,
                    call_to_schedule,
                ).is_ok() {
                    ScheduledResetTask::<T>::put(task_id.clone());
                    Self::deposit_event(Event::CounterResetScheduled {
                        when: next_reset_dispatch_time,
                        id: task_id,
                    });
                    weight = weight.saturating_add(db_write_weight);
                } else {
                    log::error!(
                        target: "runtime::trust",
                        "Failed to schedule periodic activity counter reset for block {}. This is critical.",
                        next_reset_dispatch_time
                    );
                }
            }
            weight
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::recalculate_and_store_trust_score())]
        pub fn recalculate_and_store_trust_score(
            origin: OriginFor<T>,
            user_to_update: T::AccountId,
        ) -> DispatchResult {
            T::RecalculateOrigin::ensure_origin(origin)?;

            match Self::do_calculate_and_store_trust_score(&user_to_update) {
                Ok(scaled_score) => {
                    Self::deposit_event(Event::TrustScoreUpdated {
                        who: user_to_update,
                        new_scaled_score: scaled_score,
                    });
                    Ok(())
                }
                Err(e) => {
                    log::error!(
                        target: "runtime::trust",
                        "Failed to calculate trust score for user {:?}: {:?}",
                        user_to_update, e
                    );
                    Self::deposit_event(Event::TrustScoreCalculationFailed {
                        who: user_to_update,
                        error: e,
                    });
                    Err(e)
                }
            }
        }

        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::reset_activity_counters(T::MaxActivityCounterItems::get()))]
        pub fn reset_activity_counters(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
            let pallet_origin_check = ensure_signed(origin.clone())
                .map(|who| who == Self::pallet_account_id())
                .unwrap_or(false);

            if !pallet_origin_check {
                T::ActivityResetOrigin::ensure_origin(origin)?;
            }

            // MODIFIED: Removed explicit type annotation for kill_result
            let kill_result = AccountActivityCounters::<T>::clear(u32::MAX, None);
            let items_cleared = kill_result.loops;

            if ScheduledResetTask::<T>::take().is_some() {
                // This take() consumes some weight, ensure it's accounted for if significant
                // or part of the benchmark for reset_activity_counters.
            }

            Self::deposit_event(Event::ActivityCountersReset { items_cleared });

            let actual_weight = T::WeightInfo::reset_activity_counters(items_cleared);
            Ok(Some(actual_weight).into())
        }

        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::force_apply_penalty())]
        pub fn force_apply_penalty(
            origin: OriginFor<T>,
            target: T::AccountId,
            penalty_amount_scaled: TrustScore,
        ) -> DispatchResult {
            T::PenaltyOrigin::ensure_origin(origin)?;
            ensure!(!penalty_amount_scaled.is_zero(), Error::<T>::InvalidPenaltyAmount);

            let current_score = FinalTrustScores::<T>::get(&target).unwrap_or_else(Zero::zero);
            let new_scaled_score = current_score.saturating_sub(penalty_amount_scaled);
            FinalTrustScores::<T>::insert(&target, new_scaled_score);
            Self::deposit_event(Event::TrustPenaltyApplied {
                target,
                penalty_amount_scaled,
                new_scaled_score,
            });
            Ok(())
        }

        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::record_activity_for_user())]
        pub fn record_activity_for_user(origin: OriginFor<T>, user: T::AccountId) -> DispatchResult {
            T::RecordActivityOrigin::ensure_origin(origin)?;
            let new_activity_count = AccountActivityCounters::<T>::mutate(&user, |count| {
                *count = count.saturating_add(One::one());
                *count
            });
            Self::deposit_event(Event::ActivityRecorded {
                who: user,
                new_activity_count,
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub(crate) fn pallet_account_id() -> T::AccountId {
            T::PalletId::get().into_account_truncating()
        }

        fn do_calculate_and_store_trust_score(
            who: &T::AccountId,
        ) -> Result<TrustScore, DispatchError> {
            // Ham puanları topla
            let egitim_raw: RawScore = T::EgitimScoreProvider::get_egitim_score(who);
            let staking_raw: RawScore = T::StakingScoreProvider::get_staking_score(who);
            let referral_raw: RawScore = T::ReferralScoreProvider::get_referral_score(who);
            let tiki_raw: RawScore = T::TikiScoreProvider::get_tiki_score(who);
            let activity_raw: RawScore = AccountActivityCounters::<T>::get(who);

            // Katsayıları ve ölçek faktörünü al
            let egitim_coeff_scaled = T::EgitimCoefficientScaled::get();
            let staking_coeff_scaled = T::StakingCoefficientScaled::get();
            let referral_coeff_scaled = T::ReferralCoefficientScaled::get();
            let tiki_coeff_scaled = T::TikiCoefficientScaled::get();
            let activity_coeff_scaled = T::ActivityCoefficientScaled::get();
            let scale_factor_u128 = T::ScoreScaleFactor::get() as u128;

            // Ağırlıklı toplamı, taşmaları önlemek için u128 üzerinde hesapla
            let sum_of_terms_u128 = (egitim_raw as u128).saturating_mul(egitim_coeff_scaled as u128)
                .saturating_add((staking_raw as u128).saturating_mul(staking_coeff_scaled as u128))
                .saturating_add((referral_raw as u128).saturating_mul(referral_coeff_scaled as u128))
                .saturating_add((tiki_raw as u128).saturating_mul(tiki_coeff_scaled as u128))
                .saturating_add((activity_raw as u128).saturating_mul(activity_coeff_scaled as u128));

            // 1. Karekök hesabını u128 üzerinde yap
            let sqrt_input_u128 = sum_of_terms_u128.saturating_mul(scale_factor_u128);
            let sqrt_result_u128 = sqrt_input_u128.integer_sqrt();

            // 2. Çarpan öncesi skoru u128 olarak hesapla
            let base_scaled_u128 = scale_factor_u128;
            let original_formula_score_u128 = base_scaled_u128.saturating_add(sqrt_result_u128);

            // 3. Stake çarpanını hesapla
            let staking_multiplier = Perbill::from_rational(staking_raw, staking_raw.saturating_add(1));

            // 4. Çarpanı tamsayıya (u128) uygula
            let score_with_stake_factor_u128 = staking_multiplier.mul_floor(original_formula_score_u128);

            // 5. Sonucu TrustScore'a (FixedU128) çevir ve sınırla
            let final_score_fixed = TrustScore::from_inner(score_with_stake_factor_u128);
            let max_score_fixed = T::MaxScaledTrustScore::get();
            let capped_final_score = final_score_fixed.min(max_score_fixed);

            FinalTrustScores::<T>::insert(who, capped_final_score);
            Ok(capped_final_score)
        }
    }

    impl<T: Config> PrimitivesActivityRecorder<T::AccountId> for Pallet<T> {
        fn record_activity(who: &T::AccountId) -> DispatchResult {
            let new_activity_count = AccountActivityCounters::<T>::mutate(who, |count| {
                *count = count.saturating_add(One::one());
                *count
            });
            Self::deposit_event(Event::ActivityRecorded {
                who: who.clone(),
                new_activity_count,
            });
            Ok(())
        }
    }

    impl<T: Config> TrustApiTrait<T::AccountId, TrustScore, RawScore> for Pallet<T>
    where
        T::AccountId: Into<AccountIdCore> + From<AccountIdCore> + Clone,
    {
        fn get_final_trust_score(who: &T::AccountId) -> Option<TrustScore> {
            Self::final_trust_scores(who)
        }

        fn get_raw_egitim_score(who: &T::AccountId) -> RawScore {
            T::EgitimScoreProvider::get_egitim_score(who)
        }

        fn get_raw_staking_score(who: &T::AccountId) -> RawScore {
            T::StakingScoreProvider::get_staking_score(who)
        }

        fn get_raw_referral_score(who: &T::AccountId) -> RawScore {
            T::ReferralScoreProvider::get_referral_score(who)
        }

        fn get_raw_tiki_score(who: &T::AccountId) -> RawScore {
            T::TikiScoreProvider::get_tiki_score(who)
        }

        fn get_raw_activity_score(who: &T::AccountId) -> RawScore {
            Self::account_activity_counters(who)
        }
    }
}