use ink::prelude::{
    vec::Vec,
    string::{
        String,
    },
};
use openbrush::{
    contracts::traits::{
        ownable::*,
    },
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*
    },
    traits::{
        AccountId
    }
};
use crate::traits::error::Error;

#[openbrush::wrapper]
pub type Psp34Ref = dyn PSP34 + PSP34Metadata + Ownable;

#[openbrush::trait_definition]
pub trait Psp34Traits: PSP34 + PSP34Metadata + Ownable {
    /// This function sets the baseURI for the NFT contract. Only Contract Owner can perform this function. baseURI is the location of the metadata files if the NFT collection use external source to keep their NFT artwork. ArtZero uses IPFS by default, the baseURI can have format like this: ipfs://<hash_ID>/
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
    /// This function set the attributes to each NFT. Only Contract Owner can perform this function. The metadata input is an array of [(attribute, value)]. The attributes in ArtZero platform are the NFT traits.
    #[ink(message)]
    fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>
    ) -> Result<(), Error>;
    /// This function returns all available attributes of each NFT
    #[ink(message)]
    fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
    /// This function return how many unique attributes in the contract
    #[ink(message)]
    fn get_attribute_count(&self) -> u32;
    /// This function return the attribute name using attribute index. Beacause attributes of an NFT can be set to anything by Contract Owner, AztZero uses this function to get all attributes of an NFT
    #[ink(message)]
    fn get_attribute_name(&self, index: u32) -> String;
    /// This function return the metadata location of an NFT. The format is baseURI/<token_id>.json
    #[ink(message)]
    fn token_uri(&self, token_id: u64) -> String;
    /// This function return the owner of the NFT Contract
    #[ink(message)]
    fn get_owner(&self) -> AccountId ;
    /// This function return the latest token ID, everytime new NFT is mint, last_token_id is increased by 1 in mint function. Note: This is not the same as the total supply return by the psp34 function as NFT can be burnt.
    #[ink(message)]
    fn get_last_token_id(&self) -> u64;
    /// This function lets NFT owner to lock their NFT. Once locked, the NFT traits (attributes) can not be changed
    #[ink(message)]
    fn lock(&mut self, token_id: Id) -> Result<(), Error>;
    /// This function check if an NFT is locked or not
    #[ink(message)]
    fn is_locked_nft(&self, token_id: Id) -> bool;
    /// This function returns how many NFTs have been locked by its owners
    #[ink(message)]
    fn get_locked_token_count(&self) -> u64;
}