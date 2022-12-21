use openbrush::{
    contracts::access_control::*,
    contracts::ownable::*,
    modifiers,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        AccountId,
        Hash,
        Balance
    }
};
use ink_prelude::{
    vec,
    string::String,
    vec::Vec,
};

#[openbrush::trait_definition]
pub trait ArtZeroCollectionTrait {
    #[ink(message)]
    fn get_royal_fee(&self, nft_contract_address: AccountId) -> u32;
    #[ink(message)]
    fn is_active(&self, nft_contract_address: AccountId) -> bool;
    #[ink(message)]
    fn get_contract_type(&self, nft_contract_address: AccountId) -> u8;
    #[ink(message)]
    fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId>;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
    CollectionOwnerAndAdmin,
    OnlyOwner,
    OnlyAdmin,
    InvalidCaller,
}

impl From<OwnableError> for Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}
