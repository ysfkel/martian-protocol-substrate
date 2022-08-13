#![cfg_attr(not(featuire = "std"), no_std)]

use sp_std::vec::Vec;
// pub use pallet::*;

enum Proposal {
	Raw(Vec<u8>),
}

#[frame_support::pallet]
pub mod pallet {
	use core::ops::AddAssign;

	use codec::FullCodec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use num_traits::One;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ProposalID: AddAssign
			+ FullCodec
			+ MaxEncodedLen
			+ One
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProposalCreated(),
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn create_proposal(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}
	}
}
