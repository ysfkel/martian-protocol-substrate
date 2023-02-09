use crate::{mock::*, Error};
use collective_types::models::ConvictionType;
use frame_support::{assert_noop, assert_ok};
use std::collections::BTreeSet;

#[test]
fn can_add_collective() {
	new_test_ext().execute_with(|| {
		assert_ok!(Collective::create_collective(
			Origin::signed(1),
			BTreeSet::from_iter(vec![1, 2, 3]),
			ConvictionType::Basic,
		));

		assert_ok!(Collective::add_members(
			Origin::signed(1),
			1,
			BTreeSet::from_iter(vec![4, 5, 6])
		));
		assert_ok!(Collective::add_members(
			Origin::signed(1),
			1,
			BTreeSet::from_iter(vec![7, 8, 9])
		));
		assert_ok!(Collective::add_members(
			Origin::signed(1),
			1,
			BTreeSet::from_iter(vec![1, 2, 3, 5, 5, 10])
		));

		assert_eq!(
			Collective::collectives(1),
			Some((
				BTreeSet::from_iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
				ConvictionType::Basic,
				1
			))
		);
	})
}
