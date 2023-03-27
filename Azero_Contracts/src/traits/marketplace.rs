use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*
    },
    traits::{
        AccountId,
        Balance
    },
};
use crate::traits::error::Error;
use crate::impls::marketplace::BidInformation;
use crate::impls::marketplace::ForSaleItem;
use ink::prelude::{
    vec::Vec,
};

#[openbrush::wrapper]
pub type ArtZeroMarketplaceRef = dyn ArtZeroMarketplaceTrait;

#[openbrush::trait_definition]
pub trait ArtZeroMarketplaceTrait {
    // GETTERS
    /// Get market list information using NFT Collection and token ID
    #[ink(message)]
    fn get_nft_sale_info(&self, nft_contract_address: AccountId, token_id: Id) -> Option<ForSaleItem>;

    /// Get platform fee
    #[ink(message)]
    fn get_platform_fee(&self) -> u32;

    /// Get Staking Discount Criteria
    #[ink(message)]
    fn get_staking_discount_criteria(&self) -> Vec<u8>;

    /// Get Staking Discount Rates
    #[ink(message)]
    fn get_staking_discount_rate(&self) -> Vec<u16>;

    /// Get listed token count by collection address
    #[ink(message)]
    fn get_listed_token_count_by_collection_address(&self, collection_contract_address: AccountId) -> u64;

    /// Get all token ids currently for sale for a collection (nft_contract_address,user_account)
    #[ink(message)]
    fn get_for_sale_token_id(
        &self,
        nft_contract_address: AccountId,
        user_account: AccountId,
        index: u128,
    ) -> Option<Id>;

    /// Get get total sale token ids of user account in a contract
    #[ink(message)]
    fn get_sale_tokens_ids_count(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128;

    /// Get all token ids currently for sale by a collection (nft_contract_address,user_account)
    #[ink(message)]
    fn total_tokens_for_sale(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128;

    /// Get all bids from (NFT Contract Address, User Address, token ID)
    #[ink(message)]
    fn get_all_bids(
        &self,
        nft_contract_address: AccountId,
        user_account: AccountId,
        token_id: Id,
    ) -> Option<Vec<BidInformation>>;

    /// Get collection contract address
    #[ink(message)]
    fn get_collection_contract_address(&self) -> AccountId;

    /// Get staking contract address
    #[ink(message)]
    fn get_staking_contract_address(&self) -> AccountId;

    /// Get total platform volume
    #[ink(message)]
    fn get_total_volume(&self) -> Balance;

    /// Get total Collection volume
    #[ink(message)]
    fn get_volume_by_collection(&self, collection_contract_address: AccountId) -> Balance;

    /// Get platform total Profit
    #[ink(message)]
    fn get_total_profit(&self) -> Balance;

    /// Get platform current available profit
    #[ink(message)]
    fn get_current_profit(&self) -> Balance;

    /// Get hold amount of bidder
    #[ink(message)]
    fn get_hold_amount_of_bidder(&self, bidder: AccountId) -> Option<Balance>;

    /// Get Hold Bidders by Index
    #[ink(message)]
    fn get_hold_bidders_by_index(&self, index: u64) -> Option<AccountId>;

    /// Get Hold Bidder Count
    #[ink(message)]
    fn get_hold_bidder_count(&self) -> u64;

    /// Withdraw Profit - only Contract Owner.
    #[ink(message)]
    fn withdraw_profit(&mut self, value: Balance, reciever: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn set_collection_contract_address(&mut self, collection_contract_address: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn set_platform_fee(&mut self, platform_fee: u32) -> Result<(), Error>;

    #[ink(message)]
    fn set_staking_contract_address(&mut self, staking_contract_address: AccountId) -> Result<(), Error>;

    #[ink(message)]
    fn set_discount(&mut self, criteria: Vec<u8>, rates: Vec<u16>) -> Result<(), Error>;

    #[ink(message)]
    fn receive_hold_amount(&mut self, receiver: AccountId) -> Result<(), Error>;
}
