#![cfg_attr(not(feature = "std"), no_std)]

//! # Staking Score Pallet
//!
//! A pallet for calculating time-weighted staking scores based on stake amount and duration.
//!
//! ## Overview
//!
//! The Staking Score pallet calculates reputation scores from staking behavior by considering:
//! - **Stake Amount**: How much a user has staked
//! - **Stake Duration**: How long tokens have been staked
//! - **Nomination Count**: Number of validators nominated
//! - **Unlocking Chunks**: Pending unstake operations
//!
//! These metrics combine to produce a staking score that contributes to the composite
//! trust score in `pallet-trust`.
//!
//! ## Score Calculation
//!
//! ```text
//! staking_score = base_score + time_bonus
//!
//! where:
//! base_score = (staked_amount / UNITS) * 10
//! time_bonus = (months_staked * staked_amount * 0.05) / UNITS
//! ```
//!
//! ### Time-Based Rewards
//! - First month: Base score only
//! - Each additional month: +5% bonus on staked amount
//! - Maximum benefit achieved through long-term commitment
//! - Score increases linearly with time
//!
//! ## Workflow
//!
//! 1. User stakes tokens via main staking pallet
//! 2. User calls `start_score_tracking()` to begin time tracking
//! 3. Tracking start block is recorded
//! 4. `pallet-trust` queries staking score via `StakingScoreProvider` trait
//! 5. Score calculation uses current block number vs. start block
//! 6. Time bonus accumulates automatically each month
//!
//! ## Integration with Staking
//!
//! This pallet does not handle staking operations directly. It:
//! - Reads staking data from main staking pallet via `StakingInfoProvider`
//! - Tracks when users want to start earning time bonuses
//! - Calculates scores on-demand without modifying staking state
//!
//! ## Score Components
//!
//! ### Staked Amount
//! - Primary factor in score calculation
//! - Measured in balance units (UNITS = 10^12)
//! - Higher stake = higher base score
//!
//! ### Duration
//! - Measured in months (30 days * 24 hours * 60 min * 10 blocks/min)
//! - ~432,000 blocks per month
//! - Compounds monthly for long-term stakers
//!
//! ### Additional Metrics
//! - Nomination count (contributes to complexity score)
//! - Unlocking chunks (indicates unstaking activity)
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! - `start_score_tracking()` - Begin time-based score accumulation (user, one-time)
//!
//! ### Storage
//!
//! - `StakingStartBlock` - Block number when user started score tracking
//!
//! ### Trait Implementations
//!
//! - `StakingScoreProvider` - Query staking scores for trust calculation
//!
//! ## Dependencies
//!
//! This pallet requires:
//! - Main staking pallet implementing `StakingInfoProvider`
//! - `pallet-trust` as consumer of staking scores
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_staking_score::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type Balance = Balance;
//!     type StakingInfo = Staking; // Main staking pallet
//!     type WeightInfo = pallet_staking_score::weights::SubstrateWeight<Runtime>;
//! }
//! ```

pub use pallet::*;

// Mock staking info provider for benchmarking - ADD THIS
#[cfg(feature = "runtime-benchmarks")]
pub struct BenchmarkStakingInfoProvider;

