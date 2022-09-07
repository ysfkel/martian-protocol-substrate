use codec::Decode;

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum Content {
	None,
	Raw(Vec<u8>),
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<CollectiveId> {
	pub council: Vec<CollectiveId>,
	pub content: Vec<u8>,
}

impl Default for Content {
	fn default() -> Self {
		Self::None
	}
}
