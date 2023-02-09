#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod test;

const DEMOCRACY_ID: LockIdentifier = *b"democrac";

use frame_support::traits::{
	Currency, LockIdentifier, LockableCurrency, ReservableCurrency, WithdrawReasons,
};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::FullCodec;
	use collective_types::{CollectiveAuthorize, CollectiveInspect};
	use core::{fmt::Debug, ops::AddAssign};
	use frame_support::{pallet_prelude::*, sp_runtime::ArithmeticError};
	use frame_system::pallet_prelude::*;
	use num_traits::One;
	use pallet_math::SafeAdd;
	use proposal_types::{models::Proposal, traits::ProposalInspect};
	use referendum_types::{AccountVote, Referendum, ReferendumStatus, Voting};
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

		type ProposalSource: ProposalInspect<
			AccountId = Self::AccountId,
			ProposalId = Self::ProposalId,
			CollectiveId = Self::CollectiveId,
		>;

		type CollectiveAuthorize: CollectiveAuthorize<
			Self::AccountId,
			CollectiveId = Self::CollectiveId,
		>;

		type CollectiveInspect: CollectiveInspect<CollectiveId = Self::CollectiveId>;

		#[pallet::constant]
		type MaxVotes: Get<u32>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReferendumStarted {
			referendum_id: T::ReferendumId,
			collective_id: T::CollectiveId,
			voting_period: T::BlockNumber,
		},

		Voted {
			voter: T::AccountId,
			referendum_id: T::ReferendumId,
			vote: AccountVote<BalanceOf<T>>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		CouldNotRetrieveProposal,
		NotCollectiveAdmin,
		NotCollectiveMember,
		CollectiveNotFound,
		ReferendumNotFound,
		InsufficientFunds,
		MaxVotesReached,
		AlreadyDelegating,
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
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn voting_of)]
	pub type VotingOf<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CollectiveId,
		Twox64Concat,
		T::AccountId,
		Voting<BalanceOf<T>, T::AccountId, T::BlockNumber, T::ReferendumId>,
		ValueQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(n: T::BlockNumber) -> Weight {
			// Self::close_ongoing_referendum()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn vote(
			origin: OriginFor<T>,
			collective_id: T::CollectiveId,
			referendum_id: T::ReferendumId,
			vote: AccountVote<BalanceOf<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(T::CollectiveInspect::exists(collective_id), Error::<T>::CollectiveNotFound);
			ensure!(
				T::CollectiveAuthorize::is_member(who.clone(), collective_id),
				Error::<T>::NotCollectiveMember
			);

			ensure!(
				vote.balance() <= T::Currency::free_balance(&(who.clone())),
				Error::<T>::InsufficientFunds
			);

			Self::try_vote(&who, collective_id, referendum_id, vote)
		}

		#[pallet::weight(100_000)]
		pub fn start_referendum_by_value(
			origin: OriginFor<T>,
			collective_id: T::CollectiveId,
			voting_period: T::BlockNumber,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			ensure!(T::CollectiveInspect::exists(collective_id), Error::<T>::CollectiveNotFound);
			ensure!(
				T::CollectiveAuthorize::is_admin(who, collective_id),
				Error::<T>::NotCollectiveAdmin
			);

			let referendum_id = Self::create_referendum(collective_id, voting_period)?;

			Self::deposit_event(Event::<T>::ReferendumStarted {
				referendum_id,
				collective_id,
				voting_period,
			});
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		fn try_vote(
			who: &T::AccountId,
			collective_id: T::CollectiveId,
			referendum_id: T::ReferendumId,
			vote: AccountVote<BalanceOf<T>>,
		) -> DispatchResult {
			let mut status = Self::referendum_status(collective_id, referendum_id)?;

			VotingOf::<T>::try_mutate(collective_id, who, |voting| -> DispatchResult {
				if let Voting::Direct { ref mut votes, delegations, .. } = voting {
					match votes.binary_search_by_key(&referendum_id, |i| i.0) {
						/// If the value is found then [`Result::Ok`] is returned, containing the
						/// index of the matching element.
						Ok(i) => {
							// Shouldn't be possible to fail, but we handle it gracefully.
							status.tally.remove(votes[i].1).ok_or(ArithmeticError::Underflow)?;
							if let Some(approve) = votes[i].1.as_standard() {
								status.tally.reduce(approve, *delegations);
							}
							// replace existing with new
							votes[i].1 = vote;
						},
						/// If the value is not found then [`Result::Err`] is returned, containing
						/// the index where a matching element could be inserted while maintaining
						/// sorted order.
						Err(i) => {
							ensure!(
								votes.len() as u32 <= T::MaxVotes::get(),
								Error::<T>::MaxVotesReached
							);
							votes.insert(i, (referendum_id, vote));
						},
					}
					Self::deposit_event(Event::<T>::Voted {
						voter: who.clone(),
						referendum_id,
						vote,
					});
					/// Shouldn't be possible to fail, but we handle it gracefully.
					status.tally.add(vote).ok_or(ArithmeticError::Overflow)?;
					if let Some(approve) = vote.as_standard() {
						status.tally.increase(approve, *delegations);
					}
					Ok(())
				} else {
					Err(Error::<T>::AlreadyDelegating.into())
				}
			})?;
			// Extend the lock to `balance` (rather than setting it) since we don't know what other
			// votes are in place.
			T::Currency::extend_lock(DEMOCRACY_ID, who, vote.balance(), WithdrawReasons::TRANSFER);
			Referendums::<T>::insert(collective_id, referendum_id, Referendum::Ongoing(status));
			Ok(())
		}

		fn ensure_ongoing(
			r: Referendum<T::CollectiveId, T::ReferendumId, T::BlockNumber, BalanceOf<T>>,
		) -> Result<
			ReferendumStatus<T::CollectiveId, T::ReferendumId, T::BlockNumber, BalanceOf<T>>,
			DispatchError,
		> {
			match r {
				Referendum::Ongoing(o) => Ok(o),
				_ => Err(Error::<T>::ReferendumNotFound.into()),
			}
		}

		fn referendum_status(
			collective_id: T::CollectiveId,
			referendum_id: T::ReferendumId,
		) -> Result<
			ReferendumStatus<T::CollectiveId, T::ReferendumId, T::BlockNumber, BalanceOf<T>>,
			DispatchError,
		> {
			let info = Referendums::<T>::get(collective_id, referendum_id)
				.ok_or(Error::<T>::ReferendumNotFound)?;
			Self::ensure_ongoing(info)
		}

		fn create_referendum(
			collective_id: T::CollectiveId,
			voting_period: T::BlockNumber,
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

					let end = now.saturating_add(voting_period);

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

		fn close_ongoing_referendum() {}
	}
}
