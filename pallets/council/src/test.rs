use std::collections::BTreeSet;

use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_add_council() {
	new_test_ext().execute_with(|| {
		assert_ok!(Council::add_members(Origin::signed(1), BTreeSet::from_iter(vec![1, 2, 3])));
		assert_ok!(Council::add_members(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3, 5, 5])
		));
		assert_ok!(Council::add_members(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3, 5, 5, 6])
		));

		assert_eq!(Council::councils(1), BTreeSet::from_iter(vec![1, 2, 3, 5, 6]));
	})
}
