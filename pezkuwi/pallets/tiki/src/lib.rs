#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
// Corrected import for `Into` trait for `no_std` environments
use sp_std::convert::Into;
// Corrected imports for `num_derive` and `num_traits`
use num_derive::FromPrimitive;

// Required for the `MaybeSerializeDeserialize` bound in Config::Tiki
use frame_support::pallet_prelude::MaybeSerializeDeserialize;
// Required for TypeInfo and Parameter traits
use scale_info::TypeInfo;
use frame_support::pallet_prelude::Parameter;
// `serde` derive'ları için import'u dosyanın geneline taşıyoruz.
use serde::{Deserialize, Serialize};
// Required for RuntimeDebug
use sp_runtime::RuntimeDebug;


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
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_nfts::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type WeightInfo: weights::WeightInfo;

        /// Tiki (Rol) NFT'lerinin tutulduğu koleksiyonun ID'si.
        #[pallet::constant]
        type TikiCollectionId: Get<Self::CollectionId>;

        /// Bir kullanıcının sahip olabileceği maksimum Tiki (rol) sayısı için teknik üst sınır.
        #[pallet::constant]
        type MaxTikisPerUser: Get<u32>;

        /// `Tiki` enum'ından gelen `u32`'yi `pallet-nfts`'in kullandığı `ItemId`'ye çevirebilmek için.
        type ItemId: From<u32> + IsType<<Self as pallet_nfts::Config>::ItemId>;

        /// Palet içerisinde kullanılacak Tiki enum tipi.
        type Tiki: Parameter + From<Tiki> + Into<u32> + MaxEncodedLen + TypeInfo + Copy + MaybeSerializeDeserialize + 'static;
    }

    #[derive(Serialize, Deserialize, Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy, FromPrimitive)]
    #[repr(u32)]
    pub enum Tiki {
        Hemwelatî, Parlementer, SerokiMeclise, Serok, Wezir, EndameDiwane, Dadger,
        Dozger, Hiquqnas, Noter, Xezinedar, Bacgir, GerinendeyeCavkaniye, OperatorêTorê,
        PisporêEwlehiyaSîber, GerinendeyeDaneye, Berdevk, Qeydkar, Balyoz, Navbeynkar,
        ParêzvaneÇandî, Mufetîs, KalîteKontrolker, Mela, Feqî, Perwerdekar, Rewsenbîr,
        RêveberêProjeyê, SerokêKomele, ModeratorêCivakê, Axa, Pêseng, Sêwirmend, Hekem,
    }

    impl Into<u32> for Tiki {
        fn into(self) -> u32 {
            self as u32
        }
    }

    #[pallet::storage]
    #[pallet::getter(fn tiki_holder)]
    pub type TikiHolder<T: Config> = StorageMap<_, Blake2_128Concat, Tiki, T::AccountId, OptionQuery>;

    /// Bir hesaba ait Tiki'lerin (rollerin) listesini tutan tersine arama haritası.
    /// Puan hesaplamasını optimize etmek için kullanılır.
    #[pallet::storage]
    #[pallet::getter(fn tikis_of)]
    pub type AccountToTiki<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Tiki, T::MaxTikisPerUser>, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        RoleAlreadyTaken,
        NotTheHolder,
        RoleNotAssigned,
        /// Bir kullanıcı maksimum rol sayısına ulaştı.
        ExceedsMaxRolesPerUser,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        TikiGranted { who: T::AccountId, tiki: Tiki },
        TikiRevoked { who: T::AccountId, tiki: Tiki },
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_tiki(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin.clone())?;
            let collection_id = T::TikiCollectionId::get();
            let dest_account = T::Lookup::lookup(dest.clone())?;
            ensure!(Pallet::<T>::tiki_holder(&tiki).is_none(), Error::<T>::RoleAlreadyTaken);
            
            let item_id: <T as crate::pallet::Config>::ItemId = (tiki as u32).into();

            // Önce NFT'yi mint et. Eğer bu işlem başarısız olursa, fonksiyon burada durur
            // ve kendi depolama haritalarımızda bir değişiklik yapılmaz. Bu daha güvenlidir.
            pallet_nfts::Pallet::<T>::force_mint(
                origin,
                collection_id,
                item_id.into(),
                dest,
                Default::default(),
            )?;

            // NFT mint etme başarılı olduktan sonra, kendi depolama haritalarımızı güncelleyelim.
            // tiki değişkeni 'Copy' trait'ine sahip olduğu için tekrar kullanılabilir.
            AccountToTiki::<T>::try_mutate(&dest_account, |tikis| {
                tikis.try_push(tiki).map_err(|_| Error::<T>::ExceedsMaxRolesPerUser)
            })?;
            TikiHolder::<T>::insert(&tiki, &dest_account);

            Self::deposit_event(Event::TikiGranted { who: dest_account, tiki });
            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::revoke_tiki())]
        pub fn revoke_tiki(
            origin: OriginFor<T>,
            target: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin.clone())?;
            let collection_id = T::TikiCollectionId::get();
            let target_account = T::Lookup::lookup(target.clone())?;
            let holder = Pallet::<T>::tiki_holder(&tiki).ok_or(Error::<T>::RoleNotAssigned)?;
            ensure!(holder == target_account, Error::<T>::NotTheHolder);

            let item_id: <T as crate::pallet::Config>::ItemId = (tiki as u32).into();

            // Önce NFT'yi yak.
            pallet_nfts::Pallet::<T>::do_burn(
                collection_id,
                item_id.into(),
                |_| Ok(()),
            )?;

            // NFT yakma başarılı olduktan sonra, kendi depolama haritalarımızı güncelle.
            AccountToTiki::<T>::mutate(&target_account, |tikis| {
                if let Some(pos) = tikis.iter().position(|&r| r == tiki) {
                    tikis.swap_remove(pos);
                }
            });
            TikiHolder::<T>::remove(&tiki);
            
            Self::deposit_event(Event::TikiRevoked { who: holder, tiki });
            Ok(())
        }
    }
}

