use openbrush::{
    traits::{
        AccountId,
    }
};

#[openbrush::wrapper]
pub type ArtZeroCollectionRef = dyn ArtZeroCollectionTrait;

#[openbrush::trait_definition]
pub trait ArtZeroCollectionTrait {
    /// This function returns royalty fee of a Collection
    #[ink(message)]
    fn get_royal_fee(&self, nft_contract_address: AccountId) -> u32;
    /// This function checks if the Collection is active or not. Only Active Collections can be seen on the Marketplace
    #[ink(message)]
    fn is_active(&self, nft_contract_address: AccountId) -> bool;
    /// This function returns NFT Contract Type. When the collection is created using auto_new_collection, this set to 2 and when using add_new_collection, this set to 1. Contract Type is to identify if the contract is standard or customized one.
    #[ink(message)]
    fn get_contract_type(&self, nft_contract_address: AccountId) -> u8;
    /// This function returns the Owner of a Collection
    #[ink(message)]
    fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId>;
}