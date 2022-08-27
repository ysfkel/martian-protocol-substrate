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
pub enum Target<CouncilId> {
	None,
	Council(Vec<CouncilId>),
}

impl<CouncilId> Default for Target<CouncilId> {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<CouncilId> {
	pub target: Target<CouncilId>,
	pub content: Vec<u8>, // Content
}

impl Default for Content {
	fn default() -> Self {
		Self::None
	}
}
