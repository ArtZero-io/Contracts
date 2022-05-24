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
    #[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct EnumerableMapping {
        /// Mapping from index to `Id`.
        ///
        id_to_index: Mapping<(Option<AccountId>, Option<AccountId>, Id), u128>,
        /// Mapping from owner's index to `Id`.
        ///
        index_to_id: Mapping<(Option<AccountId>, Option<AccountId>, u128), Id>,
    }

    impl EnumerableMapping {
        pub fn insert(&mut self, nft_contract_address: &Option<AccountId>, seller: &Option<AccountId>, id: &Id, index: &u128) {
            self.id_to_index.insert((nft_contract_address, seller, id), index);
            self.index_to_id.insert((nft_contract_address, seller, index), id);
        }

        pub fn remove(&mut self, nft_contract_address: &Option<AccountId>, seller: &Option<AccountId>, id: &Id, last_index: &u128) -> Result<(), PSP34Error> {
            let index = self.id_to_index.get((nft_contract_address, seller, id)).ok_or(PSP34Error::TokenNotExists)?;

            if last_index != &index {
                let last_id = self
                    .index_to_id
                    .get((nft_contract_address, seller, last_index))
                    .ok_or(PSP34Error::TokenNotExists)?;
                self.index_to_id.insert((nft_contract_address, seller, &index), &last_id);
                self.id_to_index.insert((nft_contract_address, seller, &last_id), &index);
            }

            self.index_to_id.remove((nft_contract_address, seller, &last_index));
            self.id_to_index.remove((nft_contract_address, seller, id));

            Ok(())
        }

        pub fn get_by_index(&self, nft_contract_address: &Option<AccountId>, seller: &Option<AccountId>, index: &u128) -> Result<Id, PSP34Error> {
            self.index_to_id.get((nft_contract_address, seller, index)).ok_or(PSP34Error::TokenNotExists)
        }
        pub fn get_by_id(&self, nft_contract_address: &Option<AccountId>, seller: &Option<AccountId>, id: &Id) -> Result<u128, PSP34Error> {
            self.id_to_index.get((nft_contract_address, seller, id)).ok_or(PSP34Error::TokenNotExists)
        }
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
        //(NFT Contract Address, Seller Address) EnumerableMapping--> list of all token ids
        sale_tokens_ids: EnumerableMapping,
        sale_tokens_ids_last_index: Mapping<(Option<AccountId>,Option<AccountId>), u128>,
        //(NFT Contract Address, Seller Address, token ID)
        bidders: Mapping<(AccountId,AccountId,Id),Vec<BidInformation>>,

        //(Collection Contract Address)
        // Number Listed Token
        listed_token_number_by_collection_address: Mapping<AccountId, u64>,

        //Platform Statistics
        total_volume: Balance,
        volume_by_collection: Mapping<AccountId, Balance>,
        volume_by_user: Mapping<AccountId, Balance>,

        total_profit: Balance,
        current_profit: Balance,

        //Platform discount
        staking_discount_criteria: Vec<u8>,
        staking_discount_rate: Vec<u16>,
    }

    impl Ownable for ArtZeroMarketplacePSP34 {}

    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    #[brush::wrapper]
    pub type CollectionRef = dyn CrossArtZeroCollection;
    #[brush::wrapper]
    pub type StakingRef = dyn CrossArtZeroStaking;

    #[brush::trait_definition]
    pub trait CrossArtZeroStaking {
        #[ink(message)]
        fn get_total_staked_by_account(&self,account: AccountId) -> u32;
    }
    #[brush::trait_definition]
    pub trait CrossArtZeroCollection {
        #[ink(message)]
        fn get_royal_fee(&self,nft_contract_address: AccountId) -> u32;
        #[ink(message)]
        fn is_active(&self,nft_contract_address: AccountId) -> bool;
        #[ink(message)]
        fn get_contract_type(&self,nft_contract_address: AccountId) -> u8;
        #[ink(message)]
        fn get_collection_owner(&self,nft_contract_address: AccountId) -> Option<AccountId>;
    }
    #[ink(event)]
    pub struct NewListEvent {
        trader: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance
    }
    #[ink(event)]
    pub struct UnListEvent {
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
    #[ink(event)]
    pub struct BidWinEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        platform_fee: Balance,
        royal_fee: Balance
    }

    #[ink(event)]
    pub struct BidEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        bid_value: Balance
    }

    #[ink(event)]
    pub struct RemoveBidEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        bid_value: Balance
    }

    impl ArtZeroMarketplacePSP34 {
        #[ink(constructor)]
        pub fn new(
            contract_owner: AccountId,
            collection_contract_address: AccountId,
            staking_contract_address: AccountId,
            platform_fee: u32
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                assert!(platform_fee<10000);        //must less than 100%
                instance._init_with_owner(contract_owner);
                instance.initialize(collection_contract_address, staking_contract_address, platform_fee);
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            collection_contract_address: AccountId,
            staking_contract_address: AccountId,
            platform_fee: u32
        ) -> Result<(), OwnableError> {
            self.collection_contract_address = collection_contract_address;
            self.staking_contract_address = staking_contract_address;
            self.platform_fee = platform_fee;
            let mut criteria = Vec::<u8>::new();
            criteria.push(20);
            criteria.push(9);
            criteria.push(7);
            criteria.push(5);
            criteria.push(1);
            let mut rate = Vec::<u16>::new();
            rate.push(9000);
            rate.push(8000);
            rate.push(6600);
            rate.push(5000);
            rate.push(3000);
            self.staking_discount_criteria = criteria;
            self.staking_discount_rate = rate;
            Ok(())
        }

        /* MARKETPLACE FUNCTIONS */
        /// List the token on the marketplace - FREE
        #[ink(message)]
        pub fn list(&mut self, nft_contract_address: AccountId, token_id: Id, price: Balance) -> Result<(),Error>{

            assert!(price>0);
            //Step 1: Check if the caller is the owner of the token
            let caller = self.env().caller();

            let token_owner = Psp34Ref::owner_of(&nft_contract_address,token_id.clone()).unwrap();
            assert!(caller == token_owner);

            //Step 2 - Check if this contract has been approved
            let allowance = Psp34Ref::allowance(&nft_contract_address,caller, self.env().account_id(), Some(token_id.clone()));
            assert!(allowance);

            let is_active = CollectionRef::is_active(&self.collection_contract_address,nft_contract_address);
            assert!(is_active);

            {
                let mut last_index = 0;
                if self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).is_some() {
                    last_index = self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).unwrap();
                }

                self.sale_tokens_ids.insert(&Some(nft_contract_address),&Some(caller),&token_id,&(last_index+1));
                self.sale_tokens_ids_last_index.insert((Some(nft_contract_address),Some(caller)),&(last_index+1));

                let new_sale = ForSaleItem {
                    nft_owner: token_owner,
                    listed_date: self.env().block_timestamp(),
                    price: price,
                    is_for_sale: true
                };
                //Update listed token
                self.market_list.insert(&(nft_contract_address,token_id.clone()), &new_sale);
                self.update_listed_token_by_collection_address(nft_contract_address, true);
            }

            //Step 3 - Transfer Token from Caller to Marketplace Contract
            assert!(PSP34Ref::transfer_builder(&nft_contract_address, self.env().account_id(), token_id.clone(), Vec::<u8>::new())
            .call_flags(CallFlags::default().set_allow_reentry(true))
            .fire().is_ok() );

            self.env().emit_event(NewListEvent {
                trader:Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id:token_id.clone(),
                price
            });

            Ok(())
        }
        /// Unlist the token from the marketplace - FREE
        #[ink(message)]
        pub fn unlist(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(),Error>{

            assert!(self.market_list.get(&(nft_contract_address,token_id.clone())).is_some());
            let mut sale_information = self.market_list.get(&(nft_contract_address,token_id.clone())).unwrap();
            let caller = self.env().caller();
            //General checking to ensure its from valid owner and sale is active

            assert!(sale_information.nft_owner == caller);
            assert!(sale_information.is_for_sale);

            //remove from market list
            sale_information.is_for_sale = false;
            self.market_list.insert(&(nft_contract_address,token_id.clone()), &sale_information);

            //Step 1: Check if the token is for sale
            assert!(self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).is_some());

            let mut last_index = self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).unwrap();

            assert!(self.sale_tokens_ids.remove(&Some(nft_contract_address),&Some(caller),&token_id.clone(),&last_index).is_ok());
            last_index = last_index.checked_sub(1).unwrap();

            self.sale_tokens_ids_last_index.insert((Some(nft_contract_address),Some(caller)),&last_index);

            //Clear Bidders
            let bidders = Vec::<BidInformation>::new();
            self.bidders.insert(&(nft_contract_address,caller,token_id.clone()), &bidders);

            //Send NFT back to caller
            assert!(Psp34Ref::transfer(&nft_contract_address,caller,token_id.clone(),Vec::<u8>::new()).is_ok());
            self.update_listed_token_by_collection_address(nft_contract_address, false);
            self.env().emit_event(UnListEvent {
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
            assert!(self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(seller))).is_some());

            let mut last_index = self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(seller))).unwrap();

            assert!(self.sale_tokens_ids.remove(&Some(nft_contract_address),&Some(seller),&token_id.clone(),&last_index).is_ok());
            last_index = last_index.checked_sub(1).unwrap();

            self.sale_tokens_ids_last_index.insert((Some(nft_contract_address),Some(seller)),&last_index);

            //Clear Bidders
            //Return all bidders AZERO
            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();

                let bidders_length = bidders.len();
                for index in 0..bidders_length {
                    //SEnd AZero back to lost bidder
                    assert!(self.env().transfer(bidders[index].bidder, bidders[index].bid_value).is_ok());
                }
                let clear_bidders = Vec::<BidInformation>::new();
                self.bidders.insert(&(nft_contract_address,seller,token_id.clone()), &clear_bidders);
            }

            //Calculate fee
            let platform_fee = price.checked_mul(self.platform_fee as u128).unwrap().checked_div(10000).unwrap();
            //Fee after Staking discount
            let platform_fee_after_discount = self.apply_discount(
                caller,
                platform_fee
            );
            //Save profit
            self.total_profit = self.total_profit.checked_add(platform_fee_after_discount).unwrap();
            self.current_profit = self.current_profit.checked_add(platform_fee_after_discount).unwrap();

            let royal_fee_rate = CollectionRef::get_royal_fee(&self.collection_contract_address,nft_contract_address);
            let royal_fee = price.checked_mul(royal_fee_rate as u128).unwrap().checked_div(10000).unwrap();

            //Send AZERO to collection owner
            let collection_owner = CollectionRef::get_collection_owner(&self.collection_contract_address,nft_contract_address);
            assert!(collection_owner != None);
            if royal_fee > 0{
                assert!(self.env().transfer(collection_owner.unwrap(), royal_fee).is_ok());
            }
            //Send AZERO to seller
            let seller_fee = price.checked_sub(royal_fee).unwrap().checked_sub(platform_fee_after_discount).unwrap();
            //seller_fee = self.apply_discount(caller, );
            if seller_fee  > 0{
                assert!(self.env().transfer(seller, seller_fee).is_ok());
            }
            let collection_volume = self.volume_by_collection.get(&nft_contract_address);
            let user_volume = self.volume_by_user.get(&seller);
            let mut user_volume_unwrap = 0;
            let mut collection_volume_unwarp = 0;

            if collection_volume.is_some() {
                collection_volume_unwarp = collection_volume.unwrap();
            }
            if user_volume.is_some() {
                user_volume_unwrap = user_volume.unwrap();
            }
            //Send AZERO cashback to buyer
            if platform_fee > platform_fee_after_discount {
                assert!(self.env().transfer(caller, platform_fee.checked_sub(platform_fee_after_discount).unwrap()).is_ok());
                let volume = price.checked_sub(platform_fee.checked_sub(platform_fee_after_discount).unwrap()).unwrap();
                self.total_volume = self.total_volume.checked_add(volume).unwrap();
                collection_volume_unwarp = collection_volume_unwarp.checked_add(volume).unwrap();
                user_volume_unwrap = user_volume_unwrap.checked_add(volume).unwrap();
            }
            else{
                self.total_volume = self.total_volume.checked_add(price).unwrap();
                collection_volume_unwarp = collection_volume_unwarp.checked_add(price).unwrap();
                user_volume_unwrap = user_volume_unwrap.checked_add(price).unwrap();
            }
            self.volume_by_collection.insert(&nft_contract_address,&collection_volume_unwarp);
            self.volume_by_user.insert(&seller,&user_volume_unwrap);

            self.update_listed_token_by_collection_address(nft_contract_address, false);

            //Send NFT to Buyer
            assert!(Psp34Ref::transfer(&nft_contract_address,caller,token_id.clone(),Vec::<u8>::new()).is_ok());

            self.env().emit_event(PurchaseEvent {
               buyer: Some(caller),
               seller: Some(seller),
               nft_contract_address: Some(nft_contract_address),
               token_id: token_id.clone(),
               price,
               platform_fee: platform_fee_after_discount,
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
            assert!(self.sale_tokens_ids.get_by_id(&Some(nft_contract_address),&Some(seller),&token_id).is_ok());

            let new_bid = BidInformation {
                bidder: caller,
                bid_date: self.env().block_timestamp(),
                bid_value: bid_value
            };

            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let mut bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();
                //Check if Bid already in the list --> not allow, have to remove bid and add new bid
                let length = bidders.len();
                //TODO: make 30 variable
                assert!(length<=30); //Only allow max 30 bids per token sale

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

            self.env().emit_event(BidEvent {
                bidder: Some(caller),
                seller: Some(seller),
                nft_contract_address: Some(nft_contract_address),
                token_id: token_id.clone(),
                price,
                bid_value
            });

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
            assert!(self.sale_tokens_ids.get_by_id(&Some(nft_contract_address),&Some(seller),&token_id).is_ok());

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
                assert!(self.env().transfer(caller, bid_value).is_ok());
                
                self.env().emit_event(RemoveBidEvent {
                    bidder: Some(caller),
                    seller: Some(seller),
                    nft_contract_address: Some(nft_contract_address),
                    token_id: token_id.clone(),
                    bid_value: bid_value
                });
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
            assert!(self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).is_some());

            let mut last_index = self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(caller))).unwrap();

            assert!(self.sale_tokens_ids.remove(&Some(nft_contract_address),&Some(caller),&token_id.clone(),&last_index).is_ok());
            last_index = last_index.checked_sub(1).unwrap();

            self.sale_tokens_ids_last_index.insert((Some(nft_contract_address),Some(caller)),&last_index);

            //Check Bidder and Clear
            if self.bidders.get(&(nft_contract_address,seller,token_id.clone())).is_some(){
                let bidders:Vec<BidInformation> = self.bidders.get(&(nft_contract_address,seller,token_id.clone())).unwrap();

                let bidders_length = bidders.len();
                assert!(bid_index < bidders_length as u32);
                for index in 0..bidders_length {
                    if index == bid_index as usize {
                        //Send NFT to bidder
                        assert!(Psp34Ref::transfer(&nft_contract_address,bidders[index].bidder,token_id.clone(),Vec::<u8>::new()).is_ok());

                        let price = bidders[index].bid_value;
                        //Calculate fee
                        let platform_fee = price.checked_mul(self.platform_fee as u128).unwrap().checked_div(10000).unwrap();
                        //Fee after Staking discount
                        let platform_fee_after_discount = self.apply_discount(
                            caller,
                            platform_fee
                        );

                        //Save profit
                        self.total_profit = self.total_profit.checked_add(platform_fee_after_discount).unwrap();
                        self.current_profit = self.current_profit.checked_add(platform_fee_after_discount).unwrap();

                        let royal_fee_rate = CollectionRef::get_royal_fee(&self.collection_contract_address,nft_contract_address);
                        let royal_fee = price.checked_mul(royal_fee_rate as u128).unwrap().checked_div(10000).unwrap();

                        //Send AZERO to collection owner
                        let collection_owner = CollectionRef::get_collection_owner(&self.collection_contract_address,nft_contract_address);
                        assert!(collection_owner != None);

                        if royal_fee > 0{
                            assert!(self.env().transfer(collection_owner.unwrap(), royal_fee).is_ok());
                        }
                        //Send AZERO to seller
                        let seller_fee = price.checked_sub(royal_fee).unwrap().checked_sub(platform_fee_after_discount).unwrap();
                        //seller_fee = self.apply_discount(caller, );
                        if seller_fee  > 0{
                            assert!(self.env().transfer(seller, seller_fee).is_ok());
                        }
                        let collection_volume = self.volume_by_collection.get(&nft_contract_address);
                        let user_volume = self.volume_by_user.get(&seller);
                        let mut user_volume_unwrap = 0;
                        let mut collection_volume_unwarp = 0;

                        if collection_volume.is_some() {
                            collection_volume_unwarp = collection_volume.unwrap();
                        }
                        if user_volume.is_some() {
                            user_volume_unwrap = user_volume.unwrap();
                        }
                        //Send AZERO cashback to buyer
                        if platform_fee > platform_fee_after_discount {
                            assert!(self.env().transfer(caller, platform_fee.checked_sub(platform_fee_after_discount).unwrap()).is_ok());
                            let volume = price.checked_sub(platform_fee.checked_sub(platform_fee_after_discount).unwrap()).unwrap();
                            self.total_volume = self.total_volume.checked_add(volume).unwrap();
                            collection_volume_unwarp = collection_volume_unwarp.checked_add(volume).unwrap();
                            user_volume_unwrap = user_volume_unwrap.checked_add(volume).unwrap();
                        }
                        else{
                            self.total_volume = self.total_volume.checked_add(price).unwrap();
                            collection_volume_unwarp = collection_volume_unwarp.checked_add(price).unwrap();
                            user_volume_unwrap = user_volume_unwrap.checked_add(price).unwrap();
                        }
                        self.volume_by_collection.insert(&nft_contract_address,&collection_volume_unwarp);
                        self.volume_by_user.insert(&seller,&user_volume_unwrap);

                        self.update_listed_token_by_collection_address(nft_contract_address, false);

                        self.env().emit_event(BidWinEvent {
                           bidder: Some(bidders[index].bidder),
                           seller: Some(seller),
                           nft_contract_address: Some(nft_contract_address),
                           token_id: token_id.clone(),
                           price,
                           platform_fee: platform_fee_after_discount,
                           royal_fee
                       });

                    }
                    else{
                        //SEnd AZero back to lost bidder
                        assert!(self.env().transfer(bidders[index].bidder, bidders[index].bid_value).is_ok());
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

        //Set listed token
        pub fn update_listed_token_by_collection_address(&mut self, nft_contract_address: AccountId, mode: bool) {
            let listed_token_count = self.listed_token_number_by_collection_address.get(&nft_contract_address);
            let mut listed_token_count_unwarp = 0;
            if listed_token_count.is_some() {
                listed_token_count_unwarp = listed_token_count.unwrap();
                if mode == true {
                    listed_token_count_unwarp = listed_token_count_unwarp.checked_add(1).unwrap();
                } else {
                    listed_token_count_unwarp = listed_token_count_unwarp.checked_sub(1).unwrap();
                }
                self.listed_token_number_by_collection_address.insert(&nft_contract_address, &listed_token_count_unwarp);
            } else {
                if mode == true {
                    listed_token_count_unwarp = listed_token_count_unwarp.checked_add(1).unwrap();
                    self.listed_token_number_by_collection_address.insert(&nft_contract_address, &listed_token_count_unwarp);
                }
            }
        }

        ///Set new staking contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_staking_contract_address(&mut self, staking_contract_address: AccountId) -> Result<(),Error> {
            self.staking_contract_address = staking_contract_address;
            Ok(())
        }
        ///Set criteria and discount rate - Only Owner 2 vectors same size
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_discount(&mut self, criteria:Vec<u8>,rates:Vec<u16>) -> Result<(),Error> {
            assert!(criteria.len() == rates.len());
            let length = criteria.len();
            for index in 0..length {
                assert!(rates[index] <= 10000);
            }
            self.staking_discount_criteria = criteria;
            self.staking_discount_rate = rates;
            Ok(())
        }

        ///Transfer NFT token - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn tranfer_nft(&mut self, nft_contract_address: AccountId, token_id: Id, receiver: AccountId) -> Result<(),Error> {
            assert!(Psp34Ref::transfer(&nft_contract_address,receiver,token_id.clone(),Vec::<u8>::new()).is_ok());
            Ok(())
        }

        // GETTERS
        /// Get market list information using NFT Collection and token ID
        #[ink(message)]
        pub fn get_nft_sale_info(&self,nft_contract_address:AccountId, token_id: Id) -> Option<ForSaleItem> {
            self.market_list.get(&(nft_contract_address,token_id))
        }

        /// Get platform fee
        #[ink(message)]
        pub fn get_platform_fee(&self) -> u32 {
            self.platform_fee
        }

        /// Get Staking Discount Criteria
        #[ink(message)]
        pub fn get_staking_discount_criteria(&self) -> Vec<u8> {
            self.staking_discount_criteria.clone()
        }

        /// Get Staking Discount Rates
        #[ink(message)]
        pub fn get_staking_discount_rate(&self) -> Vec<u16> {
            self.staking_discount_rate.clone()
        }

        /// Get listed token count by collection address
        #[ink(message)]
        pub fn get_listed_token_count_by_collection_address(&self, collection_contract_address: AccountId) -> u64 {
            let listed_token_number = self.listed_token_number_by_collection_address.get(&collection_contract_address);
            if listed_token_number.is_some(){
                return listed_token_number.unwrap();
            }
            return 0;
        }

        ///Get all token ids currently for sale for a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn get_for_sale_token_id(&self,nft_contract_address:AccountId, user_account:AccountId, index: u128) -> Id {
            self.sale_tokens_ids.get_by_index(&Some(nft_contract_address),&Some(user_account),&index).unwrap()
        }
        ///Get all token ids currently for sale by a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn total_tokens_for_sale(&self,nft_contract_address:AccountId, user_account:AccountId) -> u128 {
            let last_index = self.sale_tokens_ids_last_index.get((Some(nft_contract_address),Some(user_account)));
            if last_index.is_some(){
                return last_index.unwrap();
            }
            return 0;
        }
        ///Get all bids from (NFT Contract Address, User Address, token ID)
        #[ink(message)]
        pub fn get_all_bids(&self,nft_contract_address:AccountId, user_account:AccountId, token_id: Id) -> Option<Vec<BidInformation>> {
            self.bidders.get(&(nft_contract_address,user_account, token_id))
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

        /// Get total platform volume
        #[ink(message)]
        pub fn get_total_volume(&self) -> Balance {
            self.total_volume
        }
        /// Get total Collection volume
        #[ink(message)]
        pub fn get_volume_by_collection(&self, collection_contract_address: AccountId) -> Balance {
            let volume = self.volume_by_collection.get(&collection_contract_address);
            if volume.is_some(){
                return volume.unwrap();
            }
            return 0;
        }

        ///Get platform total Profit
        #[ink(message)]
        pub fn get_total_profit(&self) -> Balance {
            self.total_profit
        }
        ///Get platform current available profit
        #[ink(message)]
        pub fn get_current_profit(&self) -> Balance {
            self.current_profit
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            if value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            assert!(self.env().transfer(self.env().caller(), value).is_ok());
            Ok(())
        }

        /// Withdraw Profit - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_profit(&mut self,value: Balance, reciever: AccountId)  -> Result<(), Error> {
            if value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            if value > self.current_profit {
                return Err(Error::NotEnoughBalance);
            }
            self.current_profit = self.current_profit.checked_sub(value).unwrap();

            assert!(self.env().transfer(reciever, value).is_ok());
            Ok(())
        }

        fn apply_discount(&self, staker: AccountId, input_fee: Balance) -> Balance {
            let staked_amount = StakingRef::get_total_staked_by_account(&self.staking_contract_address,staker);

            let length = self.staking_discount_rate.len();

            for index in 0..length {
                if staked_amount >= self.staking_discount_criteria[index] as u32 {
                    return (input_fee * (10000 - self.staking_discount_rate[index] as u128))/10000;
                }
            }
            return input_fee;

        }

    }
}
