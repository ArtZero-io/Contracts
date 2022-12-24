use crate::traits::staking::*;
pub use crate::{
    impls::staking::{
        data,
        data::*,
        staking,
        *,
    },
    traits::staking::*,
};
use openbrush::{
    contracts::access_control::*,
    contracts::ownable::*,
    modifiers,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        AccountId,
        Hash,
        Balance
    }
};

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink_lang::selector_id!("ADMINER");

impl<T: Storage<Manager>> ArtZeroStakingTrait for T {
    /// Get User NFT staked in the contract
    default fn get_total_staked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Manager>().staking_list.count(account) as u64;
    }

    /// Get User NFT staked in the contract
    default fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Manager>().pending_unstaking_list.count(account) as u64;
    }
}
