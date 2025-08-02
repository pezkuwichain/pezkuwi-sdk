#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use scale_info::TypeInfo;
use frame_support::{
    traits::{Currency, Get, ReservableCurrency},
    PalletId,
};
use frame_system::pallet_prelude::BlockNumberFor;
use sp_runtime::traits::{AccountIdConversion, Saturating, Zero, CheckedDiv};

#[frame_support::pallet]
pub mod pallet {
	use super::{*, weights::WeightInfo};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::CheckedDiv;

	/// Sentetik Halving sabitleri
	pub const HALVING_PERIOD_MONTHS: u32 = 48; // 4 yıl = 48 ay
	pub const BLOCKS_PER_MONTH: u32 = 432_000; // ~30 gün * 24 saat * 60 dakika * 10 blok/dakika
	pub const HALVING_PERIOD_BLOCKS: u32 = HALVING_PERIOD_MONTHS * BLOCKS_PER_MONTH;

	/// Token dağılım sabitleri
	pub const TOTAL_SUPPLY: u128 = 5_000_000_000 * 1_000_000_000_000; // 5 milyar PEZ (12 decimal)
	pub const TREASURY_ALLOCATION: u128 = 4_812_500_000 * 1_000_000_000_000; // %96.25
	pub const PRESALE_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000; // %1.875
	pub const FOUNDER_ALLOCATION: u128 = 93_750_000 * 1_000_000_000_000; // %1.875

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + TypeInfo {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;
		type WeightInfo: weights::WeightInfo;

		/// Hazine PalletId - Sovereign account oluşturmak için
		#[pallet::constant]
		type TreasuryPalletId: Get<PalletId>;

		/// Teşvik Pot PalletId
		#[pallet::constant]
		type IncentivePotId: Get<PalletId>;

		/// Hükümet Pot PalletId  
		#[pallet::constant]
		type GovernmentPotId: Get<PalletId>;

		/// Presale cüzdanı hesabı
		#[pallet::constant]
		type PresaleAccount: Get<Self::AccountId>;

		/// Founder (Qazi Muhammed) hesabı
		#[pallet::constant]
		type FounderAccount: Get<Self::AccountId>;

		/// Root origin için yetki kontrolü
		type ForceOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Sentetik halving bilgilerini tutan storage
	#[pallet::storage]
	#[pallet::getter(fn halving_info)]
	pub type HalvingInfo<T: Config> = StorageValue<_, HalvingData<T>, ValueQuery>;

	/// Her ayın fon salınım bilgilerini tutan storage
	#[pallet::storage]
	#[pallet::getter(fn monthly_releases)]
	pub type MonthlyReleases<T: Config> = StorageMap<_, Blake2_128Concat, u32, MonthlyRelease<T>, OptionQuery>;

	/// Bir sonraki release edilecek ayın indeksi
	#[pallet::storage]
	#[pallet::getter(fn next_release_month)]
	pub type NextReleaseMonth<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// Treasury'nin başlangıç bloğu
	#[pallet::storage]
	#[pallet::getter(fn treasury_start_block)]
	pub type TreasuryStartBlock<T: Config> = StorageValue<_, BlockNumberFor<T>, OptionQuery>;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct HalvingData<T: Config> {
		pub current_period: u32,        // Mevcut halving periyodu (0, 1, 2, ...)
		pub period_start_block: BlockNumberFor<T>, // Bu periyodun başlangıç bloğu
		pub monthly_amount: BalanceOf<T>, // Bu periyotta aylık salınacak miktar
		pub total_released: BalanceOf<T>, // Toplam salınan miktar
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct MonthlyRelease<T: Config> {
		pub month_index: u32,
		pub release_block: BlockNumberFor<T>,
		pub amount_released: BalanceOf<T>,
		pub incentive_amount: BalanceOf<T>, // %75
		pub government_amount: BalanceOf<T>, // %25
	}

	impl<T: Config> Default for HalvingData<T> {
		fn default() -> Self {
			Self {
				current_period: 0,
				period_start_block: Zero::zero(),
				monthly_amount: Zero::zero(),
				total_released: Zero::zero(),
			}
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Treasury başlatıldı
		TreasuryInitialized { 
			start_block: BlockNumberFor<T>,
			initial_monthly_amount: BalanceOf<T>,
		},
		/// Aylık fon salınımı yapıldı
		MonthlyFundsReleased { 
			month_index: u32,
			total_amount: BalanceOf<T>,
			incentive_amount: BalanceOf<T>,
			government_amount: BalanceOf<T>,
		},
		/// Yeni halving periyodu başladı
		NewHalvingPeriod { 
			period: u32,
			new_monthly_amount: BalanceOf<T>,
		},
		/// Genesis token dağılımı tamamlandı
		GenesisDistributionCompleted {
			treasury_amount: BalanceOf<T>,
			presale_amount: BalanceOf<T>,
			founder_amount: BalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Treasury zaten başlatılmış
		TreasuryAlreadyInitialized,
		/// Treasury henüz başlatılmamış
		TreasuryNotInitialized,
		/// Bu ay için fon salınımı zaten yapılmış
		MonthlyReleaseAlreadyDone,
		/// Yetersiz hazine bakiyesi
		InsufficientTreasuryBalance,
		/// Geçersiz halving periyodu
		InvalidHalvingPeriod,
		/// Henüz salınım zamanı gelmemiş
		ReleaseTooEarly,
	}

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub initialize_treasury: bool,
		#[serde(skip)]
		pub _phantom: core::marker::PhantomData<T>,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if self.initialize_treasury {
				// Genesis'te token dağılımını yap
				let _ = Pallet::<T>::do_genesis_distribution();
				// Treasury'yi başlat
				let _ = Pallet::<T>::do_initialize_treasury();
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Treasury'yi manuel olarak başlat (sadece root)
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::initialize_treasury())]
		pub fn initialize_treasury(origin: OriginFor<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_initialize_treasury()
		}

		/// Aylık fon salınımını manual tetikle (scheduler tarafından çağrılır)
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::release_monthly_funds())]
		pub fn release_monthly_funds(origin: OriginFor<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_monthly_release()
		}

		/// Genesis token dağılımını manuel tetikle (sadece root)
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::force_genesis_distribution())]
		pub fn force_genesis_distribution(origin: OriginFor<T>) -> DispatchResult {
			T::ForceOrigin::ensure_origin(origin)?;
			Self::do_genesis_distribution()
		}
	}

	impl<T: Config> Pallet<T> {
		/// Treasury hesabının sovereign account'unu döndür
		pub fn treasury_account_id() -> T::AccountId {
			T::TreasuryPalletId::get().into_account_truncating()
		}

		/// Teşvik pot hesabını döndür
		pub fn incentive_pot_account_id() -> T::AccountId {
			T::IncentivePotId::get().into_account_truncating()
		}

		/// Hükümet pot hesabını döndür
		pub fn government_pot_account_id() -> T::AccountId {
			T::GovernmentPotId::get().into_account_truncating()
		}

		/// Genesis token dağılımını yap
		pub fn do_genesis_distribution() -> DispatchResult {
			let treasury_account = Self::treasury_account_id();
			let presale_account = T::PresaleAccount::get();
			let founder_account = T::FounderAccount::get();

			// Token'ları uygun hesaplara mint et
			let treasury_amount: BalanceOf<T> = TREASURY_ALLOCATION.try_into()
				.map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;
			let presale_amount: BalanceOf<T> = PRESALE_ALLOCATION.try_into()
				.map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;
			let founder_amount: BalanceOf<T> = FOUNDER_ALLOCATION.try_into()
				.map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

			// Treasury'ye fon aktar (Currency trait ile)
			T::Currency::deposit_creating(&treasury_account, treasury_amount);
			T::Currency::deposit_creating(&presale_account, presale_amount);
			T::Currency::deposit_creating(&founder_account, founder_amount);

			Self::deposit_event(Event::GenesisDistributionCompleted {
				treasury_amount,
				presale_amount,
				founder_amount,
			});

			Ok(())
		}

		/// Treasury'yi başlat
		pub fn do_initialize_treasury() -> DispatchResult {
			ensure!(
				TreasuryStartBlock::<T>::get().is_none(),
				Error::<T>::TreasuryAlreadyInitialized
			);

			let current_block = frame_system::Pallet::<T>::block_number();
			
			// İlk periyot için aylık miktar hesapla (4 yılda toplam hazineyi yarıya indir)
			let treasury_balance = TREASURY_ALLOCATION;
			let first_period_total = treasury_balance.checked_div(2)
				.ok_or(Error::<T>::InvalidHalvingPeriod)?;
			let monthly_amount = first_period_total.checked_div(HALVING_PERIOD_MONTHS.into())
				.ok_or(Error::<T>::InvalidHalvingPeriod)?;

			let monthly_amount_balance: BalanceOf<T> = monthly_amount.try_into()
				.map_err(|_| Error::<T>::InsufficientTreasuryBalance)?;

			// Halving bilgilerini kaydet
			let halving_data = HalvingData {
				current_period: 0,
				period_start_block: current_block,
				monthly_amount: monthly_amount_balance,
				total_released: Zero::zero(),
			};

			TreasuryStartBlock::<T>::put(current_block);
			HalvingInfo::<T>::put(halving_data);
			NextReleaseMonth::<T>::put(0);

			Self::deposit_event(Event::TreasuryInitialized {
				start_block: current_block,
				initial_monthly_amount: monthly_amount_balance,
			});

			Ok(())
		}

		/// Aylık fon salınımını gerçekleştir
		pub fn do_monthly_release() -> DispatchResult {
			ensure!(
				TreasuryStartBlock::<T>::get().is_some(),
				Error::<T>::TreasuryNotInitialized
			);

			let current_block = frame_system::Pallet::<T>::block_number();
			let start_block = TreasuryStartBlock::<T>::get().unwrap();
			let next_month = NextReleaseMonth::<T>::get();

			// Önce duplicate kontrolü yap - daha spesifik hata
			ensure!(
				!MonthlyReleases::<T>::contains_key(next_month),
				Error::<T>::MonthlyReleaseAlreadyDone
			);

			// Sonra zaman kontrolü yap
			let blocks_passed = current_block.saturating_sub(start_block);
			let months_passed = blocks_passed / BLOCKS_PER_MONTH.into();
			
			// İlk ay (next_month = 0) için en az 1 ay geçmiş olmalı
			// Sonraki aylar için de months_passed > next_month olmalı
			ensure!(
				months_passed > next_month.into(),
				Error::<T>::ReleaseTooEarly
			);

			let mut halving_data = HalvingInfo::<T>::get();

			// Halving kontrolü - yeni periyot başladı mı?
			let current_period_passed_months = months_passed.saturating_sub(
				(halving_data.current_period * HALVING_PERIOD_MONTHS).into()
			);

			if current_period_passed_months >= HALVING_PERIOD_MONTHS.into() {
				// Yeni halving periyodu başlat
				halving_data.current_period += 1;
				halving_data.monthly_amount = halving_data.monthly_amount / 2u32.into(); // Yarıya indir
				halving_data.period_start_block = current_block;

				Self::deposit_event(Event::NewHalvingPeriod {
					period: halving_data.current_period,
					new_monthly_amount: halving_data.monthly_amount,
				});
			}

			// Aylık tutarı hesapla
			let monthly_amount = halving_data.monthly_amount;
			let incentive_amount = monthly_amount * 75u32.into() / 100u32.into(); // %75
			let government_amount = monthly_amount.saturating_sub(incentive_amount); // %25

			// Treasury'den pot hesaplarına transfer et
			let treasury_account = Self::treasury_account_id();
			let incentive_pot = Self::incentive_pot_account_id();
			let government_pot = Self::government_pot_account_id();

			// Transfer işlemleri
			T::Currency::transfer(
				&treasury_account,
				&incentive_pot,
				incentive_amount,
				frame_support::traits::ExistenceRequirement::KeepAlive,
			)?;

			T::Currency::transfer(
				&treasury_account,
				&government_pot,
				government_amount,
				frame_support::traits::ExistenceRequirement::KeepAlive,
			)?;

			// Güncellenen verileri kaydet
			halving_data.total_released = halving_data.total_released.saturating_add(monthly_amount);
			HalvingInfo::<T>::put(halving_data);

			let release_info = MonthlyRelease {
				month_index: next_month,
				release_block: current_block,
				amount_released: monthly_amount,
				incentive_amount,
				government_amount,
			};

			MonthlyReleases::<T>::insert(next_month, release_info);
			NextReleaseMonth::<T>::put(next_month + 1);

			Self::deposit_event(Event::MonthlyFundsReleased {
				month_index: next_month,
				total_amount: monthly_amount,
				incentive_amount,
				government_amount,
			});

			Ok(())
		}

		/// Mevcut halving periyodu bilgilerini döndür
		pub fn get_current_halving_info() -> HalvingData<T> {
			HalvingInfo::<T>::get()
		}

		/// Teşvik pot bakiyesini döndür
		pub fn get_incentive_pot_balance() -> BalanceOf<T> {
			let pot_account = Self::incentive_pot_account_id();
			T::Currency::free_balance(&pot_account)
		}

		/// Hükümet pot bakiyesini döndür
		pub fn get_government_pot_balance() -> BalanceOf<T> {
			let pot_account = Self::government_pot_account_id();
			T::Currency::free_balance(&pot_account)
		}
	}
}