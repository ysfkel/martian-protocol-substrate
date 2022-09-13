pub trait CollectiveInspect<AccountId> {
	type CollectiveId;
	fn is_admin(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
	fn contains(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
}
