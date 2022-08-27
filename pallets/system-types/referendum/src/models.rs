use codec::Decode;

use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;

#[derive(Clone, Encode, Decode, Default, Debug, PartialEq, TypeInfo)]
pub struct Referendum<CouncilId> {
	pub council_id: CouncilId,
}

// impl Default for Content {
// 	fn default() -> Self {
// 		Self::None
// 	}
// }
