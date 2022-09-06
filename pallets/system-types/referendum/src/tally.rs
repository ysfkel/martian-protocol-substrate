use crate::{AccountVote, Delegations, Vote};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Bounded, CheckedAdd, CheckedDiv, CheckedMul, CheckedSub, Saturating, Zero},
	RuntimeDebug,
};

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Tally<Balance> {
	/// The number of aye votes, expressed in terms of post-conviction lock-vote.
	pub ayes: Balance,
	/// The number of nay votes, expressed in terms of post-conviction lock-vote.
	pub nays: Balance,
	/// The amount of funds currently expressing its opinion. Pre-conviction.
	pub turnout: Balance,
}

impl<
		Balance: From<u8>
			+ Zero
			+ Copy
			+ CheckedAdd
			+ CheckedSub
			+ CheckedMul
			+ CheckedDiv
			+ Bounded
			+ Saturating,
	> Tally<Balance>
{
	/// Create a new tally.
	pub fn new(vote: Vote, balance: Balance) -> Self {
		let Delegations { votes, capital } = vote.conviction.votes(balance);
		Self {
			ayes: if vote.aye { votes } else { Zero::zero() },
			nays: if vote.aye { Zero::zero() } else { votes },
			turnout: capital,
		}
	}
	/// Add an account's vote into the tally.
	pub fn add(&mut self, vote: AccountVote<Balance>) -> Option<()> {
		match vote {
			AccountVote::Standard { vote, balance } => {
				let Delegations { votes, capital } = vote.conviction.votes(balance);
				self.turnout = self.turnout.checked_add(&capital)?;
				match vote.aye {
					true => self.ayes = self.ayes.checked_add(&votes)?,
					false => self.nays = self.nays.checked_add(&votes)?,
				}
			},
		}

		Some(())
	}
}
