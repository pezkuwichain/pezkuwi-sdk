// ==============================================================================
// === KK/pallets/society/egitim/src/weights.rs  =========
// ==============================================================================

//! Pallet-Egitim için Weight bilgisi.
//! Bu dosya, pallet'in extrinsic'lerinin ağırlıklarını tanımlayan bir taslaktır.
//! Gerçek değerler benchmarking ile elde edilmelidir.

use frame_support::weights::Weight; // SADECE BU IMPORT YETERLİ

/// Pallet-Egitim için WeightInfo trait'i
pub trait WeightInfo {
    fn create_course(name_len: u32, desc_len: u32, link_len: u32) -> Weight;
    fn enroll() -> Weight;
    fn complete_course() -> Weight;
    fn archive_course() -> Weight;
    fn get_egitim_score(completed_courses_count: u32) -> Weight;
}

// Sabit ağırlık değerleri
const READ_OP_REF_TIME: u64 = 20_000_000;
const WRITE_OP_REF_TIME: u64 = 40_000_000;

const READ_OP_WEIGHT: Weight = Weight::from_parts(READ_OP_REF_TIME, 0);
const WRITE_OP_WEIGHT: Weight = Weight::from_parts(WRITE_OP_REF_TIME, 0);

impl WeightInfo for () {
    fn create_course(name_len: u32, desc_len: u32, link_len: u32) -> Weight {
        Weight::from_parts(10_000_000, 0)
            .saturating_add(READ_OP_WEIGHT)
            .saturating_add(WRITE_OP_WEIGHT)
            .saturating_add(WRITE_OP_WEIGHT)
            .saturating_add(Weight::from_parts(1_000, 0).saturating_mul(name_len as u64))
            .saturating_add(Weight::from_parts(1_000, 0).saturating_mul(desc_len as u64))
            .saturating_add(Weight::from_parts(1_000, 0).saturating_mul(link_len as u64))
    }

    fn enroll() -> Weight {
        Weight::from_parts(15_000_000, 0)
            .saturating_add(READ_OP_WEIGHT.saturating_mul(2))
            .saturating_add(WRITE_OP_WEIGHT.saturating_mul(2))
    }

    fn complete_course() -> Weight {
        Weight::from_parts(20_000_000, 0)
             .saturating_add(READ_OP_WEIGHT.saturating_mul(2))
             .saturating_add(WRITE_OP_WEIGHT)
             .saturating_add(READ_OP_WEIGHT)
             .saturating_add(WRITE_OP_WEIGHT)
    }

    fn archive_course() -> Weight {
        Weight::from_parts(10_000_000, 0)
            .saturating_add(READ_OP_WEIGHT)
            .saturating_add(WRITE_OP_WEIGHT)
    }

     fn get_egitim_score(completed_courses_count: u32) -> Weight {
        Weight::from_parts(5_000_000, 0)
            .saturating_add(READ_OP_WEIGHT)
            .saturating_add(Weight::from_parts(READ_OP_REF_TIME / 10, 0).saturating_mul(completed_courses_count as u64))
    }
}