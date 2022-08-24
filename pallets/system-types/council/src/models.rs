use codec::{Decode, Encode, EncodeLike};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;

//#[derive(Encode, Decode, TypeInfo, Clone, Default, PartialEq, RuntimeDebug)]

#[derive(Clone, Encode, Decode, Default, Eq, PartialEq, TypeInfo, RuntimeDebug)]
pub struct Council<AccountId> {
	pub owner: AccountId,
	pub members: Vec<AccountId>,
}

// impl<AccountId> EncodeLike for Council<AccountId> {}