/// `pallet-trust` gibi diğer paletlerin, bu paletten Tiki puanlarını sorgulaması için
/// kullanılacak olan arayüz (trait).
pub trait TikiScoreProvider<AccountId> {
    /// Belirtilen hesabın sahip olduğu Tiki'lerden gelen toplam bileşen puanını döndürür.
    fn get_tiki_score(who: &AccountId) -> u32;
}

impl<T: Config> TikiScoreProvider<T::AccountId> for Pallet<T> {
    /// Bir hesabın puanını, AccountToTiki haritası üzerinden verimli bir şekilde hesaplar.
    fn get_tiki_score(who: &T::AccountId) -> u32 {
        // Doğrudan kullanıcının sahip olduğu Tiki'lerin listesini alıyoruz.
        let tikis = AccountToTiki::<T>::get(who);
        // Bu liste üzerinden puanları topluyoruz.
        tikis.iter().map(Self::get_bonus_for_tiki).sum()
    }
}

// Puanlama mantığını ayrı bir impl bloğunda tutarak kodu daha düzenli hale getiriyoruz.
impl<T: Config> Pallet<T> {
    /// Belirli bir Tiki'nin Trust Puanı'na olan katkısını döndürür.
    pub fn get_bonus_for_tiki(tiki: &Tiki) -> u32 {
        match tiki {
            // Anayasada Belirlenen Özel Puanlar
            Tiki::Axa => 250,
            Tiki::RêveberêProjeyê => 250,
            Tiki::ModeratorêCivakê => 200,
            Tiki::SerokêKomele => 100,
            Tiki::Mela => 50,
            Tiki::Feqî => 50,

            // Önerilen Hiyerarşik Devlet Puanları
            // Yargı
            Tiki::EndameDiwane => 175,
            Tiki::Dadger => 150,
            Tiki::Dozger => 120,
            Tiki::Hiquqnas => 75,
            // Yürütme
            Tiki::Serok => 200,
            Tiki::Wezir => 125,
            // Yasama
            Tiki::SerokiMeclise => 150,
            Tiki::Parlementer => 100,
            
            // Atanmış Üst Düzey Memurlar
            Tiki::Xezinedar => 100,
            Tiki::PisporêEwlehiyaSîber => 100,
            Tiki::Mufetîs => 90,
            Tiki::Balyoz => 80,
            Tiki::Berdevk => 70,

            // Diğer Memurlar ve Uzmanlar
            Tiki::OperatorêTorê => 60,
            Tiki::Noter => 50,
            Tiki::Bacgir => 50,
            Tiki::Perwerdekar => 40,
            Tiki::Rewsenbîr => 40,
            Tiki::GerinendeyeCavkaniye => 40,
            Tiki::GerinendeyeDaneye => 40,
            Tiki::KalîteKontrolker => 30,
            Tiki::Navbeynkar => 30,
            Tiki::Hekem => 30,
            Tiki::Qeydkar => 25,
            Tiki::ParêzvaneÇandî => 25,
            Tiki::Sêwirmend => 20,

            // Temel Vatandaşlık ve Diğerleri
            Tiki::Hemwelatî => 10,
            // Yukarıdaki listede olmayan diğer tüm roller (Pêseng, Sêwirmend vb.) 5 puan alır.
            _ => 5,
        }
    }
}