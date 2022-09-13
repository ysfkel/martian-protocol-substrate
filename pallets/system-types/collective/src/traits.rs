pub trait CollectiveAuthorize<AccountId> {
	type CollectiveId;
	fn is_admin(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
	fn is_member(account_id: AccountId, collective_id: Self::CollectiveId) -> bool;
}
