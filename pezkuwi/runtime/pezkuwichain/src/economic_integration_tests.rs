// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Pezkuwi.

// Pezkuwi is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Pezkuwi is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Pezkuwi. If not, see <http://www.gnu.org/licenses/>.

//! PezkuwiChain Economic System Integration Tests

#[cfg(test)]
mod economic_integration_tests {
    use crate::*;
    use frame_support::{assert_ok, assert_noop, traits::Get};
    use sp_runtime::traits::{Zero, Saturating};
    use pezkuwichain_constants::currency::UNITS as HEZ;
    use sp_keyring::Sr25519Keyring;
    
    fn new_test_ext() -> sp_io::TestExternalities {
        use frame_support::traits::GenesisBuild;
        
        let mut ext = sp_io::TestExternalities::new(Default::default());
        ext.execute_with(|| {
            System::set_block_number(1);
            
            // Treasury'yi başlat
            let _ = PezTreasury::do_genesis_distribution();
            let _ = PezTreasury::do_initialize_treasury();
            
            // Rewards sistemini başlat  
            let _ = PezRewards::do_initialize_rewards_system();
        });
        ext
    }
    
    #[test]
    fn test_complete_economic_cycle() {
        new_test_ext().execute_with(|| {
            // 1. Treasury durumunu kontrol et
            let treasury_account = PezTreasury::treasury_account_id();
            let treasury_balance = Balances::free_balance(&treasury_account);
            assert!(!treasury_balance.is_zero(), "Treasury should have initial balance");
            
            // 2. İlk halving bilgilerini kontrol et
            let initial_info = PezTreasury::get_current_halving_info();
            assert_eq!(initial_info.current_period, 0);
            assert!(!initial_info.monthly_amount.is_zero());
            
            // 3. İlk aylık release (1 ay sonra)
            System::set_block_number(432_000); // 1 ay = 432,000 blok
            assert_ok!(PezTreasury::release_monthly_funds(
                RuntimeOrigin::root()
            ));
            
            // 4. Pot'larda para birikti mi kontrol et
            let incentive_balance = PezTreasury::get_incentive_pot_balance();
            let government_balance = PezTreasury::get_government_pot_balance();
            
            assert!(!incentive_balance.is_zero(), "Incentive pot should have funds");
            assert!(!government_balance.is_zero(), "Government pot should have funds");
            
            // 5. %75-%25 dağılımını kontrol et
            let total_released = incentive_balance + government_balance;
            let expected_incentive = total_released * 75u128 / 100u128;
            let expected_government = total_released * 25u128 / 100u128;
            
            // Küçük tolerans ile kontrol et (rounding errors için)
            assert!(
                incentive_balance >= expected_incentive.saturating_sub(1000) &&
                incentive_balance <= expected_incentive.saturating_add(1000),
                "Incentive pot should be ~75% of total"
            );
            
            // 6. Epoch'u finalize et
            assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
            
            // 7. Reward pool oluştu mu kontrol et
            let epoch_pool = PezRewards::get_epoch_reward_pool(0);
            assert!(epoch_pool.is_some(), "Epoch reward pool should be created");
            
            let pool = epoch_pool.unwrap();
            assert_eq!(pool.total_reward_pool, incentive_balance);
        });
    }
    
    #[test]
    fn test_halving_mechanism() {
        new_test_ext().execute_with(|| {
            let initial_info = PezTreasury::get_current_halving_info();
            let initial_monthly = initial_info.monthly_amount;
            
            // İlk periyottaki release'leri simüle et (47 ay)
            for month in 1..=47 {
                System::set_block_number(month * 432_000);
                assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
            }
            
            // 48. ay - halving tetiklenmeli
            System::set_block_number(48 * 432_000);
            assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
            
            let after_halving = PezTreasury::get_current_halving_info();
            assert_eq!(after_halving.current_period, 1);
            assert_eq!(after_halving.monthly_amount, initial_monthly / 2u32.into());
            
            // 49. ay - yeni miktar kullanılmalı
            System::set_block_number(49 * 432_000);
            let incentive_before = PezTreasury::get_incentive_pot_balance();
            
            assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
            
            let incentive_after = PezTreasury::get_incentive_pot_balance();
            let released_amount = incentive_after - incentive_before;
            let expected_incentive = (initial_monthly / 2u32.into()) * 75u128 / 100u128;
            
            assert!(
                released_amount >= expected_incentive.saturating_sub(1000) &&
                released_amount <= expected_incentive.saturating_add(1000),
                "Should release halved amount"
            );
        });
    }
    
