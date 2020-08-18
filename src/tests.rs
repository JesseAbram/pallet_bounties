use crate::mock::*;
// use crate::bounties::Bounties;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};
use sp_core::H256;


#[test]
fn create_bounty() {
    new_test_ext().execute_with(|| {
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
        assert_ok!(Bounties_Pallet::issue_bounty(Origin::signed(1), 0, 0));
        assert_eq!(Bounties_Pallet::total_bounties(), 1);
        let bounty_for_account = Bounties_Pallet::bounties_list(0);
        let mock_bounty = Bounty {
            issuer: 1, 
            deadline: 0, 
            balance: 0, 
            has_paid_out: false
        };
        
        assert_eq!(bounty_for_account, Some(mock_bounty));

    });
}

#[test]
fn gracefully_fail_create_bounty_invalid_balance() {
    new_test_ext().execute_with(|| {
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
        assert_err!(Bounties_Pallet::issue_bounty(Origin::signed(1), 1000000, 0),   Error::<Test>::Slashing);
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
    });
}
#[test]   
fn approve_submission() {
new_test_ext().execute_with(|| {
    hydrate_bounty();
        assert_ok!(Bounties_Pallet::approve_submission(Origin::signed(1), 0, 1));
        let bounty_for_account = Bounties_Pallet::bounties_list(0);
        let mock_bounty = Bounty {
            issuer: 1, 
            deadline: 0, 
            balance: 0, 
            has_paid_out: true
        };
        
        assert_eq!(bounty_for_account, Some(mock_bounty));
        assert_err!(Bounties_Pallet::approve_submission(Origin::signed(1), 0, 1),   Error::<Test>::AlreadyPaidOut);


    });
}
#[test]   
fn approve_submission_fail_bounty_non_valid() {
new_test_ext().execute_with(|| {
    hydrate_bounty();
        assert_err!(Bounties_Pallet::approve_submission(Origin::signed(1), 2, 1), Error::<Test>::InvalidBounty);
    });
}

#[test]   
fn contribute_to_bounty() {
    new_test_ext().execute_with(|| {
    hydrate_bounty();
    assert_ok!(Bounties_Pallet::contribute(Origin::signed(1), 0, 0));
    })
}

#[test]   
fn fail_to_contribute_to_bounty_after_deadline() {
    new_test_ext().execute_with(|| {
    hydrate_bounty();
    System::set_block_number(System::block_number() + 1);
    assert_err!(Bounties_Pallet::contribute(Origin::signed(1), 0, 0), Error::<Test>::PassedDeadline);
    assert_err!(Bounties_Pallet::contribute(Origin::signed(1), 2, 0), Error::<Test>::InvalidBounty);
    })
}

#[test]   
fn fail_to_contribute_to_bounty_not_enough_funds() {
    new_test_ext().execute_with(|| {
    hydrate_bounty();
    assert_err!(Bounties_Pallet::contribute(Origin::signed(1), 0, 10), Error::<Test>::Slashing);
    })
}


fn hydrate_bounty() {
    assert_ok!(Bounties_Pallet::issue_bounty(Origin::signed(1), 0, 0));
    assert_eq!(Bounties_Pallet::total_bounties(), 1);

}