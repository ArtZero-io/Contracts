use openbrush::{
    modifiers,
    traits::{
        Balance
    }
};
use crate::traits::error::Error;

#[openbrush::wrapper]
pub type ArtZeroAdminRef = dyn ArtZeroAdminTrait;

#[openbrush::trait_definition]
pub trait ArtZeroAdminTrait {
    /// This function allows owner to withdraw contract balance to his account. 
    #[ink(message)]
    #[modifiers(only_owner)]
    fn withdraw_fee(&mut self, value: Balance) -> Result<(), Error>;
}
