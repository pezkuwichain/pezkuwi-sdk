//! pallet-staking-score için testler.

use crate::{mock::*, StakingScoreProvider, MONTH_IN_BLOCKS, UNITS};
use frame_support::assert_ok;
use pallet_staking::RewardDestination;

// Testlerde kullanacağımız sabitler
const USER_STASH: AccountId = 10;
const USER_CONTROLLER: AccountId = 10;

#[test]
fn zero_stake_should_return_zero_score() {
	ExtBuilder::default().build_and_execute(|| {
		// ExtBuilder'da 10 numaralı hesap için bir staker oluşturmadık.
		// Bu nedenle, palet 0 puan vermelidir.
		assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 0);
	});
}

#[test]
fn score_is_calculated_correctly_without_time_tracking() {
	ExtBuilder::default()
		.build_and_execute(|| {
			// 50 HEZ stake edelim. Staking::bond çağrısı ile stake işlemini başlat.
			assert_ok!(Staking::bond(
				RuntimeOrigin::signed(USER_STASH),
				50 * UNITS,
				RewardDestination::Staked
			));

			// Süre takibi yokken, puan sadece miktara göre hesaplanmalı (20 puan).
			assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 20);
		});
}

#[test]
fn start_score_tracking_works_and_enables_duration_multiplier() {
	ExtBuilder::default()
		.build_and_execute(|| {
			// --- 1. Kurulum ve Başlangıç ---
			let initial_block = 10;
			System::set_block_number(initial_block);

			// 500 HEZ stake edelim. Bu, 40 temel puan demektir.
			assert_ok!(Staking::bond(
				RuntimeOrigin::signed(USER_STASH),
				500 * UNITS,
				RewardDestination::Staked
			));

			// Eylem: Süre takibini başlat. Depolamaya `10` yazılacak.
			assert_ok!(StakingScore::start_score_tracking(RuntimeOrigin::signed(USER_STASH)));

			// Doğrulama: Başlangıç puanı doğru mu?
			assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 40, "Initial score should be 40");

			// --- 2. Dört Ay Sonrası ---
			let target_block_4m = initial_block + (4 * MONTH_IN_BLOCKS) as u64;
			let expected_duration_4m = target_block_4m - initial_block;
			// Eylem: Zamanı 4 ay ileri "yaşat".
			System::set_block_number(target_block_4m);

			let (score_4m, duration_4m) = StakingScore::get_staking_score(&USER_STASH);
			assert_eq!(duration_4m, expected_duration_4m, "Duration after 4 months is wrong");
			assert_eq!(score_4m, 56, "Score after 4 months should be 56");

			// --- 3. On Üç Ay Sonrası ---
			let target_block_13m = initial_block + (13 * MONTH_IN_BLOCKS) as u64;
			let expected_duration_13m = target_block_13m - initial_block;
			// Eylem: Zamanı başlangıçtan 13 ay sonrasına "yaşat".
			System::set_block_number(target_block_13m);

			let (score_13m, duration_13m) = StakingScore::get_staking_score(&USER_STASH);
			assert_eq!(duration_13m, expected_duration_13m, "Duration after 13 months is wrong");
			assert_eq!(score_13m, 80, "Score after 13 months should be 80");
		});
}

#[test]
fn get_staking_score_works_without_explicit_tracking() {
    ExtBuilder::default().build_and_execute(|| {
		// 751 HEZ stake edelim. Bu, 50 temel puan demektir.
		assert_ok!(Staking::bond(
			RuntimeOrigin::signed(USER_STASH),
			751 * UNITS,
			RewardDestination::Staked
		));

        // Puanın 50 olmasını bekliyoruz.
        assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 50);

        // Zamanı ne kadar ileri alırsak alalım, `start_score_tracking` çağrılmadığı
        // için puan değişmemeli.
        System::set_block_number(1_000_000_000);
        assert_eq!(StakingScore::get_staking_score(&USER_STASH).0, 50);
    });
}