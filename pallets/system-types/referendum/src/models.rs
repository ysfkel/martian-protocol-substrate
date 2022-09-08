use crate::Tally;
use codec::{Decode, EncodeLike};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Saturating, Zero},
	RuntimeDebug,
};

#[derive(Clone, Encode, Decode, Default, RuntimeDebug, PartialEq, Eq, TypeInfo)]
pub struct ReferendumStatus<CollectiveId, ProposalId, BlockNumber, Balance> {
	/// collective id
	pub collective_id: CollectiveId,
	/// proposal id
	pub proposal_id: ProposalId,
	/// When voting on this referendum will end.
	pub end: BlockNumber,

	pub tally: Tally<Balance>,
}

impl<CollectiveId, ProposalId, BlockNumber, Balance: Default>
	ReferendumStatus<CollectiveId, ProposalId, BlockNumber, Balance>
{
	pub fn new(collective_id: CollectiveId, proposal_id: ProposalId, end: BlockNumber) -> Self {
		Self { collective_id, proposal_id, end, tally: Tally::default() }
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub enum Referendum<CollectiveId, ProposalId, BlockNumber, Balance> {
	/// Referendum is happening, the arg is the block number at which it will end.
	Ongoing(ReferendumStatus<CollectiveId, ProposalId, BlockNumber, Balance>),
	/// Referendum finished at `end`, and has been `approved` or rejected.
	Finished { approved: bool, end: BlockNumber },
}
