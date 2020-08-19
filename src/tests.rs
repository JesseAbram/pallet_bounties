use crate::mock::*;
// use crate::bounties::Bounties;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};
use sp_core::H256;
use system::{EventRecord, Phase};

#[test]
fn create_bounty() {
    ExtBuilder::build().execute_with(|| {
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
        assert_ok!(Bounties_Pallet::issue_bounty(Origin::signed(1), 0, 1));
        let expected_event = TestEvent::BountyPallet(RawEvent::Issued(0, 1, 0, 1));
        assert_eq!(
            System::events(),
            vec![EventRecord {
                phase: Phase::Initialization,
                event: expected_event,
                topics: vec![],
            }]
        );

        assert_eq!(Bounties_Pallet::total_bounties(), 1);
        let bounty_for_account = Bounties_Pallet::bounties_list(0);
        let mock_bounty = Bounty {
            issuer: 1,
            deadline: 1,
            balance: 0,
            has_paid_out: false,
        };

        assert_eq!(bounty_for_account, Some(mock_bounty));
    });
}

#[test]
fn gracefully_fail_create_bounty_invalid_balance() {
    ExtBuilder::build().execute_with(|| {
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
        assert_err!(
            Bounties_Pallet::issue_bounty(Origin::signed(1), 1000000, 1),
            Error::<Test>::Slashing
        );
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
    });
}
#[test]
fn approve_submission() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        assert_ok!(Bounties_Pallet::approve_submission(Origin::signed(1), 0, 1));
        let expected_event = TestEvent::BountyPallet(RawEvent::ApprovedSubmission(0, 1));
        let events = System::events();
        assert_eq!(
            events[1],
            EventRecord {
                phase: Phase::Initialization,
                event: expected_event,
                topics: vec![],
            }
        );

        let bounty_for_account = Bounties_Pallet::bounties_list(0);
        let mock_bounty = Bounty {
            issuer: 1,
            deadline: 1,
            balance: 0,
            has_paid_out: true,
        };

        assert_eq!(bounty_for_account, Some(mock_bounty));

        assert_err!(
            Bounties_Pallet::approve_submission(Origin::signed(1), 0, 1),
            Error::<Test>::AlreadyPaidOut
        );
    });
}
#[test]
fn approve_submission_fail() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        assert_err!(
            Bounties_Pallet::approve_submission(Origin::signed(1), 2, 1),
            Error::<Test>::InvalidBounty
        );
        assert_err!(
            Bounties_Pallet::approve_submission(Origin::signed(2), 0, 1),
            Error::<Test>::OnlyIssuer
        );
        System::set_block_number(System::block_number() + 1);
        assert_err!(
            Bounties_Pallet::approve_submission(Origin::signed(1), 0, 1),
            Error::<Test>::PassedDeadline
        );
    });
}

#[test]
fn contribute_to_bounty() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        assert_ok!(Bounties_Pallet::contribute(Origin::signed(1), 0, 0));
        let expected_event = TestEvent::BountyPallet(RawEvent::Contributed(0, 0));
        let events = System::events();
        assert_eq!(
            events[1],
            EventRecord {
                phase: Phase::Initialization,
                event: expected_event,
                topics: vec![],
            }
        );
    })
}

#[test]
fn fail_to_contribute_to_bounty_after_deadline() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        System::set_block_number(System::block_number() + 1);
        assert_err!(
            Bounties_Pallet::contribute(Origin::signed(1), 0, 0),
            Error::<Test>::PassedDeadline
        );
        assert_err!(
            Bounties_Pallet::contribute(Origin::signed(1), 2, 0),
            Error::<Test>::InvalidBounty
        );
    })
}

#[test]
fn fail_to_contribute_to_bounty_not_enough_funds() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        assert_err!(
            Bounties_Pallet::contribute(Origin::signed(1), 0, 10),
            Error::<Test>::Slashing
        );
    })
}

#[test]
fn reclaim_deposit() {
    ExtBuilder::build().execute_with(|| {
        hydrate_bounty();
        assert_err!(
            Bounties_Pallet::reclaim_deposit(Origin::signed(1), 0),
            Error::<Test>::StillActive
        );
        System::set_block_number(System::block_number() + 1);
        assert_err!(
            Bounties_Pallet::reclaim_deposit(Origin::signed(1), 2),
            Error::<Test>::InvalidBounty
        );
        assert_err!(
            Bounties_Pallet::reclaim_deposit(Origin::signed(2), 0),
            Error::<Test>::OnlyIssuer
        );
        assert_ok!(Bounties_Pallet::reclaim_deposit(Origin::signed(1), 0));
        let expected_event = TestEvent::BountyPallet(RawEvent::ReclaimedDeposit(0));
        let events = System::events();
        assert_eq!(
            events[1],
            EventRecord {
                phase: Phase::Initialization,
                event: expected_event,
                topics: vec![],
            }
        );
    })
}

fn hydrate_bounty() {
    assert_ok!(Bounties_Pallet::issue_bounty(Origin::signed(1), 0, 1));
    assert_eq!(Bounties_Pallet::total_bounties(), 1);
}
