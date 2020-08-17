use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};
use sp_std::vec::Vec;


pub trait Bounties {
    type AccountId;


    fn total_bounties() -> u128;

    fn bounties_list() -> Bounties;

    fn issue_bounty(who: Self::AccountId)  -> DispatchResult;
}