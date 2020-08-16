use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};
use sp_std::vec::Vec;


pub trait Bounty {
    type Id;
    type Location;
    type Status;
}

pub trait Bounties {
    fn total_bounties() -> u128;
}