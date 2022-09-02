use codec::Decode;

use codec::EncodeLike;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Referendum<CouncilId> {
	pub council_id: CouncilId,
}

// #[derive(Copy, TypeInfo, Clone, Eq, PartialEq, Default, RuntimeDebug)]
// pub struct Vote {
// 	pub aye: bool,
// 	// pub conviction: Conviction
// }

// impl Encode for Vote {
// 	fn encode_to<T: Output + ?Sized>(&self, output: &mut T) {
// 		output.push_byte(u8::from(self.conv))
// 	}
// }

// #[derive(Encode, Decode, Copy, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
// pub enum AccountVote<Balance> {
// 	Standard { vote: Vote, balance: Balance },
// }

// impl EncodeLike for Vote {}

// impl Encode

// impl Default for Content {
// 	fn default() -> Self {
// 		Self::None
// 	}
// }
