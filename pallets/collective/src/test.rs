use std::collections::BTreeSet;

use crate::mock::*;
use frame_support::{assert_noop, assert_ok};

#[test]
fn can_add_collective() {
	new_test_ext().execute_with(|| {
		assert_ok!(Collective::add_members(Origin::signed(1), BTreeSet::from_iter(vec![1, 2, 3])));
		assert_ok!(Collective::add_members(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3, 5, 5])
		));
		assert_ok!(Collective::add_members(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3, 5, 5, 6])
		));

		assert_eq!(Collective::collectives(1), BTreeSet::from_iter(vec![1, 2, 3, 5, 6]));
	})
}
