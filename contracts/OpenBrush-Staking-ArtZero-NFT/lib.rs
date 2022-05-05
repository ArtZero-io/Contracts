#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_staking_nft {
    use ink_env::CallFlags;
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::contracts::traits::{
        psp34::{
            extensions::{
                burnable::*,
                metadata::*
            },
            *,
        },
    };
    use brush::modifiers;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct EnumerableMapping {
        /// Mapping from index to `Id`.
        ///
        /// ** Note ** Owner can be `None` that means it is a contract.
        id_to_index: Mapping<(Option<AccountId>, u64), u64>,
        /// Mapping from owner's index to `Id`.
        ///
        /// ** Note ** Owner can be `None` that means it is a contract.
        index_to_id: Mapping<(Option<AccountId>, u64), u64>,
    }

    impl EnumerableMapping {
        pub fn insert(&mut self, owner: &Option<AccountId>, id: &u64, index: &u64) {
            self.id_to_index.insert((owner, id), index);
            self.index_to_id.insert((owner, index), id);
        }

        pub fn remove(&mut self, owner: &Option<AccountId>, id: &u64, last_index: &u64) -> Result<(), PSP34Error> {
            let index = self.id_to_index.get((owner, id)).ok_or(PSP34Error::TokenNotExists)?;

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

        pub fn get_by_index(&self, owner: &Option<AccountId>, index: &u64) -> Result<u64, PSP34Error> {
            self.index_to_id.get((owner, index)).ok_or(PSP34Error::TokenNotExists)
        }

        pub fn get_by_id(&self, owner: &Option<AccountId>, id: &u64) -> Result<u64, PSP34Error> {
            self.id_to_index.get((owner, id)).ok_or(PSP34Error::TokenNotExists)
        }
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        TokenOwnerNotMatch,
        NotApproved,
        CannotTransfer
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroStakingNFT{
        #[OwnableStorageField]
        ownable: OwnableData,
        staking_list: EnumerableMapping,
        staking_list_last_index: Mapping<Option<AccountId>, u64>,
        nft_contract_address: AccountId,
        total_staked: u64,
        limit_unstake_time: u64, // minutes
        pending_unstaking_list: Mapping<(AccountId, u64), u64>,
        pending_unstaking_list_token_index: EnumerableMapping,
        pending_unstaking_list_token_last_index: Mapping<Option<AccountId>, u64>
    }

    #[ink(event)]
    pub struct NewStakeEvent {
        staker: Option<AccountId>,
        token_id: u64
    }
    #[ink(event)]
    pub struct UnstakeEvent {
        staker: Option<AccountId>,
        token_id: u64
    }
    impl Ownable for ArtZeroStakingNFT {}

    #[brush::trait_definition]
    pub trait CrossArtZeroStaking {
        #[ink(message)]
        fn get_total_staked_by_account(&self,account: AccountId) -> u64;

        #[ink(message)]
        fn get_total_pending_unstaked_by_account(&self,account: AccountId) -> u64;
    }

    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;

    impl ArtZeroStakingNFT {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId,artzero_nft_contract: AccountId, limit_unstake_time: u64) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.nft_contract_address = artzero_nft_contract;
                instance.limit_unstake_time = limit_unstake_time;
            })
        }
        // SETTERS
        ///Set new NFT contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_artzero_nft_contract(&mut self, artzero_nft_contract: AccountId) -> Result<(),Error> {
            self.nft_contract_address = artzero_nft_contract;
            Ok(())
        }

        ///Set new Limit Unstake Time (Minutes) - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_limit_unstake_time(&mut self, limit_unstake_time: u64) -> Result<(),Error> {
            self.limit_unstake_time = limit_unstake_time;
            Ok(())
        }

        // GETTERS
        ///Get NFT contract address
        #[ink(message)]
        pub fn get_artzero_nft_contract(&self) -> AccountId {
            self.nft_contract_address
        }

        ///Get request unstake Time
        #[ink(message)]
        pub fn get_request_unstake_time(&self, account: AccountId, token_id: u64) -> u64 {
            let ret = self.pending_unstaking_list.get(&(account, token_id));
            if ret.is_some(){
                return ret.unwrap()
            }
            else{
                return 0
            }
        }

        ///Get staked token ids by AccountId
        #[ink(message)]
        pub fn get_staked_id(&self,account: AccountId, index: u64) -> u64 {
            self.staking_list.get_by_index(&Some(account),&index).unwrap()
        }

        ///Get pending unstaked token ids by AccountId
        #[ink(message)]
        pub fn get_pending_unstaked_id(&self,account: AccountId, index: u64) -> u64 {
            self.pending_unstaking_list_token_index.get_by_index(&Some(account),&index).unwrap()
        }

        ///Get total NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked(&self) -> u64 {
            self.total_staked
        }

        #[ink(message)]
        pub fn get_limit_unstake_time(&self) -> u64 {
            self.limit_unstake_time
        }

        /// Stake multiple NFTs - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        pub fn stake(&mut self,token_ids:Vec<u64>) -> Result<(), Error> {

            let caller = self.env().caller();
            let leng = token_ids.len();

            let mut last_index = 0;
            if self.staking_list_last_index.get(Some(caller)).is_some() {
                last_index = self.staking_list_last_index.get(Some(caller)).unwrap();
            }

            self.total_staked = self.total_staked.checked_add(leng as u64).unwrap();

            for i in 0..leng{
                //Step 1 - Check if the token is belong to caller
                let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U64(token_ids[i])).unwrap();
                assert!(caller == token_owner);

                //Step 2 - Check if this contract has been approved
                let allowance = Psp34Ref::allowance(&self.nft_contract_address,caller, self.env().account_id(), Some(Id::U64(token_ids[i])));
                assert!(allowance);

                self.staking_list.insert(&Some(caller),&token_ids[i],&(last_index+(i as u64)+1));

                //Step 3 - Transfer Token from Caller to Staking Contract
                if !PSP34Ref::transfer_builder(&self.nft_contract_address, self.env().account_id(), Id::U64(token_ids[i]), Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire().is_ok() {
                    return Err(Error::CannotTransfer);
                }
                self.env().emit_event(NewStakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            self.staking_list_last_index.insert(Some(caller),&(last_index+leng as u64));

            Ok(())
        }

        /// Request Unstake multiple NFTs
        #[ink(message)]
        pub fn request_unstake(&mut self, token_ids:Vec<u64>) -> Result<(), Error> {
            let caller = self.env().caller();
            let leng = token_ids.len();

            assert!(self.staking_list_last_index.get(Some(caller)).is_some());
            let mut last_index = self.staking_list_last_index.get(Some(caller)).unwrap();

            let mut pending_unstaking_last_index = 0;
            if self.pending_unstaking_list_token_last_index.get(Some(caller)).is_some() {
                pending_unstaking_last_index = self.pending_unstaking_list_token_last_index.get(Some(caller)).unwrap();
            }

            for i in 0..leng {
                //Step 1 - Check owner token is Contract Staking
                let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U64(token_ids[i])).unwrap();
                assert!(self.env().account_id() == token_owner);

                //Setp 2 - Check staker
                if self.staking_list.get_by_id(&Some(caller), &token_ids[i]).is_err() {
                    panic!(
                        "error: not exist staked"
                    )
                };

                //Step 3 - Remove token on staking_list
                assert!(self.staking_list.remove(&Some(caller),&token_ids[i],&last_index).is_ok());
                last_index = last_index.checked_sub(1).unwrap();

                //Step 3 - Add token to pending unstaking list and pending_unstaking_list_token_index
                let current_time = Self::env().block_timestamp();
                self.pending_unstaking_list.insert(&(caller, token_ids[i]), &current_time);
                self.pending_unstaking_list_token_index.insert(&Some(caller),&token_ids[i],&(pending_unstaking_last_index+(i as u64)+1));
            }
            self.staking_list_last_index.insert(Some(caller),&last_index);
            self.total_staked = self.total_staked.checked_sub(leng as u64).unwrap();
            self.pending_unstaking_list_token_last_index.insert(Some(caller),&(pending_unstaking_last_index+leng as u64));
            Ok(())
        }

        /// Cancel Request
        #[ink(message)]
        pub fn cancel_request_unstake(&mut self, token_ids:Vec<u64>) -> Result<(), Error> {
            let caller = self.env().caller();
            let leng = token_ids.len();

            let mut last_index = 0;
            if self.staking_list_last_index.get(Some(caller)).is_some() {
                last_index = self.staking_list_last_index.get(Some(caller)).unwrap();
            }

            assert!(self.pending_unstaking_list_token_last_index.get(Some(caller)).is_some());
            let mut pending_unstaking_last_index = self.pending_unstaking_list_token_last_index.get(Some(caller)).unwrap();

            for i in 0..leng {
                //Step 1 - Check owner token is Contract Staking
                let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U64(token_ids[i])).unwrap();
                assert!(self.env().account_id() == token_owner);

                //Setp 2 - Check staker
                if self.pending_unstaking_list_token_index.get_by_id(&Some(caller), &token_ids[i]).is_err() {
                    panic!(
                        "error: not exist request unstaked"
                    )
                };

                //Step 3 - Add token on staking_list
                self.staking_list.insert(&Some(caller),&token_ids[i],&(last_index+(i as u64)+1));
                last_index = last_index.checked_add(1).unwrap();

                //Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                assert!(self.pending_unstaking_list_token_index.remove(&Some(caller),&token_ids[i],&pending_unstaking_last_index).is_ok());
                pending_unstaking_last_index = pending_unstaking_last_index.checked_sub(1).unwrap();
            }
            self.total_staked = self.total_staked.checked_add(leng as u64).unwrap();
            self.pending_unstaking_list_token_last_index.insert(Some(caller), &(pending_unstaking_last_index));
            self.staking_list_last_index.insert(Some(caller),&(last_index));
            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        pub fn unstake(&mut self,token_ids:Vec<u64>) -> Result<(), Error> {
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let leng = token_ids.len();

            assert!(self.pending_unstaking_list_token_last_index.get(Some(caller)).is_some());
            // let mut last_index = self.staking_list_last_index.get(Some(caller)).unwrap();
            let mut pending_unstaking_last_index = self.pending_unstaking_list_token_last_index.get(Some(caller)).unwrap();

            for i in 0..leng{

                //Setp 2 - Check request unstaked
                if self.pending_unstaking_list_token_index.get_by_id(&Some(caller), &token_ids[i]).is_err() {
                    panic!(
                        "error: not exist request unstaked"
                    )
                };

                //Step 2 - transfer token to caller Check request unstake
                let request_unstake_time = self.get_request_unstake_time(caller, token_ids[i]);
                assert!(request_unstake_time>0);
                let current_time = Self::env().block_timestamp();

                if request_unstake_time + (self.limit_unstake_time * 60000) > current_time {
                    return Err(Error::Custom(String::from("Not Enough Time Request Unstake")));
                }
                // assert!(self.staking_list.remove(&Some(caller),&token_ids[i],&last_index).is_ok());
                // last_index = last_index.checked_sub(1).unwrap();
                //Step 3 - transfer token to caller
                assert!(Psp34Ref::transfer(&self.nft_contract_address,caller,Id::U64(token_ids[i]),Vec::<u8>::new()).is_ok());

                //Step 4 - Remove from pending_unstaking_list_token_index and change pending_unstaking_list to 0
                assert!(self.pending_unstaking_list_token_index.remove(&Some(caller),&token_ids[i],&pending_unstaking_last_index).is_ok());
                pending_unstaking_last_index = pending_unstaking_last_index.checked_sub(1).unwrap();
                // self.pending_unstaking_list.insert(&(caller, token_ids[i]), 0);

                self.env().emit_event(UnstakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            self.pending_unstaking_list_token_last_index.insert(Some(caller),&pending_unstaking_last_index);

            Ok(())
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            assert!( value <= self.env().balance());

            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }
        ///Transfer NFT token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn tranfer_nft(&mut self, token_id: Id, receiver: AccountId) -> Result<(),Error> {
            assert!(Psp34Ref::transfer(&self.nft_contract_address,receiver,token_id.clone(),Vec::<u8>::new()).is_ok());
            Ok(())
        }
    }

    impl CrossArtZeroStaking for ArtZeroStakingNFT{
        /* GETTERS */
        ///Get User NFT staked in the contract
        #[ink(message)]
        fn get_total_staked_by_account(&self,account: AccountId) -> u64 {
            let last_index = self.staking_list_last_index.get(Some(account));
            if last_index.is_some(){
                return last_index.unwrap();
            }
            return 0;
        }

        ///Get User NFT staked in the contract
        #[ink(message)]
        fn get_total_pending_unstaked_by_account(&self,account: AccountId) -> u64 {
            let last_index = self.pending_unstaking_list_token_last_index.get(Some(account));
            if last_index.is_some(){
                return last_index.unwrap();
            }
            return 0;
        }
    }
}
