use codec::{Decode, Encode, EncodeLike};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

#[derive(Clone, Encode, Decode, TypeInfo, Eq, PartialEq, RuntimeDebug)]
pub enum ConvictionType {
	Token,
	Basic,
}

#[derive(Clone, Encode, Decode, TypeInfo, Default, Eq, PartialEq, RuntimeDebug)]
pub struct Collective<AccountId> {
	pub owner: AccountId,
	pub members: Vec<AccountId>,
}

impl<AccountId> Collective<AccountId> {
	fn new(owner: AccountId, members: Vec<AccountId>) -> Self {
		Self { owner, members }
	}
}

impl Default for ConvictionType {
	fn default() -> Self {
		Self::Token
	}
}
// impl<AccountId> EncodeLike for Collective<AccountId> {}
