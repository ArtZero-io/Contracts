use ink_prelude::{
    string::{
        String,
    },
    vec::Vec,
};
use openbrush::{
    contracts::ownable::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*
    },
    traits::{
        AccountId
    }
};

#[openbrush::wrapper]
pub type Psp34Ref = dyn PSP34 + PSP34Metadata;

#[openbrush::trait_definition]
pub trait Psp34Traits: PSP34 + PSP34Metadata {
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
    #[ink(message)]
    fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>
    ) -> Result<(), Error>;
    #[ink(message)]
    fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
    #[ink(message)]
    fn get_attribute_count(&self) -> u32;
    #[ink(message)]
    fn get_attribute_name(&self, index: u32) -> String;
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> String;
    #[ink(message)]
    fn get_owner(&self) -> AccountId ;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
    NotOwner,
}

impl From<OwnableError> for Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}
