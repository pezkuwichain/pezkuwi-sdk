// === pallet_referral/src/mock.rs (Güncellenmiş Versiyon) ===

use crate as pallet_referral;
use frame_support::{parameter_types, traits::{ConstU32, Everything}};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

// pezkuwi_primitives importları (eğer mock'ta kullanılıyorsa)
use pezkuwi_primitives::{types::{KycStatusEnum, Tiki}};
// Palet NFT'den import (eğer mock'ta kullanılıyorsa)
use pallet_nft::NftManager as NftManagerTrait;
use frame_support::weights::Weight; // WeightInfo için Weight tipi gerekli olabilir

// Dummy KycStatusProvider implementasyonu
// Gerçek uygulamada bu, Kyc paletinin kendisi olacaktır.
pub struct MockKycStatusProvider;
impl pezkuwi_primitives::traits::KycStatusProvider<AccountId, KycStatusEnum> for MockKycStatusProvider {
    fn get_kyc_status(_account: &AccountId) -> KycStatusEnum {
        // Testlerde her zaman onaylanmış statüsü döndürebiliriz veya senaryoya göre değiştirebiliriz.
        // Basitlik için her zaman onaylanmış döndürelim:
        KycStatusEnum::Approved
    }
}

// Dummy NftManager implementasyonu
// Gerçek uygulamada bu, NFT paletinin kendisi olacaktır.
pub struct MockNftManager;
impl pezkuwi_primitives::traits::NftManager<AccountId, Tiki, KycStatusEnum, u64> for MockNftManager {
    fn mint_nft(_owner: &AccountId, _tiki: Tiki) -> frame_support::pallet_prelude::DispatchResult {
        // Testlerde başarılı olduğunu varsayabiliriz.
        Ok(())
    }
}


pub type AccountId = u64;

frame_support::construct_runtime!(
    pub enum TestRuntime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: system::{Pallet, Call, Config, Storage, Event<T>},
        Referral: pallet_referral::{Pallet, Call, Storage, Event<T>},
        // Diğer paletler mock'ta burada tanımlanmalı (örneğin, KycPallet, NftPallet)
        // KycPallet: pallet_identity_kyc::{Pallet, Call, Storage, Event<T>},
        // NftPallet: pallet_nft::{Pallet, Call, Storage, Event<T>},
    }
);

impl system::Config for TestRuntime {
    type BaseCallFilter = Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type RuntimeBlockNumber = u64;
    type BlockHashCount = ConstU32;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = Header;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU32;
    type OnSetCode = ();
}

// Dummy WeightInfo implementasyonu (tests.rs için yeterli olabilir)
pub struct TestWeightInfo;
impl pallet_referral::WeightInfo for TestWeightInfo {
    fn initiate_referral() -> Weight { Weight::from_parts(10_000, 0) }
    fn hook_on_kyc_approved_found() -> Weight { Weight::from_parts(10_000, 0) }
    fn hook_on_kyc_approved_not_found() -> Weight { Weight::from_parts(10_000, 0) }
}


impl pallet_referral::Config for TestRuntime {
    type RuntimeEvent = RuntimeEvent;
    // GovernanceOrigin bu pallette kullanılmıyor gibi görünüyor (ensure_root yok)
    // Eğer AdminOrigin kullanılıyorsa Config trait'te AdminOrigin olmalı ve mock'ta implemente edilmeli
    // type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin, Success = Self::AccountId>; // Eğer lib.rs'de varsa
    // type AdminOrigin = frame_system::EnsureRoot<AccountId>; // Örnek implementasyon

    type WeightInfo = TestWeightInfo; // Dummy WeightInfo kullanılıyor

    // KycStatusProvider ve NftManager için dummy implementasyonlar atanıyor
    type KycStatusProvider = MockKycStatusProvider;
    type NftManager = MockNftManager;

    // lib.rs Config trait'indeki diğer assosiye tipler buraya eklenecek (örneğin, MaxCourseNameLength gibi sabitler, eğer Referral'da da varsa)
    // Palet referral'ın Config'inde sabitler yoktu, bu yüzden şimdilik gerek yok.
}


// Build genesis storage for a default pallet instance.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<TestRuntime>::default().build_storage().unwrap().into()
}