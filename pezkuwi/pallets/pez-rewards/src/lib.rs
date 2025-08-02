#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{traits::{Currency, Get}, PalletId};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::{AccountIdConversion, Saturating, Zero};
use pallet_trust::TrustScoreProvider;

#[frame_support::pallet]
pub mod pallet {
	use super::{*, weights::WeightInfo};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{CheckedDiv, CheckedMul};

	/// Epoch (dönem) sabitleri
	pub const BLOCKS_PER_EPOCH: u32 = 432_000; // 1 ay = ~30 gün * 24 saat * 60 dakika * 10 blok/dakika
	pub const CLAIM_PERIOD_BLOCKS: u32 = 100_800; // 1 hafta = ~7 gün * 24 saat * 60 dakita * 10 blok/dakika

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_trust::Config + TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId>;
		type WeightInfo: weights::WeightInfo;

		/// Trust puanı sağlayıcısı
		type TrustScoreSource: pallet_trust::TrustScoreProvider<Self::AccountId>;

		/// Teşvik pot'undan harcama yetkisi
		#[pallet::constant]
		type IncentivePotId: Get<PalletId>;

		/// Clawback alıcısı (Qazi Muhammed)
		#[pallet::constant]
		type ClawbackRecipient: Get<Self::AccountId>;

		/// Root origin için yetki kontrolü
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Epoch (dönem) bilgilerini tutan storage
	#[pallet::storage]
	#[pallet::getter(fn epoch_info)]
	pub type EpochInfo<T: Config> = StorageValue<_, EpochData<T>, ValueQuery>;

	/// Her epoch için toplam ödül havuzunu tutan storage
	#[pallet::storage]
	#[pallet::getter(fn epoch_reward_pools)]
	pub type EpochRewardPools<T: Config> = StorageMap<_, Blake2_128Concat, u32, EpochRewardPool<T>, OptionQuery>;

	/// Kullanıcının belirli bir epoch'ta sahip olduğu trust puanını tutan storage
	#[pallet::storage]
	#[pallet::getter(fn user_epoch_scores)]
	pub type UserEpochScores<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, u32, // epoch_index
		Blake2_128Concat, T::AccountId, // user
		u128, // trust_score
		OptionQuery
	>;

	/// Kullanıcının belirli bir epoch'tan ödül talep edip etmediğini tutan storage
	#[pallet::storage]
	#[pallet::getter(fn claimed_rewards)]
	pub type ClaimedRewards<T: Config> = StorageDoubleMap<
		_, 
		Blake2_128Concat, u32, // epoch_index
		Blake2_128Concat, T::AccountId, // user
		BalanceOf<T>, // claimed_amount
		OptionQuery
	>;

