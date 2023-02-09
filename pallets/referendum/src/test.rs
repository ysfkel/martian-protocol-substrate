// use super::*;
use crate::{mock::*, Error};
use collective_types::models::ConvictionType;
use frame_support::{assert_noop, assert_ok, traits::ReservableCurrency};
use referendum_types::{AccountVote, Conviction, Vote};
use sp_std::{collections::btree_set::BTreeSet, prelude::*};

const AYE: Vote = Vote { aye: true, conviction: Conviction::None };
const NAY: Vote = Vote { aye: false, conviction: Conviction::None };

fn aye(who: u64) -> AccountVote<u64> {
	AccountVote::Standard { vote: AYE, balance: Balances::free_balance(&who) }
}

fn nay(who: u64) -> AccountVote<u64> {
	AccountVote::Standard { vote: NAY, balance: Balances::free_balance(&who) }
}

#[test]
fn can_vote() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Referendum::vote(Origin::signed(2), 1, 1, aye(1)),
			Error::<Test>::CollectiveNotFound
		);

		assert_ok!(CollectivePallet::create_collective(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1]),
			ConvictionType::Basic,
		));

		assert_noop!(
			Referendum::vote(Origin::signed(2), 1, 1, aye(1)),
			Error::<Test>::NotCollectiveMember
		);

		assert_ok!(CollectivePallet::add_members(
			Origin::signed(1),
			1,
			BTreeSet::from_iter(vec![2, 3, 4, 5])
		));

		assert_noop!(
			Referendum::vote(Origin::signed(1), 1, 1, aye(1)),
			Error::<Test>::ReferendumNotFound
		);

		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test".as_bytes().to_vec(),
			10_u64,
		));

		assert_ok!(Referendum::start_referendum_by_value(Origin::signed(1), 1, 2_u64));

		assert_ok!(Referendum::vote(Origin::signed(1), 1, 1, aye(1)));
		//	System::set_block_number(System::block_number() + 1);
		System::set_block_number(System::block_number() + 10);
		let mut bn = System::block_number();
		dbg!("bloc number {} ", bn);
		bn = System::block_number() + 10;
		dbg!("block number {} ", bn);
		assert_ok!(Referendum::vote(Origin::signed(2), 1, 1, aye(1)));
		assert_ok!(Referendum::vote(Origin::signed(3), 1, 1, aye(1)));
		assert_ok!(Referendum::vote(Origin::signed(4), 1, 1, aye(1)));
		assert_ok!(Referendum::vote(Origin::signed(5), 1, 1, nay(1)));
		// end vote and determine if votes pass
	})
}

#[test]
fn can_start_new_referendum() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			Referendum::start_referendum_by_value(Origin::signed(1), 1, 20_u64),
			Error::<Test>::CollectiveNotFound
		);

		assert_ok!(CollectivePallet::create_collective(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3]),
			ConvictionType::Basic,
		));

		assert_noop!(
			Referendum::start_referendum_by_value(Origin::signed(2), 1, 20_u64),
			Error::<Test>::NotCollectiveAdmin
		);

		assert_noop!(
			Referendum::start_referendum_by_value(Origin::signed(1), 1, 20_u64),
			Error::<Test>::CouldNotRetrieveProposal
		);
		assert_ok!(Proposal::create_proposal(
			Origin::signed(1),
			1,
			"test".as_bytes().to_vec(),
			10_u64,
		));
		assert_ok!(Referendum::start_referendum_by_value(Origin::signed(1), 1, 20_u64));
		assert_eq!(Referendum::proposals(1, 1).unwrap().content, "test".as_bytes().to_vec());
	});
}