#[cfg(feature = "runtime-benchmarks")]
impl<AccountId, Balance> StakingInfoProvider<AccountId, Balance> for BenchmarkStakingInfoProvider
where
    Balance: From<u128>,
{
    fn get_staking_details(_who: &AccountId) -> Option<StakingDetails<Balance>> {
        // Always return valid stake for benchmarking
        Some(StakingDetails {
            staked_amount: (1000u128 * UNITS).into(),
            nominations_count: 5,
            unlocking_chunks_count: 2,
        })
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::weights::WeightInfo; // Properly importing WeightInfo from parent module.
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::{traits::{Saturating, Zero}, Perbill};
	use sp_std::ops::Div;

	// --- Sabitler ---
	pub const MONTH_IN_BLOCKS: u32 = 30 * 24 * 60 * 10;
	pub const UNITS: u128 = 1_000_000_000_000;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + TypeInfo
	where
		// Ensuring BlockNumber is convertible from u32.
		BlockNumberFor<Self>: From<u32>,
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Balance type to be used for staking.
		/// Adding all required mathematical and comparison properties.
		type Balance: Member
			+ Parameter
			+ MaxEncodedLen
			+ Copy
			+ Default
			+ PartialOrd
			+ Saturating
			+ Zero
			+ Div<Output = Self::Balance> // Specifying that division result is also Balance.
			+ From<u128>;
		/// Interface to be used for reading staking data.
		type StakingInfo: StakingInfoProvider<Self::AccountId, Self::Balance>;
		/// To provide extrinsic weights.
		type WeightInfo: WeightInfo;
	}

	// --- Depolama (Storage) ---
	#[pallet::storage]
	#[pallet::getter(fn staking_start_block)]
	pub type StakingStartBlock<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberFor<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user started time-based scoring.
		ScoreTrackingStarted { who: T::AccountId, start_block: BlockNumberFor<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Puan takibini başlatmak için önce stake yapmış olmalısınız.
		NoStakeFound,
		/// Puan takibi zaten daha önce başlatılmış.
		TrackingAlreadyStarted,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Süreye dayalı puanlamayı manuel olarak aktive eder.
		/// Bu fonksiyon, her kullanıcı tarafından sadece bir kez çağrılmalıdır.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::start_score_tracking())]
		pub fn start_score_tracking(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			
			// 1. Kullanıcının puan takibini daha önce başlatıp başlatmadığını kontrol et.
			ensure!(StakingStartBlock::<T>::get(&who).is_none(), Error::<T>::TrackingAlreadyStarted);

			// 2. Kullanıcının ana staking paletinde stake'i var mı diye kontrol et.
			// `get_staking_details` artık Option döndürdüğü için `ok_or` ile hata yönetimi yapıyoruz.
			let details = T::StakingInfo::get_staking_details(&who).ok_or(Error::<T>::NoStakeFound)?;
			ensure!(!details.staked_amount.is_zero(), Error::<T>::NoStakeFound);

			// 3. O anki blok numarasını kaydet.
			let current_block = frame_system::Pallet::<T>::block_number();
			StakingStartBlock::<T>::insert(&who, current_block);

			Self::deposit_event(Event::ScoreTrackingStarted { who, start_block: current_block });
			Ok(())
		}
	}

	// --- Arayüz (Trait) ve Tip Tanımları ---

	/// Puanlamada kullanılacak ham skor tipi.
	pub type RawScore = u32;

	/// Staking ile ilgili detayları bir arada tutan ve dışarıdan alınacak veri yapısı.
	/// `Default` ekledik çünkü testlerde ve mock'larda işimizi kolaylaştıracak.
	#[derive(Default, Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Debug)]
	pub struct StakingDetails<Balance> {
		pub staked_amount: Balance,
		pub nominations_count: u32,
		pub unlocking_chunks_count: u32,
	}
	
	/// Bu paletin dış dünyaya sunduğu arayüz.
	pub trait StakingScoreProvider<AccountId, BlockNumber> {
		/// Returns the score and the duration in blocks used for calculation.
		fn get_staking_score(who: &AccountId) -> (RawScore, BlockNumber);
	}

	/// Bu paletin, staking verilerini almak için ihtiyaç duyduğu arayüz.
	pub trait StakingInfoProvider<AccountId, Balance> {
		/// Verilen hesap için staking detaylarını döndürür.
		/// Eğer kullanıcının stake'i yoksa `None` dönmelidir. Bu daha güvenli bir yöntemdir.
		fn get_staking_details(who: &AccountId) -> Option<StakingDetails<Balance>>;
	}

	// --- Trait Implementasyonu ---

	impl<T: Config> StakingScoreProvider<T::AccountId, BlockNumberFor<T>> for Pallet<T> {
		fn get_staking_score(who: &T::AccountId) -> (RawScore, BlockNumberFor<T>) {
			// 1. Staking detaylarını al. Eğer stake yoksa (None) 0 puan döndür.
			let staking_details = match T::StakingInfo::get_staking_details(who) {
				Some(details) => details,
				None => return (0, Zero::zero()),
			};

			// Staked miktarı ana birime (HEZ) çevir.
			let staked_hez: T::Balance = staking_details.staked_amount / UNITS.into();

			// "Sıfır stake, sıfır puan" kuralını uygula.
			if staked_hez.is_zero() {
				return (0, Zero::zero());
			}

			// Miktara dayalı temel puanı hesapla.
			let amount_score: u32 = if staked_hez <= 100u128.into() {
				20
			} else if staked_hez <= 250u128.into() {
				30
			} else if staked_hez <= 750u128.into() {
				40
			} else {
				50 // 751+ HEZ
			};

			// Süreye dayalı çarpanı ve duration'ı hesapla.
			let (duration_multiplier, duration_for_return) = match StakingStartBlock::<T>::get(who) {
				// Eğer kullanıcı `start_score_tracking` çağırdıysa...
				Some(start_block) => {
					let current_block = frame_system::Pallet::<T>::block_number();
					let duration_in_blocks = current_block.saturating_sub(start_block);

					let multiplier = if duration_in_blocks >= (12 * MONTH_IN_BLOCKS).into() {
					Perbill::from_rational(2u32, 1u32) // x2.0 (12 ay ve üstü)
					} else if duration_in_blocks >= (6 * MONTH_IN_BLOCKS).into() {
						Perbill::from_rational(17u32, 10u32) // x1.7 (6-11 ay)
					} else if duration_in_blocks >= (3 * MONTH_IN_BLOCKS).into() {
						Perbill::from_rational(7u32, 5u32) // x1.4 (3-5 ay)
					} else if duration_in_blocks >= MONTH_IN_BLOCKS.into() {
						Perbill::from_rational(6u32, 5u32) // x1.2 (1-2 ay)
					} else {
						Perbill::from_rational(1u32, 1u32) // x1.0 (< 1 ay)
					};
					
					(multiplier, duration_in_blocks)
				},
				// Eğer takip başlatılmadıysa, çarpan 1.0'dır.
				None => (Perbill::from_rational(10u32, 10u32), Zero::zero()),
			};

			// Nihai puanı hesapla ve 100 ile sınırla.
			let final_score = match StakingStartBlock::<T>::get(who) {
				Some(start_block) => {
					let current_block = frame_system::Pallet::<T>::block_number();
					let duration_in_blocks = current_block.saturating_sub(start_block);

					if duration_in_blocks >= (12 * MONTH_IN_BLOCKS).into() {
						amount_score * 2 // x2.0
					} else if duration_in_blocks >= (6 * MONTH_IN_BLOCKS).into() {
						amount_score * 17 / 10 // x1.7
					} else if duration_in_blocks >= (3 * MONTH_IN_BLOCKS).into() {
						amount_score * 14 / 10 // x1.4
					} else if duration_in_blocks >= MONTH_IN_BLOCKS.into() {
						amount_score * 12 / 10 // x1.2
					} else {
						amount_score // x1.0
					}
				},
				None => amount_score, // Takip başlatılmadıysa çarpan yok
			};
			
			(final_score.min(100), duration_for_return)
		}
	}
}