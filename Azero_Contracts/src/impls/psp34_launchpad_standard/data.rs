use ink::prelude::{
    vec::Vec,
    string::String,
};
use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*,
    },
};
use openbrush::{
    traits::{
        AccountId,
        Storage,
        DefaultEnv,
        ZERO_ADDRESS,
        Balance,
        Timestamp
    },
    storage::{
        MultiMapping,
        ValueGuard,
        Mapping,
        TypeGuard
    },
};

#[cfg(feature = "std")]
use ink::storage::traits::StorageLayout;

#[derive(
    Copy,
    Clone,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Default,
    scale::Encode,
    scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct Whitelist {
    pub whitelist_amount: u64,
    pub claimed_amount: u64,
    pub minting_fee: Balance
}

#[derive(
    Clone,
    Debug,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Default,
    scale::Encode,
    scale::Decode,
)]
#[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
pub struct Phase {
    pub is_active: bool,
    pub title: Vec<u8>,
    pub is_public: bool,
    pub public_minting_fee: Balance,
    pub public_minting_amount: u64,
    pub public_max_minting_amount: u64,
    pub public_claimed_amount: u64,
    pub whitelist_amount: u64,
    pub claimed_amount: u64,
    pub total_amount: u64,
    pub start_time: Timestamp,
    pub end_time: Timestamp
}

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub total_supply: u64,
    pub last_phase_id: u8,
    pub whitelist_count: u32,
    pub phase_account_public_claimed_amount: Mapping<(AccountId, u8), u64, PhaseAccountPublicClaimedAmountKeys>,
    pub phase_whitelists_link: Mapping<(AccountId, u8), Whitelist, PhaseWhitelistsLinkKeys>,
    pub phases: Mapping<u8, Phase>,
    pub phase_account_link: MultiMapping<u8, AccountId, ValueGuard<u8>>,
    pub limit_phase_count: u8,
    pub launchpad_contract_address: AccountId,
    pub project_info: Vec<u8>,
    pub public_minted_count: u64,
    pub active_phase_count: u8,
    pub available_token_amount: u64,
    pub owner_claimed_amount: u64,
    pub _reserved: Option<()>,
}

impl Default for Manager {
    fn default() -> Self {
        Self {
            total_supply: Default::default(),
            last_phase_id: Default::default(),
            whitelist_count: Default::default(),
            phase_account_public_claimed_amount: Default::default(),
            phase_whitelists_link: Default::default(),
            phases: Default::default(),
            phase_account_link: Default::default(),
            limit_phase_count: Default::default(),
            launchpad_contract_address: ZERO_ADDRESS.into(),
            project_info: Default::default(),
            public_minted_count: Default::default(),
            active_phase_count: Default::default(),
            available_token_amount: Default::default(),
            owner_claimed_amount: Default::default(),
            _reserved: Default::default(),
        }
    }
}

pub struct PhaseAccountPublicClaimedAmountKeys;

impl<'a> TypeGuard<'a> for PhaseAccountPublicClaimedAmountKeys {
    type Type = &'a (&'a AccountId, &'a u8);
}

pub struct PhaseWhitelistsLinkKeys;

impl<'a> TypeGuard<'a> for PhaseWhitelistsLinkKeys {
    type Type = &'a (&'a AccountId, &'a u8);
}