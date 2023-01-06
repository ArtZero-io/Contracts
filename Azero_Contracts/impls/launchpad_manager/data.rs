
use openbrush::{
    traits::{
        AccountId,
        Hash,
        Balance,
        Timestamp
    },
    storage::{
        Mapping
    }
};
use ink_storage::{
    traits::{
        PackedLayout,
        SpreadLayout,
    }
};
use ink_prelude::vec::Vec;

#[derive(
    Clone,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Default,
    PackedLayout,
    SpreadLayout,
    scale::Encode,
    scale::Decode,
)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Project {
    pub is_active: bool,
    pub project_owner: AccountId,
    pub total_supply: u64,
    pub start_time: Timestamp,
    pub end_time: Timestamp
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub standard_nft_hash: Hash,
    pub project_count: u64,
    pub projects: Mapping<AccountId, Project>,
    pub projects_by_id: Mapping<u64, AccountId>,
    pub projects_by_owner: Mapping<AccountId, Vec<AccountId>>,
    pub active_project_count: u64,
    pub max_phases_per_project: u8,
    pub project_adding_fee: Balance,
    pub project_mint_fee_rate: u32,
    pub public_max_minting_amount: u64,
    pub _reserved: Option<()>,
}
