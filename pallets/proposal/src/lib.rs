#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::{defensive_prelude::*, Currency, LockableCurrency, ReservableCurrency};
pub use pallet::*;
use proposal_types::{
	models::{Proposal, Target},
	traits::ProposalTrait,
};
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, Saturating, Zero},
	DispatchError,
};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;

// pub type ProposalIndex = u32;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use core::{fmt::Debug, ops::AddAssign};

	use codec::FullCodec;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use num_traits::One;
	use pallet_math::SafeAdd;
	use proposal_types::models::Proposal;
	use sp_runtime::traits::MaybeDisplay;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
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
			+ AtLeast32BitUnsigned
			+ SafeAdd
			+ Zero
			+ From<u128>;
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
	}

	#[pallet::storage]
	#[pallet::getter(fn proposal_count)]
	pub type ProposalCount<T: Config> =
		StorageMap<_, Twox64Concat, T::CouncilId, T::ProposalId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn proposals)]
	pub type Proposals<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CouncilId,
		Twox64Concat,
		T::ProposalId,
		Proposal<T::CouncilId>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn proposal_index)]
	pub type ProposalIndex<T: Config> =
		StorageMap<_, Twox64Concat, T::CouncilId, Vec<(T::ProposalId, T::AccountId)>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn deposit_of)]
	pub type DepositOf<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::CouncilId,
		Twox64Concat,
		T::ProposalId,
		(Vec<T::AccountId>, BalanceOf<T>),
	>;

	// #[pallet::genesis_config]
	// pub struct GenesisConfig<T: Config> {
	// 	_phantom: sp_std::marker::PhantomData<T>,
	// }

	// #[cfg(feature = "std")]
	// impl<T: Config> Default for GenesisConfig<T> {
	// 	fn default() -> Self {
	// 		GenesisConfig { _phantom: Default::default() }
	// 	}
	// }

	// #[pallet::genesys_build]
	// impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
	// 	fn build(&self) {
	// 		//ProposalIndex::<T>::put(0 as T::ProposalId);
	// 	}
	// }

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ProposalCreated {
			proposal_index: T::ProposalId,
			deposit: BalanceOf<T>,
		},
		Endorsed {
			account: T::AccountId,
			proposal_index: T::ProposalId,
			council_id: T::CouncilId,
			deposit: BalanceOf<T>,
		},
		HighestValuedProposalRemoved {
			proposal_index: T::ProposalId,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		ProposalNotFound,
		NoProposalFound,
		NoProposalToRemove,
	}

	// #[pallet::hooks]
	// impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
	// 	/// Weight: see `begin_block`
	// 	// fn on_initialize(n: T::BlockNumber) -> Weight {
	// 	// 	//Self::begin_block(n)
	// 	// }
	// }

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn create_proposal(
			origin: OriginFor<T>,
			council_id: T::CouncilId,
			content: Vec<u8>,
			#[pallet::compact] value: BalanceOf<T>,
			target: Target<T::CouncilId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			T::Currency::reserve(&who, value)?;
			let mut proposal_index: T::ProposalId = T::ProposalId::zero();
			match target.clone() {
				Target::Council(_) => {
					proposal_index =
						Self::_create_proposal(content, council_id.clone(), &who, target)?;
				},
				Target::None => {},
			}

			<DepositOf<T>>::insert(council_id, proposal_index, (&[&who][..], value));
			Self::deposit_event(Event::<T>::ProposalCreated { proposal_index, deposit: value });
			Ok(().into())
		}

		#[pallet::weight(100_000)]
		pub fn endorse(
			origin: OriginFor<T>,
			council_id: T::CouncilId,
			#[pallet::compact] proposal_id: T::ProposalId,
		) -> DispatchResultWithPostInfo {
			// do checks
			let who = ensure_signed(origin)?;
			let mut deposit_of: (Vec<T::AccountId>, BalanceOf<T>) =
				Self::deposit_of(council_id.clone(), proposal_id)
					.ok_or(Error::<T>::ProposalNotFound)?;
			T::Currency::reserve(&who, deposit_of.1)?;
			deposit_of.0.push(who.clone());
			let deposit = deposit_of.1;
			<DepositOf<T>>::insert(council_id.clone(), proposal_id.clone(), deposit_of);
			Self::deposit_event(Event::<T>::Endorsed {
				account: who,
				council_id,
				proposal_index: proposal_id,
				deposit,
			});

			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T> {
		pub fn _create_proposal(
			content: Vec<u8>,
			council_id: T::CouncilId,
			who: &T::AccountId,
			target: Target<T::CouncilId>,
		) -> Result<T::ProposalId, DispatchError> {
			let id = ProposalCount::<T>::try_mutate(
				council_id.clone(),
				|proposal_count| -> Result<T::ProposalId, DispatchError> {
					*proposal_count = proposal_count.safe_add(&T::ProposalId::one())?;

					ProposalIndex::<T>::append(council_id.clone(), (proposal_count.clone(), &who));

					<Proposals<T>>::insert(
						council_id,
						proposal_count.clone(),
						Proposal { content, target },
					);
					Ok(*proposal_count)
				},
			);
			id
		}

		pub fn backing_for(
			council_id: T::CouncilId,
			proposal_index: T::ProposalId,
		) -> Option<BalanceOf<T>> {
			Self::deposit_of(council_id, proposal_index)
				.map(|(accounts, deposit)| deposit.saturating_mul((accounts.len() as u32).into()))
		}
	}
}

impl<T: Config> ProposalTrait for Pallet<T> {
	type ProposalId = T::ProposalId;
	type AccountId = T::AccountId;
	type CouncilId = T::CouncilId;

	fn proposal(
		council_id: T::CouncilId,
		proposal_index: Self::ProposalId,
	) -> Option<Proposal<Self::CouncilId>> {
		Self::proposals(council_id, proposal_index)
	}

	fn retrieve_highest_valued_proposal(
		council_id: T::CouncilId,
	) -> Result<Proposal<Self::CouncilId>, DispatchError> {
		let mut proposals_index = Self::proposal_index(council_id.clone());
		if let Some((winner_index, _)) = proposals_index.iter().enumerate().max_by_key(|x| {
			Self::backing_for(council_id.clone(), (x.1).0).defensive_unwrap_or_else(Zero::zero)
		}) {
			let (proposal_index, _) = proposals_index.swap_remove(winner_index);
			<ProposalIndex<T>>::insert(council_id.clone(), proposals_index.clone());

			if let Some((depositors, deposit)) =
				<DepositOf<T>>::take(council_id.clone(), proposal_index.clone())
			{
				for d in &depositors {
					T::Currency::unreserve(&d, deposit);
				}
			}

			if let Some(proposal) = Self::proposals(council_id.clone(), proposal_index.clone()) {
				<Proposals<T>>::remove(council_id, proposal_index.clone());

				Self::deposit_event(Event::<T>::HighestValuedProposalRemoved { proposal_index });

				Ok(proposal)
			} else {
				Err(DispatchError::CannotLookup)
			}
		} else {
			Err(DispatchError::CannotLookup)
		}
	}
}
