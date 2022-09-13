#![cfg_attr(not(feature = "std"), no_std)]

use std::error::Error;

use crate::models::Proposal;
use sp_runtime::DispatchError;

pub trait ProposalInspect {
	type ProposalId;
	type AccountId;
	type CollectiveId;
	fn proposal(
		collective_id: Self::CollectiveId,
		index: Self::ProposalId,
	) -> Option<Proposal<Self::CollectiveId>>;
	fn retrieve_highest_valued_proposal(
		collective_id: Self::CollectiveId,
	) -> Result<Proposal<Self::CollectiveId>, DispatchError>;
}
