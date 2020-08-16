use crate::mock::*;
use crate::bounties::Bounties;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};
use sp_core::H256;

#[test]
fn total() {
    new_test_ext().execute_with(|| {
		assert_eq!(Bounties_Pallet::total_bounties(), 0);
	});

}