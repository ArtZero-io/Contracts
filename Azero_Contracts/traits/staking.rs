use openbrush::{
    traits::{
        AccountId,
    }
};

#[openbrush::wrapper]
pub type ArtZeroStakingRef = dyn ArtZeroStakingTrait;

#[openbrush::trait_definition]
pub trait ArtZeroStakingTrait {
    #[ink(message)]
    fn get_total_staked_by_account(&self, account: AccountId) -> u64;
    #[ink(message)]
    fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64;
}
