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

// pub mod bounties;
// pub use crate::bounties::Bounties;

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
type BalanceOf<T> = <<T as Trait>::Currency as Currency<AccountIdOf<T>>>::Balance;
type BountyInfoOf<T> = Bounty<AccountIdOf<T>, BalanceOf<T>>;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
 pub struct Bounty <AccountId, Balance>{
    issuers: AccountId,
    Approvers: AccountId,
    Deadline: u128,
    Balance: Balance,
    HasPaidOut: bool,
}

decl_storage!{
    trait Store for Module<T: Trait> as BountyPallet {
        TotalBounties get(fn total_bounties): u128 = 0;
        BountiesMap: map hasher(blake2_128_concat) u128 => Option<BountyInfoOf<T>>; 
    }

}

// decl_event!{
//     pub enum Event<T> where
//     {

//     }


// }

decl_error!{
    pub enum Error for Module<T: Trait> {
        BalanceZero
    }


}

decl_module!{
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // #[weight = 0]
        // fn issue_bounty(origin) -> dispatch::DispatchResult {
        //     Self::issue_bounty(origin);
        //     Ok(())


        // }
		#[weight = 10_000]
        pub fn issue_bounty(origin, amount: <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance,
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            Self::slash_value(&who, &amount);
            Self::issue(&who, &amount);
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn bounties_list(id: u128) -> Option<BountyInfoOf<T>> {
        <BountiesMap<T>>::get(id)
    }

    fn issue(who: &T::AccountId, amount: &<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance) -> dispatch::DispatchResult {    
            let id = TotalBounties::get();
            TotalBounties::mutate(|total| *total += 1);
            
            let new_bounty = Bounty {
                issuers: who.clone(),
                Approvers: who.clone(), 
                Deadline: 0,
                Balance: *amount,
                HasPaidOut: false
            };
    
            <BountiesMap<T>>::insert(id, new_bounty);
            Ok(())
    }

    fn slash_value(who: &T::AccountId, amount: &<<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance) -> dispatch::DispatchResult {    
        T::Currency::slash(&who, *amount);
        Ok(())
    }
}
