
pub use crate::{
    impls::marketplace::{
        data,
        data::*,
        marketplace,
        *,
    },
    traits::marketplace::*,
};
use openbrush::{
    modifiers,
    contracts::access_control::*,
    contracts::ownable::*,
    contracts::psp34::extensions::{
        enumerable::*
    },
    traits::{
        Storage,
        AccountId,
        Balance
    }
};
use ink::prelude::{
    vec::Vec,
};
use crate::traits::error::*;

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T: Storage<Manager> + 
    Storage<ownable::Data>
> ArtZeroMarketplaceTrait for T {
    // GETTERS
    /// Get market list information using NFT Collection and token ID
    default fn get_nft_sale_info(&self, nft_contract_address: AccountId, token_id: Id) -> Option<ForSaleItem> {
        self.data::<Manager>().market_list.get(&(&nft_contract_address, &token_id))
    }

    /// Get platform fee
    default fn get_platform_fee(&self) -> u32 {
        self.data::<Manager>().platform_fee
    }

    /// Get Staking Discount Criteria
    default fn get_staking_discount_criteria(&self) -> Vec<u8> {
        self.data::<Manager>().staking_discount_criteria.clone()
    }

    /// Get Staking Discount Rates
    default fn get_staking_discount_rate(&self) -> Vec<u16> {
        self.data::<Manager>().staking_discount_rate.clone()
    }

    /// Get listed token count by collection address
    default fn get_listed_token_count_by_collection_address(&self, collection_contract_address: AccountId) -> u64 {
        self.data::<Manager>()
            .listed_token_number_by_collection_address
            .get(&collection_contract_address).unwrap_or(0)
    }

    /// Get all token ids currently for sale for a collection (nft_contract_address,user_account)
    default fn get_for_sale_token_id(
        &self,
        nft_contract_address: AccountId,
        user_account: AccountId,
        index: u128,
    ) -> Option<Id> {
        self.data::<Manager>()
            .sale_tokens_ids
            .get_value(&(&Some(&nft_contract_address), &Some(&user_account)), &index)
    }

    /// Get get total sale token ids of user account in a contract
    default fn get_sale_tokens_ids_count(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128 {
        self.data::<Manager>().sale_tokens_ids.count(&(&Some(&nft_contract_address), &Some(&user_account)))
    }

    /// Get all token ids currently for sale by a collection (nft_contract_address,user_account)
    default fn total_tokens_for_sale(&self, nft_contract_address: AccountId, user_account: AccountId) -> u128 {
        self.data::<Manager>()
            .sale_tokens_ids_last_index
            .get(&(&Some(&nft_contract_address), &Some(&user_account))).unwrap_or(0)

    }

    /// Get all bids from (NFT Contract Address, User Address, token ID)
    default fn get_all_bids(
        &self,
        nft_contract_address: AccountId,
        user_account: AccountId,
        token_id: Id,
    ) -> Option<Vec<BidInformation>> {
        self.data::<Manager>()
            .bidders
            .get(&(&nft_contract_address, &user_account, &token_id))
    }

    /// Get collection contract address
    default fn get_collection_contract_address(&self) -> AccountId {
        self.data::<Manager>().collection_contract_address
    }
    /// Get staking contract address
    default fn get_staking_contract_address(&self) -> AccountId {
        self.data::<Manager>().staking_contract_address
    }

    /// Get total platform volume
    default fn get_total_volume(&self) -> Balance {
        self.data::<Manager>().total_volume
    }
    /// Get total Collection volume
    default fn get_volume_by_collection(&self, collection_contract_address: AccountId) -> Balance {
        self.data::<Manager>().volume_by_collection.get(&collection_contract_address).unwrap_or(0)
    }

    /// Get platform total Profit
    default fn get_total_profit(&self) -> Balance {
        self.data::<Manager>().total_profit
    }

    /// Get platform current available profit
    default fn get_current_profit(&self) -> Balance {
        self.data::<Manager>().current_profit
    }

    /// Get hold amount of bidder
    default fn get_hold_amount_of_bidder(&self, bidder: AccountId) -> Option<Balance> {
        self.data::<Manager>().hold_amount_bidders.get(&bidder)
    }

    /// Get Hold Bidders by Index
    default fn get_hold_bidders_by_index(&self, index: u64) -> Option<AccountId> {
        self.data::<Manager>().hold_bidders.get_value(1, &(index as u128))
    }

    /// Get Hold Bidder Count
    default fn get_hold_bidder_count(&self) -> u64 {
        self.data::<Manager>().hold_bidders.count(1) as u64
    }

    /// Withdraw Profit - only Contract Owner.
    #[modifiers(only_owner)]
    default fn withdraw_profit(&mut self, value: Balance, reciever: AccountId) -> Result<(), Error> {
        if value > T::env().balance() || value > self.data::<Manager>().current_profit {
            return Err(Error::NotEnoughBalance)
        }
        if let Some(current_profit_tmp) = self.data::<Manager>().current_profit.checked_sub(value) {
            self.data::<Manager>().current_profit = current_profit_tmp;
            if T::env().transfer(reciever, value).is_err(){
                return Err(Error::CannotTransfer)
            }
            Ok(())
        } else {
            return Err(Error::CheckedOperations)
        }
    }

    /// Set new collection contract address - Only Owner
    #[modifiers(only_owner)]
    default fn set_collection_contract_address(&mut self, collection_contract_address: AccountId) -> Result<(), Error> {
        self.data::<Manager>().collection_contract_address = collection_contract_address;
        Ok(())
    }

    /// Set Platform fee - only owner
    #[modifiers(only_owner)]
    default fn set_platform_fee(&mut self, platform_fee: u32) -> Result<(), Error> {
        if platform_fee >= 10000{ // must less than 100%
            return Err(Error::InvalidFee)
        }
        self.data::<Manager>().platform_fee = platform_fee;
        Ok(())
    }

    /// Set new staking contract address - Only Owner
    #[modifiers(only_owner)]
    default fn set_staking_contract_address(&mut self, staking_contract_address: AccountId) -> Result<(), Error> {
        self.data::<Manager>().staking_contract_address = staking_contract_address;
        Ok(())
    }

    /// Set criteria and discount rate - Only Owner 2 vectors same size
    #[modifiers(only_owner)]
    default fn set_discount(&mut self, criteria: Vec<u8>, rates: Vec<u16>) -> Result<(), Error> {
        if criteria.len() != rates.len(){
            return Err(Error::InvalidInput)
        }
        for &item in rates.iter() {
            if item > 10000{
                return Err(Error::InvalidInput)
            }
        }
        self.data::<Manager>().staking_discount_criteria = criteria;
        self.data::<Manager>().staking_discount_rate = rates;
        Ok(())
    }

    /// Receive hold amount
    default fn receive_hold_amount(&mut self, receiver: AccountId) -> Result<(), Error> {
        if let Some(hold_amount) = self.data::<Manager>().hold_amount_bidders.get(&receiver) {
            if hold_amount > 0 {
                if hold_amount > T::env().balance() {
                    return Err(Error::NotEnoughBalance);
                }
                if T::env().transfer(receiver, hold_amount).is_err() {
                    return Err(Error::CannotTransfer)
                }
                self.data::<Manager>().hold_amount_bidders.remove(&receiver);
                self.data::<Manager>().hold_bidders.remove_value(1, &receiver);
            }
            Ok(())
        } else {
            return Err(Error::HoldAmountBidderNotExist)
        }
    }
}
