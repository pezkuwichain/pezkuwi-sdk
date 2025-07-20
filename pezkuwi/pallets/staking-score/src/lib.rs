#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::weights::WeightInfo; // WeightInfo'yu üst modülden doğru şekilde import ediyoruz.
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
		// BlockNumber'ın u32'den dönüştürülebilir olduğunu garanti ediyoruz.
		BlockNumberFor<Self>: From<u32>,
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Staking için kullanılacak bakiye tipi.
		/// Gerekli olan tüm matematiksel ve karşılaştırma özelliklerini ekliyoruz.
		type Balance: Member
			+ Parameter
			+ MaxEncodedLen
			+ Copy
			+ Default
			+ PartialOrd
			+ Saturating
			+ Zero
			+ Div<Output = Self::Balance> // Bölme işleminin sonucunun da Balance olduğunu belirtiyoruz.
			+ From<u128>;
		/// Staking verilerini okumak için kullanılacak arayüz.
		type StakingInfo: StakingInfoProvider<Self::AccountId, Self::Balance>;
		/// Extrinsic'lerin ağırlıklarını sağlamak için.
		type WeightInfo: WeightInfo;
	}

	// --- Depolama (Storage) ---
	#[pallet::storage]
	#[pallet::getter(fn staking_start_block)]
	pub type StakingStartBlock<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BlockNumberFor<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Bir kullanıcı, süreye dayalı puanlamayı başlattı.
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
			let (staked_amount, _) = T::StakingInfo::get_staking_details(&who);
			ensure!(!staked_amount.is_zero(), Error::<T>::NoStakeFound);

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
	
	/// Bu paletin dış dünyaya sunduğu arayüz.
	pub trait StakingScoreProvider<AccountId> {
		fn get_staking_score(who: &AccountId) -> RawScore;
	}

	/// Bu paletin, staking verilerini almak için ihtiyaç duyduğu arayüz.
	pub trait StakingInfoProvider<AccountId, Balance> {
    	fn get_staking_details(who: &AccountId) -> (Balance, u32);
	}

	// --- Trait Implementasyonu ---

	impl<T: Config> StakingScoreProvider<T::AccountId> for Pallet<T> {
		fn get_staking_score(who: &T::AccountId) -> RawScore {
			let (staked_amount, _) = T::StakingInfo::get_staking_details(who);
			let staked_hez: T::Balance = staked_amount / UNITS.into();

			// Sıfır stake, sıfır puan kuralı
			if staked_hez.is_zero() {
				return 0;
			}

			let amount_score: u32 = if staked_hez <= 100u128.into() {
				20
			} else if staked_hez <= 250u128.into() {
				30
			} else if staked_hez <= 750u128.into() {
				40
			} else {
				50 // 751+ HEZ
			};

			// Hibrit Model: Süre çarpanını hesapla.
			let duration_multiplier = match StakingStartBlock::<T>::get(who) {
				// Eğer kullanıcı takibi manuel başlattıysa, süreye göre çarpan hesapla.
				Some(start_block) => {
					let current_block: BlockNumberFor<T> = frame_system::Pallet::<T>::block_number();
					let duration_in_blocks = current_block.saturating_sub(start_block);

					if duration_in_blocks > (12 * MONTH_IN_BLOCKS).into() {
						Perbill::from_rational(20u32, 10u32) // x2.0
					} else if duration_in_blocks > (6 * MONTH_IN_BLOCKS).into() {
						Perbill::from_rational(17u32, 10u32) // x1.7
					} else if duration_in_blocks > (3 * MONTH_IN_BLOCKS).into() {
						Perbill::from_rational(14u32, 10u32) // x1.4
					} else if duration_in_blocks > MONTH_IN_BLOCKS.into() {
						Perbill::from_rational(12u32, 10u32) // x1.2
					} else {
						Perbill::from_rational(10u32, 10u32) // x1.0
					}
				},
				// Eğer takip başlatılmadıysa, süre çarpanı 1.0'dır.
				None => Perbill::from_rational(10u32, 10u32), // x1.0
			};

			let final_score = duration_multiplier.mul_floor(amount_score);
			final_score.min(100)
		}
	}
}