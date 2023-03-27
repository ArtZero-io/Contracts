use openbrush::{
    contracts::access_control::*,
    traits::{
        AccountId,
        Balance,
        Hash,
        Timestamp
    }
};
use crate::traits::error::Error;
use ink::prelude::{
    vec::Vec,
};
use crate::impls::launchpad_manager::Project;


#[openbrush::wrapper]
pub type ArtZeroLaunchPadRef = dyn ArtZeroLaunchPadTrait;

#[openbrush::trait_definition]
pub trait ArtZeroLaunchPadTrait {
    /// This function returns the rate in % that the launchpad will collect for each NFT minting
    #[ink(message)]
    fn get_project_mint_fee_rate(&self) -> u32;
    /// This function returns the maximal amount of NFT that one can mint each time
    #[ink(message)]
    fn get_public_max_minting_amount(&self) -> u64;

    #[ink(message)]
    fn update_project_adding_fee(
        &mut self,
        project_adding_fee: Balance
    ) -> Result<(), Error>;

    #[ink(message)]
    fn update_public_max_minting_amount(
        &mut self,
        public_max_minting_amount: u64
    ) -> Result<(), AccessControlError>;

    #[ink(message)]
    fn update_project_mint_fee_rate(
        &mut self,
        project_mint_fee_rate: u32
    ) -> Result<(), AccessControlError>;

    #[ink(message)]
    fn update_standard_nft_hash(
        &mut self,
        standard_nft_hash: Hash
    ) -> Result<(), Error>;

    #[ink(message)]
    fn update_is_active_project(
        &mut self,
        is_active: bool,
        contract_address: AccountId
    ) -> Result<(), Error>;

    #[ink(message)]
    fn get_project_adding_fee(
        &self
    ) -> Balance;

    #[ink(message)]
    fn get_active_project_count(
        &self
    ) -> u64;

    #[ink(message)]
    fn get_project_count(
        &self
    ) -> u64;

    #[ink(message)]
    fn get_standard_nft_hash(
        &self
    ) -> Hash;

    #[ink(message)]
    fn get_project_by_id(
        &self,
        id: u64
    ) -> Option<AccountId>;

    #[ink(message)]
    fn get_projects_by_owner(
        &self,
        owner_address: AccountId
    ) -> Vec<AccountId>;

    #[ink(message)]
    fn get_project_by_nft_address(
        &self,
        nft_contract_address: AccountId
    ) -> Option<Project>;

    #[ink(message)]
    fn get_max_phases_per_project(
        &self
    ) -> u8;

    #[ink(message)]
    fn edit_project(
        &mut self,
        contract_address: AccountId,
        start_time: Timestamp,
        end_time: Timestamp
    ) -> Result<(), Error>;
}
