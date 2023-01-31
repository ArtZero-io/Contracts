#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#![allow(clippy::let_unit_value)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod artzero_marketplace_psp34 {
    use ink::env::CallFlags;
    use ink::prelude::{
        vec,
        vec::Vec,
        string::{
            String,
        },
    };
    use openbrush::{
        contracts::{
            ownable::*,
            traits::psp34::{
                *,
            },
        },
        traits::{
            Storage,
            DefaultEnv,
            ZERO_ADDRESS
        },
        storage::{
            Mapping,
            TypeGuard,
            MultiMapping,
            ValueGuard
        },
        modifiers,
    };
    use artzero_project::{
        traits::{
            staking::ArtZeroStakingRef,
            collection_manager::{
                ArtZeroCollectionRef,
                CollectionType,
            },
            psp34_standard::*,
            admin::*,
            upgradable::*,
            error::Error,
        }
    };

    #[cfg(feature = "std")]
    use ink::storage::traits::StorageLayout;

    #[derive(
        Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub struct ForSaleItem {
        nft_owner: AccountId,
        listed_date: u64,
        price: Balance,
        is_for_sale: bool,
        royalty_fee_at_listing: u32
    }

    #[derive(
        Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct BidInformation {
        bidder: AccountId,
        bid_date: u64,
        bid_value: Balance,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

    #[derive(Default)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    struct Manager {
        collection_contract_address: AccountId,
        staking_contract_address: AccountId,
        platform_fee: u32,                                  // 1% = 100
        market_list: Mapping<(AccountId, Id), ForSaleItem, MarketListKeys>, /* NFT for sale in the marketplace, (NFT Contract Address, Token ID) */
        sale_tokens_ids: MultiMapping<(Option<AccountId>, Option<AccountId>), Id, SaleTokensIdsKey>,                 //(NFT Contract Address, Seller Address)
        sale_tokens_ids_last_index: Mapping<(Option<AccountId>, Option<AccountId>), u128, SaleTokensIdsLastIndexKeys>,
        hold_amount_bidders: Mapping<AccountId, Balance>,
        hold_bidders: MultiMapping<u8, AccountId, ValueGuard<u8>>,
        bidders: Mapping<(AccountId, AccountId, Id), Vec<BidInformation>, BiddersKeys>, /* Contract Address, Seller Address, token ID) */
        listed_token_number_by_collection_address: Mapping<AccountId, u64>, /* Number Listed Token (Collection Contract Address) */
        total_volume: Balance,
        volume_by_collection: Mapping<AccountId, Balance>,
        volume_by_user: Mapping<AccountId, Balance>,
        total_profit: Balance,
        current_profit: Balance,
        staking_discount_criteria: Vec<u8>,
        staking_discount_rate: Vec<u16>,
        _reserved: Option<()>,
    }

    pub struct MarketListKeys;

    impl<'a> TypeGuard<'a> for MarketListKeys {
        type Type = &'a (&'a AccountId, &'a Id);
    }

    pub struct SaleTokensIdsKey;

    impl<'a> TypeGuard<'a> for SaleTokensIdsKey {
        type Type = &'a (&'a Option<&'a AccountId>, &'a Option<&'a AccountId>);
    }

    pub struct SaleTokensIdsLastIndexKeys;

    impl<'a> TypeGuard<'a> for SaleTokensIdsLastIndexKeys {
        type Type = &'a (&'a Option<&'a AccountId>, &'a Option<&'a AccountId>);
    }

    pub struct BiddersKeys;

    impl<'a> TypeGuard<'a> for BiddersKeys {
        type Type = &'a (&'a AccountId, &'a AccountId, &'a Id);
    }

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroMarketplacePSP34 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
        manager: Manager,
    }

    impl Ownable for ArtZeroMarketplacePSP34 {}
    impl AdminTrait for ArtZeroMarketplacePSP34 {}
    impl UpgradableTrait for ArtZeroMarketplacePSP34 {}

    #[ink(event)]
    pub struct NewListEvent {
        trader: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
    }
    #[ink(event)]
    pub struct UnListEvent {
        trader: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
    }
    #[ink(event)]
    pub struct PurchaseEvent {
        buyer: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        platform_fee: Balance,
        royalty_fee: Balance,
    }
    #[ink(event)]
    pub struct BidWinEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        platform_fee: Balance,
        royalty_fee: Balance,
    }

    #[ink(event)]
    pub struct BidEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        price: Balance,
        bid_value: Balance,
    }

    #[ink(event)]
    pub struct RemoveBidEvent {
        bidder: Option<AccountId>,
        seller: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        token_id: Id,
        bid_value: Balance,
    }

    impl ArtZeroMarketplacePSP34 {
        #[ink(constructor)]
        pub fn new(
            collection_contract_address: AccountId,
            staking_contract_address: AccountId,
            platform_fee: u32,
        ) -> Self {
            assert!(platform_fee < 10000);
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_owner(caller);
            instance
                .initialize(collection_contract_address, staking_contract_address, platform_fee)
                .ok()
                .unwrap();
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            collection_contract_address: AccountId,
            staking_contract_address: AccountId,
            platform_fee: u32,
        ) -> Result<(), Error> {
            // Make sure the initial data can only be init once
            if self.manager.collection_contract_address != ZERO_ADDRESS.into(){
                return Err(Error::AlreadyInit);
            }
            self.manager.collection_contract_address = collection_contract_address;
            self.manager.staking_contract_address = staking_contract_address;
            self.manager.platform_fee = platform_fee;
            
            // Set default discount rate for PMP Stakers.
            // >= 1 NFT - 30% off
            // >= 5 NFTs - 50% off
            // >= 7 NFTs - 66% off
            // >= 9 NFTs - 80% off
            // >= 20 NFTs - 90% off
            let criteria = vec![20, 9, 7, 5, 1];
            let rate = vec![9000, 8000, 6600, 5000, 3000];
            self.manager.staking_discount_criteria = criteria;
            self.manager.staking_discount_rate = rate;
            Ok(())
        }

        // MARKETPLACE FUNCTIONS
        /// List the NFT onto the marketplace - FREE of charge
        #[ink(message)]
        pub fn list(&mut self, nft_contract_address: AccountId, token_id: Id, price: Balance) -> Result<(), Error> {
            if price == 0 {
                return Err(Error::InvalidInput)
            }
            // Check if the caller is the owner of the token
            let caller = self.env().caller();
            let token_owner = Psp34Ref::owner_of(&nft_contract_address, token_id.clone()).unwrap();
            if caller != token_owner{
                return Err(Error::NotTokenOwner)
            }
            // Check if this contract has been approved to be able to transfer the NFT on owner behalf
            let allowance = Psp34Ref::allowance(
                &nft_contract_address,
                caller,
                self.env().account_id(),
                Some(token_id.clone()),
            );
            if !allowance {
                return Err(Error::NotApproved)
            }
            // Check if the collection is active
            let is_active = ArtZeroCollectionRef::is_active(&self.manager.collection_contract_address, nft_contract_address);
            if !is_active {
                return Err(Error::CollectionNotActive)
            }

            {
                // Add NFT token_id to sale_tokens_ids and increase the counting in sale_tokens_ids_last_index
                // This is to know how many and what NFTs are currently for sale by using (owner address and NFT Collection address)
                let mut last_index = 0;
                if self
                    .manager
                    .sale_tokens_ids_last_index
                    .get(&(&Some(&nft_contract_address), &Some(&caller)))
                    .is_some()
                {
                    last_index = self
                        .manager
                        .sale_tokens_ids_last_index
                        .get(&(&Some(&nft_contract_address), &Some(&caller)))
                        .unwrap();
                }
                self.manager.sale_tokens_ids.insert(
                    &(&Some(&nft_contract_address), &Some(&caller)),
                    &token_id
                );
                self.manager
                    .sale_tokens_ids_last_index
                    .insert(&(&Some(&nft_contract_address), &Some(&caller)), &(last_index + 1));
                // Get the Royalty Fee Rate at time of listing and save it the the Sale information
                // When the NFT is bought, this Royalty Fee will be used
                let royalty_fee_rate =
                    ArtZeroCollectionRef::get_royalty_fee(&self.manager.collection_contract_address, nft_contract_address);
                let new_sale = ForSaleItem {
                    nft_owner: token_owner,
                    listed_date: self.env().block_timestamp(),
                    price,
                    is_for_sale: true,
                    royalty_fee_at_listing: royalty_fee_rate
                };
                // Add the NFT Sale information to the market_list mapping
                self.manager
                    .market_list
                    .insert(&(&nft_contract_address, &token_id.clone()), &new_sale);
                self.update_listed_token_by_collection_address(&nft_contract_address, &true);
            }
            // Transfer Token from Caller to Marketplace Contract
            if Psp34Ref::transfer_builder(
                    &nft_contract_address,
                    self.env().account_id(),
                    token_id.clone(),
                    Vec::<u8>::new()
                )
                .call_flags(CallFlags::default().set_allow_reentry(true))
                .fire()
                .is_err(){
                    return Err(Error::CannotTransfer)
                }
            // Emit NewListEvent event for tracking purposes
            self.env().emit_event(NewListEvent {
                trader: Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id: token_id.clone(),
                price,
            });
            Ok(())
        }

        /// Unlist the NFT from the marketplace - FREE of charge
        #[ink(message)]
        pub fn unlist(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            // Check to make sure the NFT is for sale in the market
            if self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .is_none(){
                    return Err(Error::NotInMarket)
                }
            let mut sale_information = self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .unwrap();
            let caller = self.env().caller();
            // General checking to ensure its from valid owner and sale is active
            if sale_information.nft_owner != caller{
                return Err(Error::NotTokenOwner)
            }
            if !sale_information.is_for_sale {
                return Err(Error::NotForSale)
            }
            // remove the NFT from market list by set is_for_sale to false
            sale_information.is_for_sale = false;
            self.manager
                .market_list
                .insert(&(&nft_contract_address, &token_id), &sale_information);
            // Check if the token is for sale
            if self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&caller)))
                .is_none(){
                    return Err(Error::NotInSaleList)
                }
            let mut last_index = self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&caller)))
                .unwrap();
            if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone()){
                return Err(Error::NotInSaleList)
            }
            self.manager.sale_tokens_ids.remove_value(
                &(&Some(&nft_contract_address), &Some(&caller)),
                &token_id.clone()
            );
            // As the NFT is not on the market, the total count of for-sale NFT by (owner and collection) is reduced by 1
            last_index = last_index.checked_sub(1).unwrap();
            self.manager
                .sale_tokens_ids_last_index
                .insert(&(&Some(&nft_contract_address), &Some(&caller)), &last_index);
            // Clear Bidders
            if self
                .manager
                .bidders
                .get(&(&nft_contract_address, &caller, &token_id.clone()))
                .is_some()
            {
                let bidders: Vec<BidInformation> = self
                    .manager
                    .bidders
                    .get(&(&nft_contract_address, &caller, &token_id.clone()))
                    .unwrap();
                for item in bidders.iter() {
                    if self.manager.hold_amount_bidders.get(&item.bidder).is_some() {
                        let hold_amount_bidder = self.manager.hold_amount_bidders.get(&item.bidder).unwrap().checked_add(item.bid_value).unwrap();
                        self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder);
                    } else {
                        self.manager.hold_amount_bidders.insert(&item.bidder, &item.bid_value);
                        self.manager.hold_bidders.insert(1, &item.bidder);
                    }
                }
                
                let clear_bidders = Vec::<BidInformation>::new();
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &caller, &token_id.clone()), &clear_bidders);
            }
            // Send NFT back to caller or token_owner
            if Psp34Ref::transfer(&nft_contract_address, caller, token_id.clone(), Vec::<u8>::new()).is_err(){
                return Err(Error::CannotTransfer)
            }
            self.update_listed_token_by_collection_address(&nft_contract_address, &false);
            self.env().emit_event(UnListEvent {
                trader: Some(caller),
                nft_contract_address: Some(nft_contract_address),
                token_id: token_id.clone(),
            });
            Ok(())
        }

        /// Buy Token at listed price
        #[ink(message)]
        #[ink(payable)]
        pub fn buy(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .is_none(){
                    return Err(Error::NotInMarket)
                }
            let mut sale_information = self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .unwrap();
            let caller = self.env().caller();
            let seller = sale_information.nft_owner;
            let price = sale_information.price;
            // Seller cannot buy his own NFT
            if seller == caller {
                return Err(Error::InvalidCaller)
            }
            // Check if the NFT is for sale
            if !sale_information.is_for_sale{
                return Err(Error::NotForSale)
            }
            // Check if the buyer provide enough AZERO to buy the NFT
            if price != self.env().transferred_value(){
                return Err(Error::InvalidFee)
            }
            // Collection must be active so NFTs can be traded
            if !ArtZeroCollectionRef::is_active(
                &self.manager.collection_contract_address,
                nft_contract_address
            ){
                return Err(Error::CollectionNotActive)
            }
            // Check if the Collection Type is supported by this marketplace. Currently only PSP34
            let contract_type =
                ArtZeroCollectionRef::get_contract_type(&self.manager.collection_contract_address, nft_contract_address);
            if contract_type != CollectionType::Psp34Manual && contract_type != CollectionType::Psp34Auto{
                return Err(Error::InvalidType)
            }
            // Set NFT is not for sale
            sale_information.is_for_sale = false;
            self.manager
                .market_list
                .insert(&(&nft_contract_address, &token_id), &sale_information); // remove from market list
            // Check if the token is in the sale_tokens_ids list
            if self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&seller)))
                .is_none(){
                    return Err(Error::NotInSaleList)
                }
            let mut last_index = self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&seller)))
                .unwrap();
            if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&seller)), &token_id){
                return Err(Error::NotInSaleList)
            }
            // Remove the NFT from sale_tokens_ids list
            self.manager.sale_tokens_ids.remove_value(
                &(&Some(&nft_contract_address), &Some(&seller)),
                &token_id
            );
            last_index = last_index.checked_sub(1).unwrap();
            self.manager
                .sale_tokens_ids_last_index
                .insert(&(&Some(&nft_contract_address), &Some(&seller)), &last_index);
            // Clear Bidders
            // The NFT might have some bids and bidders should get back all the AZERO sent to the contract
            if self
                .manager
                .bidders
                .get(&(&nft_contract_address, &seller, &token_id.clone()))
                .is_some()
            {
                let bidders: Vec<BidInformation> = self
                    .manager
                    .bidders
                    .get(&(&nft_contract_address, &seller, &token_id.clone()))
                    .unwrap();
                for item in bidders.iter() {
                    if self.manager.hold_amount_bidders.get(&item.bidder).is_some() {
                        let hold_amount_bidder = self.manager.hold_amount_bidders.get(&item.bidder).unwrap().checked_add(item.bid_value).unwrap();
                        self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder);
                    } else {
                        self.manager.hold_amount_bidders.insert(&item.bidder, &item.bid_value);
                        self.manager.hold_bidders.insert(1, &item.bidder);
                    }
                }
                
                let clear_bidders = Vec::<BidInformation>::new();
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &seller, &token_id.clone()), &clear_bidders);
            }
            // Calculate fee that seller has to pay
            let platform_fee = price
                .checked_mul(self.manager.platform_fee as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            // Fee after Staking discount
            let platform_fee_after_discount = self.apply_discount(&seller, &platform_fee);
            // Save the sale amount into the contract for statistics purposes
            self.manager.total_profit = self
                .manager
                .total_profit
                .checked_add(platform_fee_after_discount)
                .unwrap();
            self.manager.current_profit = self
                .manager
                .current_profit
                .checked_add(platform_fee_after_discount)
                .unwrap();
            // Calculate the Royalty Fee the seller has to pay for Collection Owner
            let royalty_fee = price
                .checked_mul(sale_information.royalty_fee_at_listing as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            // Send AZERO to collection owner as Royalty Fee
            let collection_owner =
                ArtZeroCollectionRef::get_collection_owner(&self.manager.collection_contract_address, nft_contract_address);
            if collection_owner == None{
                return Err(Error::InvalidCollectionOwner)
            }
            if royalty_fee > 0 {
                if royalty_fee > self.env().balance() {
                    return Err(Error::NotEnoughBalance)
                }
                if self.env().transfer(collection_owner.unwrap(), royalty_fee).is_err() {
                    return Err(Error::CannotTransfer)
                }
            }
            // Send AZERO to seller after reduction of Royalty Fee and Platform Fee
            let seller_fee = price
                .checked_sub(royalty_fee)
                .unwrap()
                .checked_sub(platform_fee_after_discount)
                .unwrap();
                
            if seller_fee > 0 {
                if seller_fee > self.env().balance() {
                    return Err(Error::NotEnoughBalance)
                }
                if self.env().transfer(seller, seller_fee).is_err() {
                    return Err(Error::CannotTransfer)
                }
            }
            // Update Volumes for platform Statistics
            let collection_volume = self.manager.volume_by_collection.get(&nft_contract_address);
            let user_volume = self.manager.volume_by_user.get(&seller);
            let mut user_volume_unwrap = user_volume.unwrap_or(0);
            let mut collection_volume_unwarp = collection_volume.unwrap_or(0);
            self.manager.total_volume = self.manager.total_volume.checked_add(price).unwrap();
            collection_volume_unwarp = collection_volume_unwarp.checked_add(price).unwrap();
            user_volume_unwrap = user_volume_unwrap.checked_add(price).unwrap();
            self.manager
                .volume_by_collection
                .insert(&nft_contract_address, &collection_volume_unwarp);
            self.manager.volume_by_user.insert(&seller, &user_volume_unwrap);
            self.update_listed_token_by_collection_address(&nft_contract_address, &false);
            // Send NFT to Buyer
            if Psp34Ref::transfer(&nft_contract_address, caller, token_id.clone(), Vec::<u8>::new()).is_err(){
                return Err(Error::CannotTransfer)
            }
            // Emit PurchaseEvent to the network
            self.env().emit_event(PurchaseEvent {
                buyer: Some(caller),
                seller: Some(seller),
                nft_contract_address: Some(nft_contract_address),
                token_id: token_id.clone(),
                price,
                platform_fee: platform_fee_after_discount,
                royalty_fee,
            });
            Ok(())
        }
        /// Bid Token for sale, transferred_value() is the bidding price
        #[ink(message)]
        #[ink(payable)]
        pub fn bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .is_none(){
                    return Err(Error::NotInMarket)
                }
            let sale_information = self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .unwrap();
            let caller = self.env().caller(); // bidder
            let seller = sale_information.nft_owner;
            let price = sale_information.price;
            let bid_value = self.env().transferred_value();
            // General checking to ensure its from valid owner and sale is active
            // Seller cannot bid on his own NFT
            if seller == caller{
                return Err(Error::InvalidCaller)
            }
            // Make sure the NFT is for sale
            if !sale_information.is_for_sale{
                return Err(Error::NotForSale)
            }
            // Bidder can only bid the amount less than the selling price of the NFT
            if price <= bid_value{
                return Err(Error::InvalidFee)
            }
            // Collection must be active at time of bidding
            if !ArtZeroCollectionRef::is_active(
                &self.manager.collection_contract_address,
                nft_contract_address
            ){
                return Err(Error::CollectionNotActive)
            }
            // Check if the Collection Type is supported by this marketplace. Currently only PSP34
            let contract_type =
                ArtZeroCollectionRef::get_contract_type(&self.manager.collection_contract_address, nft_contract_address);
            if contract_type != CollectionType::Psp34Manual && contract_type != CollectionType::Psp34Auto{
                return Err(Error::InvalidType)
            }
            // Check if the NFT is in sale_tokens_ids list
            if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&seller)), &token_id){
                return Err(Error::NotInSaleList)
            }
            // Create new Bid Structure before saving to the mapping
            let new_bid = BidInformation {
                bidder: caller,
                bid_date: self.env().block_timestamp(),
                bid_value,
            };
            if self
                .manager
                .bidders
                .get(&(&nft_contract_address, &seller, &token_id))
                .is_some()
            {
                // Some bidders already in the list
                let mut bidders: Vec<BidInformation> = self
                    .manager
                    .bidders
                    .get(&(&nft_contract_address, &seller, &token_id))
                    .unwrap();
                // Check if Bid already in the list --> not allow, have to remove bid and add new bid
                let length = bidders.len();
                // TODO: make 30 variable
                if length > 30 { // Only allow max 30 bids per token sale
                    return Err(Error::InvalidBidLength)
                }
                for index in 0..length {
                    if bidders[index as usize].bidder == caller {
                        return Err(Error::BidAlreadyExist)
                    }
                }
                // Add bid to the list
                bidders.push(new_bid);
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &seller, &token_id.clone()), &bidders);
            } else {
                // There is no bid in the sale so insert the bid into this bidders mapping
                let bidders = vec![new_bid];
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &seller, &token_id), &bidders);
            }
            // Emit the BidEvent to the network
            self.env().emit_event(BidEvent {
                bidder: Some(caller),
                seller: Some(seller),
                nft_contract_address: Some(nft_contract_address),
                token_id: token_id.clone(),
                price,
                bid_value,
            });
            Ok(())
        }
        /// Remove Bid From Active Sale
        #[ink(message)]
        pub fn remove_bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .is_none(){
                    return Err(Error::NotInMarket)
                }
            let sale_information = self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .unwrap();
            let caller = self.env().caller(); // bidder
            let seller = sale_information.nft_owner;
            // General checking to ensure its from valid owner and sale is active
            // Seller cannot bid on his own sale
            if seller == caller {
                return Err(Error::InvalidCaller)
            }
            // The NFT must be for sale
            if !sale_information.is_for_sale{
                return Err(Error::NotForSale)
            }
            // The NFT Collection must be active
            if !ArtZeroCollectionRef::is_active(
                &self.manager.collection_contract_address,
                nft_contract_address
            ){
                return Err(Error::CollectionNotActive)
            }
            // Check if the Collection Type is supported by this marketplace. Currently only PSP34
            let contract_type =
            ArtZeroCollectionRef::get_contract_type(&self.manager.collection_contract_address, nft_contract_address);
            if contract_type != CollectionType::Psp34Manual && contract_type != CollectionType::Psp34Auto{
                return Err(Error::InvalidType)
            }
            // Check if the token is in sale_tokens_ids list
            if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&seller)), &token_id){
                return Err(Error::NotInSaleList)
            }

            if self
                .manager
                .bidders
                .get(&(&nft_contract_address, &seller, &token_id))
                .is_some()
            {
                let mut bidders: Vec<BidInformation> = self
                    .manager
                    .bidders
                    .get(&(&nft_contract_address, &seller, &token_id))
                    .unwrap();
                // Check if Bid for this caller already in the list
                let length = bidders.len();
                let mut index_bid: i32 = -1;
                let mut bid_value = 0;
                for index in 0..length {
                    if bidders[index as usize].bidder == caller {
                        index_bid = index as i32;
                        bid_value = bidders[index as usize].bid_value;
                        break
                    }
                }
                if index_bid < 0 {
                    return Err(Error::Custom(String::from("invalid index bid")))
                }
                if bid_value == 0 {
                    return Err(Error::Custom(String::from("invalid bid value")))
                }
                // remove bid from bidders list
                bidders.remove(index_bid as usize);
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &seller, &token_id), &bidders);
                if bid_value > self.env().balance() {
                    return Err(Error::NotEnoughBalance)
                }
                // Send bid_value back to caller
                if self.env().transfer(caller, bid_value).is_err(){
                    return Err(Error::CannotTransfer)
                }
                // Emit the RemoveBidEvent to the network
                self.env().emit_event(RemoveBidEvent {
                    bidder: Some(caller),
                    seller: Some(seller),
                    nft_contract_address: Some(nft_contract_address),
                    token_id,
                    bid_value,
                });
            } else {
                return Err(Error::BidNotExist)
            }
            Ok(())
        }
        /// Accept Bid
        #[ink(message)]
        pub fn accept_bid(
            &mut self,
            nft_contract_address: AccountId,
            token_id: Id,
            bid_index: u32,
        ) -> Result<(), Error> {
            if self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .is_none(){
                    return Err(Error::NotInMarket)
                }
            let mut sale_information = self
                .manager
                .market_list
                .get(&(&nft_contract_address, &token_id))
                .unwrap();
            let caller = self.env().caller(); // seller to accept bid from bidder
            let seller = sale_information.nft_owner;
            // General checking to ensure its from valid owner and sale is active
            // Only NFT seller can accept the bid
            if seller != caller{
                return Err(Error::InvalidCaller)
            }
            // The NFT must be for sale
            if !sale_information.is_for_sale{
                return Err(Error::NotForSale)
            }
            // The NFT Collection must be active
            if !ArtZeroCollectionRef::is_active(
                &self.manager.collection_contract_address,
                nft_contract_address
            ){
                return Err(Error::CollectionNotActive)
            }
            // Check if the Collection Type is supported by this marketplace. Currently only PSP34
            let contract_type =
            ArtZeroCollectionRef::get_contract_type(&self.manager.collection_contract_address, nft_contract_address);
            if contract_type != CollectionType::Psp34Manual && contract_type != CollectionType::Psp34Auto{
                return Err(Error::InvalidType)
            }
            // remove from market list
            sale_information.is_for_sale = false;
            self.manager
                .market_list
                .insert(&(&nft_contract_address, &token_id), &sale_information);
            // Check if the token is in sale_tokens_ids
            if self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&caller)))
                .is_none(){
                    return Err(Error::NotInSaleList)
                }
            let mut last_index = self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&caller)))
                .unwrap();
            if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone()){
                return Err(Error::NotInSaleList)
            }
            self.manager.sale_tokens_ids.remove_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone());
            last_index = last_index.checked_sub(1).unwrap();
            self.manager
                .sale_tokens_ids_last_index
                .insert(&(&Some(&nft_contract_address), &Some(&caller)), &last_index);
            // Check through all the bids for this NFT and return AZERO to lose bidders
            // then clear the bidders mapping
            if self
                .manager
                .bidders
                .get(&(&nft_contract_address, &seller, &token_id.clone()))
                .is_some()
            {
                let bidders: Vec<BidInformation> = self
                    .manager
                    .bidders
                    .get(&(&nft_contract_address, &seller, &token_id.clone()))
                    .unwrap();
                let bidders_length = bidders.len();
                if bid_index >= bidders_length as u32 {
                    return Err(Error::Custom(String::from("invalid index bid")))
                }
                // Loop to all the bids in bidders mapping
                for (index, item) in bidders.iter().enumerate() {
                    if index == bid_index as usize {
                        // Send NFT to bidder
                        if Psp34Ref::transfer(
                            &nft_contract_address,
                            item.bidder,
                            token_id.clone(),
                            Vec::<u8>::new()
                        )
                        .is_err(){
                            return Err(Error::CannotTransfer)
                        }
                        let price = item.bid_value;
                        // Calculate platform fee that seller has to pay
                        let platform_fee = price
                            .checked_mul(self.manager.platform_fee as u128)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        // Calculate Fee after Staking discount
                        let platform_fee_after_discount = self.apply_discount(&seller, &platform_fee);
                        // Save profit as platform statistics
                        self.manager.total_profit = self
                            .manager
                            .total_profit
                            .checked_add(platform_fee_after_discount)
                            .unwrap();
                        self.manager.current_profit = self
                            .manager
                            .current_profit
                            .checked_add(platform_fee_after_discount)
                            .unwrap();
                        // Calculate Royalty Fee that seller has to pay to Collection Owner
                        let royalty_fee = price
                            .checked_mul(sale_information.royalty_fee_at_listing as u128)
                            .unwrap()
                            .checked_div(10000)
                            .unwrap();
                        // Send AZERO to collection owner
                        let collection_owner = ArtZeroCollectionRef::get_collection_owner(
                            &self.manager.collection_contract_address,
                            nft_contract_address,
                        );
                        if collection_owner == None{
                            return Err(Error::InvalidCollectionOwner)
                        }
                        if royalty_fee > 0 {
                            if royalty_fee > self.env().balance() {
                                return Err(Error::NotEnoughBalance)
                            }
                            if self.env().transfer(collection_owner.unwrap(), royalty_fee).is_err() {
                                return Err(Error::CannotTransfer)
                            }
                        }
                        // Send AZERO to seller
                        let seller_fee = price
                            .checked_sub(royalty_fee)
                            .unwrap()
                            .checked_sub(platform_fee_after_discount)
                            .unwrap();
                        if seller_fee > 0 {
                            if seller_fee > self.env().balance() {
                                return Err(Error::NotEnoughBalance)
                            }
                            if self.env().transfer(seller, seller_fee).is_err(){
                                return Err(Error::CannotTransfer)
                            }
                        }
                        
                        // Save all volume as platform Statistics
                        let collection_volume = self.manager.volume_by_collection.get(&nft_contract_address);
                        let user_volume = self.manager.volume_by_user.get(&seller);
                        let mut user_volume_unwrap = user_volume.unwrap_or(0);
                        let mut collection_volume_unwarp = collection_volume.unwrap_or(0);
                        self.manager.total_volume = self.manager.total_volume.checked_add(price).unwrap();
                        collection_volume_unwarp = collection_volume_unwarp.checked_add(price).unwrap();
                        user_volume_unwrap = user_volume_unwrap.checked_add(price).unwrap();
                        self.manager
                            .volume_by_collection
                            .insert(&nft_contract_address, &collection_volume_unwarp);
                        self.manager.volume_by_user.insert(&seller, &user_volume_unwrap);
                        self.update_listed_token_by_collection_address(&nft_contract_address, &false);

                        // Emit BidWinEvent to the network
                        self.env().emit_event(BidWinEvent {
                            bidder: Some(item.bidder),
                            seller: Some(seller),
                            nft_contract_address: Some(nft_contract_address),
                            token_id: token_id.clone(),
                            price,
                            platform_fee: platform_fee_after_discount,
                            royalty_fee,
                        });
                    } else {
                        // Add bidder amount to hold_amount_bidders
                        if self.manager.hold_amount_bidders.get(&item.bidder).is_some() {
                            let hold_amount_bidder = self.manager.hold_amount_bidders.get(&item.bidder).unwrap().checked_add(item.bid_value).unwrap();
                            self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder);
                        } else {
                            self.manager.hold_amount_bidders.insert(&item.bidder, &item.bid_value);
                            self.manager.hold_bidders.insert(1, &item.bidder);
                        }
                    }
                }
                // Clear Bidders
                let bidders = Vec::<BidInformation>::new();
                self.manager
                    .bidders
                    .insert(&(&nft_contract_address, &caller, &token_id.clone()), &bidders);
            } else {
                return Err(Error::BidNotExist)
            }
            Ok(())
        }

        /// Set new collection contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_collection_contract_address(&mut self, collection_contract_address: AccountId) -> Result<(), Error> {
            self.manager.collection_contract_address = collection_contract_address;
            Ok(())
        }

        /// Set Platform fee - only owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_platform_fee(&mut self, platform_fee: u32) -> Result<(), Error> {
            if platform_fee >= 10000{ // must less than 100%
                return Err(Error::InvalidFee)
            }
            self.manager.platform_fee = platform_fee;
            Ok(())
        }

        // This function update the total count of NFT for sale by using NFT Collection address
        fn update_listed_token_by_collection_address(&mut self, nft_contract_address: &AccountId, mode: &bool) {
            let listed_token_count = self
                .manager
                .listed_token_number_by_collection_address
                .get(nft_contract_address);
            if let Some(mut listed_token_count_unwarp) = listed_token_count {
                if *mode {
                    listed_token_count_unwarp = listed_token_count_unwarp.checked_add(1).unwrap();
                } else {
                    listed_token_count_unwarp = listed_token_count_unwarp.checked_sub(1).unwrap();
                }
                self.manager
                    .listed_token_number_by_collection_address
                    .insert(nft_contract_address, &listed_token_count_unwarp);
            } else {
                let listed_token_count_unwarp = 1;
                if *mode {
                    self.manager
                        .listed_token_number_by_collection_address
                        .insert(nft_contract_address, &listed_token_count_unwarp);
                }
            }
        }

        /// Set new staking contract address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_staking_contract_address(&mut self, staking_contract_address: AccountId) -> Result<(), Error> {
            self.manager.staking_contract_address = staking_contract_address;
            Ok(())
        }

        /// Set criteria and discount rate - Only Owner 2 vectors same size
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_discount(&mut self, criteria: Vec<u8>, rates: Vec<u16>) -> Result<(), Error> {
            if criteria.len() != rates.len(){
                return Err(Error::InvalidInput)
            }
            for &item in rates.iter() {
                if item > 10000{
                    return Err(Error::InvalidInput)
                }
            }
            self.manager.staking_discount_criteria = criteria;
            self.manager.staking_discount_rate = rates;
            Ok(())
        }

        /// Receive hold amount
        #[ink(message)]
        pub fn receive_hold_amount(&mut self, receiver: AccountId) -> Result<(), Error> {
            if self.manager.hold_amount_bidders.get(&receiver).is_some() {
                let hold_amount = self.manager.hold_amount_bidders.get(&receiver).unwrap();
                if hold_amount > 0 {
                    if hold_amount > self.env().balance() {
                        return Err(Error::NotEnoughBalance);
                    }
                    assert!(self.env().transfer(receiver, hold_amount).is_ok());
                    self.manager.hold_amount_bidders.remove(&receiver);
                    self.manager.hold_bidders.remove_value(1, &receiver);
                }
            }
            Ok(())
        }

        // GETTERS
        /// Get market list information using NFT Collection and token ID
        #[ink(message)]
        pub fn get_nft_sale_info(&self, nft_contract_address: AccountId, token_id: Id) -> Option<ForSaleItem> {
            self.manager.market_list.get(&(&nft_contract_address, &token_id))
        }

        /// Get platform fee
        #[ink(message)]
        pub fn get_platform_fee(&self) -> u32 {
            self.manager.platform_fee
        }

        /// Get Staking Discount Criteria
        #[ink(message)]
        pub fn get_staking_discount_criteria(&self) -> Vec<u8> {
            self.manager.staking_discount_criteria.clone()
        }

        /// Get Staking Discount Rates
        #[ink(message)]
        pub fn get_staking_discount_rate(&self) -> Vec<u16> {
            self.manager.staking_discount_rate.clone()
        }

        /// Get listed token count by collection address
        #[ink(message)]
        pub fn get_listed_token_count_by_collection_address(&self, collection_contract_address: AccountId) -> u64 {
            self
                .manager
                .listed_token_number_by_collection_address
                .get(&collection_contract_address).unwrap_or(0)
        }

        /// Get all token ids currently for sale for a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn get_for_sale_token_id(
            &self,
            nft_contract_address: AccountId,
            user_account: AccountId,
            index: u128,
        ) -> Option<Id> {
            Some(self.manager
                .sale_tokens_ids
                .get_value(&(&Some(&nft_contract_address), &Some(&user_account)), &index)
                .unwrap())
        }

        /// Get get total sale token ids of user account in a contract
        #[ink(message)]
        pub fn get_sale_tokens_ids_count(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128 {
            self.manager.sale_tokens_ids.count(&(&Some(&nft_contract_address), &Some(&user_account)))

        }

        /// Get all token ids currently for sale by a collection (nft_contract_address,user_account)
        #[ink(message)]
        pub fn total_tokens_for_sale(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128 {
            self
                .manager
                .sale_tokens_ids_last_index
                .get(&(&Some(&nft_contract_address), &Some(&user_account))).unwrap_or(0)

        }

        /// Get all bids from (NFT Contract Address, User Address, token ID)
        #[ink(message)]
        pub fn get_all_bids(
            &self,
            nft_contract_address: AccountId,
            user_account: AccountId,
            token_id: Id,
        ) -> Option<Vec<BidInformation>> {
            self.manager
                .bidders
                .get(&(&nft_contract_address, &user_account, &token_id))
        }

        /// Get collection contract address
        #[ink(message)]
        pub fn get_collection_contract_address(&self) -> AccountId {
            self.manager.collection_contract_address
        }
        /// Get staking contract address
        #[ink(message)]
        pub fn get_staking_contract_address(&self) -> AccountId {
            self.manager.staking_contract_address
        }

        /// Get total platform volume
        #[ink(message)]
        pub fn get_total_volume(&self) -> Balance {
            self.manager.total_volume
        }
        /// Get total Collection volume
        #[ink(message)]
        pub fn get_volume_by_collection(&self, collection_contract_address: AccountId) -> Balance {
            self.manager.volume_by_collection.get(&collection_contract_address).unwrap_or(0)
        }

        /// Get platform total Profit
        #[ink(message)]
        pub fn get_total_profit(&self) -> Balance {
            self.manager.total_profit
        }

        /// Get platform current available profit
        #[ink(message)]
        pub fn get_current_profit(&self) -> Balance {
            self.manager.current_profit
        }

        /// Get hold amount of bidder
        #[ink(message)]
        pub fn get_hold_amount_of_bidder(&self, bidder: AccountId) -> Option<Balance> {
            self.manager.hold_amount_bidders.get(&bidder)
        }

        /// Get Hold Bidders by Index
        #[ink(message)]
        pub fn get_hold_bidders_by_index(&self, index: u64) -> Option<AccountId> {
            self.manager.hold_bidders.get_value(1, &(index as u128))
        }

        /// Get Hold Bidder Count
        #[ink(message)]
        pub fn get_hold_bidder_count(&self) -> u64 {
            self.manager.hold_bidders.count(1) as u64
        }

        /// Withdraw Profit - only Contract Owner.
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_profit(&mut self, value: Balance, reciever: AccountId) -> Result<(), Error> {
            if value > self.env().balance() || value > self.manager.current_profit {
                return Err(Error::NotEnoughBalance)
            }
            self.manager.current_profit = self.manager.current_profit.checked_sub(value).unwrap();
            if self.env().transfer(reciever, value).is_err(){
                return Err(Error::CannotTransfer)
            }
            Ok(())
        }
        // This function calculate the discount from the transaction fee based on number of PMP NFTs staked
        // By default the discounts are:
        // >= 1 NFT - 30% off
        // >= 5 NFTs - 50% off
        // >= 7 NFTs - 66% off
        // >= 9 NFTs - 80% off
        // >= 20 NFTs - 90% off
        fn apply_discount(&self, staker: &AccountId, input_fee: &Balance) -> Balance {
            let staked_amount = ArtZeroStakingRef::get_total_staked_by_account(&self.manager.staking_contract_address, *staker);
            let length = self.manager.staking_discount_rate.len();
            for index in 0..length {
                if staked_amount >= self.manager.staking_discount_criteria[index] as u64 {
                    return (*input_fee * (10000 - self.manager.staking_discount_rate[index] as u128)) / 10000
                }
            }
            *input_fee
        }
    }
}
