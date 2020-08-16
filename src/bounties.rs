use frame_support::{
    dispatch::{result::Result, DispatchError, DispatchResult},
    traits::Get,
};
use sp_std::vec::Vec;


// pub trait Bounty {
//     type Issuers;
//     // type Approvers;
//     // type Deadline; 
//     // type Balance;
//     // type HasPaidOut;
//     // type Fulfillments;
//     // type Contributions;
// }

pub trait Bounties {
    fn total_bounties() -> u128;
}