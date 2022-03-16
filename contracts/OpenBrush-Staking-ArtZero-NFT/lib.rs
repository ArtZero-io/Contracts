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
        staking_list: Mapping<AccountId, Vec<u32>>,
        nft_contract_address: AccountId,
        total_staked: u32
    }

    #[ink(event)]
    pub struct NewStakeEvent {
        staker: Option<AccountId>,
        token_id: u32
    }
    #[ink(event)]
    pub struct UnstakeEvent {
        staker: Option<AccountId>,
        token_id: u32
    }
    impl Ownable for ArtZeroStakingNFT {}

    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;

    impl ArtZeroStakingNFT {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId,artzero_nft_contract: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.nft_contract_address = artzero_nft_contract;
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

        // GETTERS
        ///Get NFT contract address
        #[ink(message)]
        pub fn get_artzero_nft_contract(&self) -> AccountId {
            self.nft_contract_address
        }
        ///Get staked token ids by AccountId
        #[ink(message)]
        pub fn get_staked_ids(&self,account: AccountId) -> Vec<u32> {
            self.staking_list.get(&account).unwrap()
        }
        ///Get total NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked(&self) -> u32 {
            self.total_staked
        }
        ///Get contract account_id
        #[ink(message)]
        pub fn get_account_id(&self) -> AccountId {
            self.env().account_id()
        }

        ///Get User NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked_by_account(&self,account: AccountId) -> u32 {
            self.staking_list.get(&account).unwrap().len() as u32
        }
        /// Stake NFT - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        pub fn stake(&mut self,token_id:u32) -> Result<(), Error> {
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U32(token_id)).unwrap();
            assert!(caller == token_owner);
            //Step 2 - Check if this contract has been approved
            if !Psp34Ref::is_approved_for_all(&self.nft_contract_address,caller,self.env().account_id()){
                let approved_account = Psp34Ref::get_approved(&self.nft_contract_address,Id::U32(token_id)).unwrap();
                assert!(approved_account == self.env().account_id());
            }

            self.total_staked += 1;
            if self.staking_list.get(&caller).is_some()
            {
                let mut staked_ids:Vec<u32> = self.staking_list.get(&caller).unwrap();
                staked_ids.push(token_id);
                self.staking_list.insert(&caller, &staked_ids);
            }
            else{
                let mut new_staked_ids = Vec::<u32>::new();
                new_staked_ids.push(token_id);
                self.staking_list.insert(&caller, &new_staked_ids);
            }

            //Step 3 - Transfer Token from Caller to Staking Contract
            // if !Psp34Ref::transfer_from(&self.nft_contract_address,caller,self.env().account_id(),Id::U32(token_id),Vec::<u8>::new()).is_ok(){
            //     return Err(Error::CannotTransfer);
            // }
            if !PSP34Ref::transfer_from_builder(&self.nft_contract_address, caller, self.env().account_id(), Id::U32(token_id), Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire().is_ok() {
                return Err(Error::CannotTransfer);
            }
            self.env().emit_event(NewStakeEvent {
                staker:Some(caller),
                token_id:token_id
            });

            Ok(())

        }
        /// unStake NFT
        #[ink(message)]
        pub fn unstake(&mut self,token_id:u32) -> Result<(), Error> {
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let mut staked_ids:Vec<u32> = self.staking_list.get(&caller).unwrap();
            assert!(staked_ids.contains(&token_id));

            self.total_staked = self.total_staked.checked_sub(1).unwrap();
            let index = staked_ids.iter().position(|&r| r == token_id).unwrap();
            let checked_id = staked_ids[index];
            assert_eq!(staked_ids.remove(index), checked_id);

            self.staking_list.insert(&caller, &staked_ids);

            //Step 2 - transfer token to caller
            assert!(Psp34Ref::transfer(&self.nft_contract_address,caller,Id::U32(token_id),Vec::<u8>::new()).is_ok());

            self.env().emit_event(UnstakeEvent {
                staker:Some(caller),
                token_id:token_id
            });
            Ok(())
        }

        /// Stake multiple NFTs - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        pub fn multiple_stake(&mut self,token_ids:Vec<u32>) -> Result<(), Error> {

            let caller = self.env().caller();
            let leng = token_ids.len();
            let is_approve_all = Psp34Ref::is_approved_for_all(&self.nft_contract_address,caller,self.env().account_id());
            for i in 0..leng{
                //Step 1 - Check if the token is belong to caller
                let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U32(token_ids[i])).unwrap();
                assert!(caller == token_owner);
                if !is_approve_all{
                    //Step 2 - Check if this contract has been approved
                    let approved_account = Psp34Ref::get_approved(&self.nft_contract_address,Id::U32(token_ids[i])).unwrap();
                    assert!(approved_account == self.env().account_id());
                }
            }

            self.total_staked = self.total_staked.checked_add(leng as u32).unwrap();

            if self.staking_list.get(&caller).is_some()
            {
                let mut staked_ids:Vec<u32> = self.staking_list.get(&caller).unwrap();
                for i in 0..leng{
                    staked_ids.push(token_ids[i]);
                }
                self.staking_list.insert(&caller, &staked_ids);
            }
            else{
                let mut new_staked_ids = Vec::<u32>::new();
                for i in 0..leng{
                    new_staked_ids.push(token_ids[i]);
                }
                self.staking_list.insert(&caller, &new_staked_ids);
            }
            for i in 0..leng {
                //Step 3 - Transfer Token from Caller to Staking Contract
                if !PSP34Ref::transfer_from_builder(&self.nft_contract_address, caller, self.env().account_id(), Id::U32(token_ids[i]), Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire().is_ok() {
                    return Err(Error::CannotTransfer);
                }
                self.env().emit_event(NewStakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        pub fn multiple_unstake(&mut self,token_ids:Vec<u32>) -> Result<(), Error> {
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let leng = token_ids.len();

            self.total_staked = self.total_staked.checked_sub(leng as u32).unwrap();
            let mut token_indexes = Vec::<usize>::new();
            let mut staked_ids:Vec<u32> = self.staking_list.get(&caller).unwrap();

            for i in 0..leng{
                assert!(staked_ids.contains(&token_ids[i]));
                token_indexes.push(staked_ids.iter().position(|&r| r == token_ids[i]).unwrap());
            }

            token_indexes.sort();
            token_indexes.reverse();
            for r in token_indexes {
                staked_ids.remove(r);
            }
            self.staking_list.insert(&caller, &staked_ids);

            for i in 0..leng{
                //Step 2 - transfer token to caller
                assert!(Psp34Ref::transfer(&self.nft_contract_address,caller,Id::U32(token_ids[i]),Vec::<u8>::new()).is_ok());

                self.env().emit_event(UnstakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            Ok(())
        }
    }
}
