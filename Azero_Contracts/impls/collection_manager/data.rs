
use ink_prelude::{
    vec::Vec,
};
use ink_storage::{
    traits::{
        PackedLayout,
        SpreadLayout,
    },
};
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Hash,
        Balance
    }
};


#[derive(
    Clone, Debug, Ord, PartialOrd, Eq, PartialEq, PackedLayout, SpreadLayout, scale::Encode, scale::Decode,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Collection {
    pub collection_owner: AccountId,     // to receive Royal Fee - OnlyAdmin can update
    pub nft_contract_address: AccountId, // OnlyAdmin can update
    pub contract_type: u8,               // 1 - PSP34 ERC721 Manual; 2 - PSP34 ERC721 Auto
    pub is_collect_royal_fee: bool,      // OnlyAdmin can update
    pub royal_fee: u32,                  // 100 = 1% 10000 = 100% OnlyAdmin
    pub is_active: bool,                 // OnlyAdmin can update
    pub show_on_chain_metadata: bool,
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub standard_nft_hash: Hash,
    pub admin_address: AccountId,
    pub collection_count: u64,
    pub simple_mode_adding_fee: Balance,
    pub advance_mode_adding_fee: Balance,
    pub collections: Mapping<AccountId, Collection>, // save collection by contract address
    pub collections_by_id: Mapping<u64, AccountId>,  // save contract address by id
    pub collections_by_owner: Mapping<AccountId, Vec<AccountId>>, // save contract address by owner ID
    pub max_royal_fee_rate: u32,
    pub active_collection_count: u64,
    pub attributes: Mapping<(AccountId, Vec<u8>), Vec<u8>, AttributesKey>,
    pub _reserved: Option<()>,
}

pub struct AttributesKey;

impl<'a> TypeGuard<'a> for AttributesKey {
    type Type = &'a (&'a AccountId, &'a Vec<u8>);
}