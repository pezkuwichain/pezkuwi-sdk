#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;

use sp_std::{convert::Into, vec::Vec};
use alloc::format;
use frame_support::pallet_prelude::MaybeSerializeDeserialize;
use scale_info::TypeInfo;
use frame_support::pallet_prelude::Parameter;
use serde::{Deserialize, Serialize};
use sp_runtime::{
    RuntimeDebug,
    DispatchError,
};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;
pub mod ensure; // Origin validation için

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::StaticLookup;

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_nfts::Config<ItemId = u32> + pallet_identity_kyc::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
        type WeightInfo: weights::WeightInfo;

        /// Tiki (Rol) NFT'lerının tutulduğu koleksiyonun ID'si.
        #[pallet::constant]
        type TikiCollectionId: Get<Self::CollectionId>;

        /// Bir kullanıcının sahip olabileceği maksimum Tiki (rol) sayısı için teknik üst sınır.
        #[pallet::constant]
        type MaxTikisPerUser: Get<u32>;

        /// Palet içerisinde kullanılacak Tiki enum tipi.
        type Tiki: Parameter + From<Tiki> + Into<u32> + MaxEncodedLen + TypeInfo + Copy + MaybeSerializeDeserialize + 'static;
    }

    #[derive(Serialize, Deserialize, Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum RoleAssignmentType {
        /// Otomatik olarak verilen roller (KYC sonrası Hemwelatî gibi)
        Automatic,
        /// Admin tarafından atanan roller (Wezir, Dadger gibi)
        Appointed,
        /// Topluluk tarafından seçilen roller (Parlementer gibi) - pallet-voting tarafından verilir
        Elected,
        /// Kazanılabilen roller (Axa, sınavla alınan roller)
        Earned,
    }

    #[derive(Serialize, Deserialize, Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[repr(u32)]
    pub enum Tiki {
        Hemwelatî, Parlementer, SerokiMeclise, Serok, Wezir, EndameDiwane, Dadger,
        Dozger, Hiquqnas, Noter, Xezinedar, Bacgir, GerinendeyeCavkaniye, OperatorêTorê,
        PisporêEwlehiyaSîber, GerinendeyeDaneye, Berdevk, Qeydkar, Balyoz, Navbeynkar,
        ParêzvaneÇandî, Mufetîs, KalîteKontrolker, Mela, Feqî, Perwerdekar, Rewsenbîr,
        RêveberêProjeyê, SerokêKomele, ModeratorêCivakê, Axa, Pêseng, Sêwirmend, Hekem, Mamoste,
        // Yeni eklenen ekonomik roller
        Bazargan,
        // hukumet rolleri 
        SerokWeziran, WezireDarayiye, WezireParez, WezireDad, WezireBelaw, WezireTend, WezireAva, WezireCand, 

    }

    impl Into<u32> for Tiki {
        fn into(self) -> u32 {
            self as u32
        }
    }

    /// Her kullanıcının Vatandaşlık NFT'sinin ID'sini tutar
    #[pallet::storage]
    #[pallet::getter(fn citizen_nft)]
    pub type CitizenNft<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;

    /// Her kullanıcının sahip olduğu Tiki'lerin (rollerin) listesi
    #[pallet::storage]
    #[pallet::getter(fn user_tikis)]
    pub type UserTikis<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Tiki, T::MaxTikisPerUser>, ValueQuery>;

    /// Belirli bir Tiki'nin hangi kullanıcıya ait olduğunu gösterir (benzersiz roller için)
    #[pallet::storage]
    #[pallet::getter(fn tiki_holder)]
    pub type TikiHolder<T: Config> = StorageMap<_, Blake2_128Concat, Tiki, T::AccountId, OptionQuery>;

    /// Bir sonraki NFT için kullanılacak Item ID
    #[pallet::storage]
    #[pallet::getter(fn next_item_id)]
    pub type NextItemId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// Rol zaten başka birine ait
        RoleAlreadyTaken,
        /// Belirtilen kişi bu rolün sahibi değil
        NotTheHolder,
        /// Rol atanmamış
        RoleNotAssigned,
        /// Bir kullanıcı maksimum rol sayısına ulaştı
        ExceedsMaxRolesPerUser,
        /// KYC tamamlanmamış
        KycNotCompleted,
        /// Vatandaşlık NFT'si zaten mevcut
        CitizenNftAlreadyExists,
        /// Vatandaşlık NFT'si bulunamadı
        CitizenNftNotFound,
        /// Kullanıcı zaten bu role sahip
        UserAlreadyHasRole,
        /// Yeterli Trust Puanı yok
        InsufficientTrustScore,
        /// Bu rol türü bu yöntemle verilemez
        InvalidRoleAssignmentMethod,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Yeni vatandaşlık NFT'si basıldı
        CitizenNftMinted { who: T::AccountId, nft_id: u32 },
        /// Yeni Tiki (rol) eklendi
        TikiGranted { who: T::AccountId, tiki: Tiki },
        /// Tiki (rol) kaldırıldı
        TikiRevoked { who: T::AccountId, tiki: Tiki },
        /// NFT transfer'i engellendi
        TransferBlocked { collection_id: T::CollectionId, item_id: u32, from: T::AccountId, to: T::AccountId },
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: BlockNumberFor<T>) -> Weight {
            // KYC tamamlanan yeni kullanıcıları kontrol et ve vatandaşlık NFT'si bas
            Self::check_and_mint_citizen_nfts();
            
            T::DbWeight::get().reads_writes(10, 5)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Admin tarafından belirli bir kullanıcıya Tiki (rol) verme
        #[pallet::call_index(0)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_tiki(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?;
            let dest_account = T::Lookup::lookup(dest)?;
            
            // Rolün atanabilir olup olmadığını kontrol et
            ensure!(
                Self::can_grant_role_type(&tiki, &RoleAssignmentType::Appointed),
                Error::<T>::InvalidRoleAssignmentMethod
            );
            
            Self::internal_grant_role(&dest_account, tiki)?;
            Ok(())
        }

        /// Admin tarafından belirli bir kullanıcıdan Tiki (rol) alma
        #[pallet::call_index(1)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::revoke_tiki())]
        pub fn revoke_tiki(
            origin: OriginFor<T>,
            target: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?;
            let target_account = T::Lookup::lookup(target)?;
            
            Self::internal_revoke_role(&target_account, tiki)?;
            Ok(())
        }

        /// Manual olarak vatandaşlık NFT'si basma (test/acil durum için)
        #[pallet::call_index(2)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn force_mint_citizen_nft(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?;
            let dest_account = T::Lookup::lookup(dest)?;
            
            Self::mint_citizen_nft_for_user(&dest_account)?;
            Ok(())
        }

        /// Seçim sistemi tarafından rol verme (pallet-voting'den çağrılır)
        #[pallet::call_index(3)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_elected_role(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?; // pallet-voting Root origin ile çağıracak
            let dest_account = T::Lookup::lookup(dest)?;
            
            // Rolün seçilebilir olup olmadığını kontrol et
            ensure!(
                Self::can_grant_role_type(&tiki, &RoleAssignmentType::Elected),
                Error::<T>::InvalidRoleAssignmentMethod
            );
            
            Self::internal_grant_role(&dest_account, tiki)?;
            Ok(())
        }

        /// Sınav/test sistemi tarafından rol verme
        #[pallet::call_index(4)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_earned_role(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?; // Şimdilik admin, sonra exam pallet
            let dest_account = T::Lookup::lookup(dest)?;
            
            // Rolün kazanılabilir olup olmadığını kontrol et
            ensure!(
                Self::can_grant_role_type(&tiki, &RoleAssignmentType::Earned),
                Error::<T>::InvalidRoleAssignmentMethod
            );
            
            Self::internal_grant_role(&dest_account, tiki)?;
            Ok(())
        }

        /// KYC tamamlandıktan sonra vatandaşlık başvurusu
        #[pallet::call_index(5)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn apply_for_citizenship(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Kullanıcının KYC'sinin onaylanmış olup olmadığını kontrol et
            let kyc_status = pallet_identity_kyc::Pallet::<T>::kyc_status_of(&who);
            ensure!(
                kyc_status == pallet_identity_kyc::types::KycLevel::Approved,
                Error::<T>::KycNotCompleted
            );

            // Vatandaşlık NFT'si bas
            Self::mint_citizen_nft_for_user(&who)?;

            Ok(())
        }

        /// Transfer engelleme sistemi için NFT transfer'i kontrol et
        #[pallet::call_index(6)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn check_transfer_permission(
            _origin: OriginFor<T>,
            collection_id: T::CollectionId,
            item_id: u32,
            from: T::AccountId,
            to: T::AccountId,
        ) -> DispatchResult {
            // Tiki NFT koleksiyonu ise transfer'e izin verme
            if collection_id == T::TikiCollectionId::get() {
                Self::deposit_event(Event::TransferBlocked { 
                    collection_id, 
                    item_id, 
                    from, 
                    to 
                });
                return Err(DispatchError::Other("Citizen NFTs are non-transferable"));
            }
            Ok(())
        }
    }

    // Pallet'in yardımcı fonksiyonları
    impl<T: Config> Pallet<T> {
        /// KYC tamamlanan yeni kullanıcıları kontrol eder ve vatandaşlık NFT'si basar
        fn check_and_mint_citizen_nfts() {
            // KYC pallet'teki tüm onaylanmış kullanıcıları kontrol et
            for (account, kyc_status) in pallet_identity_kyc::KycStatuses::<T>::iter() {
                // KYC onaylanmış mı kontrol et
                if kyc_status == pallet_identity_kyc::types::KycLevel::Approved {
                    // Vatandaşlık NFT'si var mı kontrol et
                    if Self::citizen_nft(&account).is_none() {
                        // NFT bas (hata durumunda log et ama devam et)
                        if let Err(_) = Self::mint_citizen_nft_for_user(&account) {
                            log::warn!("Failed to mint citizen NFT for account: {:?}", account);
                        }
                    }
                }
            }
        }


        /// Belirli bir kullanıcı için vatandaşlık NFT'si basar
        pub fn mint_citizen_nft_for_user(user: &T::AccountId) -> DispatchResult {
            // Zaten NFT'si var mı kontrol et
            ensure!(Self::citizen_nft(user).is_none(), Error::<T>::CitizenNftAlreadyExists);

            let collection_id = T::TikiCollectionId::get();
            let next_id_u32 = Self::next_item_id();
            
            // NFT'yi mint et
            pallet_nfts::Pallet::<T>::mint(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(user.clone())),
                collection_id,
                next_id_u32,
                T::Lookup::unlookup(user.clone()),
                None,
            )?;

            // NFT'yi transfer edilemez yap
            Self::lock_nft_transfer(&collection_id, &next_id_u32)?;

            // Storage'ı güncelle
            CitizenNft::<T>::insert(user, next_id_u32);
            NextItemId::<T>::put(next_id_u32 + 1);
            
            // Hemwelatî rolünü otomatik ekle
            UserTikis::<T>::mutate(user, |tikis| {
                let _ = tikis.try_push(Tiki::Hemwelatî);
            });

            // NFT metadata'sını ayarla
            Self::update_nft_metadata(user)?;

            Self::deposit_event(Event::CitizenNftMinted { who: user.clone(), nft_id: next_id_u32 });
            Ok(())
        }

        /// Internal rol verme fonksiyonu (kod tekrarını önlemek için)
        pub fn internal_grant_role(dest_account: &T::AccountId, tiki: Tiki) -> DispatchResult {
            // Vatandaşlık NFT'sinin olup olmadığını kontrol et
            ensure!(Self::citizen_nft(dest_account).is_some(), Error::<T>::CitizenNftNotFound);
            
            // Eğer bu rol benzersiz ise (tek kişiye ait olabilir), kontrol et
            if Self::is_unique_role(&tiki) {
                ensure!(Self::tiki_holder(&tiki).is_none(), Error::<T>::RoleAlreadyTaken);
            }
            
            // Kullanıcının zaten bu role sahip olup olmadığını kontrol et
            let user_tikis = Self::user_tikis(dest_account);
            ensure!(!user_tikis.contains(&tiki), Error::<T>::UserAlreadyHasRole);

            // Kullanıcının Tiki listesine ekle
            UserTikis::<T>::try_mutate(dest_account, |tikis| {
                tikis.try_push(tiki).map_err(|_| Error::<T>::ExceedsMaxRolesPerUser)
            })?;

            // Eğer benzersiz rol ise, TikiHolder'a da ekle
            if Self::is_unique_role(&tiki) {
                TikiHolder::<T>::insert(&tiki, dest_account);
            }

            // NFT metadata'sını güncelle
            Self::update_nft_metadata(dest_account)?;

            Self::deposit_event(Event::TikiGranted { who: dest_account.clone(), tiki });
            Ok(())
        }

        /// Internal rol kaldırma fonksiyonu
        pub fn internal_revoke_role(target_account: &T::AccountId, tiki: Tiki) -> DispatchResult {
            // Kullanıcının bu role sahip olup olmadığını kontrol et
            let user_tikis = Self::user_tikis(target_account);
            let _position = user_tikis.iter().position(|&r| r == tiki)
                .ok_or(Error::<T>::RoleNotAssigned)?;

            // Hemwelatî rolü kaldırılamaz
            ensure!(tiki != Tiki::Hemwelatî, Error::<T>::RoleNotAssigned);

            // Kullanıcının Tiki listesinden kaldır
            UserTikis::<T>::mutate(target_account, |tikis| {
                if let Some(pos) = tikis.iter().position(|&r| r == tiki) {
                    tikis.swap_remove(pos);
                }
            });

            // Eğer benzersiz rol ise, TikiHolder'dan da kaldır
            if Self::is_unique_role(&tiki) {
                TikiHolder::<T>::remove(&tiki);
            }

            // NFT metadata'sını güncelle
            Self::update_nft_metadata(target_account)?;
            
            Self::deposit_event(Event::TikiRevoked { who: target_account.clone(), tiki });
            Ok(())
        }

        /// NFT'yi transfer edilemez hale getirir
        fn lock_nft_transfer(collection_id: &T::CollectionId, item_id: &u32) -> DispatchResult {
            // NFT'yi lock attribute'u ile işaretle
            let _ = pallet_nfts::Pallet::<T>::set_attribute(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Root),
                *collection_id,
                Some(*item_id),
                pallet_nfts::AttributeNamespace::Pallet,
                b"locked".to_vec().try_into().map_err(|_| DispatchError::Other("Key too long"))?,
                b"true".to_vec().try_into().map_err(|_| DispatchError::Other("Value too long"))?,
            );

            Ok(())
        }

        /// NFT'nin metadata'sını kullanıcının rollerine göre günceller
        fn update_nft_metadata(user: &T::AccountId) -> DispatchResult {
            let nft_id_u32 = Self::citizen_nft(user).ok_or(Error::<T>::CitizenNftNotFound)?;
            let collection_id = T::TikiCollectionId::get();
            let user_tikis = Self::user_tikis(user);

            let total_score = Self::get_tiki_score(user);
            
            // Kısa metadata - sadece temel bilgiler
            let metadata = format!(
                r#"{{"citizen":true,"roles":{},"score":{}}}"#,
                user_tikis.len(),
                total_score
            );

            // Set metadata - hata durumunda log et ama çökme
            if let Err(_) = pallet_nfts::Pallet::<T>::set_metadata(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Root),
                collection_id,
                nft_id_u32,
                metadata.as_bytes().to_vec().try_into().map_err(|_| DispatchError::Other("Metadata too long"))?,
            ) {
                log::warn!("Failed to set metadata for NFT: {:?}", nft_id_u32);
            }

            Ok(())
        }

        /// Belirli bir rolün benzersiz (tek kişiye ait) olup olmadığını kontrol eder
        pub fn is_unique_role(tiki: &Tiki) -> bool {
            match tiki {
                Tiki::Serok | Tiki::SerokiMeclise | Tiki::Xezinedar | Tiki::Balyoz => true,
                _ => false,
            }
        }

        /// Belirli bir rolün atama türünü döndürür
        pub fn get_role_assignment_type(tiki: &Tiki) -> RoleAssignmentType {
            match tiki {
                // Otomatik roller
                Tiki::Hemwelatî => RoleAssignmentType::Automatic,
                
                // Seçilen roller
                Tiki::Parlementer | Tiki::SerokiMeclise | Tiki::Serok => RoleAssignmentType::Elected,
                
                // Kazanılan roller (pallet-referral tarafından otomatik verilir)
                Tiki::Axa | Tiki::Mamoste | Tiki::Rewsenbîr | 
                Tiki::SerokêKomele | Tiki::ModeratorêCivakê => RoleAssignmentType::Earned,
                
                // Atanan roller (varsayılan)
                _ => RoleAssignmentType::Appointed,
            }
        }

        /// Belirli bir rolün veriliş şeklini kontrol eder
        pub fn can_grant_role_type(tiki: &Tiki, assignment_type: &RoleAssignmentType) -> bool {
            let required_type = Self::get_role_assignment_type(tiki);
            match (&required_type, assignment_type) {
                // Otomatik roller sadece sistem tarafından verilebilir
                (RoleAssignmentType::Automatic, RoleAssignmentType::Automatic) => true,
                // Atanan roller admin tarafından verilebilir
                (RoleAssignmentType::Appointed, RoleAssignmentType::Appointed) => true,
                // Seçilen roller seçim sistemi tarafından verilebilir
                (RoleAssignmentType::Elected, RoleAssignmentType::Elected) => true,
                // Kazanılan roller sınav/test sistemi tarafından verilebilir
                (RoleAssignmentType::Earned, RoleAssignmentType::Earned) => true,
                _ => false,
            }
        }

        /// KYC sonrası otomatik Hemwelatî rolü verme
        pub fn auto_grant_citizenship(account: &T::AccountId) -> DispatchResult {
            // KYC kontrolü
            let kyc_status = pallet_identity_kyc::Pallet::<T>::kyc_status_of(account);
            if kyc_status == pallet_identity_kyc::types::KycLevel::Approved {
                // Vatandaşlık NFT'si yoksa bas
                if Self::citizen_nft(account).is_none() {
                    Self::mint_citizen_nft_for_user(account)?;
                }
            }
            Ok(())
        }

        /// Kullanıcının belirli bir Tiki'ye sahip olup olmadığını kontrol eder
        pub fn has_tiki(who: &T::AccountId, tiki: &Tiki) -> bool {
            Self::user_tikis(who).contains(tiki)
        }

        /// Kullanıcının vatandaş olup olmadığını kontrol eder
        pub fn is_citizen(who: &T::AccountId) -> bool {
            Self::citizen_nft(who).is_some()
        }
    }
}

