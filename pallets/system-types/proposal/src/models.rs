use codec::Decode;

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::{collections::btree_set::BTreeSet, vec::Vec};
#[derive(Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
pub enum Content {
	None,
	Raw(Vec<u8>),
}

// #[derive(Debug, Clone, Encode, Decode, Eq, PartialEq, TypeInfo)]
// pub enum Target<CollectiveId> {
// 	None,
// 	// id's of collectives that will vote
// 	Collective(Vec<CollectiveId>),
// }

// impl<CollectiveId> Default for Target<CollectiveId> {
// 	fn default() -> Self {
// 		Self::None
// 	}
// }

// enum Targe {
// 	Council(BTreeSet<CollectiveId>),
// 	Collective(BTreeSet<CollectiveId>),
// }

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Proposal<CollectiveId> {
	pub council: BTreeSet<CollectiveId>,
	pub content: Vec<u8>,
}

impl Default for Content {
	fn default() -> Self {
		Self::None
	}
}
