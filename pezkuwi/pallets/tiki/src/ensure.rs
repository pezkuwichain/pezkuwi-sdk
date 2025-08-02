//! Tiki sahipliğine dayalı özel Yetki (Origin) doğrulama mekanizmaları.

use crate::{Config, Pallet as TikiPallet};
use frame_support::traits::EnsureOrigin;
use frame_system::ensure_signed;
use sp_std::marker::PhantomData;

// --- 1. Adım: Gerekli Trait'i Tanımla ---

/// Belirli bir `Tiki` enum variant'ını döndürmek için kullanılacak genel bir trait.
pub trait GetTiki {
	fn tiki() -> crate::Tiki;
}

// --- 2. Adım: Ana Yetki Kontrol Yapısını Oluştur ---

/// Belirli bir Tiki'ye sahip olmayı gerektiren bir `EnsureOrigin` implementasyonu.
pub struct EnsureTiki<T, I>(PhantomData<(T, I)>);

impl<T, I> EnsureOrigin<T::RuntimeOrigin> for EnsureTiki<T, I>
where
	T: Config,
	I: GetTiki,
{
	type Success = T::AccountId;

	fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
		let who = match ensure_signed(o.clone()) {
            Ok(account) => account,
            Err(_) => return Err(o),
        };
		let required_tiki = I::tiki();
		match TikiPallet::<T>::tiki_holder(required_tiki) {
			Some(holder) if holder == who => Ok(who),
			_ => Err(o),
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<T::RuntimeOrigin, ()> {
		use frame_benchmarking::account;
		let zero_account: T::AccountId = account("tiki_holder", 0, 0);
		Ok(T::RuntimeOrigin::from(frame_system::RawOrigin::Signed(zero_account)))
	}
}

// --- 3. Adım: Her Rol İçin Marker Struct'ları ve Implementasyonlarını Tanımla ---

/// `Serok` rolünü temsil eden marker.
pub struct SerokRole;
impl GetTiki for SerokRole {
	fn tiki() -> crate::Tiki { crate::Tiki::Serok }
}

/// `Wezir` rolünü temsil eden marker.
pub struct WezirRole;
impl GetTiki for WezirRole {
	fn tiki() -> crate::Tiki { crate::Tiki::Wezir }
}

/// `Parlementer` rolünü temsil eden marker.
pub struct ParlementerRole;
impl GetTiki for ParlementerRole {
	fn tiki() -> crate::Tiki { crate::Tiki::Parlementer }
}