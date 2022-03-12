#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_marketplace_psp34 {
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
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
        TokenOwnerNotMatch,
        NotApproved,
        CannotTransfer,
        NotListed
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[derive(
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        PackedLayout,
        SpreadLayout,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct ForSaleItem {
        nft_owner: AccountId,
        listed_date: u64,
        price: Balance,
        is_for_sale: bool
    }

    #[derive(
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        PackedLayout,
        SpreadLayout,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct BidInformation {
        bidder: AccountId,
        bid_date: u64,
        price: Balance
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroMarketplacePSP34{
        #[OwnableStorageField]
        ownable: OwnableData,
        collection_contract_address: AccountId,
        staking_contract_address: AccountId,
        //List of all tokens for sale in the marketplace,
        // (NFT Contract Address, Token ID) --> Token Sale Information
        market_list: Mapping<(AccountId,Id),ForSaleItem>,
        //To find (NFT Contract Address, Token ID) quickly
        //the mapping save all tokens for sale in a collection to an array
        sale_tokens_ids: Mapping<AccountId, Vec<u32>>,
        bidders: Mapping<(AccountId,Id),Vec<BidInformation>>
    }

    impl Ownable for ArtZeroMarketplacePSP34 {}

    #[brush::trait_definition]
    pub trait ArtZeroStaking {
        #[ink(message)]
        fn get_total_staked_by_account(&self,account: AccountId) -> u32;
    }

    #[brush::trait_definition]
    pub trait ArtZeroCollection {
        #[ink(message)]
        fn get_royal_fee(&self,nft_contract_address: AccountId) -> u32;
        fn is_active(&self,nft_contract_address: AccountId) -> bool;
        fn get_contract_type(&self,nft_contract_address: AccountId) -> u8;
    }

    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    pub type StakingRef = dyn ArtZeroStaking;
    pub type CollectionRef = dyn ArtZeroCollection;

    #[ink(event)]
    pub struct NewListEvent {
        trader: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id
    }
    #[ink(event)]
    pub struct DeListEvent {
        trader: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id
    }

    impl ArtZeroMarketplacePSP34 {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId,collection_contract_address: AccountId, staking_contract_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.collection_contract_address = collection_contract_address;
                instance.staking_contract_address = staking_contract_address;
            })
        }

        /* MARKETPLACE FUNCTIONS */
        /// List the token on the marketplace - FREE
        #[ink(message)]
        pub fn list(&mut self, nft_contract_address: AccountId, token_id: Id, price: Balance) -> Result<(),Error>{
            //Step 1: Check if the callet is the owner of the token
            let caller = self.env().caller();
            let token_owner = Psp34Ref::owner_of(&nft_contract_address,token_id.clone()).unwrap();
            assert!(caller == token_owner);

            //Step 2 - Check if this contract has been approved
            let approved_account = Psp34Ref::get_approved(&nft_contract_address,token_id.clone()).unwrap();
            assert!(approved_account == self.env().account_id());

            {
                //Check the sale token ids list
                if self.sale_tokens_ids.get(&nft_contract_address).is_some(){
                    let mut token_ids:Vec<Id> = self.sale_tokens_ids.get(&nft_contract_address).unwrap();
                    token_ids.push(token_id.clone());
                    self.sale_tokens_ids.insert(&nft_contract_address, Id::U32(&token_ids));
                }
                else{
                    //new listing
                    let mut token_ids = Vec::<Id>::new();
                    token_ids.push(token_id.clone());
                    self.sale_tokens_ids.insert(&nft_contract_address, Id::U32(&token_ids));
                }

                let new_sale = ForSaleItem {
                    nft_owner: token_owner,
                    listed_date: self.env().block_timestamp(),
                    price: price,
                    is_for_sale: true
                };
                self.market_list.insert(&(nft_contract_address,token_id.clone()), &new_sale);

            }

            //Step 3 - Transfer Token from Caller to Marketplace Contract
            if !PSP34Ref::transfer_from_builder(&nft_contract_address, caller, self.env().account_id(), token_id.clone(), Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire().is_ok() {
                return Err(Error::CannotTransfer);
            }
            self.env().emit_event(NewListEvent {
                trader:Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id:token_id.clone()
            });

            Ok(())
        }
        /// Delist the token from the marketplace - FREE
        #[ink(message)]
        pub fn delist(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(),Error>{
            //Step 1: Check if the token is for sale and belongs to caller
            let caller = self.env().caller();
            let mut for_sale_token_ids:Vec<u32> = self.sale_tokens_ids.get(&nft_contract_address).unwrap();
            assert!(for_sale_token_ids.contains(&Id::U32(token_id.clone())));

            //remove from sale array
            let index = for_sale_token_ids.iter().position(|&r| r == Id::U32(token_id.clone())).unwrap();
            let checked_id = for_sale_token_ids[index];
            assert_eq!(for_sale_token_ids.remove(index), checked_id);

            self.sale_tokens_ids.insert(&nft_contract_address, &for_sale_token_ids);

            //remove from market list
            if self.market_list.get(&(nft_contract_address,token_id.clone())).is_some(){
                let mut token_sale_info:ForSaleItem = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
                assert!(token_sale_info.nft_owner == caller);

                token_sale_info.is_for_sale = false;
                self.market_list.insert(&(nft_contract_address,token_id.clone()), &token_sale_info);
            }
            else{
                return Err(Error::NotListed);
            }

            //Clear Bidders
            let bidders = Vec::<BidInformation>::new();
            self.bidders.insert(&(nft_contract_address,token_id.clone()), &bidders);

            Ok(())
        }
        // SETTERS
        ///Set new collection contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_collection_contract_address(&mut self, collection_contract_address: AccountId) -> Result<(),Error> {
            self.collection_contract_address = collection_contract_address;
            Ok(())
        }
        ///Set new staking contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_staking_contract_address(&mut self, staking_contract_address: AccountId) -> Result<(),Error> {
            self.staking_contract_address = staking_contract_address;
            Ok(())
        }

        // GETTERS
        /// Get market list information using NFT Collection and token ID
        #[ink(message)]
        pub fn get_nft_sale_info(&self,nft_contract_address:AccountId, token_id: Id) -> ForSaleItem {
            self.market_list.get(&(nft_contract_address,token_id)).unwrap()
        }
        ///Get all token ids currently for sale for a collection (nft_contract_address)
        #[ink(message)]
        pub fn get_for_sale_token_ids(&self,nft_contract_address:AccountId) -> Vec<Id> {
            self.sale_tokens_ids.get(&nft_contract_address).unwrap()
        }
        ///Get all token ids currently for sale by a collection (nft_contract_address)
        #[ink(message)]
        pub fn total_tokens_for_sale(&self,nft_contract_address:AccountId) -> u64 {
            self.sale_tokens_ids.get(&nft_contract_address).unwrap().len() as u64
        }

        ///Get collection contract address
        #[ink(message)]
        pub fn get_collection_contract_address(&self) -> AccountId {
            self.collection_contract_address
        }
        ///Get staking contract address
        #[ink(message)]
        pub fn get_staking_contract_address(&self) -> AccountId {
            self.staking_contract_address
        }

    }
}
