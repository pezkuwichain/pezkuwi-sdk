
// === pallet_trust/src/tests.rs ===

use super::*;
use crate::mock::*;
use frame_support::{assert_ok, assert_noop};

#[test]
fn update_trust_parameters_works() {
    new_test_ext().execute_with(|| {
        let new_score = TrustScore::default();
        assert_ok!(Trust::update_trust_parameters(Origin::root(), new_score));
    });
}

#[test]
fn record_contribution_works() {
    new_test_ext().execute_with(|| {
        assert_ok!(Trust::record_contribution(Origin::signed(1), 1, ContributionType::Egitim));
    });
}

