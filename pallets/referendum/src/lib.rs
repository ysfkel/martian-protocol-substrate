#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;
use frame_support::traits::{Currency, LockableCurrency, ReservableCurrency};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::FullCodec;
	use core::{fmt::Debug, ops::AddAssign};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use num_traits::One;
	use pallet_math::SafeAdd;
	use proposal_types::{models::Proposal, traits::ProposalTrait};
	use referendum_types::{Referendum, ReferendumStatus};
	use sp_runtime::{
		traits::{AtLeast32BitUnsigned, CheckedAdd, MaybeDisplay, Saturating, Zero},
		DispatchError,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
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

		type ReferendumId: AddAssign
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
			+ AtLeast32BitUnsigned
			+ SafeAdd
			+ Zero;

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
	pub type Proposals<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CollectiveId,
		Twox64Concat,
		T::ReferendumId,
		Proposal<T::CollectiveId>,
		OptionQuery,
	>;

	/// The next free referendum index, aka the number of referenda started so far.
	#[pallet::storage]
	#[pallet::getter(fn referendum_count)]
	pub type ReferendumCount<T: Config> =
		StorageMap<_, Twox64Concat, T::CollectiveId, T::ReferendumId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn referendums)]
	pub type Referendums<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CollectiveId,
		Twox64Concat,
		T::ReferendumId,
		Referendum<T::CollectiveId, T::ReferendumId, T::BlockNumber, BalanceOf<T>>,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn vote(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			Ok(().into())
		}

		#[pallet::weight(100_000)]
		pub fn start_referendum_by_value(
			origin: OriginFor<T>,
			collective_id: T::CollectiveId,
			voting_priod: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			// - todo - caller must be collective
			Self::create_referendum(collective_id, voting_priod);
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn create_referendum(
			collective_id: T::CollectiveId,
			voting_priod: T::BlockNumber,
		) -> Result<T::ReferendumId, DispatchError> {
			let id = <ReferendumCount<T>>::try_mutate(
				collective_id,
				|referendum_count| -> Result<T::ReferendumId, DispatchError> {
					*referendum_count = referendum_count.safe_add(&T::ReferendumId::one())?;

					let referendum_id = referendum_count.clone();

					/// create proposal
					let proposal = T::ProposalSource::retrieve_highest_valued_proposal(collective_id)
						.map_err(|_| Error::<T>::CouldNotRetrieveProposal)?;

					let now = <frame_system::Pallet<T>>::block_number();

					let end = now.saturating_add(voting_priod);

					let status = ReferendumStatus::<
						T::CollectiveId,
						T::ReferendumId,
						T::BlockNumber,
						BalanceOf<T>,
					>::new(collective_id, referendum_id.clone(), end);

					let item = Referendum::Ongoing(status);

					<Referendums<T>>::insert(collective_id, referendum_id.clone(), item);

					<Proposals<T>>::insert(collective_id, referendum_id, proposal);

					Ok(*referendum_count)
				},
			);

			id
		}
	}
}
