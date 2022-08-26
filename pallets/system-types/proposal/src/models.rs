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
pub enum Target<AccountId> {
	None,
	Council(Vec<AccountId>),
	Electorate(Vec<AccountId>),
}

impl<AccountId> Default for Target<AccountId> {
	fn default() -> Self {
		Self::None
	}
}

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<AccountId> {
	pub target: Target<AccountId>,
	pub content: Vec<u8>, // Content
}

impl Default for Content {
	fn default() -> Self {
		Self::None
	}
}
