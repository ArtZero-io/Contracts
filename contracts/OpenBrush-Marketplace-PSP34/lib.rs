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
        //(NFT Contract Address, User Address) --> list of all token ids
        sale_tokens_ids: Mapping<(AccountId,AccountId), Vec<Id>>,
        //(NFT Contract Address, User Address, token ID)
        bidders: Mapping<(AccountId,AccountId,Id),Vec<BidInformation>>
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
                if self.sale_tokens_ids.get(&(nft_contract_address,caller)).is_some(){
                    let mut token_ids:Vec<Id> = self.sale_tokens_ids.get(&(nft_contract_address,caller)).unwrap();
                    token_ids.push(token_id.clone());
                    self.sale_tokens_ids.insert(&(nft_contract_address,caller), &token_ids);
                }
                else{
                    //new listing
                    let mut token_ids = Vec::<Id>::new();
                    token_ids.push(token_id.clone());
                    self.sale_tokens_ids.insert(&(nft_contract_address,caller), &token_ids);
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

            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let mut sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();
            //General checking to ensure its from valid owner and sale is active

            assert!(sale_information.nft_owner == caller);
            assert!(sale_information.is_for_sale);

            //remove from market list
            sale_information.is_for_sale = false;

            //Step 1: Check if the token is for sale
            let mut for_sale_token_ids:Vec<Id> = self.sale_tokens_ids.get(&(nft_contract_address,caller)).unwrap();
            assert!(for_sale_token_ids.contains(&token_id.clone()));

            //remove from sale array
            let length = for_sale_token_ids.len();
            for index in 0..length {
                if for_sale_token_ids[index as usize] == token_id {
                    for_sale_token_ids.remove(index);
                    break;
                }
            }
            //Save the information
            self.sale_tokens_ids.insert(&(nft_contract_address,caller), &for_sale_token_ids);
            self.market_list.insert(&(nft_contract_address,token_id.clone()), &sale_information);

            //Clear Bidders
            let bidders = Vec::<BidInformation>::new();
            self.bidders.insert(&(nft_contract_address,caller,token_id.clone()), &bidders);

            Ok(())
        }
        /// Buy Token at listed price

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
        ///Get all token ids currently for sale for a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn get_for_sale_token_ids(&self,nft_contract_address:AccountId, user_account:AccountId) -> Vec<Id> {
            self.sale_tokens_ids.get(&(nft_contract_address,user_account)).unwrap()
        }
        ///Get all token ids currently for sale by a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn total_tokens_for_sale(&self,nft_contract_address:AccountId, user_account:AccountId) -> u64 {
            self.sale_tokens_ids.get(&(nft_contract_address,user_account)).unwrap().len() as u64
        }
        ///Get all bids from (NFT Contract Address, User Address, token ID)
        #[ink(message)]
        pub fn get_all_bids(&self,nft_contract_address:AccountId, user_account:AccountId, token_id: Id) -> Vec<BidInformation> {
            self.bidders.get(&(nft_contract_address,user_account, token_id)).unwrap()
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
