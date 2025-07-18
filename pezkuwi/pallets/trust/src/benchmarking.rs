
// === pallet_trust/src/benchmarking.rs ===

use super::*;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
    update_trust_parameters {
        let score = TrustScore::default();
    }: _(RawOrigin::Root, score)

    record_contribution {
        let user: T::AccountId = account("user", 0, 0);
    }: _(RawOrigin::Signed(user.clone()), user.clone(), ContributionType::Referral)

    force_penalty {
        let offender: T::AccountId = account("offender", 0, 1);
    }: _(RawOrigin::Root, offender.clone(), PenaltyReason::Manual)

    schedule_reset {
        let who: T::AccountId = account("target", 0, 2);
    }: _(RawOrigin::Root, who.clone(), b"scheduled_event".to_vec())

    recalculate_and_cache_trust {
        let user: T::AccountId = account("target", 0, 3);
    }: _(RawOrigin::Root, user.clone())
    verify {
        assert!(TrustScores::<T>::contains_key(&user));
    }
}
