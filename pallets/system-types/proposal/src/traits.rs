#![cfg_attr(not(feature = "std"), no_std)]

use crate::models::Proposal;

pub trait ProposalTrait<AccountId> {
	type ProposalId;
	fn proposal(index: Self::ProposalId) -> Option<Proposal<AccountId>>;
}
