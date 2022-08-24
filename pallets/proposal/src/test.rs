// use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, traits::ReservableCurrency};
use proposal_types::models::Target;

#[test]
fn can_make_a_proposal() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_eq!(Proposal::proposal_count(), 0_u128);
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			"test1".as_bytes().to_vec(),
			10_u64,
			Target::Council(vec![1, 3, 4]),
		));
		assert_ok!(Proposal::create_proposal(
			Origin::signed(2),
			"test2".as_bytes().to_vec(),
			10_u64,
			Target::Council(vec![1, 3, 4]),
		));
		assert_ok!(Proposal::create_proposal(
			Origin::signed(3),
			"test3".as_bytes().to_vec(),
			10_u64,
			Target::Council(vec![1, 3, 4]),
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(Proposal::proposal_count(), 3_u128);
		assert_eq!(Proposal::proposal(1).unwrap().content, "test1".as_bytes().to_vec());
		assert_eq!(Proposal::proposal(2).unwrap().content, "test2".as_bytes().to_vec());
		assert_eq!(Proposal::proposal(3).unwrap().content, "test3".as_bytes().to_vec());

		assert_eq!(Proposal::proposal_index().len(), 3);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&1), 10);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&2), 10);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&3), 10);
	});
}

fn can_endorse_a_proposal() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let proposal_index = 1;
		assert_eq!(Proposal::proposal_count(), 0_u128);
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			"test1".as_bytes().to_vec(),
			10_u64,
			Target::Council(vec![1, 3, 4]),
		));
		assert_ok!(Proposal::endorse(Origin::signed(2), 1));

		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&3), 10);
		let deposit = Proposal::deposit_of(proposal_index).unwrap();
		assert_eq!(deposit.0.len(), 2);
	});
}
