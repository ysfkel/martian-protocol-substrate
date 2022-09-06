use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{Saturating, Zero},
	RuntimeDebug,
};

#[derive(Encode, Decode, Default, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Delegations<Balance> {
	/// The number of votes (this is post-conviction).
	pub votes: Balance,
	/// The amount of raw capital, used for the turnout.
	pub capital: Balance,
}

impl<Balance: Saturating> Saturating for Delegations<Balance> {
	fn saturating_add(self, rhs: Self) -> Self {
		Self {
			votes: self.votes.saturating_add(rhs.votes),
			capital: self.capital.saturating_add(rhs.capital),
		}
	}

	fn saturating_sub(self, rhs: Self) -> Self {
		Self {
			votes: self.votes.saturating_sub(rhs.votes),
			capital: self.capital.saturating_sub(rhs.capital),
		}
	}

	fn saturating_mul(self, rhs: Self) -> Self {
		Self {
			votes: self.votes.saturating_mul(rhs.votes),
			capital: self.capital.saturating_mul(rhs.capital),
		}
	}

	fn saturating_pow(self, exp: usize) -> Self {
		Self { votes: self.votes.saturating_pow(exp), capital: self.capital.saturating_pow(exp) }
	}
}
