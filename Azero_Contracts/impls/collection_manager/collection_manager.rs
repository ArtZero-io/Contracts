
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
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return 0
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        if !collection.is_collect_royalty_fee || !collection.is_active {
            return 0
        } else {
            collection.royalty_fee
        }
    }

    default fn is_active(&self, nft_contract_address: AccountId) -> bool {
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return false
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        return collection.is_active
    }

    default fn get_contract_type(&self, nft_contract_address: AccountId) -> CollectionType {
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return CollectionType::Unknown
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        return collection.contract_type
    }

    default fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId> {
        return Some(Some(self.data::<Manager>().collections.get(&nft_contract_address).unwrap())?.collection_owner);
    }
}
