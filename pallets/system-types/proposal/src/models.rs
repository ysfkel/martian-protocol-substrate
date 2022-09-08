use codec::Decode;

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<CollectiveId> {
	pub collecive: CollectiveId,
	pub content: Vec<u8>,
}
