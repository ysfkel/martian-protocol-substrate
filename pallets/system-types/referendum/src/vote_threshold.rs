use crate::Tally;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::traits::{IntegerSquareRoot, Zero};
use sp_std::ops::{Add, Div, Mul, Rem};

/// A means of determining if a vote is past pass threshold.
#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, sp_runtime::RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum VoteThreshold {
	/// A simple majority of approvals is needed to pass this vote.
	SimpleMajority,
}

pub trait Approved<Balance> {
	/// Given a `tally` of votes and a total size of `electorate`, this returns `true` if the
	/// overall outcome is in favor of approval according to `self`'s threshold method.
	fn approved(&self, tally: Tally<Balance>, electorate: Balance) -> bool;
}

impl<
		Balance: IntegerSquareRoot
			+ Zero
			+ Ord
			+ Add<Balance, Output = Balance>
			+ Mul<Balance, Output = Balance>
			+ Div<Balance, Output = Balance>
			+ Rem<Balance, Output = Balance>
			+ Copy,
	> Approved<Balance> for VoteThreshold
{
	fn approved(&self, tally: Tally<Balance>, electorate: Balance) -> bool {
		let sqrt_voters = tally.turnout.integer_sqrt();

		if sqrt_voters.is_zero() {
			return false
		}
		match *self {
			VoteThreshold::SimpleMajority => tally.ayes > tally.nays,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_work() {
		assert!(!VoteThreshold::SuperMajorityApprove
			.approved(Tally { ayes: 60, nays: 50, turnout: 110 }, 210));
		assert!(VoteThreshold::SuperMajorityApprove
			.approved(Tally { ayes: 100, nays: 50, turnout: 150 }, 210));
	}
}
