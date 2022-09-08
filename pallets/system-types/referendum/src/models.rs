use crate::Tally;
use codec::{Decode, EncodeLike, WrapperTypeEncode};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Saturating, Zero},
	RuntimeDebug,
};

#[derive(Clone, Encode, Decode, Default, RuntimeDebug, MaxEncodedLen, PartialEq, Eq, TypeInfo)]
pub struct ReferendumStatus<CollectiveId, ReferendumId, BlockNumber, Balance> {
	/// collective id
	pub collective_id: CollectiveId,
	/// proposal id
	pub referendum_id: ReferendumId,
	/// When voting on this referendum will end.
	pub end: BlockNumber,

	pub tally: Tally<Balance>,
}

impl<CollectiveId, ReferendumId, BlockNumber, Balance: Default>
	ReferendumStatus<CollectiveId, ReferendumId, BlockNumber, Balance>
{
	pub fn new(collective_id: CollectiveId, referendum_id: ReferendumId, end: BlockNumber) -> Self {
		Self { collective_id, referendum_id, end, tally: Tally::default() }
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, MaxEncodedLen, RuntimeDebug, TypeInfo)]
pub enum Referendum<CollectiveId, ReferendumId, BlockNumber, Balance> {
	/// Referendum is happening, the arg is the block number at which it will end.
	Ongoing(ReferendumStatus<CollectiveId, ReferendumId, BlockNumber, Balance>),
	/// Referendum finished at `end`, and has been `approved` or rejected.
	Finished {
		approved: bool,
		end: BlockNumber,
	},

	Unavailable,
}

impl<CollectiveId, ReferendumId, BlockNumber, Balance: Default>
	Referendum<CollectiveId, ReferendumId, BlockNumber, Balance>
{
	pub fn new(collective_id: CollectiveId, referendum_id: ReferendumId, end: BlockNumber) -> Self {
		let status = ReferendumStatus::new(collective_id, referendum_id, end);
		Self::Ongoing(status)
	}
}

impl<CollectiveId, ReferendumId, BlockNumber, Balance: Default> Default
	for Referendum<CollectiveId, ReferendumId, BlockNumber, Balance>
{
	fn default() -> Self {
		Self::Unavailable
	}
}

// impl<CollectiveId, ReferendumId, BlockNumber, Balance> EncodeLike
// 	for Referendum<CollectiveId, ReferendumId, BlockNumber, Balance>
// {
// }

// impl<CollectiveId, ReferendumId, BlockNumber, Balance> WrapperTypeEncode
// 	for Referendum<CollectiveId, ReferendumId, BlockNumber, Balance>
// {
// }
