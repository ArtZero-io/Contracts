#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_staking_nft {
    use ink_env::CallFlags;
    use ink_prelude::string::String;
    use ink_prelude::vec::Vec;
    use ink_storage::traits::SpreadAllocate;
    use ink_storage::Mapping;
    use openbrush::contracts::ownable::*;
    use openbrush::contracts::traits::psp34::{
        extensions::{burnable::*, metadata::*},
        *,
    };
    use openbrush::modifiers;

    #[derive(
        Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate,
    )]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct EnumerableMapping {
        id_to_index: Mapping<(Option<AccountId>, u64), u64>,
        index_to_id: Mapping<(Option<AccountId>, u64), u64>,
    }

    #[derive(
        Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate,
    )]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct EnumerableMappingForStakedAccount {
        account_to_index: Mapping<AccountId, u64>,
        index_to_account: Mapping<u64, AccountId>,
    }

    impl EnumerableMapping {
        pub fn insert(&mut self, owner: &Option<AccountId>, id: &u64, index: &u64) {
            self.id_to_index.insert((owner, id), index);
            self.index_to_id.insert((owner, index), id);
        }

        pub fn remove(
            &mut self,
            owner: &Option<AccountId>,
            id: &u64,
            last_index: &u64,
        ) -> Result<(), PSP34Error> {
            let index = self
                .id_to_index
                .get((owner, id))
                .ok_or(PSP34Error::TokenNotExists)?;

            if last_index != &index {
                let last_id = self
                    .index_to_id
                    .get((owner, last_index))
                    .ok_or(PSP34Error::TokenNotExists)?;
                self.index_to_id.insert((owner, &index), &last_id);
                self.id_to_index.insert((owner, &last_id), &index);
            }

            self.index_to_id.remove((owner, &last_index));
            self.id_to_index.remove((owner, id));

            Ok(())
        }

        pub fn get_by_index(
            &self,
            owner: &Option<AccountId>,
            index: &u64,
        ) -> Result<u64, PSP34Error> {
            self.index_to_id
                .get((owner, index))
                .ok_or(PSP34Error::TokenNotExists)
        }

        pub fn get_by_id(&self, owner: &Option<AccountId>, id: &u64) -> Result<u64, PSP34Error> {
            self.id_to_index
                .get((owner, id))
                .ok_or(PSP34Error::TokenNotExists)
        }
    }

    impl EnumerableMappingForStakedAccount {
        pub fn insert(&mut self, account: &AccountId, index: &u64) {
            self.account_to_index.insert(account, index);
            self.index_to_account.insert(index, account);
        }

        pub fn remove(&mut self, account: &AccountId, last_index: &u64) -> Result<(), PSP34Error> {
            let index = self
                .account_to_index
                .get(account)
                .ok_or(PSP34Error::TokenNotExists)?;

            if last_index != &index {
                let last_account = self
                    .index_to_account
                    .get(last_index)
                    .ok_or(PSP34Error::TokenNotExists)?;
                self.index_to_account.insert(index, &last_account);
                self.account_to_index.insert(last_account, &index);
            }

            self.account_to_index.remove(account);
            self.index_to_account.remove(last_index);
            Ok(())
        }

        pub fn get_by_index(&self, index: &u64) -> Result<AccountId, PSP34Error> {
            self.index_to_account
                .get(index)
                .ok_or(PSP34Error::TokenNotExists)
        }

        pub fn get_by_account(&self, account: &AccountId) -> Result<u64, PSP34Error> {
            self.account_to_index
                .get(account)
                .ok_or(PSP34Error::TokenNotExists)
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        TokenOwnerNotMatch,
        NotApproved,
        CannotTransfer,
        OnlyOwner,
        OnlyAdmin,
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => {
                    Error::Custom(String::from("O::CallerIsNotOwner"))
                }
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    pub const STORAGE_KEY: [u8; 32] = ink_lang::blake2x256!("ArtZeroStakingNFT");

    #[derive(Default)]
    #[openbrush::storage(STORAGE_KEY)]
    struct Manager {
        is_locked: bool,
        admin_address: AccountId,
        staked_accounts: EnumerableMappingForStakedAccount,
        staked_accounts_last_index: u64,
        staking_list: EnumerableMapping,
        staking_list_last_index: Mapping<Option<AccountId>, u64>,
        nft_contract_address: AccountId,
        total_staked: u64,
        limit_unstake_time: u64, // minutes
        pending_unstaking_list: Mapping<(AccountId, u64), u64>,
        pending_unstaking_list_token_index: EnumerableMapping,
        pending_unstaking_list_token_last_index: Mapping<Option<AccountId>, u64>,
        reward_pool: Balance,
        claimable_reward: Balance,
        is_claimed: Mapping<AccountId, bool>,
        _reserved: Option<()>,
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroStakingNFT {
        #[OwnableStorageField]
        ownable: OwnableData,
        manager: Manager,
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

    impl Ownable for ArtZeroStakingNFT {}

    #[openbrush::trait_definition]
    pub trait CrossArtZeroStaking {
        #[ink(message)]
        fn get_total_staked_by_account(&self, account: AccountId) -> u64;
        #[ink(message)]
        fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64;
    }

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;

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
            self.manager.staked_accounts_last_index = 0;
            self.manager.is_locked = false;
            Ok(())
        }

        ///Set new NFT contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_artzero_nft_contract(
            &mut self,
            artzero_nft_contract: AccountId,
        ) -> Result<(), Error> {
            self.manager.nft_contract_address = artzero_nft_contract;
            Ok(())
        }

        ///Set new Limit Unstake Time (Minutes) - Only Owner
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

        /// Update is locked - only Admin
        #[ink(message)]
        pub fn update_is_locked(&mut self, is_locked: bool) -> Result<(), Error> {
            assert!(is_locked != self.manager.is_locked);
            if self.env().caller() == self.manager.admin_address {
                self.manager.is_locked = is_locked;
                if is_locked == true { //when locked, we also lock the reward amount to use for reward calculation
                    self.manager.claimable_reward = self.manager.reward_pool;
                }
                else {
                    self.manager.reward_pool = self.manager.claimable_reward; //unclaimed Reward set back to reward_pool
                }

                Ok(())
            } else {
                return Err(Error::OnlyAdmin);
            }
        }

        /// Add reward to reward_pool
        #[ink(message)]
        #[ink(payable)]
        pub fn add_reward(&mut self) -> Result<(), Error> {
            let reward = self.env().transferred_value();
            assert!(reward>0);
            assert!(self.manager.is_locked); //Only allow adding reward when contract is locked
            self.manager.reward_pool = self.manager.reward_pool.checked_add(reward).unwrap();
            self.env().emit_event(AddReward {
                reward_amount: reward,
                total_staked_amount: self.manager.total_staked
            });
            Ok(())
        }

        /// Set Account so it can claim the reward. Must run by backend every month before add_reward
        #[ink(message)]
        pub fn set_claimable(&mut self) -> Result<(), Error> {
            assert!(self.manager.is_locked);
            let caller = self.env().caller();
            assert!(caller == self.manager.admin_address);
            self.manager.is_claimed.insert(&caller,&false); //Can only claim once
            Ok(())
        }

        /// Claim Reward
        #[ink(message)]
        pub fn claim_reward(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            let is_claimed = self.manager.is_claimed.get(caller);
            assert!(is_claimed.is_some());
            assert!(is_claimed.unwrap() == false);
            self.manager.is_claimed.insert(&caller,&true); //Can only claim once

            assert!(self.manager.total_staked>0);
            assert!(self.manager.is_locked); //Only allow when locked

            let staked_amount = self.manager.staking_list_last_index.get(Some(caller));
            assert!(staked_amount.is_some());
            assert!(staked_amount.unwrap()>0);
            assert!(self.manager.reward_pool>0);

            let reward = (self.manager.reward_pool * (staked_amount.unwrap() as u128)) / (self.manager.total_staked as u128);

            if self.manager.claimable_reward >= reward {
                self.manager.claimable_reward = self.manager.claimable_reward.checked_sub(reward).unwrap();
                if self.env().transfer(caller, reward).is_err() {
                    panic!("error claim reward")
                }
                self.env().emit_event(ClaimReward {
                    staker: Some(caller),
                    reward_amount: reward,
                    staked_amount: staked_amount.unwrap(),
                });
            }
            else {
                if self.env().transfer(caller, self.manager.claimable_reward).is_err() {
                    panic!("error claim reward")
                }
                self.env().emit_event(ClaimReward {
                    staker: Some(caller),
                    reward_amount: self.manager.claimable_reward,
                    staked_amount: staked_amount.unwrap(),
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
        pub fn is_claimed(&self) -> bool {
            let is_claimed = self.manager.is_claimed.get(self.env().caller());
            if is_claimed.is_some() {
                return is_claimed.unwrap();
            }
            return false;
        }
        ///Get NFT contract address
        #[ink(message)]
        pub fn get_artzero_nft_contract(&self) -> AccountId {
            self.manager.nft_contract_address
        }

        ///Get Is Locked Status
        #[ink(message)]
        pub fn get_is_locked(&self) -> bool {
            self.manager.is_locked
        }

        ///Get Admin Account
        #[ink(message)]
        pub fn get_admin_address(&self) -> AccountId {
            self.manager.admin_address
        }

        ///Get request unstake Time
        #[ink(message)]
        pub fn get_request_unstake_time(&self, account: AccountId, token_id: u64) -> u64 {
            let ret = self
                .manager
                .pending_unstaking_list
                .get(&(account, token_id));
            if ret.is_some() {
                return ret.unwrap();
            } else {
                return 0;
            }
        }

        ///Get staked token ids by AccountId
        #[ink(message)]
        pub fn get_staked_id(&self, account: AccountId, index: u64) -> u64 {
            self.manager
                .staking_list
                .get_by_index(&Some(account), &index)
                .unwrap()
        }

        ///Get staked accounts: index by AccountId
        #[ink(message)]
        pub fn get_staked_accounts_index_by_account(&self, account: AccountId) -> u64 {
            if self
                .manager
                .staked_accounts
                .get_by_account(&account)
                .is_err()
            {
                return 0;
            } else {
                return self
                    .manager
                    .staked_accounts
                    .get_by_account(&account)
                    .unwrap();
            }
        }

        ///Get staked accounts: account by Index
        #[ink(message)]
        pub fn get_staked_accounts_account_by_index(&self, index: u64) -> Option<AccountId> {
            if self.manager.staked_accounts.get_by_index(&index).is_err() {
                return None;
            } else {
                return Some(self.manager.staked_accounts.get_by_index(&index).unwrap());
            }
        }

        ///Get pending unstaked token ids by AccountId
        #[ink(message)]
        pub fn get_pending_unstaked_id(&self, account: AccountId, index: u64) -> u64 {
            self.manager
                .pending_unstaking_list_token_index
                .get_by_index(&Some(account), &index)
                .unwrap()
        }

        ///Get total NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked(&self) -> u64 {
            self.manager.total_staked
        }

        #[ink(message)]
        pub fn get_limit_unstake_time(&self) -> u64 {
            self.manager.limit_unstake_time
        }

        /// Get staked accounts last index
        #[ink(message)]
        pub fn get_staked_accounts_last_index(&self) -> u64 {
            self.manager.staked_accounts_last_index
        }

        /// Stake multiple NFTs - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        pub fn stake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();
            let mut last_index = 0;
            if self
                .manager
                .staking_list_last_index
                .get(Some(caller))
                .is_some()
            {
                last_index = self
                    .manager
                    .staking_list_last_index
                    .get(Some(caller))
                    .unwrap();
            }
            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            for i in 0..leng {
                //Step 1 - Check if the token is belong to caller
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i]))
                        .unwrap();
                assert!(caller == token_owner);
                //Step 2 - Check if this contract has been approved
                let allowance = Psp34Ref::allowance(
                    &self.manager.nft_contract_address,
                    caller,
                    self.env().account_id(),
                    Some(Id::U64(token_ids[i])),
                );
                assert!(allowance);
                self.manager.staking_list.insert(
                    &Some(caller),
                    &token_ids[i],
                    &(last_index + (i as u64) + 1),
                );
                //Step 3 - Transfer Token from Caller to Staking Contract
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
                    return Err(Error::CannotTransfer);
                }
                self.env().emit_event(NewStakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            self.manager
                .staking_list_last_index
                .insert(Some(caller), &(last_index + leng as u64));
            if self
                .manager
                .staked_accounts
                .get_by_account(&caller)
                .is_err()
            {
                self.manager.staked_accounts_last_index = self
                    .manager
                    .staked_accounts_last_index
                    .checked_add(1)
                    .unwrap();
                self.manager
                    .staked_accounts
                    .insert(&caller, &self.manager.staked_accounts_last_index);
            }
            Ok(())
        }

        /// Request Unstake multiple NFTs
        #[ink(message)]
        pub fn request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();
            assert!(self
                .manager
                .staking_list_last_index
                .get(Some(caller))
                .is_some());
            let mut last_index = self
                .manager
                .staking_list_last_index
                .get(Some(caller))
                .unwrap();
            let mut pending_unstaking_last_index = 0;
            if self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(caller))
                .is_some()
            {
                pending_unstaking_last_index = self
                    .manager
                    .pending_unstaking_list_token_last_index
                    .get(Some(caller))
                    .unwrap();
            }
            for i in 0..leng {
                //Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i]))
                        .unwrap();
                assert!(self.env().account_id() == token_owner);
                //Setp 2 - Check staker
                if self
                    .manager
                    .staking_list
                    .get_by_id(&Some(caller), &token_ids[i])
                    .is_err()
                {
                    panic!("error: not exist staked")
                };
                //Step 3 - Remove token on staking_list
                assert!(self
                    .manager
                    .staking_list
                    .remove(&Some(caller), &token_ids[i], &last_index)
                    .is_ok());
                last_index = last_index.checked_sub(1).unwrap();
                //Step 4 - Add token to pending unstaking list and pending_unstaking_list_token_index
                let current_time = Self::env().block_timestamp();
                self.manager
                    .pending_unstaking_list
                    .insert(&(caller, token_ids[i]), &current_time);
                self.manager.pending_unstaking_list_token_index.insert(
                    &Some(caller),
                    &token_ids[i],
                    &(pending_unstaking_last_index + (i as u64) + 1),
                );
                self.env().emit_event(RequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            if last_index == 0 {
                self.manager
                    .staked_accounts
                    .remove(&caller, &self.manager.staked_accounts_last_index);
                self.manager.staked_accounts_last_index = self
                    .manager
                    .staked_accounts_last_index
                    .checked_sub(1)
                    .unwrap();
            }
            self.manager
                .staking_list_last_index
                .insert(Some(caller), &last_index);
            self.manager.total_staked = self.manager.total_staked.checked_sub(leng as u64).unwrap();
            self.manager
                .pending_unstaking_list_token_last_index
                .insert(Some(caller), &(pending_unstaking_last_index + leng as u64));
            Ok(())
        }

        /// Cancel Request
        #[ink(message)]
        pub fn cancel_request_unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            let caller = self.env().caller();
            let leng = token_ids.len();
            let mut last_index = 0;
            if self
                .manager
                .staking_list_last_index
                .get(Some(caller))
                .is_some()
            {
                last_index = self
                    .manager
                    .staking_list_last_index
                    .get(Some(caller))
                    .unwrap();
            }
            assert!(self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(caller))
                .is_some());
            let mut pending_unstaking_last_index = self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(caller))
                .unwrap();
            for i in 0..leng {
                //Step 1 - Check owner token is Contract Staking
                let token_owner =
                    Psp34Ref::owner_of(&self.manager.nft_contract_address, Id::U64(token_ids[i]))
                        .unwrap();
                assert!(self.env().account_id() == token_owner);
                //Setp 2 - Check staker
                if self
                    .manager
                    .pending_unstaking_list_token_index
                    .get_by_id(&Some(caller), &token_ids[i])
                    .is_err()
                {
                    panic!("error: not exist request unstaked")
                };
                //Step 3 - Add token on staking_list
                self.manager.staking_list.insert(
                    &Some(caller),
                    &token_ids[i],
                    &(last_index + (i as u64) + 1),
                );
                last_index = last_index.checked_add(1).unwrap();
                //Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                assert!(self
                    .manager
                    .pending_unstaking_list_token_index
                    .remove(&Some(caller), &token_ids[i], &pending_unstaking_last_index)
                    .is_ok());
                pending_unstaking_last_index = pending_unstaking_last_index.checked_sub(1).unwrap();
                self.env().emit_event(CancelRequestUnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            if self
                .manager
                .staked_accounts
                .get_by_account(&caller)
                .is_err()
            {
                self.manager.staked_accounts_last_index = self
                    .manager
                    .staked_accounts_last_index
                    .checked_add(1)
                    .unwrap();
                self.manager
                    .staked_accounts
                    .insert(&caller, &self.manager.staked_accounts_last_index);
            }
            self.manager.total_staked = self.manager.total_staked.checked_add(leng as u64).unwrap();
            self.manager
                .pending_unstaking_list_token_last_index
                .insert(Some(caller), &(pending_unstaking_last_index));
            self.manager
                .staking_list_last_index
                .insert(Some(caller), &(last_index));
            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        pub fn unstake(&mut self, token_ids: Vec<u64>) -> Result<(), Error> {
            assert!(self.manager.is_locked == false);
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let leng = token_ids.len();
            assert!(self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(caller))
                .is_some());
            let mut pending_unstaking_last_index = self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(caller))
                .unwrap();
            for i in 0..leng {
                //Setp 2 - Check request unstaked
                if self
                    .manager
                    .pending_unstaking_list_token_index
                    .get_by_id(&Some(caller), &token_ids[i])
                    .is_err()
                {
                    panic!("error: not exist request unstaked")
                };
                //Step 2 - transfer token to caller Check request unstake
                let request_unstake_time = self.get_request_unstake_time(caller, token_ids[i]);
                assert!(request_unstake_time > 0);
                let current_time = Self::env().block_timestamp();

                if request_unstake_time + (self.manager.limit_unstake_time * 60000) > current_time {
                    return Err(Error::Custom(String::from(
                        "Not Enough Time Request Unstake",
                    )));
                }
                //Step 3 - transfer token to caller
                assert!(Psp34Ref::transfer(
                    &self.manager.nft_contract_address,
                    caller,
                    Id::U64(token_ids[i]),
                    Vec::<u8>::new()
                )
                .is_ok());
                //Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                assert!(self
                    .manager
                    .pending_unstaking_list_token_index
                    .remove(&Some(caller), &token_ids[i], &pending_unstaking_last_index)
                    .is_ok());
                pending_unstaking_last_index = pending_unstaking_last_index.checked_sub(1).unwrap();
                self.env().emit_event(UnstakeEvent {
                    staker: Some(caller),
                    token_id: token_ids[i],
                });
            }
            self.manager
                .pending_unstaking_list_token_last_index
                .insert(Some(caller), &pending_unstaking_last_index);
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
        ///Transfer NFT token
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

    impl CrossArtZeroStaking for ArtZeroStakingNFT {
        /* GETTERS */
        ///Get User NFT staked in the contract
        #[ink(message)]
        fn get_total_staked_by_account(&self, account: AccountId) -> u64 {
            let last_index = self.manager.staking_list_last_index.get(Some(account));
            if last_index.is_some() {
                return last_index.unwrap();
            }
            return 0;
        }

        ///Get User NFT staked in the contract
        #[ink(message)]
        fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64 {
            let last_index = self
                .manager
                .pending_unstaking_list_token_last_index
                .get(Some(account));
            if last_index.is_some() {
                return last_index.unwrap();
            }
            return 0;
        }
    }
}
