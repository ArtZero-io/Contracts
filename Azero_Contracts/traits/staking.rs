use openbrush::{
    traits::{
        AccountId,
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
}
