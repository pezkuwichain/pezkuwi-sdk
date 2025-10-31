use super::*;
use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn wrap_works() {
    new_test_ext().execute_with(|| {
        let user = 1;
        let amount = 1000;

        assert_eq!(Balances::free_balance(&user), 10000);
        assert_eq!(Assets::balance(0, &user), 0);

        assert_ok!(TokenWrapper::wrap(RuntimeOrigin::signed(user), amount));

        assert_eq!(Balances::free_balance(&user), 10000 - amount);
        assert_eq!(Assets::balance(0, &user), amount);
        assert_eq!(TokenWrapper::total_locked(), amount);
    });
}

#[test]
fn unwrap_works() {
    new_test_ext().execute_with(|| {
        let user = 1;
        let amount = 1000;

        assert_ok!(TokenWrapper::wrap(RuntimeOrigin::signed(user), amount));
        let native_balance = Balances::free_balance(&user);

        assert_ok!(TokenWrapper::unwrap(RuntimeOrigin::signed(user), amount));

        assert_eq!(Balances::free_balance(&user), native_balance + amount);
        assert_eq!(Assets::balance(0, &user), 0);
        assert_eq!(TokenWrapper::total_locked(), 0);
    });
}

#[test]
fn wrap_fails_insufficient_balance() {
    new_test_ext().execute_with(|| {
        let user = 1;
        let amount = 20000;

        assert_noop!(
            TokenWrapper::wrap(RuntimeOrigin::signed(user), amount),
            Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn unwrap_fails_insufficient_wrapped_balance() {
    new_test_ext().execute_with(|| {
        let user = 1;
        let amount = 1000;

        assert_noop!(
            TokenWrapper::unwrap(RuntimeOrigin::signed(user), amount),
            Error::<Test>::InsufficientWrappedBalance
        );
    });
}