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
	use sp_runtime::traits::{CheckedAdd, MaybeDisplay};

	#[pallet::pallet]
	#[pallet::without_storage_info]
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

		type CouncilId: FullCodec
			+ Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ Debug
			+ MaybeDisplay
			+ Ord
			+ MaxEncodedLen
			+ Eq
			+ PartialEq
			+ Copy
			+ TypeInfo;

		type ProposalSource: ProposalTrait<
			AccountId = Self::AccountId,
			ProposalId = Self::ProposalId,
			CouncilId = Self::CouncilId,
		>;
	}

	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> =
		StorageMap<_, Twox64Concat, T::CouncilId, Proposal<T::CouncilId>, OptionQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn referendums)]
	// pub type Referendums<T: Config> =
	// 	StorageMap<_, Twox64Concat, T::CouncilId, Referendum<T::CouncilId>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn start_referendum_by_value(
			origin: OriginFor<T>,
			council_id: T::CouncilId,
		) -> DispatchResultWithPostInfo {
			if let Some(proposal) = Self::get_proposal(council_id) {
				<Proposals<T>>::insert(council_id, proposal);
			}
			// - todo - caller must be council

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn get_proposal(council_id: T::CouncilId) -> Option<Proposal<T::AccountId>> {
			if let Some(proposal) = T::ProposalSource::retrieve_highest_valued_proposal(council_id)
			{
				return Some(proposal)
			}

			None
		}
	}
}
