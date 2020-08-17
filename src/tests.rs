use crate::mock::*;
// use crate::bounties::Bounties;
use crate::*;
use frame_support::{assert_err, assert_ok, Hashable};
use sp_core::H256;


#[test]
fn total() {
    new_test_ext().execute_with(|| {
        assert_eq!(Bounties_Pallet::total_bounties(), 0);
        assert_ok!(Bounties_Pallet::issue_bounty(Origin::signed(1)));
        assert_eq!(Bounties_Pallet::total_bounties(), 1);
        let bounty_for_account = Bounties_Pallet::bounties_list(0);
        let mock_bounty = Bounty {
            issuers: 1, 
            Approvers: 1, 
            Deadline: 0, 
            Balance: 0, 
            HasPaidOut: false
        };
        
        assert_eq!(bounty_for_account, mock_bounty);

	});

}