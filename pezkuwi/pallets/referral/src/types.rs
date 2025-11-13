use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

// --- GENEL TİPLER ---

/// Basit bir NFT'yi temsil eden yapı.
/// Not: Gerçek NFT yapısı `pallet-tiki`'de daha detaylı olacaktır.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct Tiki {
	pub id: u32,
	// metadata vb. alanlar ileride eklenebilir.
}

/// Puanlamada kullanılacak ham skor tipi.
pub type RawScore = u32;


// --- DIŞ DÜNYA İÇİN ARAYÜZLER (TRAITS) ---

/// Bir hesabın davet edenini (inviter) sorgulamak için arayüz.
pub trait InviterProvider<AccountId> {
	fn get_inviter(who: &AccountId) -> Option<AccountId>;
}

/// Bir hesabın referans puanını hesaplayan arayüz.
pub trait ReferralScoreProvider<AccountId> {
	type Score;
	fn get_referral_score(who: &AccountId) -> Self::Score;
}

