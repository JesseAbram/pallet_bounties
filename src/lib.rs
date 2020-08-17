#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, FullCodec};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{EnsureOrigin, Get, Currency},
    Hashable
};
// use sp-timestamp::;
use frame_system::{self as system, ensure_signed, Event};
use sp_runtime::{
    traits::{Hash, Member},
    RuntimeDebug,
};
use sp_std::{
    cmp::{Eq, Ordering},
    fmt::Debug,
    vec::Vec,
};

pub mod bounties;
pub use crate::bounties::Bounties;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    
    type MaxIssuers: Get<u32>;
    type Currency: Currency<Self::AccountId>;

}

type AccountIdOf<T> = <T as system::Trait>::AccountId;
type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;
type BountyInfoOf<T> = Bounty<AccountIdOf<T>>;

#[derive(Encode, Decode, Default)]
 pub struct Bounty <AccountId>{
    issuers: AccountId,
    Approvers: AccountId,
    Deadline: u128,
    Balance: u128,
    HasPaidOut: bool,
}

decl_storage!{
    trait Store for Module<T: Trait> as BountyPallet {
        TotalBounties get(fn total_bounties): u128 = 0;
        BountiesMap: map hasher(blake2_128_concat) u128 => BountyInfoOf<T>; 
    }

}

// decl_event!{
//     pub enum Event<T> where
//     {

//     }


// }

// decl_error!{
//     pub enum Error for Module<T: Trait> {

//     }


// }

decl_module!{
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        #[weight = 0]
        fn issue_bounty(origin) -> dispatch::DispatchResult {
            Self::issue_bounty(origin);
            Ok(())


        }
    }
}

impl<T: Trait> Bounties for Module<T> {
    type AccountId = <T as system::Trait>::AccountId;

    fn total_bounties() -> u128 {
        Self::total_bounties()
    }
    fn issue_bounty(who: T::AccountId) -> dispatch::DispatchResult {
        Ok(())
    }

    }
