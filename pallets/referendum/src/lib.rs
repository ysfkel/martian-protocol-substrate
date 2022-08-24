#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::{fmt::Debug, ops::AddAssign};
	use proposal_types::{models::Proposal, traits::ProposalTrait};

	use codec::FullCodec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use num_traits::One;
	use pallet_math::SafeAdd;
	use sp_runtime::traits::CheckedAdd;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type ProposalId: AddAssign
			+ FullCodec
			+ MaxEncodedLen
			+ One
			+ Eq
			+ PartialEq
			+ Copy
			+ MaybeSerializeDeserialize
			+ CheckedAdd
			+ Debug
			+ Default
			+ TypeInfo
			+ SafeAdd
			+ Into<u128>;
		type ProposalSource: ProposalTrait<ProposalId = Self::ProposalId>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn start_referendum_by_value(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			//let id: T::ProposalId = T::ProposalId::from(1);
			//let x = T::ProposalSource::proposal(id);
			Ok(().into())
		}

		#[pallet::weight(100_000)]
		pub fn start_referendum_by_index(
			origin: OriginFor<T>,
			proposal_index: T::ProposalId,
		) -> DispatchResultWithPostInfo {
			//let id: T::ProposalId = 1_; //T::ProposalId::from(1);
			//let x = T::ProposalSource::proposal(id);
			Ok(().into())
		}
	}
}
