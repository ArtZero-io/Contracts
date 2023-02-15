#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#![allow(clippy::let_unit_value)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod artzero_staking_nft {
    use ink::env::CallFlags;
    use ink::prelude::{
        string::String,
        vec::Vec,
    };
    use openbrush::{
        contracts::{
            ownable::*,
            access_control::*,
            traits::psp34::{
                extensions::{
                    burnable::*,
                },
                *,
            },
        },
        traits::{
            Storage,
            DefaultEnv,
            ZERO_ADDRESS
        },
        modifiers,
    };
    use artzero_project::{
        impls::{
            staking::*,
        },
        traits::{
            psp34_standard::*,
            admin::*,
            upgradable::*,
            error::Error,
        },

    };

    impl AccessControl for ArtZeroStakingNFT {}
    impl Ownable for ArtZeroStakingNFT {}
    impl ArtZeroStakingTrait for ArtZeroStakingNFT {}
    impl AdminTrait for ArtZeroStakingNFT {}
    impl UpgradableTrait for ArtZeroStakingNFT {}
    
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroStakingNFT {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: artzero_project::impls::staking::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
    }

    #[ink(event)]
    pub struct NewStakeEvent {
        staker: Option<AccountId>,
        token_id: u64,
    }
    #[ink(event)]
    pub struct UnstakeEvent {
        staker: Option<AccountId>,
        token_id: u64,
    }
    #[ink(event)]
    pub struct RequestUnstakeEvent {
        staker: Option<AccountId>,
        token_id: u64,
    }
    #[ink(event)]
    pub struct CancelRequestUnstakeEvent {
        staker: Option<AccountId>,
        token_id: u64,
    }

    #[ink(event)]
    pub struct ClaimReward {
        staker: Option<AccountId>,
        reward_amount: Balance,
        staked_amount: u64,
    }

    #[ink(event)]
    pub struct AddReward {
        reward_amount: Balance,
        total_staked_amount: u64,
    }
    // Staking contract allows PMP NFT owners to stake the NFT and earn AZERO as rewards
    /* The Reward Distribution has 6 steps:
        Step 1: Lock Staking Contract to make sure noone can stake or unstake during the reward distribution
        Step 2: Add Rewards - ArtZero sends AZERO to the staking contract as Reward Pool
        Step 3: Set all stakers Claimable - ArtZero runs a script on server to set all active stakers to be reward claimable
        Step 4: Enable Reward Distribution - to allow stakers to claim the reward
        Step 5: Stop Reward Distribution - Once the Reward Distribution period is over (around 3 days), Admin stops the process
        Step 6: Unlock Staking Contract - Let stakers to stake/unstake their NFTs
        Note: Any rewards that not been claimed by the stakers will be accumulated for next Reward Distribution
    */
    impl ArtZeroStakingNFT {
        #[ink(constructor)]
        pub fn new(
            admin_address: AccountId,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
        ) -> Self {
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_owner(caller);
            instance
                .initialize(artzero_nft_contract, limit_unstake_time, admin_address)
                .ok()
                .unwrap();
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
            admin_address: AccountId,
        ) -> Result<(), Error> {
            if self.manager.nft_contract_address != ZERO_ADDRESS.into(){
                return Err(Error::AlreadyInit);
            }
            self.manager.nft_contract_address = artzero_nft_contract;
            self.manager.limit_unstake_time = limit_unstake_time;
            self.manager.admin_address = admin_address;
            self.manager.is_locked = false;
            self._init_with_admin(self.env().caller());
            self.grant_role(ADMINER, admin_address).expect("Should grant the role");
            Ok(())
        }

        /// Set new NFT contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_artzero_nft_contract(&mut self, artzero_nft_contract: AccountId) -> Result<(), Error> {
            self.manager.nft_contract_address = artzero_nft_contract;
            Ok(())
        }

        /// Set new Limit Unstake Time (Minutes) - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_limit_unstake_time(&mut self, limit_unstake_time: u64) -> Result<(), Error> {
            self.manager.limit_unstake_time = limit_unstake_time;
            Ok(())
        }

        /// Update Admin Address - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(&mut self, admin_address: AccountId) -> Result<(), Error> {
            self.manager.admin_address = admin_address;
            Ok(())
        }

        /// Update is locked - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_locked(&mut self, is_locked: bool) -> Result<(), Error> {
            if is_locked == self.manager.is_locked {
                return Err(Error::InvalidInput)
            }
            self.manager.is_locked = is_locked;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        #[modifiers(only_locked)]
        pub fn start_reward_distribution(&mut self) -> Result<(), Error> {
            self.manager.claimable_reward = self.manager.reward_pool;
            self.manager.reward_started = true;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        #[modifiers(only_locked)]
        pub fn stop_reward_distribution(&mut self) -> Result<(), Error> {
            self.manager.reward_pool = self.manager.claimable_reward; // unclaimed Rewards send back to reward_pool
            self.manager.claimable_reward = 0;
            self.manager.reward_started = false;
            Ok(())
        }
        /// Add reward to reward_pool
        #[ink(message)]
        #[ink(payable)]
        #[modifiers(only_locked)]
        pub fn add_reward(&mut self) -> Result<(), Error> {
            let reward = self.env().transferred_value();
            if reward == 0 {
                return Err(Error::InvalidInput)
            }
            if self.manager.reward_started { // only when reward distribution is not started
                return Err(Error::RewardStarted)
            }
            self.manager.reward_pool = self.manager.reward_pool.checked_add(reward).unwrap();
            self.env().emit_event(AddReward {
                reward_amount: reward,
                total_staked_amount: self.manager.total_staked,
            });
            Ok(())
        }

        /// Set Account so it can claim the reward. Must run by backend every month before add_reward
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        #[modifiers(only_locked)]
        pub fn set_claimed_status(&mut self, staker: AccountId) -> Result<(), Error> {
            if self.manager.reward_started { // only when reward distribution is not started
                return Err(Error::RewardStarted)
            }
            self.manager.is_claimed.insert(&staker, &false); // Can only claim once
            Ok(())
        }

        /// Claim Reward
        #[ink(message)]
        #[modifiers(only_locked)]
        pub fn claim_reward(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let is_claimed = self.manager.is_claimed.get(&caller);
            // Check if the claim exist and must be FALSE
            if is_claimed.is_none(){
                return Err(Error::ClaimMustBeFalse)
            }
            if is_claimed.unwrap() {
                return Err(Error::ClaimMustBeFalse)
            }
            self.manager.is_claimed.insert(&caller, &true); // Can only claim once

            // Check if the total NFT staked is greater than 0 to avoid division by ZERO error
            if self.manager.total_staked == 0 {
                return Err(Error::Custom(String::from("Invalid Total Stake")))
            }
            if !self.manager.reward_started { // Only allow when reward distribution is started
                return Err(Error::RewardNotStarted)
            }
            // How many NFT the user staker, it must be greater than ZERO
            let staked_amount = self.manager.staking_list.count(caller);
            if staked_amount == 0{
                return Err(Error::Custom(String::from("Invalid User Stake")))
            }
            // Check if Reward Pool has balance to pay for stakers
            if self.manager.reward_pool == 0{
                return Err(Error::Custom(String::from("Invalid Reward Pool")))
            }
            // Calculate how much reward to pay for staker
            let reward =
                (self.manager.reward_pool * staked_amount) / (self.manager.total_staked as u128);

            if self.manager.claimable_reward >= reward {
                // Send the reward to the staker
                self.manager.claimable_reward = self.manager.claimable_reward.checked_sub(reward).unwrap();
                if reward > self.env().balance() {
                    return Err(Error::NotEnoughBalance)
                }
                if self.env().transfer(caller, reward).is_err(){
                    return Err(Error::CannotTransfer)
                }
                // Emit ClaimReward event to the network
                self.env().emit_event(ClaimReward {
                    staker: Some(caller),
                    reward_amount: reward,
                    staked_amount: staked_amount as u64,
                });
            } else {
                if self.manager.claimable_reward > self.env().balance() {
                    return Err(Error::NotEnoughBalance)
                }
                // If there is not enough fund to pay, transfer everything in the pool to staker
                if self.env().transfer(caller, self.manager.claimable_reward).is_err() {
                    return Err(Error::CannotTransfer)
                }
                self.env().emit_event(ClaimReward {
                    staker: Some(caller),
                    reward_amount: self.manager.claimable_reward,
                    staked_amount: staked_amount as u64,
                });
                self.manager.claimable_reward = 0;
            }

            Ok(())
        }

        // GETTERS
        #[ink(message)]
        pub fn get_reward_pool(&self) -> Balance {
            self.manager.reward_pool
        }
        #[ink(message)]
        pub fn get_claimable_reward(&self) -> Balance {
            self.manager.claimable_reward
        }
        #[ink(message)]
        pub fn is_claimed(&self, account: AccountId) -> bool {
            self.manager.is_claimed.get(&account).unwrap_or(false)
        }
        #[ink(message)]
        pub fn get_reward_started(&self) -> bool {
            self.manager.reward_started
        }

        /// Get NFT contract address
        #[ink(message)]
        pub fn get_artzero_nft_contract(&self) -> AccountId {
            self.manager.nft_contract_address
        }

        /// Get Is Locked Status
        #[ink(message)]
        pub fn get_is_locked(&self) -> bool {
            self.manager.is_locked
        }

        /// Get Admin Account
        #[ink(message)]
        pub fn get_admin_address(&self) -> AccountId {
            self.manager.admin_address
        }

        /// Get request unstake Time
        #[ink(message)]
        pub fn get_request_unstake_time(&self, account: AccountId, token_id: u64) -> u64 {
           self.manager.request_unstaking_time.get(&(&account, &token_id)).unwrap_or(0)
        }

        /// Get staked token ids by AccountId
        #[ink(message)]
        pub fn get_staked_id(&self, account: AccountId, index: u64) -> Option<u64> {
            self.manager.staking_list.get_value(account, &(index as u128))
        }

        /// Get staked accounts: index by AccountId
        #[ink(message)]
        pub fn get_staked_accounts_index_by_account(&self, account: AccountId) -> Option<u128> {
            self.manager.staked_accounts.get_index(0, &account)
        }

        /// Get staked accounts: account by Index
        #[ink(message)]
        pub fn get_staked_accounts_account_by_index(&self, index: u64) -> Option<AccountId> {
            self.manager.staked_accounts.get_value(0, &(index as u128))
        }

        /// Get staked accounts last index
        #[ink(message)]
        pub fn get_staked_accounts_last_index(&self) -> u64 {
            self.manager.staked_accounts.count(0) as u64
        }

        /// Get pending unstaked token ids by AccountId
        #[ink(message)]
        pub fn get_pending_unstaked_id(&self, account: AccountId, index: u64) -> Option<u64> {
            self.manager.pending_unstaking_list.get_value(account, &(index as u128))
        }

        /// Get total NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked(&self) -> u64 {
            self.manager.total_staked
        }

        /// Get limit unstake time
        #[ink(message)]
        pub fn get_limit_unstake_time(&self) -> u64 {
            self.manager.limit_unstake_time
        }

        /// Stake multiple NFTs - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        #[modifiers(only_not_locked)]
        pub fn stake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            let caller = self.env().caller();
            let leng = token_ids.len();

            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            for &item in token_ids.iter() {
                // Step 1 - Check if the token is belong to caller
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(item)).unwrap();
                if caller != token_owner {
                    return Err(Error::NotTokenOwner)
                }
                // Step 2 - Check if this contract has been approved
                let allowance = Psp34Ref::allowance(
                    &self.manager.nft_contract_address,
                    caller,
                    self.env().account_id(),
                    Some(Id::U64(item)),
                );
                if !allowance {
                    return Err(Error::NotApproved)
                }
                self.manager.staking_list.insert(caller, &item);
                // Step 3 - Transfer Token from Caller to Staking Contract
                if PSP34Ref::transfer_builder(
                    &self.manager.nft_contract_address,
                    self.env().account_id(),
                    Id::U64(item),
                    Vec::<u8>::new(),
                )
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .is_ok() {
                    if self.manager.is_claimed.get(&caller).is_none() {
                        self.manager.is_claimed.insert(&caller, &false);
                    }
                    self.env().emit_event(NewStakeEvent {
                        staker: Some(caller),
                        token_id: item,
                    });

                } else {
                    return Err(Error::CannotTransfer)
                }

            }
            if !self.manager.staked_accounts.contains_value(0, &caller) {
                self.manager.staked_accounts.insert(0, &caller);
            }

            Ok(())
        }

        /// Request Unstake multiple NFTs
        #[ink(message)]
        #[modifiers(only_not_locked)]
        pub fn request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            let caller = self.env().caller();
            let leng = token_ids.len();

            for &item in token_ids.iter() {
                // Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(item)).unwrap();
                if self.env().account_id() != token_owner {
                    return Err(Error::InvalidCaller)
                }
                // Setp 2 - Check staker
                if !self.manager.staking_list.contains_value(caller, &item) {
                    return Err(Error::InvalidInput)
                }
                // Step 3 - Remove token on staking_list
                self.manager.staking_list.remove_value(caller, &item);
                // Step 4 - Add token to pending unstaking list
                let current_time = <ArtZeroStakingNFT as DefaultEnv>::env().block_timestamp();
                self.manager.request_unstaking_time.insert(&(&caller, &item), &current_time);
                self.manager.pending_unstaking_list.insert(caller, &item);
                self.env().emit_event(RequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: item,
                });
            }

            if self.manager.staking_list.count(caller) == 0 {
                self.manager.staked_accounts.remove_value(0, &caller);
            }

            if !self.manager.staked_accounts.contains_value(1, &caller) {
                self.manager.staked_accounts.insert(1, &caller);
            }

            self.manager.total_staked = self.manager.total_staked.checked_sub(leng as u64).unwrap();
            Ok(())
        }

        /// Cancel Request
        #[ink(message)]
        #[modifiers(only_not_locked)]
        pub fn cancel_request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            let caller = self.env().caller();
            let leng = token_ids.len();

            if self.manager.pending_unstaking_list.count(caller) == 0 {
                return Err(Error::InvalidInput)
            }
            for &item in token_ids.iter() {
                // Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(item)).unwrap();
                if self.env().account_id() != token_owner {
                    return Err(Error::TokenOwnerNotMatch)
                }
                // Setp 2 - Check staker
                if !self.manager.pending_unstaking_list.contains_value(caller, &item) {
                    return Err(Error::InvalidInput)
                }
                // Step 3 - Add token on staking_list
                self.manager
                    .staking_list
                    .insert(caller, &item);
                // Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                self.manager.pending_unstaking_list.remove_value(caller, &item);
                self.env().emit_event(CancelRequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: item,
                });
            }

            if self.manager.pending_unstaking_list.count(caller) == 0 {
                self.manager.staked_accounts.remove_value(1, &caller);
            }

            if !self.manager.staked_accounts.contains_value(0, &caller) {
                self.manager.staked_accounts.insert(0, &caller);
            }

            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        #[modifiers(only_not_locked)]
        pub fn unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            // Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            if self.manager.pending_unstaking_list.count(caller) == 0 {
                return Err(Error::InvalidInput)
            }
            for &item in token_ids.iter() {
                // Setp 2 - Check request unstaked and time request unstaked
                if !self.manager.pending_unstaking_list.contains_value(caller, &item) {
                    return Err(Error::InvalidInput)
                }
                let request_unstake_time = self.get_request_unstake_time(caller, item);
                if request_unstake_time == 0 {
                    return Err(Error::InvalidTime)
                }
                let current_time = <ArtZeroStakingNFT as DefaultEnv>::env().block_timestamp();

                if request_unstake_time + (self.manager.limit_unstake_time * 60000) > current_time {
                    return Err(Error::Custom(String::from("Not Enough Time Request Unstake")))
                }
                // Step 3 - transfer token to caller
                if Psp34Ref::transfer(
                    &self.manager.nft_contract_address,
                    caller,
                    Id::U64(item),
                    Vec::<u8>::new()
                )
                .is_err(){
                    return Err(Error::CannotTransfer)
                }
                // Step 4 - Remove from pending_unstaking_list and change request_unstaking_time to 0
                self.manager.pending_unstaking_list.remove_value(caller, &item);
                self.manager.request_unstaking_time.insert(&(&caller, &item), &0);
                if self.manager.pending_unstaking_list.count(caller) == 0 {
                    self.manager.staked_accounts.remove_value(1, &caller);
                }
                self.env().emit_event(UnstakeEvent {
                    staker: Some(caller),
                    token_id: item,
                });
            }
            Ok(())
        }
    }
}
