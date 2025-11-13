use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::{BoundedVec, Get, RuntimeDebug};
use scale_info::TypeInfo;

/// KYC (Müşterini Tanı) süreçlerinin durumunu temsil eden enum.
#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen, Copy, Default)]
pub enum KycLevel {
	/// KYC süreci henüz başlamadı.
	#[default]
	NotStarted,
	/// Başvuru yapıldı, incelenmeyi bekliyor.
	Pending,
	/// Başvuru onaylandı.
	Approved,
	/// Başvuru reddedildi (pending state'den).
	Rejected,
	/// Onaylanmış KYC iptal edildi.
	Revoked,
}

#[derive(Encode, Decode, Clone, Default, MaxEncodedLen)]
pub struct IdentityInfo<MaxStringLength: Get<u32>> {
	pub name: BoundedVec<u8, MaxStringLength>,
	pub email: BoundedVec<u8, MaxStringLength>,
}

// Manually implement PartialEq to avoid requiring `MaxStringLength: PartialEq`
impl<MaxStringLength: Get<u32>> PartialEq for IdentityInfo<MaxStringLength> {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name && self.email == other.email
	}
}
impl<MaxStringLength: Get<u32>> Eq for IdentityInfo<MaxStringLength> {}

// Manually implement Debug as well for the same reason.
impl<MaxStringLength: Get<u32>> sp_std::fmt::Debug for IdentityInfo<MaxStringLength> {
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		f.debug_struct("IdentityInfo")
			.field("name", &self.name)
			.field("email", &self.email)
			.finish()
	}
}

impl<MaxStringLength: Get<u32> + 'static> TypeInfo for IdentityInfo<MaxStringLength>
where
	BoundedVec<u8, MaxStringLength>: TypeInfo,
{
	type Identity = Self;

	fn type_info() -> scale_info::Type {
		scale_info::Type::builder()
			.path(scale_info::Path::new("IdentityInfo", "pallet_identity_kyc::types"))
			.composite(
				scale_info::build::Fields::named()
					.field(|f| f.ty::<BoundedVec<u8, MaxStringLength>>().name("name").type_name("BoundedVec<u8, MaxStringLength>"))
					.field(|f| f.ty::<BoundedVec<u8, MaxStringLength>>().name("email").type_name("BoundedVec<u8, MaxStringLength>")),
			)
	}
}

#[derive(Encode, Decode, Clone, Default, MaxEncodedLen)]
pub struct KycApplication<MaxStringLength: Get<u32>, MaxCidLength: Get<u32>> {
	pub cids: BoundedVec<BoundedVec<u8, MaxCidLength>, MaxCidLength>,
	pub notes: BoundedVec<u8, MaxStringLength>,
}

// Manually implement PartialEq to avoid requiring generic bounds to be PartialEq
impl<MaxStringLength: Get<u32>, MaxCidLength: Get<u32>> PartialEq
	for KycApplication<MaxStringLength, MaxCidLength>
{
	fn eq(&self, other: &Self) -> bool {
		self.cids == other.cids && self.notes == other.notes
	}
}
impl<MaxStringLength: Get<u32>, MaxCidLength: Get<u32>> Eq
	for KycApplication<MaxStringLength, MaxCidLength>
{
}

// Manually implement Debug as well for the same reason.
impl<MaxStringLength: Get<u32>, MaxCidLength: Get<u32>> sp_std::fmt::Debug
	for KycApplication<MaxStringLength, MaxCidLength>
{
	fn fmt(&self, f: &mut sp_std::fmt::Formatter<'_>) -> sp_std::fmt::Result {
		f.debug_struct("KycApplication")
			.field("cids", &self.cids)
			.field("notes", &self.notes)
			.finish()
	}
}

impl<MaxStringLength: Get<u32> + 'static, MaxCidLength: Get<u32> + 'static> TypeInfo
	for KycApplication<MaxStringLength, MaxCidLength>
where
	BoundedVec<BoundedVec<u8, MaxCidLength>, MaxCidLength>: TypeInfo,
	BoundedVec<u8, MaxStringLength>: TypeInfo,
{
	type Identity = Self;

	fn type_info() -> scale_info::Type {
		scale_info::Type::builder()
			.path(scale_info::Path::new("KycApplication", "pallet_identity_kyc::types"))
			.composite(
				scale_info::build::Fields::named()
					.field(|f| {
						f.ty::<BoundedVec<BoundedVec<u8, MaxCidLength>, MaxCidLength>>()
							.name("cids")
							.type_name("BoundedVec<BoundedVec<u8, MaxCidLength>, MaxCidLength>")
					})
					.field(|f| {
						f.ty::<BoundedVec<u8, MaxStringLength>>()
							.name("notes")
							.type_name("BoundedVec<u8, MaxStringLength>")
					}),
			)
	}
}
// --- Dış Dünya İçin Arayüzler (Traits) ---

/// Bir hesabın KYC durumunu sorgulamak için arayüz.
pub trait KycStatus<AccountId> {
	fn get_kyc_status(who: &AccountId) -> KycLevel;
}

/// Bir hesabın kimlik bilgilerini sorgulamak için arayüz.
pub trait IdentityInfoProvider<AccountId, MaxStringLength: Get<u32>> {
	fn get_identity_info(who: &AccountId) -> Option<IdentityInfo<MaxStringLength>>;
}

/// KYC onaylandığında tetiklenecek eylemleri tanımlayan arayüz.
/// Bu trait identity-kyc palletinde tanımlanır ve diğer palletler (örn. referral)
/// tarafından implement edilir, böylece circular dependency oluşmaz.
pub trait OnKycApproved<AccountId> {
	fn on_kyc_approved(who: &AccountId);
}

/// Vatandaşlık NFT'si mintlemek için arayüz.
/// Bu trait identity-kyc palletinde tanımlanır ve tiki pallet tarafından
/// implement edilir, böylece circular dependency oluşmaz.
pub trait CitizenNftProvider<AccountId> {
	fn mint_citizen_nft(who: &AccountId) -> sp_runtime::DispatchResult;
}