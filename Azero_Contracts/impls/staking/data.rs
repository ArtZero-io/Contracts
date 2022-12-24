use crate::traits::staking::*;
use ink_env::CallFlags;
use ink_prelude::{
    string::String,
    vec::Vec,
};
use ink_storage::{
    traits::SpreadAllocate,
};
use openbrush::{
    contracts::{
        ownable::*,
        traits::psp34::{
            extensions::{
                burnable::*,
                metadata::*,
            },
            *,
        },
    },
    storage::{
        MultiMapping,
        ValueGuard,
        Mapping,
        TypeGuard
    },
    traits::{
        Storage,
        AccountId,
        Balance
    },
    modifiers,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub is_locked: bool,
    pub admin_address: AccountId,
    pub staked_accounts: MultiMapping<u8, AccountId, ValueGuard<u8>>, // 0 is staked status, 1 is request unstake status
    pub staking_list: MultiMapping<AccountId, u64, ValueGuard<AccountId>>,
    pub pending_unstaking_list: MultiMapping<AccountId, u64, ValueGuard<AccountId>>,
    pub nft_contract_address: AccountId,
    pub total_staked: u64,
    pub limit_unstake_time: u64, // minutes
    pub request_unstaking_time: Mapping<(AccountId, u64), u64, RequestUnstakingTimeKey>,
    pub reward_pool: Balance,
    pub claimable_reward: Balance,
    pub reward_started: bool,
    pub is_claimed: Mapping<AccountId, bool>,
    pub _reserved: Option<()>,
}

pub struct RequestUnstakingTimeKey;

impl<'a> TypeGuard<'a> for RequestUnstakingTimeKey {
    type Type = &'a (&'a AccountId, &'a u64);
}
