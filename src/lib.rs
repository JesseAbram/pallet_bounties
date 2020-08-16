#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, FullCodec};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{EnsureOrigin, Get},
    Hashable,
};
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
pub use crate::bounties::{Bounties, Bounty};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

}

decl_storage!{
    trait Store for Module<T: Trait> as Bounties {
        TotalBounties get(fn total_bounties): u128 = 0;
        
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
    }
}

impl<T: Trait> Bounties for Module<T> {
    fn total_bounties() -> u128 {
        Self::total_bounties()
    }

    }
