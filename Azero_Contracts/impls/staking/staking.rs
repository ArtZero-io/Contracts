
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
    modifier_definition,
    contracts::access_control::*,
    traits::{
        Storage,
        AccountId,
    }
};
use crate::traits::error::*;

/// Throws if is_locked is false
#[modifier_definition]
pub fn only_locked<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Manager>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<LockError>,
{
    if !instance.data().is_locked {
        return Err(From::from(LockError::NotLocked))
    }
    body(instance)
}

/// Throws if is_locked is true
#[modifier_definition]
pub fn only_not_locked<T, F, R, E>(instance: &mut T, body: F) -> Result<R, E>
where
    T: Storage<Manager>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<LockError>,
{
    if instance.data().is_locked {
        return Err(From::from(LockError::Locked))
    }
    body(instance)
}

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

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
