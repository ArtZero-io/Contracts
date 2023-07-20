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

pub trait AdminTraitImpl: 
    Storage<data::Data> + 
    Storage<ownable::Data>
{
    fn _emit_withdraw_fee(&self, _value: Balance, _receiver: AccountId) {}

    #[modifiers(only_owner)]
    fn withdraw_fee(&mut self, value: Balance, receiver: AccountId) -> Result<(), Error> {
        if value > Self::env().balance() {
            return Err(Error::NotEnoughBalance);
        }
        if Self::env().transfer(receiver, value).is_err() {
            return Err(Error::WithdrawFeeError);
        }
        self._emit_withdraw_fee(value, receiver);
        Ok(())
    }

    #[modifiers(only_owner)]
    fn tranfer_nft(&mut self, nft_contract_address: AccountId, token_id: Id, receiver: AccountId) -> Result<(), Error> {
        let builder = Psp34Ref::transfer_builder(
            &nft_contract_address,
            receiver,
            token_id.clone(),
            Vec::<u8>::new()
        )
        .call_flags(CallFlags::default().set_allow_reentry(true));

        let result = match builder.try_invoke() {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(e))) => Err(e.into()),
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            Err(ink::env::Error::NotCallable) => Ok(()),
            _ => {
                Err(Error::CannotTransfer)
            }
        };

        result
    }

    #[modifiers(only_owner)]
    fn tranfer_psp22(&mut self, psp22_contract_address: AccountId, amount: Balance, receiver: AccountId) -> Result<(), Error>{
        let builder = Psp22Ref::transfer_builder(
            &psp22_contract_address,
            receiver,
            amount,
            Vec::<u8>::new()
        )
        .call_flags(CallFlags::default().set_allow_reentry(true));

        let result = match builder.try_invoke() {
            Ok(Ok(Ok(_))) => Ok(()),
            Ok(Ok(Err(e))) => Err(e.into()),
            Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
            Err(ink::env::Error::NotCallable) => Ok(()),
            _ => {
                Err(Error::CannotTransfer)
            }
        };

        result
    }
}
