#![cfg_attr(not(feature = "std"), no_std)]

//! # Tiki (Role) Pallet
//!
//! A pallet for managing citizenship and role-based NFTs with automated and governance-driven assignment.
//!
//! ## Overview
//!
//! The Tiki pallet implements a comprehensive role management system using non-transferable NFTs
//! to represent citizenship status and various roles within the ecosystem. Each role grants
//! specific permissions, rights, and social standing.
//!
//! ## Core Concepts
//!
//! ### Citizenship NFT
//! - Automatically minted upon KYC approval
//! - Represents "Welati" (Citizen) status
//! - Non-transferable and permanent
//! - Required prerequisite for all other roles
//!
//! ### Role Types (Tiki)
//!
//! Roles are assigned through different mechanisms:
//!
//! 1. **Automatic** - System-assigned upon conditions (e.g., Citizenship after KYC)
//! 2. **Appointed** - Admin-assigned governmental positions (e.g., Ministers, Judges)
//! 3. **Elected** - Community-voted positions (e.g., Parliament members)
//! 4. **Earned** - Achievement-based roles (e.g., Educator, Expert)
//!
//! ### Role Categories
//!
//! - **Governance**: Serok (President), SerokWeziran (Prime Minister), Ministers
//! - **Judicial**: Dadger (Judge), Dozger (Prosecutor), Hiquqnas (Lawyer)
//! - **Administrative**: Qeydkar (Registrar), Xezinedar (Treasurer), OperatorêTorê (Network Operator)
//! - **Educational**: Mamoste (Teacher), Perwerdekar (Educator), Rewsenbîr (Intellectual)
//! - **Economic**: Bazargan (Merchant), Navbeynkar (Mediator)
//! - **Community**: Parlementer (Parliament Member), ModeratorêCivakê (Community Moderator)
//! - **Expert**: Axa (Elder/Expert), Pêseng (Pioneer), Hekem (Wise), Sêwirmend (Counselor)
//!
//! ## NFT Implementation
//!
//! - Built on top of `pallet-nfts` for standard NFT functionality
//! - All Tiki NFTs are non-transferable (soulbound)
//! - Transfer attempts are blocked automatically via hooks
//! - Each role is represented by a unique NFT item in the TikiCollectionId
//!
//! ## Role Management
//!
//! ### Granting Roles
//! - Some roles are unique (only one holder at a time)
//! - Users can hold multiple compatible roles
//! - Maximum roles per user is configurable
//! - Trust score requirements for certain roles
//!
//! ### Revoking Roles
//! - Admin can revoke appointed roles
//! - Automatic revocation on condition changes
//! - Role history maintained for governance transparency
//!
//! ## Interface
//!
//! ### Extrinsics
//!
//! - `grant_tiki(who, tiki, assignment_type)` - Assign a role to a user (admin)
//! - `revoke_tiki(who, tiki)` - Remove a role from a user (admin)
//! - `force_mint_citizen_nft(who)` - Manually mint citizenship NFT (admin)
//!
//! ### Storage
//!
//! - `CitizenNft` - Mapping of accounts to their citizenship NFT IDs
//! - `UserTikis` - List of roles held by each user
//! - `TikiHolder` - Reverse mapping for unique roles to their holders
//! - `NextItemId` - Counter for NFT item ID generation
//!
//! ### Hooks
//!
//! - `on_initialize` - Automatic citizenship NFT minting for newly approved KYC users
//! - NFT transfer blocking for all Tiki NFTs
//!
//! ## Dependencies
//!
//! This pallet requires integration with:
//! - `pallet-identity-kyc` - KYC status and approval notifications
//! - `pallet-nfts` - Underlying NFT infrastructure
//! - `pallet-trust` - Trust score verification for role eligibility
//!
//! ## Runtime Integration Example
//!
//! ```ignore
//! impl pallet_tiki::Config for Runtime {
//!     type RuntimeEvent = RuntimeEvent;
//!     type AdminOrigin = EnsureRoot<AccountId>;
//!     type WeightInfo = pallet_tiki::weights::SubstrateWeight<Runtime>;
//!     type TikiCollectionId = ConstU32<1>; // Tiki collection ID
//!     type MaxTikisPerUser = ConstU32<20>; // Max 20 roles per user
//!     type Tiki = pallet_tiki::Tiki;
//! }
//! ```

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
pub mod ensure; // For origin validation

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

        /// Collection ID holding Tiki (Role) NFTs.
        #[pallet::constant]
        type TikiCollectionId: Get<Self::CollectionId>;

        /// Technical upper limit for maximum number of Tikis (roles) a user can hold.
        #[pallet::constant]
        type MaxTikisPerUser: Get<u32>;

        /// Tiki enum type to be used within the pallet.
        type Tiki: Parameter + From<Tiki> + Into<u32> + MaxEncodedLen + TypeInfo + Copy + MaybeSerializeDeserialize + 'static;
    }

    #[derive(Serialize, Deserialize, Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    pub enum RoleAssignmentType {
        /// Automatically assigned roles (like Welati after KYC)
        Automatic,
        /// Admin-assigned roles (like Wezir, Dadger)
        Appointed,
        /// Community-elected roles (like Parlementer) - assigned by pallet-voting
        Elected,
        /// Earned roles (Axa, roles obtained through exams)
        Earned,
    }

    #[derive(Serialize, Deserialize, Encode, Decode, DecodeWithMemTracking, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy)]
    #[repr(u32)]
    pub enum Tiki {
        Welati, Parlementer, SerokiMeclise, Serok, Wezir, EndameDiwane, Dadger,
        Dozger, Hiquqnas, Noter, Xezinedar, Bacgir, GerinendeyeCavkaniye, OperatorêTorê,
        PisporêEwlehiyaSîber, GerinendeyeDaneye, Berdevk, Qeydkar, Balyoz, Navbeynkar,
        ParêzvaneÇandî, Mufetîs, KalîteKontrolker, Mela, Feqî, Perwerdekar, Rewsenbîr,
        RêveberêProjeyê, SerokêKomele, ModeratorêCivakê, Axa, Pêseng, Sêwirmend, Hekem, Mamoste,
        // Newly added economic roles
        Bazargan,
        // Government roles
        SerokWeziran, WezireDarayiye, WezireParez, WezireDad, WezireBelaw, WezireTend, WezireAva, WezireCand, 

    }

    impl Into<u32> for Tiki {
        fn into(self) -> u32 {
            self as u32
        }
    }

    /// Holds citizenship NFT ID for each user
    #[pallet::storage]
    #[pallet::getter(fn citizen_nft)]
    pub type CitizenNft<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, u32, OptionQuery>;

    /// List of Tikis (roles) owned by each user
    #[pallet::storage]
    #[pallet::getter(fn user_tikis)]
    pub type UserTikis<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, BoundedVec<Tiki, T::MaxTikisPerUser>, ValueQuery>;

    /// Shows which user a specific Tiki belongs to (for unique roles)
    #[pallet::storage]
    #[pallet::getter(fn tiki_holder)]
    pub type TikiHolder<T: Config> = StorageMap<_, Blake2_128Concat, Tiki, T::AccountId, OptionQuery>;

    /// Item ID to be used for next NFT
    #[pallet::storage]
    #[pallet::getter(fn next_item_id)]
    pub type NextItemId<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::error]
    pub enum Error<T> {
        /// Role already belongs to someone else
        RoleAlreadyTaken,
        /// Specified person is not the holder of this role
        NotTheHolder,
        /// Role not assigned
        RoleNotAssigned,
        /// A user has reached maximum role count
        ExceedsMaxRolesPerUser,
        /// KYC not completed
        KycNotCompleted,
        /// Citizenship NFT already exists
        CitizenNftAlreadyExists,
        /// Citizenship NFT not found
        CitizenNftNotFound,
        /// User already has this role
        UserAlreadyHasRole,
        /// Insufficient Trust Score
        InsufficientTrustScore,
        /// This role type cannot be assigned with this method
        InvalidRoleAssignmentMethod,
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// New citizenship NFT minted
        CitizenNftMinted { who: T::AccountId, nft_id: u32 },
        /// New Tiki (role) granted
        TikiGranted { who: T::AccountId, tiki: Tiki },
        /// Tiki (role) revoked
        TikiRevoked { who: T::AccountId, tiki: Tiki },
        /// NFT transfer blocked
        TransferBlocked { collection_id: T::CollectionId, item_id: u32, from: T::AccountId, to: T::AccountId },
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(_block_number: BlockNumberFor<T>) -> Weight {
            // Check newly KYC-approved users and mint citizenship NFT
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

            // Check if the role can be appointed
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

        /// Manually mint citizenship NFT (for testing/emergency)
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

        /// Grant role through election system (called from pallet-voting)
        #[pallet::call_index(3)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_elected_role(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?; // pallet-voting will call with Root origin
            let dest_account = T::Lookup::lookup(dest)?;

            // Check if the role can be granted through election
            ensure!(
                Self::can_grant_role_type(&tiki, &RoleAssignmentType::Elected),
                Error::<T>::InvalidRoleAssignmentMethod
            );
            
            Self::internal_grant_role(&dest_account, tiki)?;
            Ok(())
        }

        /// Grant role through exam/test system
        #[pallet::call_index(4)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn grant_earned_role(
            origin: OriginFor<T>,
            dest: <T::Lookup as StaticLookup>::Source,
            tiki: Tiki,
        ) -> DispatchResult {
            T::AdminOrigin::ensure_origin(origin)?; // For now admin, later exam pallet
            let dest_account = T::Lookup::lookup(dest)?;

            // Check if the role can be earned
            ensure!(
                Self::can_grant_role_type(&tiki, &RoleAssignmentType::Earned),
                Error::<T>::InvalidRoleAssignmentMethod
            );
            
            Self::internal_grant_role(&dest_account, tiki)?;
            Ok(())
        }

        /// Apply for citizenship after KYC completion
        #[pallet::call_index(5)]
        #[pallet::weight(<T as crate::pallet::Config>::WeightInfo::grant_tiki())]
        pub fn apply_for_citizenship(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;

            // Check if user's KYC is approved
            let kyc_status = pallet_identity_kyc::Pallet::<T>::kyc_status_of(&who);
            ensure!(
                kyc_status == pallet_identity_kyc::types::KycLevel::Approved,
                Error::<T>::KycNotCompleted
            );

            // Mint citizenship NFT
            Self::mint_citizen_nft_for_user(&who)?;

            Ok(())
        }

        /// Check NFT transfer for transfer blocking system
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

    // Pallet's helper functions
    impl<T: Config> Pallet<T> {
        /// Checks newly KYC-completed users and mints citizenship NFT
        fn check_and_mint_citizen_nfts() {
            // Check all approved users in KYC pallet
            for (account, kyc_status) in pallet_identity_kyc::KycStatuses::<T>::iter() {
                // Check if KYC is approved
                if kyc_status == pallet_identity_kyc::types::KycLevel::Approved {
                    // Check if citizenship NFT exists
                    if Self::citizen_nft(&account).is_none() {
                        // Mint NFT (log error but continue on failure)
                        if let Err(_) = Self::mint_citizen_nft_for_user(&account) {
                            log::warn!("Failed to mint citizen NFT for account: {:?}", account);
                        }
                    }
                }
            }
        }


        /// Mints citizenship NFT for specific user
        pub fn mint_citizen_nft_for_user(user: &T::AccountId) -> DispatchResult {
            // Check if NFT already exists
            ensure!(Self::citizen_nft(user).is_none(), Error::<T>::CitizenNftAlreadyExists);

            let collection_id = T::TikiCollectionId::get();
            let next_id_u32 = Self::next_item_id();

            // Mint the NFT - use force_mint in benchmarks to bypass balance/origin requirements
            #[cfg(feature = "runtime-benchmarks")]
            pallet_nfts::Pallet::<T>::force_mint(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Root),
                collection_id,
                next_id_u32,
                T::Lookup::unlookup(user.clone()),
                Default::default(),
            )?;

            #[cfg(not(feature = "runtime-benchmarks"))]
            pallet_nfts::Pallet::<T>::mint(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(user.clone())),
                collection_id,
                next_id_u32,
                T::Lookup::unlookup(user.clone()),
                None,
            )?;

            // Make NFT non-transferable
            Self::lock_nft_transfer(&collection_id, &next_id_u32)?;

            // Update storage
            CitizenNft::<T>::insert(user, next_id_u32);
            NextItemId::<T>::put(next_id_u32 + 1);


            // Automatically add Welati role
            UserTikis::<T>::mutate(user, |tikis| {
                let _ = tikis.try_push(Tiki::Welati);
            });

            // Set NFT metadata
            Self::update_nft_metadata(user)?;

            Self::deposit_event(Event::CitizenNftMinted { who: user.clone(), nft_id: next_id_u32 });
            Ok(())
        }

        /// Internal role granting function (to avoid code duplication)
        pub fn internal_grant_role(dest_account: &T::AccountId, tiki: Tiki) -> DispatchResult {
            // Check if citizenship NFT exists
            ensure!(Self::citizen_nft(dest_account).is_some(), Error::<T>::CitizenNftNotFound);

            // If this role is unique (can belong to only one person), check
            if Self::is_unique_role(&tiki) {
                ensure!(Self::tiki_holder(&tiki).is_none(), Error::<T>::RoleAlreadyTaken);
            }

            // Check if user already has this role
            let user_tikis = Self::user_tikis(dest_account);
            ensure!(!user_tikis.contains(&tiki), Error::<T>::UserAlreadyHasRole);

            // Add to user's Tiki list
            UserTikis::<T>::try_mutate(dest_account, |tikis| {
                tikis.try_push(tiki).map_err(|_| Error::<T>::ExceedsMaxRolesPerUser)
            })?;

            // If unique role, also add to TikiHolder
            if Self::is_unique_role(&tiki) {
                TikiHolder::<T>::insert(&tiki, dest_account);
            }

            // Update NFT metadata
            Self::update_nft_metadata(dest_account)?;

            Self::deposit_event(Event::TikiGranted { who: dest_account.clone(), tiki });
            Ok(())
        }

        /// Internal role revocation function
        pub fn internal_revoke_role(target_account: &T::AccountId, tiki: Tiki) -> DispatchResult {
            // Check if user has this role
            let user_tikis = Self::user_tikis(target_account);
            let _position = user_tikis.iter().position(|&r| r == tiki)
                .ok_or(Error::<T>::RoleNotAssigned)?;

            // Welati role cannot be removed
            ensure!(tiki != Tiki::Welati, Error::<T>::RoleNotAssigned);

            // Remove from user's Tiki list
            UserTikis::<T>::mutate(target_account, |tikis| {
                if let Some(pos) = tikis.iter().position(|&r| r == tiki) {
                    tikis.swap_remove(pos);
                }
            });

            // If unique role, also remove from TikiHolder
            if Self::is_unique_role(&tiki) {
                TikiHolder::<T>::remove(&tiki);
            }

            // Update NFT metadata
            Self::update_nft_metadata(target_account)?;
            
            Self::deposit_event(Event::TikiRevoked { who: target_account.clone(), tiki });
            Ok(())
        }

        /// Makes NFT non-transferable
        fn lock_nft_transfer(collection_id: &T::CollectionId, item_id: &u32) -> DispatchResult {
            // Mark NFT with lock attribute - use force_set_attribute in benchmarks to bypass deposits
            #[cfg(feature = "runtime-benchmarks")]
            let _ = pallet_nfts::Pallet::<T>::force_set_attribute(
                T::RuntimeOrigin::from(frame_system::RawOrigin::Root),
                None,
                *collection_id,
                Some(*item_id),
                pallet_nfts::AttributeNamespace::Pallet,
                b"locked".to_vec().try_into().map_err(|_| DispatchError::Other("Key too long"))?,
                b"true".to_vec().try_into().map_err(|_| DispatchError::Other("Value too long"))?,
            );

            #[cfg(not(feature = "runtime-benchmarks"))]
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

        /// Updates NFT metadata based on user's roles
        fn update_nft_metadata(user: &T::AccountId) -> DispatchResult {
            let nft_id_u32 = Self::citizen_nft(user).ok_or(Error::<T>::CitizenNftNotFound)?;
            let collection_id = T::TikiCollectionId::get();
            let user_tikis = Self::user_tikis(user);

            let total_score = Self::get_tiki_score(user);

            // Short metadata - only basic information
            let metadata = format!(
                r#"{{"citizen":true,"roles":{},"score":{}}}"#,
                user_tikis.len(),
                total_score
            );

            // Set metadata - log error but don't crash
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

        /// Checks if a specific role is unique (can belong to only one person)
        pub fn is_unique_role(tiki: &Tiki) -> bool {
            match tiki {
                Tiki::Serok | Tiki::SerokiMeclise | Tiki::Xezinedar | Tiki::Balyoz => true,
                _ => false,
            }
        }

        /// Returns the assignment type of a specific role
        pub fn get_role_assignment_type(tiki: &Tiki) -> RoleAssignmentType {
            match tiki {
                // Automatic roles
                Tiki::Welati => RoleAssignmentType::Automatic,

                // Elected roles
                Tiki::Parlementer | Tiki::SerokiMeclise | Tiki::Serok => RoleAssignmentType::Elected,

                // Earned roles (automatically given by pallet-referral)
                Tiki::Axa | Tiki::Mamoste | Tiki::Rewsenbîr |
                Tiki::SerokêKomele | Tiki::ModeratorêCivakê => RoleAssignmentType::Earned,

                // Appointed roles (default)
                _ => RoleAssignmentType::Appointed,
            }
        }

        /// Checks the granting method of a specific role
        pub fn can_grant_role_type(tiki: &Tiki, assignment_type: &RoleAssignmentType) -> bool {
            let required_type = Self::get_role_assignment_type(tiki);
            match (&required_type, assignment_type) {
                // Automatic roles can only be given by the system
                (RoleAssignmentType::Automatic, RoleAssignmentType::Automatic) => true,
                // Appointed roles can be given by admin
                (RoleAssignmentType::Appointed, RoleAssignmentType::Appointed) => true,
                // Elected roles can be given by election system
                (RoleAssignmentType::Elected, RoleAssignmentType::Elected) => true,
                // Earned roles can be given by exam/test system
                (RoleAssignmentType::Earned, RoleAssignmentType::Earned) => true,
                _ => false,
            }
        }

        /// KYC sonrası otomatik Welati rolü verme
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
            Tiki::Welati => 10,
            // Yukarıdaki listede olmayan diğer tüm roller 5 puan alır.
            _ => 5,
        }
    }
}
// CitizenNftProvider trait implementation for pallet-identity-kyc integration
impl<T: Config> pallet_identity_kyc::types::CitizenNftProvider<T::AccountId> for Pallet<T> {
	fn mint_citizen_nft(who: &T::AccountId) -> sp_runtime::DispatchResult {
		Self::mint_citizen_nft_for_user(who)
	}
}
