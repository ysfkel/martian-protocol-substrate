// use super::*;
use crate::{mock::*, Error, ProposalIndex};
use collective_types::models::ConvictionType;
use frame_support::{assert_noop, assert_ok, traits::ReservableCurrency};
use proposal_types::traits::ProposalInspect;
use std::collections::BTreeSet;

#[test]
fn can_make_a_proposal() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_eq!(Proposal::proposal_count(1), 0_u128);
		assert_ok!(CollectivePallet::create_collective(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3]),
			ConvictionType::Basic,
		));

		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test1".as_bytes().to_vec(),
			10_u64,
		));

		assert_ok!(Proposal::create_proposal(
			Origin::signed(2),
			1,
			"test2".as_bytes().to_vec(),
			10_u64,
		));
		assert_ok!(Proposal::create_proposal(
			Origin::signed(3),
			1,
			"test3".as_bytes().to_vec(),
			10_u64,
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(Proposal::proposal_count(1), 3_u128);
		assert_eq!(Proposal::proposals(1, 1).unwrap().content, "test1".as_bytes().to_vec());
		assert_eq!(Proposal::proposals(1, 2).unwrap().content, "test2".as_bytes().to_vec());
		assert_eq!(Proposal::proposals(1, 3).unwrap().content, "test3".as_bytes().to_vec());

		assert_eq!(Proposal::proposal_index(1).len(), 3);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&1), 10);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&2), 10);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&3), 10);
	});
}

#[test]
fn can_endorse_a_proposal() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		let proposal_index = 1;
		assert_eq!(Proposal::proposal_count(1), 0_u128);
		assert_ok!(CollectivePallet::create_collective(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3]),
			ConvictionType::Basic,
		));
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test1".as_bytes().to_vec(),
			10_u64,
		));
		assert_ok!(Proposal::endorse(Origin::signed(2), 1, 1));

		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&2), 10);
		let deposit = Proposal::deposit_of(1, proposal_index).unwrap();
		assert_eq!(deposit.0.len(), 2);

		<Proposal as ProposalInspect>::retrieve_highest_valued_proposal(1);

		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&2), 0);
		assert_eq!(<Balances as ReservableCurrency<_>>::reserved_balance(&1), 0);
		assert_eq!(Proposal::deposit_of(1, 1), None);
		assert_eq!(Proposal::proposals(1, 1), None);
		assert_eq!(Proposal::proposal_index(1), vec![]);
	});
}
