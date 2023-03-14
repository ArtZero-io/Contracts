use openbrush::{
    traits::{
        AccountId
    }
};
use crate::traits::error::Error;
use ink::prelude::{
    string::{
        String,
    },
    vec,
    vec::Vec,
};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub enum CollectionType {
    Unknown,
    Psp34Manual,
    Psp34Auto,
    Psp1155Manual,
    Psp1155Auto,
    Reserved1,
    Reserved2,
}

#[openbrush::wrapper]
pub type ArtZeroCollectionRef = dyn ArtZeroCollectionTrait;

#[openbrush::trait_definition]
pub trait ArtZeroCollectionTrait {
    /// This function returns royalty fee of a Collection
    #[ink(message)]
    fn get_royalty_fee(&self, nft_contract_address: AccountId) -> u32;
    /// This function checks if the Collection is active or not. Only Active Collections can be seen on the Marketplace
    #[ink(message)]
    fn is_active(&self, nft_contract_address: AccountId) -> bool;
    /// This function returns NFT Contract Type. When the collection is created using auto_new_collection, this set to 2 and when using add_new_collection, this set to 1. Contract Type is to identify if the contract is standard or customized one.
    #[ink(message)]
    fn get_contract_type(&self, nft_contract_address: AccountId) -> CollectionType;
    /// This function returns the Owner of a Collection
    #[ink(message)]
    fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId>;
    /// This function updates Owner of Collecion - who receive royalty fee - Only Admin can change
    #[ink(message)]
    fn update_collection_owner(&mut self, contract_address: AccountId, new_owner: AccountId,) -> Result<(), Error>;

    #[ink(message)]
    fn set_multiple_attributes(&mut self, contract_address: AccountId, attributes: Vec<String>, values: Vec<String>) -> Result<(), Error>;

    #[ink(message)]
    fn has_attribute(&self, contract_address: AccountId, attribute_key: String) -> bool;

    #[ink(message)]
    fn get_collection_attribute_index(&self, contract_address: AccountId, attribute_key: String) -> Option<u64>;
}