/// Diğer paletlerin, bu paletten Tiki puanlarını sorgulaması için kullanılacak trait
pub trait TikiScoreProvider<AccountId> {
    fn get_tiki_score(who: &AccountId) -> u32;
}

/// Diğer paletlerin, Tiki sahipliğini sorgulaması için kullanılacak trait
pub trait TikiProvider<AccountId> {
    fn has_tiki(who: &AccountId, tiki: &Tiki) -> bool;
    fn get_user_tikis(who: &AccountId) -> Vec<Tiki>;
    fn is_citizen(who: &AccountId) -> bool;
}

/// Trait implementasyonları
impl<T: Config> TikiScoreProvider<T::AccountId> for Pallet<T> {
    fn get_tiki_score(who: &T::AccountId) -> u32 {
        let tikis = Self::user_tikis(who);
        tikis.iter().map(Self::get_bonus_for_tiki).sum()
    }
}

impl<T: Config> TikiProvider<T::AccountId> for Pallet<T> {
    fn has_tiki(who: &T::AccountId, tiki: &Tiki) -> bool {
        Self::has_tiki(who, tiki)
    }

    fn get_user_tikis(who: &T::AccountId) -> Vec<Tiki> {
        Self::user_tikis(who).into_inner()
    }

    fn is_citizen(who: &T::AccountId) -> bool {
        Self::is_citizen(who)
    }
}

