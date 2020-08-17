#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, FullCodec};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, Parameter,
    traits::{EnsureOrigin, Get, Currency},
    Hashable
};
use frame_system::{self as system, ensure_signed, Event};
use sp_runtime::{
    traits::{Hash, Member, AtLeast32Bit, Scale},
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
type BountyInfoOf<T> = Bounty<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber>;

#[derive(Encode, Decode, Default, Debug, PartialEq)]
 pub struct Bounty <AccountId, Balance, BlockNumber>{
    issuer: AccountId,
    Deadline: BlockNumber,
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
        BalanceZero,
        Slashing
    }


}

decl_module!{
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		#[weight = 10_000]
        pub fn issue_bounty(origin, amount: BalanceOf<T>, block_number: T::BlockNumber
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            let result = Self::slash_value(&who, &amount);
            ensure! (
                result, 
                Error::<T>::Slashing
            );
            Self::issue(&who, &amount, &block_number);
            Ok(())
        }
        #[weight = 10_000]
        fn approve_submission(origin, id: u128, who: T::AccountId) -> dispatch::DispatchResult {
            ensure_signed(origin)?;
            Self::submission(id, &who);
            Ok(())
        }
}
}

impl<T: Trait> Module<T> {
    pub fn bounties_list(id: u128) -> Option<BountyInfoOf<T>> {
        <BountiesMap<T>>::get(id)
    }

    fn issue(who: &T::AccountId, amount: &BalanceOf<T>, block_number: &T::BlockNumber) {    
            let id = TotalBounties::get();
            TotalBounties::mutate(|total| *total += 1);
            
            let new_bounty = Bounty {
                issuer: who.clone(),
                Deadline: *block_number,
                Balance: *amount,
                HasPaidOut: false
            };
    
            <BountiesMap<T>>::insert(id, new_bounty);
    }

    fn submission(id: u128, who: &T::AccountId) {
        // TODO, only owner, before deadline, payout claimer
        // Remove unwrap handle error
        let mut target_bounty: Bounty<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber> = Self::bounties_list(id).unwrap();
        target_bounty.HasPaidOut = true;
        <BountiesMap<T>>::insert(id, target_bounty);
    }

    fn slash_value(who: &T::AccountId, amount: &BalanceOf<T>) -> bool {    
        if !T::Currency::can_slash(&who, *amount) {
            false
        } else {
            T::Currency::slash(&who, *amount);
            true
        }
    }
}
