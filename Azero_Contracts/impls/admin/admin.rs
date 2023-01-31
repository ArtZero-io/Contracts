use crate::{
    impls::admin::{
        data
    },
    traits::admin::*,
};
use crate::traits::error::Error;
use ink::prelude::{
    vec::Vec,
};
use ink::env::CallFlags;
use openbrush::{
    modifiers,
    traits::{
        Storage,
        Balance,
        AccountId
    },
    contracts::{
        traits::psp34::{
            Id,
        },
        ownable::*,
    },
};
use crate::traits::psp34_standard::*;

impl<T: Storage<data::Data> + Storage<ownable::Data>> AdminTrait for T
{
    #[modifiers(only_owner)]
    default fn withdraw_fee(&mut self, value: Balance, receiver: AccountId) -> Result<(), Error> {
        if value > T::env().balance() {
            return Err(Error::NotEnoughBalance);
        }
        if T::env().transfer(receiver, value).is_err() {
            return Err(Error::WithdrawFeeError);
        }
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn tranfer_nft(&mut self, nft_contract_address: AccountId, token_id: Id, receiver: AccountId) -> Result<(), Error> {
        if Psp34Ref::transfer_builder(
            &nft_contract_address,
            receiver,
            token_id.clone(),
            Vec::<u8>::new()
        )
        .call_flags(CallFlags::default().set_allow_reentry(true))
        .fire()
        .is_err(){
            return Err(Error::WithdrawNFTError)
        }

        Ok(())
    }

    #[modifiers(only_owner)]
    default fn tranfer_psp22(&mut self, psp22_contract_address: AccountId, amount: Balance, receiver: AccountId) -> Result<(), Error>{
        if Psp22Ref::transfer_builder(
            &psp22_contract_address,
            receiver,
            amount,
            Vec::<u8>::new()
        )
        .call_flags(CallFlags::default().set_allow_reentry(true))
        .fire()
        .is_err(){
            return Err(Error::WithdrawPSP22Error);
        }
        Ok(())
    }
}
