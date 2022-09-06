use codec::Decode;

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum Content {
	None,
	Raw(Vec<u8>),
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum Target<CollectiveId> {
	None,
	Collective(Vec<CollectiveId>),
}

impl<CollectiveId> Default for Target<CollectiveId> {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<CollectiveId> {
	pub target: Target<CollectiveId>,
	pub content: Vec<u8>, // Content
}

impl Default for Content {
	fn default() -> Self {
		Self::None
	}
}
