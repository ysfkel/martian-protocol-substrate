#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

use frame_support::traits::{LockableCurrency, ReservableCurrency};

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
	use sp_runtime::{
		traits::{CheckedAdd, MaybeDisplay},
		DispatchError,
	};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Currency: ReservableCurrency<Self::AccountId>
			+ LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

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

		type CollectiveId: FullCodec
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
			CollectiveId = Self::CollectiveId,
		>;
	}

	#[pallet::event]
	pub enum Event<T: Config> {}

	#[pallet::error]
	pub enum Error<T> {
		CouldNotRetrieveProposal,
	}

	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> =
		StorageMap<_, Twox64Concat, T::CollectiveId, Proposal<T::CollectiveId>, OptionQuery>;

	// #[pallet::storage]
	// #[pallet::getter(fn referendums)]
	// pub type Referendums<T: Config> =
	// 	StorageMap<_, Twox64Concat, T::CollectiveId, Referendum<T::CollectiveId>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight]
		pub fn vote(
			origin: OriginFor<T>,
			value: T::Balance,
			collection_id: CollectiveId,
		) -> DispatchResultWithPostInfo {
		}

		#[pallet::weight(100_000)]
		pub fn start_referendum_by_value(
			origin: OriginFor<T>,
			collective_id: T::CollectiveId,
		) -> DispatchResultWithPostInfo {
			// - todo - caller must be collective
			let proposal = T::ProposalSource::retrieve_highest_valued_proposal(collective_id)
				.map_err(|_| Error::<T>::CouldNotRetrieveProposal)?;

			<Proposals<T>>::insert(collective_id, proposal);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {}
}
