#![cfg_attr(not(feature = "std"), no_std)]

use crate::models::Proposal;

pub trait ProposalTrait {
	type ProposalId;
	type AccountId;
	type CouncilId;
	fn proposal(
		council_id: Self::CouncilId,
		index: Self::ProposalId,
	) -> Option<Proposal<Self::AccountId>>;
	fn remove_highest_valued_proposal_index(
		council_id: Self::CouncilId,
	) -> Option<Proposal<Self::AccountId>>;
}
