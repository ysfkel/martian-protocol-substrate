#[cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
pub use sp_std::prelude::*;
use sp_std::{collections::btree_set::BTreeSet, fmt::Debug, iter::FromIterator, prelude::*};

#[cfg(test)]
mod mock;
#[cfg(test)]
mod test;
//use frame_support::traits::{defensive_prelude::*, Currency, LockableCurrency,
// ReservableCurrency};
pub type CouncilId<T> = <T as frame_system::Config>::AccountId;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use codec::FullCodec;
	use council_types::models::Council;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::MaybeDisplay;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
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
	#[pallet::getter(fn councils)]
	pub type Councils<T: Config> =
		StorageMap<_, Twox64Concat, CouncilId<T>, BTreeSet<T::AccountId>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MembersAdded { council: CouncilId<T>, members: Vec<T::AccountId> },
	}
	#[pallet::error]
	pub enum Error<T> {
		NoMembersToAdd,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(100_000)]
		pub fn add_members(
			origin: OriginFor<T>,
			members: BTreeSet<T::AccountId>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let mut council = Self::councils(&who);

			let new_members: Vec<_> = members.difference(&council).cloned().collect();

			if council.len() > 0 {
				ensure!(new_members.len() > 0, Error::<T>::NoMembersToAdd);
			}

			let mut m = BTreeSet::from_iter(new_members.clone());

			council.append(&mut m);

			<Councils<T>>::insert(&who, &council);

			Self::deposit_event(Event::<T>::MembersAdded { council: who, members: new_members });

			Ok(().into())
		}
	}
}
