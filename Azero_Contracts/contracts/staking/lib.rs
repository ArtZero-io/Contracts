#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_staking_nft {
    use ink_env::CallFlags;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate
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
        traits::Storage,
        modifiers,
    };
    use artzero_project::{
        impls::staking::*,
        traits::{
            staking::{
                Error,
                *
            }
        }
    };
    use psp34_nft::psp34_nft::Psp34NftRef;
    use artzero_project::traits::psp34_standard::*;

    impl AccessControl for ArtZeroStakingNFT {}
    impl Ownable for ArtZeroStakingNFT {}
    impl ArtZeroStakingTrait for ArtZeroStakingNFT {}

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct ArtZeroStakingNFT {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: artzero_project::impls::staking::data::Manager,
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

    impl ArtZeroStakingNFT {
        #[ink(constructor)]
        pub fn new(
            contract_owner: AccountId,
            admin_address: AccountId,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance._init_with_admin(instance.env().caller());
                instance.grant_role(ADMINER, admin_address).expect("Should grant the role");
                instance
                    .initialize(artzero_nft_contract, limit_unstake_time, admin_address)
                    .ok()
                    .unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
            admin_address: AccountId,
        ) -> Result<(), OwnableError> {
            self.manager.nft_contract_address = artzero_nft_contract;
            self.manager.limit_unstake_time = limit_unstake_time;
            self.manager.admin_address = admin_address;
            self.manager.is_locked = false;
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
        pub fn update_is_locked(&mut self, is_locked: bool) -> Result<(), AccessControlError> {
            assert!(is_locked != self.manager.is_locked);
            self.manager.is_locked = is_locked;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn start_reward_distribution(&mut self) -> Result<(), AccessControlError> {
            assert!(self.manager.is_locked);
            self.manager.claimable_reward = self.manager.reward_pool;
            self.manager.reward_started = true;
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn stop_reward_distribution(&mut self) -> Result<(), AccessControlError> {
            assert!(self.manager.is_locked); // Only allow adding reward when contract is locked
            self.manager.reward_pool = self.manager.claimable_reward; // unclaimed Rewards send back to reward_pool
            self.manager.claimable_reward = 0;
            self.manager.reward_started = false;
            Ok(())
        }
        /// Add reward to reward_pool
        #[ink(message)]
        #[ink(payable)]
        pub fn add_reward(&mut self) -> Result<(), Error> {
            let reward = self.env().transferred_value();
            assert!(reward > 0);
            assert!(self.manager.is_locked); // Only allow adding reward when contract is locked
            assert!(self.manager.reward_started == false); // only when reward distribution is not started
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
        pub fn set_claimed_status(&mut self, staker: AccountId) -> Result<(), AccessControlError> {
            assert!(self.manager.is_locked);
            assert!(self.manager.reward_started == false); // only when reward distribution is not started
            self.manager.is_claimed.insert(&staker, &false); // Can only claim once
            Ok(())
        }

        /// Claim Reward
        #[ink(message)]
        pub fn claim_reward(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let is_claimed = self.manager.is_claimed.get(&caller);
            assert!(is_claimed.is_some());
            assert!(is_claimed.unwrap() == false);
            self.manager.is_claimed.insert(&caller, &true); // Can only claim once

            assert!(self.manager.total_staked > 0);
            assert!(self.manager.is_locked); // Only allow when locked
            assert!(self.manager.reward_started); // Only allow when reward distribution is started

            let staked_amount = self.manager.staking_list.count(caller);
            assert!(staked_amount > 0);
            assert!(self.manager.reward_pool > 0);

            let reward =
                (self.manager.reward_pool * staked_amount) / (self.manager.total_staked as u128);

            if self.manager.claimable_reward >= reward {
                self.manager.claimable_reward = self.manager.claimable_reward.checked_sub(reward).unwrap();
                assert!(self.env().transfer(caller, reward).is_ok(),"error claim reward");
                self.env().emit_event(ClaimReward {
                    staker: Some(caller),
                    reward_amount: reward,
                    staked_amount: staked_amount as u64,
                });
            } else {
                assert!(self.env().transfer(caller, self.manager.claimable_reward).is_ok(),"error claim reward");
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
            return self.manager.is_claimed.get(&account).unwrap_or(false);
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
           return self.manager.request_unstaking_time.get(&(&account, &token_id)).unwrap_or(0);
        }

        /// Get staked token ids by AccountId
        #[ink(message)]
        pub fn get_staked_id(&self, account: AccountId, index: u64) -> Option<u64> {
            return self.manager.staking_list.get_value(account, &(index as u128));
        }

        /// Get staked accounts: index by AccountId
        #[ink(message)]
        pub fn get_staked_accounts_index_by_account(&self, account: AccountId) -> Option<u128> {
            return self.manager.staked_accounts.get_index(0, &account);
        }

        /// Get staked accounts: account by Index
        #[ink(message)]
        pub fn get_staked_accounts_account_by_index(&self, index: u64) -> Option<AccountId> {
            return self.manager.staked_accounts.get_value(0, &(index as u128))
        }

        /// Get staked accounts last index
        #[ink(message)]
        pub fn get_staked_accounts_last_index(&self) -> u64 {
            return self.manager.staked_accounts.count(0) as u64;
        }

        /// Get pending unstaked token ids by AccountId
        #[ink(message)]
        pub fn get_pending_unstaked_id(&self, account: AccountId, index: u64) -> Option<u64> {
            return self.manager.pending_unstaking_list.get_value(account, &(index as u128));
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
        pub fn stake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();

            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            for i in 0..leng {
                // Step 1 - Check if the token is belong to caller
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i])).unwrap();
                assert!(caller == token_owner);
                // Step 2 - Check if this contract has been approved
                let allowance = Psp34Ref::allowance(
                    &self.manager.nft_contract_address,
                    caller,
                    self.env().account_id(),
                    Some(Id::U64(token_ids[i])),
                );
                assert!(allowance);
                self.manager.staking_list.insert(caller, &token_ids[i]);
                // Step 3 - Transfer Token from Caller to Staking Contract
                if !PSP34Ref::transfer_builder(
                    &self.manager.nft_contract_address,
                    self.env().account_id(),
                    Id::U64(token_ids[i]),
                    Vec::<u8>::new(),
                )
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .is_ok()
                {
                    return Err(Error::CannotTransfer)
                }
                if self.manager.is_claimed.get(&caller).is_none() {
                    self.manager.is_claimed.insert(&caller, &false);
                }
                self.env().emit_event(NewStakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            if self.manager.staked_accounts.contains_value(0, &caller) == false {
                self.manager.staked_accounts.insert(0, &caller);
            }

            Ok(())
        }

        /// Request Unstake multiple NFTs
        #[ink(message)]
        pub fn request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();

            for i in 0..leng {
                // Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i])).unwrap();
                assert!(self.env().account_id() == token_owner);
                // Setp 2 - Check staker
                assert_eq!(self.manager.staking_list.contains_value(caller, &token_ids[i]), true);
                // Step 3 - Remove token on staking_list
                self.manager.staking_list.remove_value(caller, &token_ids[i]);
                // Step 4 - Add token to pending unstaking list
                let current_time = Self::env().block_timestamp();
                self.manager.request_unstaking_time.insert(&(&caller, &token_ids[i]), &current_time);
                self.manager.pending_unstaking_list.insert(caller, &token_ids[i]);
                self.env().emit_event(RequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }

            if self.manager.staking_list.count(caller) == 0 {
                self.manager.staked_accounts.remove_value(0, &caller);
            }

            if self.manager.staked_accounts.contains_value(1, &caller) == false {
                self.manager.staked_accounts.insert(1, &caller);
            }

            self.manager.total_staked = self.manager.total_staked.checked_sub(leng as u64).unwrap();
            Ok(())
        }

        /// Cancel Request
        #[ink(message)]
        pub fn cancel_request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();

            assert!(self.manager.pending_unstaking_list.count(caller) > 0);
            for i in 0..leng {
                // Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i])).unwrap();
                assert!(self.env().account_id() == token_owner);
                // Setp 2 - Check staker
                assert_eq!(self
                    .manager
                    .pending_unstaking_list.contains_value(caller, &token_ids[i]), true);
                // Step 3 - Add token on staking_list
                self.manager
                    .staking_list
                    .insert(caller, &token_ids[i]);
                // Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                self.manager.pending_unstaking_list.remove_value(caller, &token_ids[i]);
                self.env().emit_event(CancelRequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }

            if self.manager.pending_unstaking_list.count(caller) == 0 {
                self.manager.staked_accounts.remove_value(1, &caller);
            }

            if self.manager.staked_accounts.contains_value(0, &caller) == false {
                self.manager.staked_accounts.insert(0, &caller);
            }

            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        pub fn unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            // Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let leng = token_ids.len();
            assert!(self.manager.pending_unstaking_list.count(caller) > 0);
            for i in 0..leng {
                // Setp 2 - Check request unstaked and time request unstaked
                assert_eq!(self.manager.pending_unstaking_list.contains_value(caller, &token_ids[i]), true);
                let request_unstake_time = self.get_request_unstake_time(caller, token_ids[i]);
                assert!(request_unstake_time > 0);
                let current_time = Self::env().block_timestamp();

                if request_unstake_time + (self.manager.limit_unstake_time * 60000) > current_time {
                    return Err(Error::Custom(String::from("Not Enough Time Request Unstake")))
                }
                // Step 3 - transfer token to caller
                assert!(Psp34Ref::transfer(
                    &self.manager.nft_contract_address,
                    caller,
                    Id::U64(token_ids[i]),
                    Vec::<u8>::new()
                )
                .is_ok());
                // Step 4 - Remove from pending_unstaking_list and change request_unstaking_time to 0
                self.manager.pending_unstaking_list.remove_value(caller, &token_ids[i]);
                self.manager.request_unstaking_time.insert(&(&caller, &token_ids[i]), &0);
                if self.manager.pending_unstaking_list.count(caller) == 0 {
                    self.manager.staked_accounts.remove_value(1, &caller);
                }
                self.env().emit_event(UnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            Ok(())
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self, value: Balance) -> Result<(), Error> {
            assert!(value <= self.env().balance());
            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!("error withdraw_fee")
            }
            Ok(())
        }
        /// Transfer NFT token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn tranfer_nft(&mut self, token_id: Id, receiver: AccountId) -> Result<(), Error> {
            assert!(Psp34Ref::transfer(
                &self.manager.nft_contract_address,
                receiver,
                token_id.clone(),
                Vec::<u8>::new()
            )
            .is_ok());
            Ok(())
        }
    }
}
