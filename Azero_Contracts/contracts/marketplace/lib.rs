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
                CollectionType
            },
            psp34_standard::*,
            admin::*,
            upgradable::*,
            error::Error
        }
    };
    use artzero_project::{
        impls::{
            marketplace::*,
        }
    };

    #[cfg(feature = "std")]
    use ink::storage::traits::StorageLayout;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroMarketplacePSP34 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
        #[storage_field]
        manager: artzero_project::impls::marketplace::data::Manager,
    }

    impl Ownable for ArtZeroMarketplacePSP34 {}
    impl AdminTrait for ArtZeroMarketplacePSP34 {}
    impl UpgradableTrait for ArtZeroMarketplacePSP34 {}
    impl ArtZeroMarketplaceTrait for ArtZeroMarketplacePSP34 {}
    
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
            if let Some(token_owner) = Psp34Ref::owner_of(&nft_contract_address, token_id.clone()) {
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
                    if let Some(last_index_tmp) = self.manager.sale_tokens_ids_last_index.get(&(&Some(&nft_contract_address), &Some(&caller))) {
                        last_index = last_index_tmp;
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
                let builder = Psp34Ref::transfer_builder(
                        &nft_contract_address,
                        self.env().account_id(),
                        token_id.clone(),
                        Vec::<u8>::new()
                    )
                    .call_flags(CallFlags::default().set_allow_reentry(true));
                let result = match builder.try_invoke() {
                    Ok(Ok(Ok(_))) => Ok(()),
                    Ok(Ok(Err(e))) => Err(e.into()),
                    Ok(Err(ink::LangError::CouldNotReadInput)) => Ok(()),
                    Err(ink::env::Error::NotCallable) => Ok(()),
                    _ => {
                        Err(Error::CannotTransfer)
                    }
                };
                if result.is_ok() {
                    // Emit NewListEvent event for tracking purposes
                    self.env().emit_event(NewListEvent {
                        trader: Some(caller),
                        nft_contract_address: Some(nft_contract_address),
                        token_id: token_id.clone(),
                        price,
                    });
                }
            
                Ok(())
            } else {
                return Err(Error::NotTokenOwner)
            }
        }

        /// Unlist the NFT from the marketplace - FREE of charge
        #[ink(message)]
        pub fn unlist(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if let Some(mut sale_information) = self.manager.market_list.get(&(&nft_contract_address, &token_id)) {
                // Check to make sure the NFT is for sale in the market
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
                if let Some(mut last_index) = self.manager.sale_tokens_ids_last_index.get(&(&Some(&nft_contract_address), &Some(&caller))) {
                    if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone()){
                        return Err(Error::NotInSaleList)
                    }
                    self.manager.sale_tokens_ids.remove_value(
                        &(&Some(&nft_contract_address), &Some(&caller)),
                        &token_id.clone()
                    );
                    
                    if let Some(last_index_tmp) = last_index.checked_sub(1) {
                        // As the NFT is not on the market, the total count of for-sale NFT by (owner and collection) is reduced by 1
                        last_index = last_index_tmp;
                        self.manager
                        .sale_tokens_ids_last_index
                        .insert(&(&Some(&nft_contract_address), &Some(&caller)), &last_index);
                        // Clear Bidders
                        if let Some(bidders) = self.manager.bidders.get(&(&nft_contract_address, &caller, &token_id.clone())) {
                            for item in bidders.iter() {
                                if let Some(mut hold_amount_bidder) = self.manager.hold_amount_bidders.get(&item.bidder) {
                                    if let Some(hold_amount_bidder_tmp) = hold_amount_bidder.checked_add(item.bid_value) {
                                        hold_amount_bidder = hold_amount_bidder_tmp;
                                        self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder);
                                    } else {
                                        return Err(Error::CheckedOperations)
                                    }
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
                    } else {
                        return Err(Error::CheckedOperations)
                    }
                } else {
                    return Err(Error::NotInSaleList)
                }
            } else {
                return Err(Error::NotInMarket)
            }
        }

        /// Buy Token at listed price
        #[ink(message)]
        #[ink(payable)]
        pub fn buy(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if let Some(mut sale_information) = self.manager.market_list.get(&(&nft_contract_address, &token_id)) {
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
                ) {
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
                if let Some(mut last_index) = self.manager.sale_tokens_ids_last_index.get(&(&Some(&nft_contract_address), &Some(&seller))) {
                    if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&seller)), &token_id){
                        return Err(Error::NotInSaleList)
                    }
                    // Remove the NFT from sale_tokens_ids list
                    self.manager.sale_tokens_ids.remove_value(
                        &(&Some(&nft_contract_address), &Some(&seller)),
                        &token_id
                    );
                    if let Some(last_index_tmp) = last_index.checked_sub(1) {
                        last_index = last_index_tmp;
                        self.manager
                            .sale_tokens_ids_last_index
                            .insert(&(&Some(&nft_contract_address), &Some(&seller)), &last_index);
                        // Clear Bidders
                        // The NFT might have some bids and bidders should get back all the AZERO sent to the contract
                        if let Some(bidders) = self.manager.bidders.get(&(&nft_contract_address, &seller, &token_id.clone())) {
                            for item in bidders.iter() {
                                if let Some(mut hold_amount_bidder) = self.manager.hold_amount_bidders.get(&item.bidder) {
                                    if let Some(hold_amount_bidder_tmp) = hold_amount_bidder.checked_add(item.bid_value){
                                        hold_amount_bidder = hold_amount_bidder_tmp;
                                        self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder);
                                    }
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
                        if let Some(price_mul_platform_fee_tmp) =  price.checked_mul(self.manager.platform_fee as u128) {
                            if let Some(platform_fee) = price_mul_platform_fee_tmp.checked_div(10000) {
                                // Fee after Staking discount
                                let platform_fee_after_discount = self.apply_discount(&seller, &platform_fee);
                                if let Some(total_profit_tmp) = self.manager.total_profit.checked_add(platform_fee_after_discount) {
                                    self.manager.total_profit = total_profit_tmp;
                                }
                                if let Some(current_profit_tmp) = self.manager.current_profit.checked_add(platform_fee_after_discount) {
                                    self.manager.current_profit = current_profit_tmp;
                                }
                                if let Some(price_mul_sale_royalty_fee_tmp) = price.checked_mul(sale_information.royalty_fee_at_listing as u128) {
                                    if let Some(royalty_fee) = price_mul_sale_royalty_fee_tmp.checked_div(10000) {
                                        if let Some(collection_owner) = ArtZeroCollectionRef::get_collection_owner(&self.manager.collection_contract_address, nft_contract_address) {
                                            if royalty_fee > 0 {
                                                if royalty_fee > self.env().balance() {
                                                    return Err(Error::NotEnoughBalance)
                                                }
                                                if self.env().transfer(collection_owner, royalty_fee).is_err() {
                                                    return Err(Error::CannotTransfer)
                                                }
                                            }
                                            if let Some(pice_mul_royalty_fee_tmp) = price.checked_sub(royalty_fee) {
                                                if let Some(seller_fee) = pice_mul_royalty_fee_tmp.checked_sub(platform_fee_after_discount) {
                                                    // Send AZERO to seller after reduction of Royalty Fee and Platform Fee
                                                    if seller_fee > 0 {
                                                        if seller_fee > self.env().balance() {
                                                            return Err(Error::NotEnoughBalance)
                                                        }
                                                        if self.env().transfer(seller, seller_fee).is_err() {
                                                            return Err(Error::CannotTransfer)
                                                        }
                                                    }
                                                    // Update Volumes for platform Statistics
                                                    let mut collection_volume = self.manager.volume_by_collection.get(&nft_contract_address).unwrap_or(0);
                                                    let mut user_volume = self.manager.volume_by_user.get(&seller).unwrap_or(0);
                                                    if let Some(total_volume_tmp) = self.manager.total_volume.checked_add(price) {
                                                        self.manager.total_volume = total_volume_tmp;
                                                        if let Some(collection_volume_tmp) = collection_volume.checked_add(price) {
                                                            if let Some(user_volume_tmp) = user_volume.checked_add(price) {
                                                                collection_volume = collection_volume_tmp;
                                                                user_volume = user_volume_tmp;
                                                                self.manager
                                                                    .volume_by_collection
                                                                    .insert(&nft_contract_address, &collection_volume);
                                                                self.manager.volume_by_user.insert(&seller, &user_volume);
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
                                                            } else {
                                                                return Err(Error::CheckedOperations)
                                                            }
                                                        } else {
                                                            return Err(Error::CheckedOperations)
                                                        }
                                                    } else {
                                                        return Err(Error::CheckedOperations)
                                                    }
                                                } else {
                                                    return Err(Error::CheckedOperations)
                                                }
                                            } else {
                                                return Err(Error::CheckedOperations)
                                            }
                                        } else {
                                            return Err(Error::InvalidCollectionOwner)
                                        }
                                    } else {
                                        return Err(Error::CheckedOperations)
                                    }
                                } else {
                                    return Err(Error::CheckedOperations)
                                }
                            } else {
                                return Err(Error::CheckedOperations)
                            }
                        } else {
                            return Err(Error::CheckedOperations)
                        }
                    } else {
                        return Err(Error::CheckedOperations)
                    }
                } else {
                    return Err(Error::NotInSaleList)
                }
            } else {
                return Err(Error::NotInMarket)
            }
        }
        /// Bid Token for sale, transferred_value() is the bidding price
        #[ink(message)]
        #[ink(payable)]
        pub fn bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if let Some(sale_information) = self.manager.market_list.get(&(&nft_contract_address, &token_id)) {
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
                // Some bidders already in the list
                if let Some(mut bidders) = self.manager.bidders.get(&(&nft_contract_address, &seller, &token_id)) {
                    // Check if Bid already in the list --> not allow, have to remove bid and add new bid
                    let length = bidders.len();
                    // TODO: make 30 variable
                    if length > 30 { // Only allow max 30 bids per token sale
                        return Err(Error::InvalidBidLength)
                    }
                    for item in bidders.iter().take(length) {
                        if item.bidder == caller {
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
            } else {
                return Err(Error::NotInMarket)
            }
        }
        /// Remove Bid From Active Sale
        #[ink(message)]
        pub fn remove_bid(&mut self, nft_contract_address: AccountId, token_id: Id) -> Result<(), Error> {
            if let Some(sale_information) = self.manager.market_list.get(&(&nft_contract_address, &token_id)) {
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
                if let Some(mut bidders) = self.manager.bidders.get(&(&nft_contract_address, &seller, &token_id)) {
                    // Check if Bid for this caller already in the list
                    let mut index_bid: i32 = -1;
                    let mut bid_value = 0;
                    for (index, item) in bidders.iter().enumerate() {
                        if item.bidder == caller {
                            index_bid = index as i32;
                            bid_value = item.bid_value;
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
                    Ok(())
                } else {
                    return Err(Error::BidNotExist)
                }
            } else {
                return Err(Error::NotInMarket)
            }
        }
        /// Accept Bid
        #[ink(message)]
        pub fn accept_bid(
            &mut self,
            nft_contract_address: AccountId,
            token_id: Id,
            bid_index: u32,
        ) -> Result<(), Error> {
            if let Some(mut sale_information) = self.manager.market_list.get(&(&nft_contract_address, &token_id)) {
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
                if let Some(mut last_index) = self.manager.sale_tokens_ids_last_index.get(&(&Some(&nft_contract_address), &Some(&caller))) {
                    if !self.manager.sale_tokens_ids.contains_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone()){
                        return Err(Error::NotInSaleList)
                    }
                    self.manager.sale_tokens_ids.remove_value(&(&Some(&nft_contract_address), &Some(&caller)), &token_id.clone());
                    if let Some(last_index_tmp) = last_index.checked_sub(1) {
                        last_index = last_index_tmp;
                    } else {
                        return Err(Error::CheckedOperations)
                    }
                    self.manager
                        .sale_tokens_ids_last_index
                        .insert(&(&Some(&nft_contract_address), &Some(&caller)), &last_index);

                    // Check through all the bids for this NFT and return AZERO to lose bidders
                    // then clear the bidders mapping
                    if let Some(bidders) = self.manager.bidders.get(&(&nft_contract_address, &seller, &token_id.clone())) {
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
                                ).is_err(){
                                    return Err(Error::CannotTransfer)
                                }
                                let price = item.bid_value;
                                // Calculate platform fee that seller has to pay
                                if let Some(price_mul_platform_fee) = price.checked_mul(self.manager.platform_fee as u128) {
                                    if let Some(platform_fee) = price_mul_platform_fee.checked_div(10000) {
                                        // Calculate Fee after Staking discount
                                        let platform_fee_after_discount = self.apply_discount(&seller, &platform_fee);
                                        // Save profit as platform statistics
                                        if let Some(total_profit_tmp) = self.manager.total_profit.checked_add(platform_fee_after_discount) {
                                            self.manager.total_profit = total_profit_tmp;
                                        }
                                        if let Some(current_profit_tmp) = self.manager.current_profit.checked_add(platform_fee_after_discount) {
                                            self.manager.current_profit = current_profit_tmp;
                                        }
                                        // Calculate Royalty Fee that seller has to pay to Collection Owner
                                        if let Some(price_mul_sale_royalty_fee) = price.checked_mul(sale_information.royalty_fee_at_listing as u128){
                                            if let Some(royalty_fee) = price_mul_sale_royalty_fee.checked_div(10000) {
                                                if royalty_fee > 0 || royalty_fee > self.env().balance() {
                                                    if let Some(collection_owner) = ArtZeroCollectionRef::get_collection_owner(
                                                        &self.manager.collection_contract_address,
                                                        nft_contract_address,
                                                    ) {
                                                        // Send AZERO to collection owner
                                                        if self.env().transfer(collection_owner, royalty_fee).is_err() {
                                                            return Err(Error::CannotTransfer)
                                                        }
                                                    } else {
                                                        return Err(Error::InvalidCollectionOwner)
                                                    }
                                                } else {
                                                    return Err(Error::NotEnoughBalance)
                                                }
                                                if let Some(price_mul_royalty_fee) = price.checked_sub(royalty_fee) {
                                                    if let Some(seller_fee) = price_mul_royalty_fee.checked_sub(platform_fee_after_discount) {
                                                        // Send AZERO to seller
                                                        if seller_fee > 0 {
                                                            if seller_fee > self.env().balance() {
                                                                return Err(Error::NotEnoughBalance)
                                                            }
                                                            if self.env().transfer(seller, seller_fee).is_err(){
                                                                return Err(Error::CannotTransfer)
                                                            }
                                                        }
                                                        
                                                        // Save all volume as platform Statistics
                                                        let mut collection_volume = self.manager.volume_by_collection.get(&nft_contract_address).unwrap_or(0);
                                                        let mut user_volume = self.manager.volume_by_user.get(&seller).unwrap_or(0);
                                                        if let Some(collection_volume_tmp) = collection_volume.checked_add(price) {
                                                            if let Some(user_volume_tmp) = user_volume.checked_add(price) {
                                                                collection_volume = collection_volume_tmp;
                                                                user_volume = user_volume_tmp;
                                                                self.manager.volume_by_collection.insert(&nft_contract_address, &collection_volume);
                                                                self.manager.volume_by_user.insert(&seller, &user_volume);
                                                                self.update_listed_token_by_collection_address(&nft_contract_address, &false);
                                                                if let Some(total_volume_tmp) = self.manager.total_volume.checked_add(price) {
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
                                                                    self.manager.total_volume = total_volume_tmp;
                                                                }
                                                            } else {
                                                                return Err(Error::CheckedOperations)
                                                            }
                                                        } else {
                                                            return Err(Error::CheckedOperations)
                                                        }
                                                    } else {
                                                        return Err(Error::CheckedOperations)
                                                    }
                                                } else {
                                                    return Err(Error::CheckedOperations)
                                                }
                                            } else {
                                                return Err(Error::CheckedOperations)
                                            }
                                        } else {
                                            return Err(Error::CheckedOperations)
                                        }
                                    } else {
                                        return Err(Error::CheckedOperations)
                                    }
                                } else {
                                    return Err(Error::CheckedOperations)
                                }
                            } else {
                                // Add bidder amount to hold_amount_bidders
                                if let Some(hold_amount_bidder) = self.manager.hold_amount_bidders.get(&item.bidder) {
                                    if let Some(hold_amount_bidder_tmp) = hold_amount_bidder.checked_add(item.bid_value) {
                                        self.manager.hold_amount_bidders.insert(&item.bidder, &hold_amount_bidder_tmp);
                                    } else {
                                        return Err(Error::CheckedOperations)
                                    }
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
                        Ok(())
                    } else {
                        return Err(Error::BidNotExist)
                    }
                } else {
                    return Err(Error::NotInSaleList)
                }
            } else {
                return Err(Error::NotInMarket)
            }
        }

        // This function update the total count of NFT for sale by using NFT Collection address
        fn update_listed_token_by_collection_address(&mut self, nft_contract_address: &AccountId, mode: &bool) {
            let listed_token_count = self
                .manager
                .listed_token_number_by_collection_address
                .get(nft_contract_address);
            if let Some(mut listed_token_count_unwarp) = listed_token_count {
                if *mode {
                    if let Some(listed_token_count_unwarp_tmp) = listed_token_count_unwarp.checked_add(1) {
                        listed_token_count_unwarp = listed_token_count_unwarp_tmp;
                    }
                } else {
                    if let Some(listed_token_count_unwarp_tmp) = listed_token_count_unwarp.checked_sub(1) {
                        listed_token_count_unwarp = listed_token_count_unwarp_tmp;
                    }
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
                    return (*input_fee * (10000_u128.checked_sub(self.manager.staking_discount_rate[index] as u128).unwrap())).checked_div(10000).unwrap()
                }
            }
            *input_fee
        }
    }
}
