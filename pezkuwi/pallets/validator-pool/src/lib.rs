#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub mod weights;
pub mod types;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use crate::types::*;
use frame_support::{
    dispatch::DispatchResult,
    pallet_prelude::*,
    traits::{Get, Randomness},
    weights::Weight,
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Zero;
use sp_std::vec::Vec;

/// Trust score provider trait
pub trait TrustScoreProvider<AccountId> {
    fn trust_score_of(who: &AccountId) -> u128;
}

/// Tiki score provider trait  
pub trait TikiScoreProvider<AccountId> {
    fn get_tiki_score(who: &AccountId) -> u32;
}

/// Weight functions trait for this pallet.
pub trait WeightInfo {
    fn join_validator_pool() -> Weight;
    fn leave_validator_pool() -> Weight;
    fn update_performance_metrics() -> Weight;
    fn force_new_era() -> Weight;
    fn update_category() -> Weight;
    fn set_pool_parameters() -> Weight;
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type WeightInfo: crate::WeightInfo;
        type Randomness: Randomness<Self::Hash, BlockNumberFor<Self>>;
        
        /// Trust score provider
        type TrustSource: TrustScoreProvider<Self::AccountId>;
        /// Tiki score provider  
        type TikiSource: TikiScoreProvider<Self::AccountId>;
        /// Referral system provider
        type ReferralSource: ReferralProvider<Self::AccountId>;
        /// Perwerde score provider
        type PerwerdeSource: PerwerdeProvider<Self::AccountId>;
        
        /// Origin that can manage the pool
        type PoolManagerOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        
        /// Maximum number of validators per era
        #[pallet::constant]
        type MaxValidators: Get<u32>;
        
        /// Maximum size of validator pool
        #[pallet::constant]
        type MaxPoolSize: Get<u32>;
        
        /// Minimum stake amount for stake validators
        #[pallet::constant]
        type MinStakeAmount: Get<u128>;
    }

    // ============================================================================
    // STORAGE ITEMS
    // ============================================================================

    /// Current era index
    #[pallet::storage]
    #[pallet::getter(fn current_era)]
    pub type CurrentEra<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// When current era started
    #[pallet::storage]
    #[pallet::getter(fn era_start)]
    pub type EraStart<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    /// Current selected validator set for this era
    #[pallet::storage]
    #[pallet::getter(fn current_validator_set)]
    pub type CurrentValidatorSet<T: Config> = 
        StorageValue<_, ValidatorSet<T::AccountId>, OptionQuery>;

    /// Validator pool members and their categories
    #[pallet::storage]
    #[pallet::getter(fn pool_members)]
    pub type PoolMembers<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, ValidatorPoolCategory, OptionQuery>;

    /// Performance metrics for each validator
    #[pallet::storage]
    #[pallet::getter(fn performance_metrics)]
    pub type PerformanceMetrics<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, ValidatorPerformance, ValueQuery>;

    /// Validator selection history (last 5 eras)
    #[pallet::storage]
    pub type SelectionHistory<T: Config> = 
        StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<u32, ConstU32<5>>, ValueQuery>;

    /// Pool size counter
    #[pallet::storage]
    #[pallet::getter(fn pool_size)]
    pub type PoolSize<T: Config> = StorageValue<_, u32, ValueQuery>;

    /// Era length in blocks
    #[pallet::storage]
    #[pallet::getter(fn era_length)]
    pub type EraLength<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

    // ============================================================================
    // EVENTS
    // ============================================================================

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// A validator joined the pool
        ValidatorJoinedPool {
            validator: T::AccountId,
            category: ValidatorPoolCategory,
        },

        /// A validator left the pool
        ValidatorLeftPool {
            validator: T::AccountId,
        },

        /// New era started with new validator set
        NewEraStarted {
            era_index: u32,
            validator_set: ValidatorSet<T::AccountId>,
        },

        /// Validator performance updated
        PerformanceUpdated {
            validator: T::AccountId,
            metrics: ValidatorPerformance,
        },

        /// Pool parameters updated
        PoolParametersUpdated {
            max_validators: u32,
            era_length: BlockNumberFor<T>,
        },

        /// Validator category updated
        CategoryUpdated {
            validator: T::AccountId,
            old_category: ValidatorPoolCategory,
            new_category: ValidatorPoolCategory,
        },
    }

    // ============================================================================
    // ERRORS
    // ============================================================================

    #[pallet::error]
    pub enum Error<T> {
        /// Validator already in pool
        AlreadyInPool,
        /// Validator not in pool
        NotInPool,
        /// Pool is full
        PoolFull,
        /// Insufficient stake amount
        InsufficientStake,
        /// Insufficient trust score
        InsufficientTrustScore,
        /// Missing required Tiki
        MissingRequiredTiki,
        /// Not enough community support
        InsufficientCommunitySupport,
        /// Era transition too early
        EraTransitionTooEarly,
        /// Invalid category
        InvalidCategory,
        /// Not enough eligible validators
        NotEnoughValidators,
    }

    // ============================================================================
    // HOOKS
    // ============================================================================

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(block_number: BlockNumberFor<T>) -> Weight {
            let mut weight = Weight::zero();

            // Check if we need to transition to new era
            let era_start = Self::era_start();
            let era_length = Self::era_length();
            
            if block_number >= era_start + era_length && era_length > Zero::zero() {
                weight = weight.saturating_add(T::DbWeight::get().reads(2));
                
                // Trigger new era if enough time has passed
                if let Err(_) = Self::do_new_era() {
                    // Log error but don't panic
                }
                weight = weight.saturating_add(T::WeightInfo::force_new_era());
            }

            weight
        }
    }

    // ============================================================================
    // EXTRINSICS
    // ============================================================================

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Join the validator pool
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::join_validator_pool())]
        pub fn join_validator_pool(
            origin: OriginFor<T>,
            category: ValidatorPoolCategory,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            ensure!(!PoolMembers::<T>::contains_key(&who), Error::<T>::AlreadyInPool);
            ensure!(Self::pool_size() < T::MaxPoolSize::get(), Error::<T>::PoolFull);
            
            // Validate category requirements
            Self::validate_category_requirements(&who, &category)?;
            
            // Add to pool
            PoolMembers::<T>::insert(&who, &category);
            PoolSize::<T>::mutate(|size| *size = size.saturating_add(1));
            
            // Initialize performance metrics
            let initial_performance = ValidatorPerformance {
                blocks_produced: 0,
                blocks_missed: 0,
                era_points: 0,
                last_active_era: Self::current_era(),
                reputation_score: 100, // Start with neutral reputation
            };
            PerformanceMetrics::<T>::insert(&who, initial_performance);
            
            Self::deposit_event(Event::ValidatorJoinedPool {
                validator: who,
                category,
            });
            
            Ok(())
        }

        /// Leave the validator pool
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::leave_validator_pool())]
        pub fn leave_validator_pool(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            ensure!(PoolMembers::<T>::contains_key(&who), Error::<T>::NotInPool);
            
            // Remove from pool
            PoolMembers::<T>::remove(&who);
            PoolSize::<T>::mutate(|size| *size = size.saturating_sub(1));
            
            // Clean up performance metrics
            PerformanceMetrics::<T>::remove(&who);
            SelectionHistory::<T>::remove(&who);
            
            Self::deposit_event(Event::ValidatorLeftPool { validator: who });
            
            Ok(())
        }

        /// Force new era (sudo only)
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::force_new_era())]
        pub fn force_new_era(origin: OriginFor<T>) -> DispatchResult {
            T::PoolManagerOrigin::ensure_origin(origin)?;
            Self::do_new_era()?;
            Ok(())
        }

        /// Update validator category
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::update_category())]
        pub fn update_category(
            origin: OriginFor<T>,
            new_category: ValidatorPoolCategory,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            
            let old_category = PoolMembers::<T>::get(&who)
                .ok_or(Error::<T>::NotInPool)?;
            
            // Validate new category requirements
            Self::validate_category_requirements(&who, &new_category)?;
            
            PoolMembers::<T>::insert(&who, &new_category);
            
            Self::deposit_event(Event::CategoryUpdated {
                validator: who,
                old_category,
                new_category,
            });
            
            Ok(())
        }

        /// Set pool parameters (sudo only)
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::set_pool_parameters())]
        pub fn set_pool_parameters(
            origin: OriginFor<T>,
            era_length: BlockNumberFor<T>,
        ) -> DispatchResult {
            T::PoolManagerOrigin::ensure_origin(origin)?;
            
            EraLength::<T>::put(era_length);
            
            Self::deposit_event(Event::PoolParametersUpdated {
                max_validators: T::MaxValidators::get(),
                era_length,
            });
            
            Ok(())
        }

        /// Update performance metrics (called by consensus)
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::update_performance_metrics())]
        pub fn update_performance_metrics(
            origin: OriginFor<T>,
            validator: T::AccountId,
            blocks_produced: u32,
            blocks_missed: u32,
            era_points: u32,
        ) -> DispatchResult {
            T::PoolManagerOrigin::ensure_origin(origin)?;
            
            PerformanceMetrics::<T>::mutate(&validator, |metrics| {
                metrics.blocks_produced = metrics.blocks_produced.saturating_add(blocks_produced);
                metrics.blocks_missed = metrics.blocks_missed.saturating_add(blocks_missed);
                metrics.era_points = era_points;
                metrics.last_active_era = Self::current_era();
                
                // Update reputation based on performance
                let total_blocks = metrics.blocks_produced + metrics.blocks_missed;
                if total_blocks > 0 {
                    let success_rate = (metrics.blocks_produced * 100) / total_blocks;
                    metrics.reputation_score = success_rate.min(100) as u8;
                }
            });
            
            let updated_metrics = Self::performance_metrics(&validator);
            Self::deposit_event(Event::PerformanceUpdated {
                validator,
                metrics: updated_metrics,
            });
            
            Ok(())
        }
    }

    // ============================================================================
    // INTERNAL METHODS
    // ============================================================================

    impl<T: Config> Pallet<T> {
        /// Validate category requirements
        fn validate_category_requirements(
            who: &T::AccountId,
            category: &ValidatorPoolCategory,
        ) -> DispatchResult {
            // Skip validation during benchmarking
            #[cfg(feature = "runtime-benchmarks")]
            return Ok(());
            
            match category {
                ValidatorPoolCategory::StakeValidator { min_stake, trust_threshold } => {
                    // Check minimum stake (implementation depends on staking pallet)
                    ensure!(*min_stake >= T::MinStakeAmount::get(), Error::<T>::InsufficientStake);
                    
                    // Check trust score
                    let trust_score = T::TrustSource::trust_score_of(who);
                    ensure!(trust_score >= *trust_threshold, Error::<T>::InsufficientTrustScore);
                },
                ValidatorPoolCategory::ParliamentaryValidator => {
                    // Check if user has Parlementer tiki
                    let tiki_score = T::TikiSource::get_tiki_score(who);
                    ensure!(tiki_score > 0, Error::<T>::MissingRequiredTiki);
                },
                ValidatorPoolCategory::MeritValidator { special_tikis: _, community_threshold } => {
                    // Check special tikis
                    let user_tiki_score = T::TikiSource::get_tiki_score(who);
                    ensure!(user_tiki_score > 0, Error::<T>::MissingRequiredTiki);
                    
                    // Check community support (referral count)
                    let referral_count = T::ReferralSource::get_referral_count(who);
                    ensure!(referral_count >= *community_threshold, Error::<T>::InsufficientCommunitySupport);
                },
            }
            Ok(())
        }

        /// Perform new era transition
        pub fn do_new_era() -> DispatchResult {
            let current_era = Self::current_era();
            let new_era = current_era.saturating_add(1);
            
            // Select new validator set
            let new_validator_set = Self::select_validators_for_era()?;
            
            // Update storage
            CurrentEra::<T>::put(new_era);
            EraStart::<T>::put(frame_system::Pallet::<T>::block_number());
            CurrentValidatorSet::<T>::put(&new_validator_set);
            
            // Update selection history for selected validators
            for validator in new_validator_set.all_validators() {
                SelectionHistory::<T>::mutate(validator, |history| {
                    if history.try_push(new_era).is_err() {
                        // If full, remove oldest and add new
                        history.remove(0);
                        let _ = history.try_push(new_era);
                    }
                });
            }
            
            Self::deposit_event(Event::NewEraStarted {
                era_index: new_era,
                validator_set: new_validator_set,
            });
            
            Ok(())
        }

        /// Select validators for new era using randomness and constraints
        fn select_validators_for_era() -> Result<ValidatorSet<T::AccountId>, Error<T>> {
            let target_validators = T::MaxValidators::get();
            
            // Target distribution: 10 stake, 6 parliamentary, 5 merit
            let stake_target = (target_validators * 10) / 21;
            let parliamentary_target = (target_validators * 6) / 21;  
            let merit_target = target_validators - stake_target - parliamentary_target;
            
            let mut stake_validators = Vec::new();
            let mut parliamentary_validators = Vec::new();
            let mut merit_validators = Vec::new();
            
            // Get randomness for selection
            let random_seed = T::Randomness::random(b"validator_selection").0;
            let mut random_index = 0u32;
            
            // Collect eligible validators by category
            for (validator, category) in PoolMembers::<T>::iter() {
                // Skip if selected in last 3 eras (rotation rule)
                let history = SelectionHistory::<T>::get(&validator);
                let current_era = Self::current_era();
                if history.iter().any(|&era| current_era.saturating_sub(era) < 3) {
                    continue;
                }
                
                // Check performance threshold
                let performance = Self::performance_metrics(&validator);
                if performance.reputation_score < 70 {
                    continue;
                }
                
                match category {
                    ValidatorPoolCategory::StakeValidator { .. } => {
                        if stake_validators.len() < stake_target as usize {
                            stake_validators.push(validator);
                        }
                    },
                    ValidatorPoolCategory::ParliamentaryValidator => {
                        if parliamentary_validators.len() < parliamentary_target as usize {
                            parliamentary_validators.push(validator);
                        }
                    },
                    ValidatorPoolCategory::MeritValidator { .. } => {
                        if merit_validators.len() < merit_target as usize {
                            merit_validators.push(validator);
                        }
                    },
                }
            }
            
            // Shuffle using randomness
            Self::shuffle_validators(&mut stake_validators, &random_seed, &mut random_index);
            Self::shuffle_validators(&mut parliamentary_validators, &random_seed, &mut random_index);
            Self::shuffle_validators(&mut merit_validators, &random_seed, &mut random_index);
            
            // Take required amounts
            stake_validators.truncate(stake_target as usize);
            parliamentary_validators.truncate(parliamentary_target as usize);
            merit_validators.truncate(merit_target as usize);
            
            // Ensure minimum validator count
            let total_selected = stake_validators.len() + parliamentary_validators.len() + merit_validators.len();
            ensure!(total_selected >= 3, Error::<T>::NotEnoughValidators); // BFT minimum
            
            let validator_set = ValidatorSet {
                era_index: Self::current_era().saturating_add(1),
                stake_validators: stake_validators.try_into().map_err(|_| Error::<T>::NotEnoughValidators)?,
                parliamentary_validators: parliamentary_validators.try_into().map_err(|_| Error::<T>::NotEnoughValidators)?,
                merit_validators: merit_validators.try_into().map_err(|_| Error::<T>::NotEnoughValidators)?,
            };
            
            Ok(validator_set)
        }
        
        /// Simple shuffle implementation using randomness
        fn shuffle_validators(
            validators: &mut Vec<T::AccountId>,
            seed: &T::Hash,
            index: &mut u32,
        ) {
            let seed_bytes = seed.as_ref();
            for i in (1..validators.len()).rev() {
                let random_byte = seed_bytes.get(*index as usize % seed_bytes.len()).unwrap_or(&0);
                *index = index.saturating_add(1);
                let j = (*random_byte as usize) % (i + 1);
                validators.swap(i, j);
            }
        }
    }

    // ============================================================================
    // SESSION MANAGER IMPLEMENTATION
    // ============================================================================

    impl<T: Config> pallet_session::SessionManager<T::AccountId> for Pallet<T> {
        fn new_session(_new_index: u32) -> Option<Vec<T::AccountId>> {
            // Return current validator set
            Self::current_validator_set().map(|set| set.all_validators())
        }

        fn end_session(_end_index: u32) {
            // Update performance metrics for ending session
            // This would be called by the consensus mechanism
        }

        fn start_session(_start_index: u32) {
            // Called when new session starts
            // Can be used for additional initialization
        }
    }
}