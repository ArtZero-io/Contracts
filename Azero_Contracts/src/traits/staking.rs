use openbrush::{
    modifier_definition,
    contracts::access_control::*,
    traits::{
        Storage,
        AccountId,
        Balance,
        Hash
    }
};

#[openbrush::wrapper]
pub type ArtZeroStakingRef = dyn ArtZeroStakingTrait;

#[openbrush::trait_definition]
pub trait ArtZeroStakingTrait {
    /// This function returns the total PMP NFT Staked by an account
    #[ink(message)]
    fn get_total_staked_by_account(&self, account: AccountId) -> u64;
    /// This function returns the total PMP NFT that is pending to be unstaked by an account
    #[ink(message)]
    fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64;

    #[ink(message)]
    fn get_reward_pool(&self) -> Balance;

    #[ink(message)]
    fn get_claimable_reward(&self) -> Balance;

    #[ink(message)]
    fn is_claimed(&self, account: AccountId) -> bool;

    #[ink(message)]
    fn get_reward_started(&self) -> bool;

    #[ink(message)]
    fn get_artzero_nft_contract(&self) -> AccountId;

    #[ink(message)]
    fn get_is_locked(&self) -> bool;

    #[ink(message)]
    fn get_admin_address(&self) -> AccountId;

    #[ink(message)]
    fn get_request_unstake_time(&self, account: AccountId, token_id: u64) -> u64;

    #[ink(message)]
    fn get_staked_id(&self, account: AccountId, index: u64) -> Option<u64>;

    #[ink(message)]
    fn get_staked_accounts_index_by_account(&self, account: AccountId) -> Option<u128>;

    #[ink(message)]
    fn get_staked_accounts_account_by_index(&self, index: u64) -> Option<AccountId>;

    #[ink(message)]
    fn get_staked_accounts_last_index(&self) -> u64;

    #[ink(message)]
    fn get_pending_unstaked_id(&self, account: AccountId, index: u64) -> Option<u64>;

    #[ink(message)]
    fn get_total_staked(&self) -> u64;

    #[ink(message)]
    fn get_limit_unstake_time(&self) -> u64;
}
