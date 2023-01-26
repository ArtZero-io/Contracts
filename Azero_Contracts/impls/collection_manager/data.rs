
use ink::prelude::{
    vec::Vec,
};
use openbrush::{
    storage::{
        Mapping
    },
    traits::{
        AccountId,
        Hash,
        Balance,
        String
    }
};
use crate::traits::collection_manager::CollectionType;

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(
    Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct Collection {
    pub collection_owner: AccountId,     // to receive Royalty Fee - OnlyAdmin can update
    pub nft_contract_address: AccountId, // OnlyAdmin can update
    pub contract_type: CollectionType,   // 1 - PSP34 ERC721 Manual; 2 - PSP34 ERC721 Auto
    pub is_collect_royalty_fee: bool,      // OnlyAdmin can update
    pub royalty_fee: u32,                  // 100 = 1% 10000 = 100% OnlyAdmin
    pub is_active: bool,                 // OnlyAdmin can update
    pub show_on_chain_metadata: bool,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub standard_nft_hash: Hash,
    pub collection_count: u64,
    pub simple_mode_adding_fee: Balance,
    pub advance_mode_adding_fee: Balance,
    pub collections: Mapping<AccountId, Collection>, // save collection by contract address
    pub collections_by_id: Mapping<u64, AccountId>,  // save contract address by id
    pub collections_by_owner: Mapping<AccountId, Vec<AccountId>>, // save contract address by owner ID
    pub max_royalty_fee_rate: u32,
    pub active_collection_count: u64,
    pub attributes: Mapping<AccountId, Vec<(String, String)>>,
    pub _reserved: Option<()>,
}
