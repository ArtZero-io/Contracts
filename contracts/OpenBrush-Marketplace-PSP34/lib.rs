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
        NotListed,
        BidAlreadyExist,
        BidNotExist,
        NotEnoughBalance
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
        bid_value: Balance
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroMarketplacePSP34{
        #[OwnableStorageField]
        ownable: OwnableData,
        collection_contract_address: AccountId,
        staking_contract_address: AccountId,
        platform_fee: u32,                          //1% = 100
        //List of all tokens for sale in the marketplace,
        // (NFT Contract Address, Token ID) --> Token Sale Information
        market_list: Mapping<(AccountId,Id),ForSaleItem>,
        //To find (NFT Contract Address, Token ID) quickly
        //the mapping save all tokens for sale in a collection to an array
        //(NFT Contract Address, Seller Address) --> list of all token ids
        sale_tokens_ids: Mapping<(AccountId,AccountId), Vec<Id>>,
        //(NFT Contract Address, Seller Address, token ID)
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
        #[ink(message)]
        fn is_active(&self,nft_contract_address: AccountId) -> bool;
        #[ink(message)]
        fn get_contract_type(&self,nft_contract_address: AccountId) -> u8;
        #[ink(message)]
        fn get_collection_owner(&self,nft_contract_address: AccountId) -> AccountId;
    }

    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    #[brush::wrapper]
    pub type StakingRef = dyn ArtZeroStaking;
    #[brush::wrapper]
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
    #[ink(event)]
    pub struct PurchaseEvent {
        buyer: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        platform_fee: Balance,
        royal_fee: Balance
    }

    impl ArtZeroMarketplacePSP34 {
        #[ink(constructor)]
        pub fn new( contract_owner: AccountId,
                    collection_contract_address: AccountId,
                    staking_contract_address: AccountId,
                    platform_fee: u32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                assert!(platform_fee<10000);        //must less than 100%
                instance._init_with_owner(contract_owner);
                instance.collection_contract_address = collection_contract_address;
                instance.staking_contract_address = staking_contract_address;
                instance.platform_fee = platform_fee;
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

            //Check Collection active
            assert!(CollectionRef::is_active(&self.collection_contract_address,nft_contract_address));

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

            //Send NFT back to caller
            assert!(Psp34Ref::transfer(&nft_contract_address,caller,token_id.clone(),Vec::<u8>::new()).is_ok());

            self.env().emit_event(DeListEvent {
                trader:Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id:token_id.clone()
            });

            Ok(())
        }
        /// Buy Token at listed price
        #[ink(message)]
        #[ink(payable)]
        pub fn buy(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(),Error>{

            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let mut sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();
            let seller = sale_information.nft_owner;
            let price = sale_information.price;
            //General checking to ensure its from valid owner and sale is active
            assert!(seller != caller);
            assert!(sale_information.is_for_sale);
            assert!(price == self.env().transferred_value());
            assert!(CollectionRef::is_active(&self.collection_contract_address,nft_contract_address));              //collection must be active
            let contract_type = CollectionRef::get_contract_type(&self.collection_contract_address,nft_contract_address);
            assert!(contract_type<=2 && contract_type>=1);   //psp34 only
            //remove from market list
            sale_information.is_for_sale = false;
            self.market_list.insert(&(nft_contract_address,token_id.clone()), &sale_information);

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

            //Clear Bidders
            let bidders = Vec::<BidInformation>::new();
            self.bidders.insert(&(nft_contract_address,caller,token_id.clone()), &bidders);

            //TODO: Add discount from Staking Amount
            //Calculate fee
            let platform_fee = price.checked_mul(self.platform_fee as u128).unwrap().checked_div(10000).unwrap();
            let royal_fee_rate = CollectionRef::get_royal_fee(&self.collection_contract_address,nft_contract_address);
            let royal_fee = price.checked_mul(royal_fee_rate as u128).unwrap().checked_div(10000).unwrap();

            //Send AZERO to seller / platform and collection owner
            let collection_owner = CollectionRef::get_collection_owner(&self.collection_contract_address,nft_contract_address);
            if royal_fee > 0{
                if self.env().transfer(collection_owner, royal_fee).is_err() {
                    panic!(
                        "error royal_fee"
                    )
                }
            }
            let seller_fee = price.checked_sub(royal_fee).unwrap().checked_sub(platform_fee).unwrap();
            if seller_fee  > 0{
                if self.env().transfer(seller, seller_fee).is_err() {
                    panic!(
                        "error seller_fee"
                    )
                }
            }
            //Send NFT to Buyer
            assert!(Psp34Ref::transfer(&nft_contract_address,caller,token_id.clone(),Vec::<u8>::new()).is_ok());
            self.env().emit_event(DeListEvent {
                trader:Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id:token_id.clone()
            });
            self.env().emit_event(PurchaseEvent {
               buyer: Some(caller),
               seller: Some(seller),
               nft_contract_address: Some(nft_contract_address),
               token_id: token_id.clone(),
               price,
               platform_fee,
               royal_fee
           });
            Ok(())
        }
        /// Bid Token for sale, transferred_value() is the bidding price
        #[ink(message)]
        #[ink(payable)]
        pub fn bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(),Error>{
            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();   //bidder
            let seller = sale_information.nft_owner;
            let price = sale_information.price;
            let bid_value = self.env().transferred_value();
            //General checking to ensure its from valid owner and sale is active
            assert!(seller != caller);
            assert!(sale_information.is_for_sale);
            assert!(price > bid_value);
            assert!(CollectionRef::is_active(&self.collection_contract_address,nft_contract_address));              //collection must be active
            let contract_type = CollectionRef::get_contract_type(&self.collection_contract_address,nft_contract_address);
            assert!(contract_type<=2 && contract_type>=1);   //psp34 only

            //Step 1: Check if the token is for sale
            let for_sale_token_ids:Vec<Id> = self.sale_tokens_ids.get(&(nft_contract_address,seller)).unwrap();
            assert!(for_sale_token_ids.contains(&token_id.clone()));

            let new_bid = BidInformation {
                bidder: caller,
                bid_date: self.env().block_timestamp(),
                bid_value: bid_value
            };

            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let mut bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();
                //Check if Bid already in the list --> not allow, have to remove bid and add new bid
                let length = bidders.len();
                for index in 0..length {
                    if bidders[index as usize].bidder == caller {
                        return Err(Error::BidAlreadyExist);
                    }
                }

                bidders.push(new_bid);
                self.bidders.insert(&(nft_contract_address,seller,token_id.clone()), &bidders);
            }
            else{
                //new bid
                let mut bidders = Vec::<BidInformation>::new();
                bidders.push(new_bid);
                self.bidders.insert(&(nft_contract_address,seller,token_id.clone()), &bidders);
            }

            Ok(())
        }

        /// Remove Bid From Active Sale
        #[ink(message)]
        pub fn remove_bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(),Error>{
            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();   //bidder
            let seller = sale_information.nft_owner;
            //General checking to ensure its from valid owner and sale is active
            assert!(seller != caller);
            assert!(sale_information.is_for_sale);
            assert!(CollectionRef::is_active(&self.collection_contract_address,nft_contract_address));              //collection must be active
            let contract_type = CollectionRef::get_contract_type(&self.collection_contract_address,nft_contract_address);
            assert!(contract_type<=2 && contract_type>=1);   //psp34 only

            //Step 1: Check if the token is for sale
            let for_sale_token_ids:Vec<Id> = self.sale_tokens_ids.get(&(nft_contract_address,seller)).unwrap();
            assert!(for_sale_token_ids.contains(&token_id.clone()));

            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let mut bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();

                //Check if Bid for this caller already in the list
                let length = bidders.len();
                let mut index_bid:i32 = -1;
                let mut bid_value = 0;
                for index in 0..length {
                    if bidders[index as usize].bidder == caller {
                        index_bid = index as i32;
                        bid_value = bidders[index as usize].bid_value;
                        break;
                    }
                }
                assert!(index_bid>=0);
                assert!(bid_value>0);

                //remove bid from bidders list
                bidders.remove(index_bid as usize);
                self.bidders.insert(&(nft_contract_address,seller,token_id.clone()), &bidders);

                //Send bid_value back to caller
                if self.env().transfer(caller, bid_value).is_err() {
                    panic!(
                        "error seller_fee"
                    )
                }

            }
            else{
                return Err(Error::BidNotExist);
            }

            Ok(())
        }
        /// Accept Bid
        #[ink(message)]
        pub fn accept_bid(&mut self, nft_contract_address: AccountId, token_id: Id, bid_index: u32) -> Result<(),Error>{
            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let mut sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();   //seller to accept bid from bidder
            let seller = sale_information.nft_owner;
            //General checking to ensure its from valid owner and sale is active
            assert!(seller == caller);
            assert!(sale_information.is_for_sale);
            assert!(CollectionRef::is_active(&self.collection_contract_address,nft_contract_address));              //collection must be active
            let contract_type = CollectionRef::get_contract_type(&self.collection_contract_address,nft_contract_address);
            assert!(contract_type<=2 && contract_type>=1);   //psp34 only

            //remove from market list
            sale_information.is_for_sale = false;
            self.market_list.insert(&(nft_contract_address,token_id.clone()), &sale_information);

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

            //Check Bidder and Clear
            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();

                let bidders_length = bidders.len();
                for index in 0..bidders_length {
                    if index == bid_index as usize {
                        //Send NFT to bidder
                        assert!(Psp34Ref::transfer(&nft_contract_address,bidders[index].bidder,token_id.clone(),Vec::<u8>::new()).is_ok());
                        //SEnd Azero to seller
                        if self.env().transfer(caller, bidders[index].bid_value).is_err() {
                            panic!(
                                "error"
                            )
                        }
                    }
                    else{
                        //SEnd AZero back to lost bidder
                        if self.env().transfer(bidders[index].bidder, bidders[index].bid_value).is_err() {
                            panic!(
                                "error"
                            )
                        }
                    }
                }



                //Return all money to other lost bidders

                //Clear Bidders
                let bidders = Vec::<BidInformation>::new();
                self.bidders.insert(&(nft_contract_address,caller,token_id.clone()), &bidders);

            }
            else{
                return Err(Error::BidNotExist);
            }
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

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            if value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }

    }
}