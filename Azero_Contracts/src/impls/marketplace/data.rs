use openbrush::{
    storage::{
        MultiMapping,
        ValueGuard,
        Mapping,
        TypeGuard
    },
    traits::{
        AccountId,
        Balance,
        ZERO_ADDRESS
    },
};
use ink::prelude::{
    string::{
        String,
        ToString,
    },
    vec::Vec,
};
use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*,
    },
};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(
    Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct ForSaleItem {
    pub nft_owner: AccountId,
    pub listed_date: u64,
    pub price: Balance,
    pub is_for_sale: bool,
    pub royalty_fee_at_listing: u32
}

#[derive(
    Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct BidInformation {
    pub bidder: AccountId,
    pub bid_date: u64,
    pub bid_value: Balance,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub collection_contract_address: AccountId,
    pub staking_contract_address: AccountId,
    pub platform_fee: u32,                                  // 1% = 100
    pub market_list: Mapping<(AccountId, Id), ForSaleItem, MarketListKeys>, /* NFT for sale in the marketplace, (NFT Contract Address, Token ID) */
    pub sale_tokens_ids: MultiMapping<(Option<AccountId>, Option<AccountId>), Id, SaleTokensIdsKey>,                 //(NFT Contract Address, Seller Address)
    pub sale_tokens_ids_last_index: Mapping<(Option<AccountId>, Option<AccountId>), u128, SaleTokensIdsLastIndexKeys>,
    pub hold_amount_bidders: Mapping<AccountId, Balance>,
    pub hold_bidders: MultiMapping<u8, AccountId, ValueGuard<u8>>,
    pub bidders: Mapping<(AccountId, AccountId, Id), Vec<BidInformation>, BiddersKeys>, /* Contract Address, Seller Address, token ID) */
    pub listed_token_number_by_collection_address: Mapping<AccountId, u64>, /* Number Listed Token (Collection Contract Address) */
    pub total_volume: Balance,
    pub volume_by_collection: Mapping<AccountId, Balance>,
    pub volume_by_user: Mapping<AccountId, Balance>,
    pub total_profit: Balance,
    pub current_profit: Balance,
    pub staking_discount_criteria: Vec<u8>,
    pub staking_discount_rate: Vec<u16>,
    pub _reserved: Option<()>,
}

pub struct MarketListKeys;

impl<'a> TypeGuard<'a> for MarketListKeys {
    type Type = &'a (&'a AccountId, &'a Id);
}

pub struct SaleTokensIdsKey;

impl<'a> TypeGuard<'a> for SaleTokensIdsKey {
    type Type = &'a (&'a Option<&'a AccountId>, &'a Option<&'a AccountId>);
}

pub struct SaleTokensIdsLastIndexKeys;

impl<'a> TypeGuard<'a> for SaleTokensIdsLastIndexKeys {
    type Type = &'a (&'a Option<&'a AccountId>, &'a Option<&'a AccountId>);
}

pub struct BiddersKeys;

impl<'a> TypeGuard<'a> for BiddersKeys {
    type Type = &'a (&'a AccountId, &'a AccountId, &'a Id);
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            collection_contract_address: ZERO_ADDRESS.into(),
            staking_contract_address: ZERO_ADDRESS.into(),
            platform_fee: Default::default(),                                  // 1% = 100
            market_list: Default::default(), /* NFT for sale in the marketplace, (NFT Contract Address, Token ID) */
            sale_tokens_ids: Default::default(),                 //(NFT Contract Address, Seller Address)
            sale_tokens_ids_last_index: Default::default(),
            hold_amount_bidders: Default::default(),
            hold_bidders: Default::default(),
            bidders: Default::default(), /* Contract Address, Seller Address, token ID) */
            listed_token_number_by_collection_address: Default::default(), /* Number Listed Token (Collection Contract Address) */
            total_volume: Default::default(),
            volume_by_collection: Default::default(),
            volume_by_user: Default::default(),
            total_profit: Default::default(),
            current_profit: Default::default(),
            staking_discount_criteria: Default::default(),
            staking_discount_rate: Default::default(),
            _reserved: Default::default(),
        }
    }
}