// Puanlama mantığını ayrı bir impl bloğunda tutarak kodu daha düzenli hale getiriyoruz.
impl<T: Config> Pallet<T> {
    /// Belirli bir Tiki'nin Trust Puanı'na olan katkısını döndürür.
    pub fn get_bonus_for_tiki(tiki: &Tiki) -> u32 {
        match tiki {
            // Anayasa v5.0'da Belirlenen Özel Puanlar
            Tiki::Axa => 250,
            Tiki::RêveberêProjeyê => 250,
            Tiki::ModeratorêCivakê => 200,
            Tiki::SerokêKomele => 100,
            Tiki::Mela => 50,
            Tiki::Feqî => 50,

            // Hiyerarşik Devlet Puanları
            // Yargı
            Tiki::EndameDiwane => 175,
            Tiki::Dadger => 150,
            Tiki::Dozger => 120,
            Tiki::Hiquqnas => 75,
            // Yürütme
            Tiki::Serok => 200,
            Tiki::Wezir => 100,
            Tiki::SerokWeziran => 125,
            Tiki::WezireDarayiye => 100,
            Tiki::WezireParez => 100,
            Tiki::WezireDad => 100,
            Tiki::WezireBelaw => 100,
            Tiki::WezireTend => 100,
            Tiki::WezireAva => 100,
            Tiki::WezireCand => 100,

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
            Tiki::Mamoste => 70,
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
            Tiki::Bazargan => 60, // Yeni eklenen ekonomik rol

            // Temel Vatandaşlık ve Diğerleri
            Tiki::Hemwelatî => 10,
            // Yukarıdaki listede olmayan diğer tüm roller 5 puan alır.
            _ => 5,
        }
    }
}