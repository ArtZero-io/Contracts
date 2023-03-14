use crate::{
    impls::upgradable::{
        data
    },
    traits::upgradable::*,
};
use crate::traits::error::Error;
use openbrush::{
    modifiers,
    traits::{
        Storage
    },
    contracts::{
        ownable::*,
    },
};

impl<T: Storage<data::Data> + Storage<ownable::Data>> UpgradableTrait for T
{
    #[modifiers(only_owner)]
    default fn set_code(&mut self, code_hash: [u8; 32]) -> Result<(), Error> {
        ink::env::set_code_hash(&code_hash).unwrap_or_else(|err| {
            panic!(
                "Failed to `set_code_hash` to {:?} due to {:?}",
                code_hash, err
            )
        });
        ink::env::debug_println!("Switched code hash to {:?}.", code_hash);
        Ok(())
    }
}
