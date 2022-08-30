// use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::ReservableCurrency};
use proposal_types::models::Target;

#[test]
fn can_start_new_referendum() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Referendum::start_referendum_by_value(Origin::signed(1), 1),
			Error::<Test>::CouldNotRetrieveProposal
		);
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test".as_bytes().to_vec(),
			10_u64,
			Target::Council(vec![1, 2, 3])
		));
		assert_ok!(Referendum::start_referendum_by_value(Origin::signed(1), 1));
		assert_eq!(Referendum::proposals(1).unwrap().content, "test".as_bytes().to_vec());
	});
}
