
pub use crate::{
    impls::collection_manager::{
        data,
        data::*,
        collection_manager,
        *,
    },
    traits::collection_manager::*,
};
use openbrush::{
    contracts::access_control::*,
    traits::{
        Storage,
        AccountId,
    }
};

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T: Storage<Manager>> ArtZeroCollectionTrait for T {

    default fn get_royalty_fee(&self, nft_contract_address: AccountId) -> u32 {
        if let Some(collection) = self.data::<Manager>().collections.get(&nft_contract_address) {
            if !collection.is_collect_royalty_fee || !collection.is_active {
                return 0
            } else {
                collection.royalty_fee
            }
        } else {
            return 0;
        }
    }

    default fn is_active(&self, nft_contract_address: AccountId) -> bool {
        if let Some(collection) =  self.data::<Manager>().collections.get(&nft_contract_address) {
            return collection.is_active
        } else {
            return false
        }
    }

    default fn get_contract_type(&self, nft_contract_address: AccountId) -> CollectionType {
        if let Some(collection) = self.data::<Manager>().collections.get(&nft_contract_address) {
            collection.contract_type
        } else {
            return CollectionType::Unknown
        }
    }

    default fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId> {
        return Some(self.data::<Manager>().collections.get(&nft_contract_address)?.collection_owner);
    }
}
