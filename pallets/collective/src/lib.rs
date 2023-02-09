use collective_types::{
	models::{Collective, ConvictionType},
	CollectiveAuthorize, CollectiveInspect,
};
#[cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use sp_runtime::{
	traits::{AtLeast32BitUnsigned, CheckedAdd, Zero},
	DispatchError,
};
pub use sp_std::prelude::*;
use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator, prelude::*};
#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;

pub type CollectiveId<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::FullCodec;
	use core::ops::AddAssign;

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_math::SafeAdd;
	use sp_runtime::traits::{MaybeDisplay, One, Zero};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type CollectiveId: AddAssign
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
	}

	#[pallet::storage]
	#[pallet::getter(fn collective_count)]
	pub type CollectiveCount<T: Config> = StorageValue<_, T::CollectiveId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collectives)]
	pub type Collectives<T: Config> = StorageMap<
		_,
		Twox64Concat,
		T::CollectiveId,
		(BTreeSet<T::AccountId>, ConvictionType, T::AccountId),
		OptionQuery,
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MembersAdded { collective: T::CollectiveId, members: Vec<T::AccountId> },
		CollectiveCreated { collective_id: T::CollectiveId, admin: T::AccountId },
	}

	#[pallet::error]
	pub enum Error<T> {
		NoNewMembersToAdd,
		NotAmin,
		CollectiveNotFound,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn create_collective(
			origin: OriginFor<T>,
			members: BTreeSet<T::AccountId>,
			conviction: ConvictionType,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let id = CollectiveCount::<T>::try_mutate(
				|collective_count| -> Result<T::CollectiveId, DispatchError> {
					*collective_count = collective_count.safe_add(&T::CollectiveId::one())?;

					Collectives::<T>::insert(
						collective_count.clone(),
						(members, conviction, who.clone()),
					);

					Ok(*collective_count)
				},
			)?;

			Self::deposit_event(Event::<T>::CollectiveCreated { collective_id: id, admin: who });

			Ok(().into())
		}

		#[pallet::weight(100_000)]
		pub fn add_members(
			origin: OriginFor<T>,
			collective_id: T::CollectiveId,
			members: BTreeSet<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let mut collective = Collectives::<T>::try_get(&collective_id)
				.map_err(|_| Error::<T>::CollectiveNotFound)?;

			ensure!(who == collective.2, Error::<T>::NotAmin);

			let new_members: Vec<_> = members.difference(&collective.0).cloned().collect();

			if collective.0.len() > 0 {
				ensure!(new_members.len() > 0, Error::<T>::NoNewMembersToAdd);
			}

			let mut m = BTreeSet::from_iter(new_members.clone());

			collective.0.append(&mut m);

			<Collectives<T>>::insert(&collective_id, &collective);

			Self::deposit_event(Event::<T>::MembersAdded {
				collective: collective_id,
				members: new_members,
			});

			Ok(().into())
		}
	}
}

impl<T: Config> CollectiveAuthorize<T::AccountId> for Pallet<T> {
	type CollectiveId = T::CollectiveId;

	fn is_admin(account_id: T::AccountId, collective_id: Self::CollectiveId) -> bool {
		if let Some(collective) = Self::collectives(collective_id) {
			collective.2 == account_id
		} else {
			false
		}
	}

	fn is_member(account_id: T::AccountId, collective_id: Self::CollectiveId) -> bool {
		if let Some(collective) = Self::collectives(collective_id) {
			collective.0.contains(&account_id)
		} else {
			false
		}
	}
}

impl<T: Config> CollectiveInspect for Pallet<T> {
	type CollectiveId = T::CollectiveId;

	fn exists(collective_id: Self::CollectiveId) -> bool {
		if let Some(collective) = Self::collectives(collective_id) {
			true
		} else {
			false
		}
	}
}
