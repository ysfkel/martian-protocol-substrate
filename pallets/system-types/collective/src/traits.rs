// use sp_runtime::DispatchError;
use frame_support::pallet_prelude::*;

pub trait CollectiveAuthorize<AccountId> {
	type CollectiveId;
	fn is_admin(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
	fn is_member(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
}

pub trait CollectiveInspect {
	type CollectiveId;
	fn exists(collective_id: Self::CollectiveId) -> bool;
}
