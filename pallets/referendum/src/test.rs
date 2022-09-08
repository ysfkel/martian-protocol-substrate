// use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::ReservableCurrency};
use sp_std::{collections::btree_set::BTreeSet, prelude::*};

#[test]
fn can_start_new_referendum() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Referendum::start_referendum_by_value(Origin::signed(1), 1, 20_u64),
			Error::<Test>::CouldNotRetrieveProposal
		);
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test".as_bytes().to_vec(),
			10_u64,
			BTreeSet::from_iter(vec![1, 2, 3]),
		));
		assert_ok!(Referendum::start_referendum_by_value(Origin::signed(1), 1, 20_u64));
		assert_eq!(Referendum::proposals(1).unwrap().content, "test".as_bytes().to_vec());
	});
}
