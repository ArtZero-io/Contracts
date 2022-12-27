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
        assert!(value <= T::env().balance());
        if T::env().transfer(T::env().caller(), value).is_err() {
            panic!("error withdraw_fee")
        }
        Ok(())
    }

}