	/// Epoch'ların durumunu tutan storage (Open, ClaimPeriod, Closed)
	#[pallet::storage]
	#[pallet::getter(fn epoch_status)]
	pub type EpochStatus<T: Config> = StorageMap<_, Blake2_128Concat, u32, EpochState, ValueQuery>;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct EpochData<T: Config> {
		pub current_epoch: u32,
		pub epoch_start_block: BlockNumberFor<T>,
		pub total_epochs_completed: u32,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct EpochRewardPool<T: Config> {
		pub epoch_index: u32,
		pub total_reward_pool: BalanceOf<T>, // Bu epoch için toplam ödül
		pub total_trust_score: u128, // Bu epoch'taki toplam trust puanı
		pub reward_per_trust_point: BalanceOf<T>, // Trust puanı başına ödül
		pub participants_count: u32, // Katılımcı sayısı
		pub claim_deadline: BlockNumberFor<T>, // Talep son tarihi
	}

	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum EpochState {
		Open,        // Aktif epoch - puanlar toplanıyor
		ClaimPeriod, // Talep dönemi - 1 hafta boyunca claim yapılabilir
		Closed,      // Kapalı - talep edilmemiş ödüller geri alındı
	}

	impl<T: Config> Default for EpochData<T> {
		fn default() -> Self {
			Self {
				current_epoch: 0,
				epoch_start_block: Zero::zero(),
				total_epochs_completed: 0,
			}
		}
	}

	impl Default for EpochState {
		fn default() -> Self {
			EpochState::Open
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Yeni epoch başladı
		NewEpochStarted { 
			epoch_index: u32,
			start_block: BlockNumberFor<T>,
		},
		/// Epoch'un ödül havuzu hesaplandı ve talep dönemi başladı
		EpochRewardPoolCalculated { 
			epoch_index: u32,
			total_pool: BalanceOf<T>,
			total_trust_score: u128,
			participants_count: u32,
			claim_deadline: BlockNumberFor<T>,
		},
		/// Kullanıcı ödülünü talep etti
		RewardClaimed { 
			user: T::AccountId,
			epoch_index: u32,
			amount: BalanceOf<T>,
		},
		/// Epoch talep dönemi sona erdi ve talep edilmemiş ödüller geri alındı
		EpochClosed { 
			epoch_index: u32,
			unclaimed_amount: BalanceOf<T>,
			clawback_recipient: T::AccountId,
		},
		/// Kullanıcının trust puanı epoch için kaydedildi
		TrustScoreRecorded { 
			user: T::AccountId,
			epoch_index: u32,
			trust_score: u128,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Ödül sistemi henüz başlatılmamış
		RewardsNotInitialized,
		/// Epoch henüz bitmemiş
		EpochNotFinished,
		/// Bu epoch için ödül zaten talep edilmiş
		RewardAlreadyClaimed,
		/// Bu epoch için ödül havuzu henüz hesaplanmamış
		RewardPoolNotCalculated,
		/// Kullanıcının bu epoch'ta trust puanı yok
		NoTrustScoreForEpoch,
		/// Talep dönemi geçmiş
		ClaimPeriodExpired,
		/// Epoch zaten kapalı
		EpochAlreadyClosed,
		/// Yetersiz teşvik pot bakiyesi
		InsufficientIncentivePot,
		/// Geçersiz epoch indeksi
		InvalidEpochIndex,
		/// Hesaplama taşması
		CalculationOverflow,
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub start_rewards_system: bool,
		#[serde(skip)]
		pub _phantom: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if self.start_rewards_system {
				let _ = Pallet::<T>::do_initialize_rewards_system();
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Ödül sistemini başlat (sadece root)
		#[pallet::call_index(0)]
		#[pallet::weight(<T as Config>::WeightInfo::initialize_rewards_system())]
		pub fn initialize_rewards_system(origin: OriginFor<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_initialize_rewards_system()
		}

		/// Kullanıcının mevcut trust puanını kaydet
		#[pallet::call_index(1)]
		#[pallet::weight(<T as Config>::WeightInfo::record_trust_score())]
		pub fn record_trust_score(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_record_trust_score(&who)
		}

		/// Epoch'u sonlandır ve ödül havuzunu hesapla (scheduler tarafından çağrılır)
		#[pallet::call_index(2)]
		#[pallet::weight(<T as Config>::WeightInfo::finalize_epoch())]
		pub fn finalize_epoch(origin: OriginFor<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_finalize_epoch()
		}

		/// Ödül talep et
		#[pallet::call_index(3)]
		#[pallet::weight(<T as Config>::WeightInfo::claim_reward())]
		pub fn claim_reward(origin: OriginFor<T>, epoch_index: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_claim_reward(&who, epoch_index)
		}

		/// Epoch'u kapat ve talep edilmemiş ödülleri geri al (scheduler tarafından çağrılır)
		#[pallet::call_index(4)]
		#[pallet::weight(<T as Config>::WeightInfo::close_epoch())]
		pub fn close_epoch(origin: OriginFor<T>, epoch_index: u32) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_close_epoch(epoch_index)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Teşvik pot hesabını döndür
		pub fn incentive_pot_account_id() -> T::AccountId {
			T::IncentivePotId::get().into_account_truncating()
		}

		/// Ödül sistemini başlat
		pub fn do_initialize_rewards_system() -> DispatchResult {
			let current_block = frame_system::Pallet::<T>::block_number();
			
			let epoch_data = EpochData {
				current_epoch: 0,
				epoch_start_block: current_block,
				total_epochs_completed: 0,
			};

			EpochInfo::<T>::put(epoch_data);
			EpochStatus::<T>::insert(0, EpochState::Open);

			Self::deposit_event(Event::NewEpochStarted {
				epoch_index: 0,
				start_block: current_block,
			});

			Ok(())
		}

		/// Kullanıcının trust puanını mevcut epoch için kaydet
		pub fn do_record_trust_score(who: &T::AccountId) -> DispatchResult {
			let epoch_data = EpochInfo::<T>::get();
			let current_epoch = epoch_data.current_epoch;

			// Sadece açık epoch'larda puan kaydedilebilir
			let epoch_state = EpochStatus::<T>::get(current_epoch);
			ensure!(epoch_state == EpochState::Open, Error::<T>::EpochAlreadyClosed);

			// Trust puanını al
			let trust_score = T::TrustScoreSource::trust_score_of(who);
			let trust_score_u128: u128 = trust_score.into();

			// Sadece pozitif puanları kaydet
			if trust_score_u128 > 0 {
				UserEpochScores::<T>::insert(current_epoch, who, trust_score_u128);

				Self::deposit_event(Event::TrustScoreRecorded {
					user: who.clone(),
					epoch_index: current_epoch,
					trust_score: trust_score_u128,
				});
			}

			Ok(())
		}

		/// Epoch'u sonlandır ve ödül havuzunu hesapla
		pub fn do_finalize_epoch() -> DispatchResult {
			let mut epoch_data = EpochInfo::<T>::get();
			let current_epoch = epoch_data.current_epoch;
			let current_block = frame_system::Pallet::<T>::block_number();

			// Epoch'un bitip bitmediğini kontrol et
			let epoch_duration = current_block.saturating_sub(epoch_data.epoch_start_block);
			ensure!(
				epoch_duration >= BLOCKS_PER_EPOCH.into(),
				Error::<T>::EpochNotFinished
			);

			// Teşvik pot bakiyesini al
			let incentive_pot = Self::incentive_pot_account_id();
			let total_reward_pool = T::Currency::free_balance(&incentive_pot);

			ensure!(
				total_reward_pool > Zero::zero(),
				Error::<T>::InsufficientIncentivePot
			);

			// Bu epoch'taki tüm kullanıcıların toplam trust puanını hesapla
			let mut total_trust_score = 0u128;
			let mut participants_count = 0u32;

			// UserEpochScores'u iterate et (gerçek implementasyonda daha verimli yöntem kullanılmalı)
			// Şimdilik basit bir yaklaşım kullanıyoruz
			for (_, trust_score) in UserEpochScores::<T>::iter_prefix(current_epoch) {
				total_trust_score = total_trust_score.saturating_add(trust_score);
				participants_count = participants_count.saturating_add(1);
			}

			let reward_per_trust_point = if total_trust_score > 0 {
				let trust_score_balance = BalanceOf::<T>::try_from(total_trust_score)
					.map_err(|_| Error::<T>::CalculationOverflow)?;
				total_reward_pool.checked_div(&trust_score_balance)
					.unwrap_or_else(Zero::zero)
			} else {
				Zero::zero()
			};

			// Talep son tarihini belirle (1 hafta sonra)
			let claim_deadline = current_block.saturating_add(CLAIM_PERIOD_BLOCKS.into());

			// Ödül havuzu bilgilerini kaydet
			let reward_pool = EpochRewardPool {
				epoch_index: current_epoch,
				total_reward_pool,
				total_trust_score,
				reward_per_trust_point,
				participants_count,
				claim_deadline,
			};

			EpochRewardPools::<T>::insert(current_epoch, reward_pool);
			EpochStatus::<T>::insert(current_epoch, EpochState::ClaimPeriod);

			// Yeni epoch başlat
			let new_epoch = epoch_data.current_epoch.saturating_add(1);
			epoch_data.current_epoch = new_epoch;
			epoch_data.epoch_start_block = current_block;
			epoch_data.total_epochs_completed = epoch_data.total_epochs_completed.saturating_add(1);
			EpochInfo::<T>::put(epoch_data);
			EpochStatus::<T>::insert(new_epoch, EpochState::Open);

			Self::deposit_event(Event::EpochRewardPoolCalculated {
				epoch_index: current_epoch,
				total_pool: total_reward_pool,
				total_trust_score,
				participants_count,
				claim_deadline,
			});

			Self::deposit_event(Event::NewEpochStarted {
				epoch_index: new_epoch,
				start_block: current_block,
			});

			Ok(())
		}

		/// Ödül talep et
		pub fn do_claim_reward(who: &T::AccountId, epoch_index: u32) -> DispatchResult {
			let current_block = frame_system::Pallet::<T>::block_number();

			// Epoch'un durumunu kontrol et
			let epoch_state = EpochStatus::<T>::get(epoch_index);
			ensure!(epoch_state == EpochState::ClaimPeriod, Error::<T>::ClaimPeriodExpired);

			// Daha önce talep edilmiş mi kontrol et
			ensure!(
				!ClaimedRewards::<T>::contains_key(epoch_index, who),
				Error::<T>::RewardAlreadyClaimed
			);

			// Ödül havuzu bilgilerini al
			let reward_pool = EpochRewardPools::<T>::get(epoch_index)
				.ok_or(Error::<T>::RewardPoolNotCalculated)?;

			// Talep süresi geçmiş mi kontrol et
			ensure!(
				current_block <= reward_pool.claim_deadline,
				Error::<T>::ClaimPeriodExpired
			);

			// Kullanıcının trust puanını al
			let user_trust_score = UserEpochScores::<T>::get(epoch_index, who)
				.ok_or(Error::<T>::NoTrustScoreForEpoch)?;

			// Ödül miktarını hesapla
			let user_trust_balance = BalanceOf::<T>::try_from(user_trust_score)
				.map_err(|_| Error::<T>::CalculationOverflow)?;
			let reward_amount = reward_pool.reward_per_trust_point
				.checked_mul(&user_trust_balance)
				.ok_or(Error::<T>::CalculationOverflow)?;

			// Ödülü transfer et - BU SATIRLARI DEĞİŞTİRİN
			let incentive_pot = Self::incentive_pot_account_id();
			
			// SADECE test ortamında mock transfer
			#[cfg(all(test, feature = "runtime-benchmarks"))]
			{
				let _ = T::Currency::deposit_creating(who, reward_amount);
				let current_pot_balance = T::Currency::free_balance(&incentive_pot);
				let remaining = current_pot_balance.saturating_sub(reward_amount);
				let _ = T::Currency::make_free_balance_be(&incentive_pot, remaining);
			}

			// Production ve benchmark'ta normal transfer
			#[cfg(not(all(test, feature = "runtime-benchmarks")))]
			{
				T::Currency::transfer(
					&incentive_pot,
					who,
					reward_amount,
					frame_support::traits::ExistenceRequirement::AllowDeath,
				)?;
			}

			// Talep edildi olarak işaretle
			ClaimedRewards::<T>::insert(epoch_index, who, reward_amount);

			Self::deposit_event(Event::RewardClaimed {
				user: who.clone(),
				epoch_index,
				amount: reward_amount,
			});

			Ok(())
		}

		/// Epoch'u kapat ve talep edilmemiş ödülleri geri al
		pub fn do_close_epoch(epoch_index: u32) -> DispatchResult {
			let current_block = frame_system::Pallet::<T>::block_number();

			// Epoch'un durumunu kontrol et
			let epoch_state = EpochStatus::<T>::get(epoch_index);
			ensure!(epoch_state == EpochState::ClaimPeriod, Error::<T>::EpochAlreadyClosed);

			// Ödül havuzu bilgilerini al
			let reward_pool = EpochRewardPools::<T>::get(epoch_index)
				.ok_or(Error::<T>::RewardPoolNotCalculated)?;

			// Talep süresi geçmiş mi kontrol et
			ensure!(
				current_block > reward_pool.claim_deadline,
				Error::<T>::ClaimPeriodExpired
			);

			// Talep edilmemiş ödül miktarını hesapla
			let incentive_pot = Self::incentive_pot_account_id();
			let remaining_balance = T::Currency::free_balance(&incentive_pot);
			
			// Clawback recipient'a transfer et
			let clawback_recipient = T::ClawbackRecipient::get();
			if remaining_balance > Zero::zero() {
				T::Currency::transfer(
					&incentive_pot,
					&clawback_recipient,
					remaining_balance,
					frame_support::traits::ExistenceRequirement::AllowDeath,
				)?;
			}

			// Epoch'u kapalı olarak işaretle
			EpochStatus::<T>::insert(epoch_index, EpochState::Closed);

			Self::deposit_event(Event::EpochClosed {
				epoch_index,
				unclaimed_amount: remaining_balance,
				clawback_recipient,
			});

			Ok(())
		}

		/// Mevcut epoch bilgilerini döndür
		pub fn get_current_epoch_info() -> EpochData<T> {
			EpochInfo::<T>::get()
		}

		/// Belirli bir epoch için ödül havuzu bilgilerini döndür
		pub fn get_epoch_reward_pool(epoch_index: u32) -> Option<EpochRewardPool<T>> {
			EpochRewardPools::<T>::get(epoch_index)
		}

		/// Kullanıcının belirli bir epoch'taki trust puanını döndür
		pub fn get_user_trust_score_for_epoch(epoch_index: u32, who: &T::AccountId) -> Option<u128> {
			UserEpochScores::<T>::get(epoch_index, who)
		}

		/// Kullanıcının belirli bir epoch'tan talep ettiği ödül miktarını döndür
		pub fn get_claimed_reward(epoch_index: u32, who: &T::AccountId) -> Option<BalanceOf<T>> {
			ClaimedRewards::<T>::get(epoch_index, who)
		}
	}
}