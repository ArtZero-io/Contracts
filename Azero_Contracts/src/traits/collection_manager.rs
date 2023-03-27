use openbrush::{
    traits::{
        AccountId,
        Balance,
        Hash
    }
};
use crate::traits::error::Error;
use crate::impls::collection_manager::Collection;
use ink::prelude::{
    string::{
        String,
    },
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

    #[ink(message)]
    fn get_attributes(&self, contract_address: AccountId, attributes: Vec<String>) -> Vec<String>;

    #[ink(message)]
    fn get_attribute(&self, contract_address: AccountId, attribute_key: String) -> String;

    #[ink(message)]
    fn get_collection_attribute_count(&self, contract_address: AccountId) -> Option<u64>;

    #[ink(message)]
    fn update_contract_type(&mut self, contract_address: AccountId, contract_type: CollectionType) -> Result<(), Error>;

    #[ink(message)]
    fn update_is_collect_royalty_fee(&mut self, contract_address: AccountId,is_collect_royalty_fee: bool) -> Result<(), Error>;

    #[ink(message)]
    fn update_royalty_fee(&mut self, contract_address: AccountId, new_fee: u32) -> Result<(), Error>;

    #[ink(message)]
    fn update_show_on_chain_metadata(&mut self,contract_address: AccountId,show_on_chain_metadata: bool,) -> Result<(), Error>;

    #[ink(message)]
    fn update_is_active(&mut self, contract_address: AccountId, is_active: bool) -> Result<(), Error>;

    #[ink(message)]
    fn update_simple_mode_adding_fee(&mut self, simple_mode_adding_fee: Balance) -> Result<(), Error>;

    #[ink(message)]
    fn update_standard_nft_hash(&mut self, standard_nft_hash: Hash) -> Result<(), Error>;

    #[ink(message)]
    fn update_advance_mode_adding_fee(&mut self, advance_mode_adding_fee: Balance) -> Result<(), Error>;

    #[ink(message)]
    fn update_max_royalty_fee_rate(&mut self, max_royalty_fee_rate: u32) -> Result<(), Error>;

    #[ink(message)]
    fn get_collection_by_address(&self, nft_contract_address: AccountId) -> Option<Collection>;

    #[ink(message)]
    fn get_collections_by_owner(&self, owner_address: AccountId) -> Option<Vec<AccountId>>;

    #[ink(message)]
    fn get_standard_nft_hash(&self) -> Hash;

    #[ink(message)]
    fn get_contract_by_id(&self, id: u64) -> Option<AccountId>;

    #[ink(message)]
    fn get_collection_count(&self) -> u64;

    #[ink(message)]
    fn get_active_collection_count(&self) -> u64;

    #[ink(message)]
    fn get_simple_mode_adding_fee(&self) -> Balance;

    #[ink(message)]
    fn get_advance_mode_adding_fee(&self) -> Balance;

    #[ink(message)]
    fn get_max_royalty_fee_rate(&self) -> u32;
}