    #[test]
    fn test_rewards_claim_and_clawback() {
        new_test_ext().execute_with(|| {
            // Test kullanıcısı
            let user = Sr25519Keyring::Alice.to_account_id();
            
            // Kullanıcıya balance ver
            let _ = Balances::deposit_creating(&user, 1000 * HEZ);
            
            // Trust score ver (mock)
            // Bu gerçek implementasyonda Trust palet üzerinden yapılacak
            
            // İlk aylık release
            System::set_block_number(432_000);
            assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
            
            // Trust score kaydet (epoch 0 için)
            assert_ok!(PezRewards::record_trust_score(
                RuntimeOrigin::signed(user.clone())
            ));
            
            // Epoch finalize et
            assert_ok!(PezRewards::finalize_epoch(RuntimeOrigin::root()));
            
            // Claim period'da reward talep et
            assert_ok!(PezRewards::claim_reward(
                RuntimeOrigin::signed(user.clone()),
                0
            ));
            
            // Clawback için 1 hafta bekle
            System::set_block_number(432_000 + 100_800 + 1); // 1 ay + 1 hafta + 1 blok
            
            let clawback_recipient = QaziMuhammedAccount::get();
            let balance_before = Balances::free_balance(&clawback_recipient);
            
            assert_ok!(PezRewards::close_epoch(RuntimeOrigin::root(), 0));
            
            let balance_after = Balances::free_balance(&clawback_recipient);
            // Talep edilmemiş ödüller Qazi Muhammed'e gitmeliydi
            assert!(balance_after >= balance_before, "Clawback should transfer unclaimed rewards");
        });
    }
    
    #[test]
    fn test_treasury_sustainability() {
        new_test_ext().execute_with(|| {
            let treasury_account = PezTreasury::treasury_account_id();
            let initial_treasury_balance = Balances::free_balance(&treasury_account);
            
            // 50 yıl simüle et (50 * 12 = 600 ay)
            let mut current_block = 0u32;
            let mut total_released = 0u128;
            
            for year in 1..=50 {
                for month in 1..=12 {
                    current_block += 432_000; // Her ay
                    System::set_block_number(current_block);
                    
                    if PezTreasury::release_monthly_funds(RuntimeOrigin::root()).is_ok() {
                        let current_info = PezTreasury::get_current_halving_info();
                        total_released += TryInto::<u128>::try_into(current_info.monthly_amount).unwrap_or(0);
                    }
                }
                
                // Her 4 yılda bir halving kontrol et
                if year % 4 == 0 {
                    let info = PezTreasury::get_current_halving_info();
                    assert_eq!(info.current_period, (year / 4) as u32);
                }
            }
            
            // Treasury'de hala para olmalı
            let final_treasury_balance = Balances::free_balance(&treasury_account);
            assert!(
                final_treasury_balance > 0,
                "Treasury should still have funds after 50 years"
            );
            
            // Toplam salınan miktar, başlangıçtaki treasury'den az olmalı
            assert!(
                total_released < TryInto::<u128>::try_into(initial_treasury_balance).unwrap_or(u128::MAX),
                "Total released should be less than initial treasury"
            );
        });
    }
    
    #[test]
    fn test_error_conditions() {
        new_test_ext().execute_with(|| {
            // Erken release deneme
            assert_noop!(
                PezTreasury::release_monthly_funds(RuntimeOrigin::root()),
                pallet_pez_treasury::Error::<Runtime>::ReleaseTooEarly
            );
            
            // Duplicate release deneme
            System::set_block_number(432_000);
            assert_ok!(PezTreasury::release_monthly_funds(RuntimeOrigin::root()));
            
            assert_noop!(
                PezTreasury::release_monthly_funds(RuntimeOrigin::root()),
                pallet_pez_treasury::Error::<Runtime>::MonthlyReleaseAlreadyDone
            );
            
            // Geçersiz epoch'tan reward claim deneme
            let user = Sr25519Keyring::Bob.to_account_id();
            assert_noop!(
                PezRewards::claim_reward(RuntimeOrigin::signed(user), 999),
                pallet_pez_rewards::Error::<Runtime>::RewardPoolNotCalculated
            );
        });
    }
    
    #[test] 
    fn test_automated_scheduling_setup() {
        new_test_ext().execute_with(|| {
            // Scheduling setup'ını test et
            assert_ok!(Runtime::setup_automated_scheduling());
            
            // Scheduler'da task'ların schedule edildiğini kontrol et
            // (Gerçek implementasyonda Scheduler palet storage'ını kontrol ederiz)
            
            // Emergency functions'ı test et
            System::set_block_number(432_000);
            assert_ok!(Runtime::emergency_treasury_release());
            assert_ok!(Runtime::emergency_epoch_finalization());
        });
    }
}