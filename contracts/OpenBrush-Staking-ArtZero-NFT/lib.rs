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
        id_to_index: Mapping<(Option<AccountId>, u32), u32>,
        /// Mapping from owner's index to `Id`.
        ///
        /// ** Note ** Owner can be `None` that means it is a contract.
        index_to_id: Mapping<(Option<AccountId>, u32), u32>,
    }

    impl EnumerableMapping {
        pub fn insert(&mut self, owner: &Option<AccountId>, id: &u32, index: &u32) {
            self.id_to_index.insert((owner, id), index);
            self.index_to_id.insert((owner, index), id);
        }

        pub fn remove(&mut self, owner: &Option<AccountId>, id: &u32, last_index: &u32) -> Result<(), PSP34Error> {
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

        pub fn get_by_index(&self, owner: &Option<AccountId>, index: &u32) -> Result<u32, PSP34Error> {
            self.index_to_id.get((owner, index)).ok_or(PSP34Error::TokenNotExists)
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
        staking_list_last_index: Mapping<Option<AccountId>, u32>,
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

    #[brush::trait_definition]
    pub trait CrossArtZeroStaking {
        #[ink(message)]
        fn get_total_staked_by_account(&self,account: AccountId) -> u32;
    }

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
        pub fn get_staked_id(&self,account: AccountId, index: u32) -> u32 {
            self.staking_list.get_by_index(&Some(account),&index).unwrap()
        }
        ///Get total NFT staked in the contract
        #[ink(message)]
        pub fn get_total_staked(&self) -> u32 {
            self.total_staked
        }

        /// Stake multiple NFTs - Make sure approve this contract can send token on owner behalf
        #[ink(message)]
        pub fn stake(&mut self,token_ids:Vec<u32>) -> Result<(), Error> {

            let caller = self.env().caller();
            let leng = token_ids.len();

            let mut last_index = 0;
            if self.staking_list_last_index.get(Some(caller)).is_some() {
                last_index = self.staking_list_last_index.get(Some(caller)).unwrap();
            }

            self.total_staked = self.total_staked.checked_add(leng as u32).unwrap();

            for i in 0..leng{
                //Step 1 - Check if the token is belong to caller
                let token_owner = Psp34Ref::owner_of(&self.nft_contract_address,Id::U32(token_ids[i])).unwrap();
                assert!(caller == token_owner);

                //Step 2 - Check if this contract has been approved
                let allowance = Psp34Ref::allowance(&self.nft_contract_address,caller, self.env().account_id(), Some(Id::U32(token_ids[i])));
                assert!(allowance);

                self.staking_list.insert(&Some(caller),&token_ids[i],&(last_index+(i as u32)+1));

                //Step 3 - Transfer Token from Caller to Staking Contract
                if !PSP34Ref::transfer_builder(&self.nft_contract_address, self.env().account_id(), Id::U32(token_ids[i]), Vec::<u8>::new())
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire().is_ok() {
                    return Err(Error::CannotTransfer);
                }
                self.env().emit_event(NewStakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            self.staking_list_last_index.insert(Some(caller),&(last_index+leng as u32));

            Ok(())
        }

        /// unStake multiple NFTs
        #[ink(message)]
        pub fn unstake(&mut self,token_ids:Vec<u32>) -> Result<(), Error> {
            //Step 1 - Check if the token is belong to caller
            let caller = self.env().caller();
            let leng = token_ids.len();

            assert!(self.staking_list_last_index.get(Some(caller)).is_some());

            let mut last_index = self.staking_list_last_index.get(Some(caller)).unwrap();

            self.total_staked = self.total_staked.checked_sub(leng as u32).unwrap();

            for i in 0..leng{
                assert!(self.staking_list.remove(&Some(caller),&token_ids[i],&last_index).is_ok());
                last_index = last_index.checked_sub(1).unwrap();
                //Step 2 - transfer token to caller
                assert!(Psp34Ref::transfer(&self.nft_contract_address,caller,Id::U32(token_ids[i]),Vec::<u8>::new()).is_ok());

                self.env().emit_event(UnstakeEvent {
                    staker:Some(caller),
                    token_id:token_ids[i]
                });
            }
            self.staking_list_last_index.insert(Some(caller),&last_index);

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
        fn get_total_staked_by_account(&self,account: AccountId) -> u32 {
            let last_index = self.staking_list_last_index.get(Some(account));
            if last_index.is_some(){
                return last_index.unwrap();
            }
            return 0;
        }
    }
}
