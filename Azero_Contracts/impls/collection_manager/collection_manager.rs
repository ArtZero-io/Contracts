use crate::traits::collection_manager::*;
pub use crate::{
    impls::collection_manager::{
        data,
        data::*,
        collection_manager,
        *,
    },
    traits::collection_manager::*,
};
use ink_lang::ToAccountId;
use ink_prelude::{
    vec,
    string::String,
    vec::Vec,
};
use ink_storage::{
    traits::{
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
};
use openbrush::{
    contracts::access_control::*,
    contracts::ownable::*,
    modifiers,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        AccountId,
        Hash,
        Balance
    }
};

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink_lang::selector_id!("ADMINER");

impl<T: Storage<Manager>> ArtZeroCollectionTrait for T {
    /// Get royal fee of the Collection
    fn get_royal_fee(&self, nft_contract_address: AccountId) -> u32 {
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return 0
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        if !collection.is_collect_royal_fee || !collection.is_active {
            return 0
        } else {
            collection.royal_fee
        }
    }

    /// Check if the Collection is active not
    fn is_active(&self, nft_contract_address: AccountId) -> bool {
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return false
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        return collection.is_active
    }

    /// Get NFT Contract Type 1 or 2 for PSP34
    fn get_contract_type(&self, nft_contract_address: AccountId) -> u8 {
        if self.data::<Manager>().collections.get(&nft_contract_address).is_none() {
            return 0
        }
        let collection = self.data::<Manager>().collections.get(&nft_contract_address).unwrap();
        return collection.contract_type
    }

    /// Get Collection Owner by Collection Address (NFT address)
    fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId> {
        return Some(Some(self.data::<Manager>().collections.get(&nft_contract_address).unwrap())?.collection_owner);
    }
}
