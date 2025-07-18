//==============================================================================
//=== KK/pallets/trust/api/src/lib.rs  =========
//==============================================================================
#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::prelude::*;
// AccountIdCore, TrustScore, RawScore pezkuwi_primitives::types altından geliyor olmalı
use pezkuwi_primitives::types::{AccountIdCore, TrustScore, RawScore};
use parity_scale_codec::{Codec, Decode, MaxEncodedLen}; // MaxEncodedLen eklendi
use scale_info::TypeInfo;
use sp_runtime::traits::AtLeast32BitUnsigned;

// Bu trait, pallet_trust/src/lib.rs içindeki `impl TrustApiTrait for Pallet<T>`
// implementasyonu ile uyumlu olmalı.
pub trait TrustApi<AccountId, TrustScorePrimitive, RawScorePrimitive>
where
    AccountId: Codec + Decode + Ord + MaxEncodedLen + TypeInfo, // Gerekli bound'lar eklendi
    TrustScorePrimitive: Codec + Decode + Default + TypeInfo + MaxEncodedLen, // Gerekli bound'lar eklendi
    RawScorePrimitive: AtLeast32BitUnsigned + Copy + Default + MaxEncodedLen + TypeInfo + Codec + Decode, // Gerekli bound'lar eklendi
{
    fn get_final_trust_score(who: &AccountId) -> Option<TrustScorePrimitive>;
    fn get_raw_egitim_score(who: &AccountId) -> RawScorePrimitive;
    fn get_raw_staking_score(who: &AccountId) -> RawScorePrimitive;
    fn get_raw_referral_score(who: &AccountId) -> RawScorePrimitive;
    fn get_raw_tiki_score(who: &AccountId) -> RawScorePrimitive;
    fn get_raw_activity_score(who: &AccountId) -> RawScorePrimitive;
}

sp_api::decl_runtime_apis! {
    #[api_version(1)]
    pub trait TrustRuntimeApi { // AccountIdCore, TrustScore, RawScore tiplerini kullanacak şekilde güncellendi
        fn get_final_trust_score(who: AccountIdCore) -> Option<TrustScore>; // & kaldırıldı, sp_api genellikle sahip olunan tiplerle çalışır
        fn get_raw_egitim_score(who: AccountIdCore) -> RawScore;
        fn get_raw_staking_score(who: AccountIdCore) -> RawScore;
        fn get_raw_referral_score(who: AccountIdCore) -> RawScore;
        fn get_raw_tiki_score(who: AccountIdCore) -> RawScore;
        fn get_raw_activity_score(who: AccountIdCore) -> RawScore;
        // version_check() kaldırıldı, sp_api bunu otomatik olarak yönetir.
    }
}