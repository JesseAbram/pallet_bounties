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
    deadline: BlockNumber,
    balance: Balance,
    has_paid_out: bool,
}

decl_storage!{
    trait Store for Module<T: Trait> as BountyPallet {
        TotalBounties get(fn total_bounties): u128 = 0;
        BountiesMap: map hasher(blake2_128_concat) u128 => Option<BountyInfoOf<T>>; 
    }

}

// decl_event!{
//     pub enum Event<T> where		
//     <T as frame_system::Trait>::AccountId,
//     <T as Trait>::Balance,
//     <T as system::Trait>::BlockNumber>
//     {
//         Issued(AccountId, Balance, BlockNumber),
//     }
// }

decl_error!{
    pub enum Error for Module<T: Trait> {
        BalanceZero,
        Slashing,
        AlreadyPaidOut
    }

}

decl_module!{
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

		#[weight = 10_000]
        pub fn issue_bounty(origin, amount: BalanceOf<T>, block_number: T::BlockNumber
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            Self::slash_value(&who, &amount)?;
            Self::issue(&who, &amount, &block_number);
            Ok(())
        }
        #[weight = 10_000]
        fn approve_submission(origin, id: u128, who: T::AccountId) -> dispatch::DispatchResult {
            ensure_signed(origin)?;
            Self::submission(id, &who)?;
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
                deadline: *block_number,
                balance: *amount,
                has_paid_out: false
            };
    
            <BountiesMap<T>>::insert(id, new_bounty);
    }

    fn submission(id: u128, who: &T::AccountId) -> dispatch::DispatchResult {
        // TODO, only owner, before deadline, payout claimer
        // Remove unwrap handle error
        let mut target_bounty: Bounty<AccountIdOf<T>, BalanceOf<T>, <T as system::Trait>::BlockNumber> = Self::bounties_list(id).unwrap();

        ensure!(
            !target_bounty.has_paid_out,
            Error::<T>::AlreadyPaidOut
        );

        target_bounty.has_paid_out = true;
        T::Currency::deposit_into_existing(who, target_bounty.balance)?;
        <BountiesMap<T>>::insert(id, target_bounty);
        Ok(())
    }


    fn slash_value(who: &T::AccountId, amount: &BalanceOf<T>) -> dispatch::DispatchResult {    
        ensure! (
            T::Currency::can_slash(&who, *amount), 
            Error::<T>::Slashing
        );
            T::Currency::slash(&who, *amount);
            Ok(())
    }
}
