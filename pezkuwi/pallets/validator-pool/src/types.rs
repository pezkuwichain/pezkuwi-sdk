use codec::{Decode, DecodeWithMemTracking, Encode, MaxEncodedLen};
use frame_support::{
    traits::{ConstU32},
    BoundedVec,
};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Types of validators in the pool
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound())]
pub enum ValidatorPoolCategory {
    /// Stake-based validators (high stake + trust score)
    StakeValidator {
        min_stake: u128,
        trust_threshold: u128,
    },
    /// Parliamentary validators (elected parliament members)
    ParliamentaryValidator,
    /// Merit-based validators (special Tikis + community support)
    MeritValidator {
        special_tikis: BoundedVec<u8, ConstU32<5>>, // Tiki types they hold
        community_threshold: u32, // Minimum referral count
    },
}

/// Performance metrics for a validator
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen, Default)]
#[codec(mel_bound())]
pub struct ValidatorPerformance {
    /// Total blocks produced
    pub blocks_produced: u32,
    /// Total blocks missed
    pub blocks_missed: u32,
    /// Era points earned
    pub era_points: u32,
    /// Last era when this validator was active
    pub last_active_era: u32,
    /// Reputation score (0-100)
    pub reputation_score: u8,
}

/// Current validator set for an era
#[derive(Encode, Decode, DecodeWithMemTracking, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[codec(mel_bound())]
pub struct ValidatorSet<AccountId> 
where 
    AccountId: Encode + Decode + DecodeWithMemTracking + Clone + PartialEq + Eq + MaxEncodedLen,
{
    /// Era index
    pub era_index: u32,
    /// Stake-based validators (target: 10)
    pub stake_validators: BoundedVec<AccountId, ConstU32<10>>,
    /// Parliamentary validators (target: 6)
    pub parliamentary_validators: BoundedVec<AccountId, ConstU32<6>>,
    /// Merit-based validators (target: 5)
    pub merit_validators: BoundedVec<AccountId, ConstU32<5>>,
}

impl<AccountId> ValidatorSet<AccountId> 
where 
    AccountId: Encode + Decode + DecodeWithMemTracking + Clone + PartialEq + Eq + MaxEncodedLen,
{
    /// Get all validators in the set
    pub fn all_validators(&self) -> Vec<AccountId> {
        let mut all = Vec::new();
        all.extend(self.stake_validators.iter().cloned());
        all.extend(self.parliamentary_validators.iter().cloned());
        all.extend(self.merit_validators.iter().cloned());
        all
    }
    
    /// Get total validator count
    pub fn total_count(&self) -> u32 {
        self.stake_validators.len() as u32 + 
        self.parliamentary_validators.len() as u32 + 
        self.merit_validators.len() as u32
    }
}

/// Trait for referral system integration
pub trait ReferralProvider<AccountId> {
    /// Get referral count for an account
    fn get_referral_count(who: &AccountId) -> u32;
}

/// Trait for Perwerde system integration  
pub trait PerwerdeProvider<AccountId> {
    /// Get Perwerde score for an account
    fn get_perwerde_score(who: &AccountId) -> u32;
}

/// Default implementation for tests
impl<AccountId> ReferralProvider<AccountId> for () {
    fn get_referral_count(_who: &AccountId) -> u32 {
        0
    }
}

impl<AccountId> PerwerdeProvider<AccountId> for () {
    fn get_perwerde_score(_who: &AccountId) -> u32 {
        0
    }
}