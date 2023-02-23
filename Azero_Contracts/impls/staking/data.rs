
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

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Debug)]
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

impl Default for Manager {
    fn default() -> Self {
        Self {
            is_locked: Default::default(),
            admin_address: ZERO_ADDRESS.into(),
            staked_accounts: Default::default(),
            staking_list: Default::default(),
            pending_unstaking_list: Default::default(),
            nft_contract_address: ZERO_ADDRESS.into(),
            total_staked: Default::default(),
            limit_unstake_time: Default::default(),
            request_unstaking_time: Default::default(),
            reward_pool: Default::default(),
            claimable_reward: Default::default(),
            reward_started: Default::default(),
            is_claimed: Default::default(),
            _reserved: Default::default(),
        }
    }
}

pub struct RequestUnstakingTimeKey;

impl<'a> TypeGuard<'a> for RequestUnstakingTimeKey {
    type Type = &'a (&'a AccountId, &'a u64);
}
