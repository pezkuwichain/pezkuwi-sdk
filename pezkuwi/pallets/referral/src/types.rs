use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

// --- GENERAL TYPES ---

/// Structure representing a simple NFT.
/// Note: The actual NFT structure will be more detailed in `pallet-tiki`.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
pub struct Tiki {
	pub id: u32,
	// metadata and other fields can be added in the future.
}

/// Raw score type to be used in scoring.
pub type RawScore = u32;


// --- EXTERNAL INTERFACES (TRAITS) ---

/// Interface for querying an account's inviter.
pub trait InviterProvider<AccountId> {
	fn get_inviter(who: &AccountId) -> Option<AccountId>;
}

/// Interface for calculating an account's referral score.
pub trait ReferralScoreProvider<AccountId> {
	type Score;
	fn get_referral_score(who: &AccountId) -> Self::Score;
}

