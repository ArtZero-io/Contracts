use crate::{
    impls::admin::{
        data::*,
    },
    traits::admin::*,
};
use crate::traits::error::Error;
use openbrush::{
    traits::{
        Storage,
        Balance
    }
};
impl<T: Storage<Data>> ArtZeroAdminTrait for T {

    default fn withdraw_fee(&mut self, value: Balance) -> Result<(), Error> {
        if value > T::env().balance() {
            return Err(Error::NotEnoughBalance)
        }
        if T::env().transfer(T::env().caller(), value).is_err() {
            return Err(Error::WithdrawFeeError)
        }
        Ok(())
    }

}
