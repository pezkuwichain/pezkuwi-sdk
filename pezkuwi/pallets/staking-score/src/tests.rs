use crate::mock::{new_test_ext, Test, StakingScore, MockStakedAmount, MockStakingStartBlock};
use pezkuwi_primitives::traits::StakingScoreProvider;
use crate::{MONTH_IN_BLOCKS};

const UNITS: u128 = 1_000_000_000_000;

#[test]
fn scoring_logic_works_correctly() {
    new_test_ext().execute_with(|| {
        // Test için mevcut bloğu 15. ay olarak ayarlayalım.
        let now = 15 * MONTH_IN_BLOCKS;
        frame_system::Pallet::<Test>::set_block_number(now);

        // --- Senaryo 1: Sıfır stake ---
        MockStakedAmount::set(0);
        assert_eq!(StakingScore::get_staking_score(&1), 0);

        // --- Senaryo 2: Giriş Seviyesi, Yeni Katılımcı (< 1 ay) ---
        MockStakedAmount::set(50 * UNITS);
        MockStakingStartBlock::set(now - (MONTH_IN_BLOCKS / 2)); // Yarım ay önce
        // Puan = 20 (miktar) * 1.0 (süre) = 20
        assert_eq!(StakingScore::get_staking_score(&1), 20);

        // --- Senaryo 3: Destekçi Seviyesi, Orta Vadeli (3-6 ay) ---
        MockStakedAmount::set(500 * UNITS);
        MockStakingStartBlock::set(now - (4 * MONTH_IN_BLOCKS)); // 4 ay önce
        // Puan = 40 (miktar) * 1.4 (süre) = 56
        assert_eq!(StakingScore::get_staking_score(&1), 56);
        
        // --- Senaryo 4: Güçlü Destekçi, Yıllık & Plus Üye (> 1 yıl) ---
        MockStakedAmount::set(1000 * UNITS);
        MockStakingStartBlock::set(1); // Çok uzun zaman önce
        // Puan = 50 (miktar) * 2.0 (süre) = 100
        assert_eq!(StakingScore::get_staking_score(&1), 100);
        
        // --- Senaryo 5: Küçük ama Sadık Yatırımcı (6-12 ay) ---
        MockStakedAmount::set(100 * UNITS); // Miktar puanı = 20
        MockStakingStartBlock::set(now - (7 * MONTH_IN_BLOCKS)); // 7 ay önce
        // Puan = 20 (miktar) * 1.7 (süre) = 34
        assert_eq!(StakingScore::get_staking_score(&1), 34);
    });
}