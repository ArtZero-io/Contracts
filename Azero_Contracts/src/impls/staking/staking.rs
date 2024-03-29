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
    modifiers,
    modifier_definition,
    contracts::access_control::*,
    contracts::ownable::*,
    traits::{
        Storage,
        AccountId,
        Balance,
        OccupiedStorage
    }
};
use ink::{
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
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
    if instance.data().is_locked == false {
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
    if instance.data().is_locked == true {
        return Err(From::from(LockError::Locked))
    }
    body(instance)
}

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T, M> ArtZeroStakingTrait for T
where
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>, Type = M>,
    T: Storage<Manager>,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    T: Storage<ownable::Data>
{
    /// Get User NFT staked in the contract
    default fn get_total_staked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Manager>().staking_list.count(account) as u64;
    }

    /// Get User NFT staked in the contract
    default fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64 {
        return self.data::<Manager>().pending_unstaking_list.count(account) as u64;
    }

    default fn get_reward_pool(&self) -> Balance {
        self.data::<Manager>().reward_pool
    }
    
    default fn get_claimable_reward(&self) -> Balance {
        self.data::<Manager>().claimable_reward
    }
    
    default fn is_claimed(&self, account: AccountId) -> bool {
        self.data::<Manager>().is_claimed.get(&account).unwrap_or(false)
    }
    
    default fn get_reward_started(&self) -> bool {
        self.data::<Manager>().reward_started
    }
    
    default fn get_artzero_nft_contract(&self) -> AccountId {
        self.data::<Manager>().nft_contract_address
    }
    
    default fn get_is_locked(&self) -> bool {
        self.data::<Manager>().is_locked
    }
    
    default fn get_admin_address(&self) -> AccountId {
        self.data::<Manager>().admin_address
    }
    
    default fn get_request_unstake_time(&self, account: AccountId, token_id: u64) -> u64 {
       self.data::<Manager>().request_unstaking_time.get(&(&account, &token_id)).unwrap_or(0)
    }
    
    default fn get_staked_id(&self, account: AccountId, index: u64) -> Option<u64> {
        self.data::<Manager>().staking_list.get_value(account, &(index as u128))
    }
    
    default fn get_staked_accounts_index_by_account(&self, account: AccountId) -> Option<u128> {
        self.data::<Manager>().staked_accounts.get_index(0, &account)
    }
    
    default fn get_staked_accounts_account_by_index(&self, index: u64) -> Option<AccountId> {
        self.data::<Manager>().staked_accounts.get_value(0, &(index as u128))
    }
    
    default fn get_staked_accounts_last_index(&self) -> u64 {
        self.data::<Manager>().staked_accounts.count(0) as u64
    }
    
    default fn get_pending_unstaked_id(&self, account: AccountId, index: u64) -> Option<u64> {
        self.data::<Manager>().pending_unstaking_list.get_value(account, &(index as u128))
    }
    
    default fn get_total_staked(&self) -> u64 {
        self.data::<Manager>().total_staked
    }
    
    default fn get_limit_unstake_time(&self) -> u64 {
        self.data::<Manager>().limit_unstake_time
    }

    #[modifiers(only_owner)]
    default fn set_artzero_nft_contract(&mut self, artzero_nft_contract: AccountId) -> Result<(), Error> {
        self.data::<Manager>().nft_contract_address = artzero_nft_contract;
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn set_limit_unstake_time(&mut self, limit_unstake_time: u64) -> Result<(), Error> {
        self.data::<Manager>().limit_unstake_time = limit_unstake_time;
        Ok(())
    }

    #[modifiers(only_owner)]
    default fn update_admin_address(&mut self, admin_address: AccountId) -> Result<(), Error> {
        self.data::<Manager>().admin_address = admin_address;
        Ok(())
    }

    #[modifiers(only_role(ADMINER))]
    default fn update_is_locked(&mut self, is_locked: bool) -> Result<(), Error> {
        if is_locked == self.data::<Manager>().is_locked {
            return Err(Error::InvalidInput)
        }
        self.data::<Manager>().is_locked = is_locked;
        Ok(())
    }

    #[modifiers(only_role(ADMINER))]
    #[modifiers(only_locked)]
    default fn start_reward_distribution(&mut self) -> Result<(), Error> {
        self.data::<Manager>().claimable_reward = self.data::<Manager>().reward_pool;
        self.data::<Manager>().reward_started = true;
        Ok(())
    }

    #[modifiers(only_role(ADMINER))]
    #[modifiers(only_locked)]
    default fn stop_reward_distribution(&mut self) -> Result<(), Error> {
        self.data::<Manager>().reward_pool = self.data::<Manager>().claimable_reward; // unclaimed Rewards send back to reward_pool
        self.data::<Manager>().claimable_reward = 0;
        self.data::<Manager>().reward_started = false;
        Ok(())
    }
}
