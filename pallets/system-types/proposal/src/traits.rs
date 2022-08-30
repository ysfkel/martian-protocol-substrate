#![cfg_attr(not(feature = "std"), no_std)]

use std::error::Error;

use crate::models::Proposal;
use sp_runtime::DispatchError;

pub trait ProposalTrait {
	type ProposalId;
	type AccountId;
	type CouncilId;
	fn proposal(
		council_id: Self::CouncilId,
		index: Self::ProposalId,
	) -> Option<Proposal<Self::CouncilId>>;
	fn retrieve_highest_valued_proposal(
		council_id: Self::CouncilId,
	) -> Result<Proposal<Self::CouncilId>, DispatchError>;
}
