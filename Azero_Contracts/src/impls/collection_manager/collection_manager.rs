
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
    modifiers,
    contracts::access_control::*,
    traits::{
        Storage,
        AccountId
    }
};
use ink::prelude::{
    string::{
        String,
    },
    vec,
    vec::Vec,
};
use crate::traits::error::Error;

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T: Storage<Manager> + 
    Storage<access_control::Data>
> ArtZeroCollectionTrait for T 
where T: access_control::AccessControl {

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

    #[modifiers(only_role(ADMINER))]
    default fn update_collection_owner(&mut self, contract_address: AccountId, new_owner: AccountId,) -> Result<(), Error> {
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            let older_owner = collection.collection_owner;
            let collections_older_owner = self.data::<Manager>().collections_by_owner.get(&older_owner);
            if let Some(mut collections_older) = collections_older_owner {
                // Remove collections_by_owner of older owner
                let index = collection.collection_id as usize;
                let last_index = collections_older.len() - 1;
                
                if index != last_index {
                    // Swap address at index pos to the last one
                    let temp = collections_older[index];
                    collections_older[index] = collections_older[last_index];
                    collections_older[last_index] = temp;                        

                    // Update collection id of the last collection  
                    if let Some(mut updated_last_collection) = self.data::<Manager>().collections.get(&collections_older[index]) {
                        updated_last_collection.collection_id = index as u32;
                        self.data::<Manager>().collections.insert(&collections_older[index], &updated_last_collection);
                    } else {
                        return Err(Error::Custom(String::from("The collection data of last index does not exist")))
                    }
                }
                // Remove the last one after swapping check and update collections_by_owner
                collections_older.pop();
                self.data::<Manager>().collections_by_owner.insert(&older_owner, &collections_older);
                
                // Update collections_by_owner
                let collections_by_owner = self.data::<Manager>().collections_by_owner.get(&new_owner);
                let mut collection_id = 0;
                if let Some(mut collections) = collections_by_owner {
                    collections.push(contract_address);
                    collection_id = (collections.len() as u32) - 1;
                    self.data::<Manager>().collections_by_owner.insert(&new_owner, &collections);
                } else {
                    let collections = vec![contract_address];
                    self.data::<Manager>().collections_by_owner.insert(&new_owner, &collections);
                }
                // Update collections
                collection.collection_owner = new_owner;
                collection.collection_id = collection_id;
                self.data::<Manager>().collections.insert(&contract_address, &collection);
                Ok(())                    
            } else {
                return Err(Error::Custom(String::from("This owner not exist")))
            }
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    default fn set_multiple_attributes(&mut self, contract_address: AccountId, attributes: Vec<String>, values: Vec<String>) -> Result<(), Error> {
        if attributes.len() != values.len() {
            return Err(Error::InvalidInput)
        }
        if let Some(collection) = self.data::<Manager>().collections.get(&contract_address) {
            if collection.collection_owner == T::env().caller() || self.has_role(ADMINER, T::env().caller()) {
                let mut collection_attributes = self.data::<Manager>().attributes.get(&contract_address).unwrap_or_default();
                for i in 0..attributes.len() {
                    let attribute = attributes[i].clone();
                    let value = values[i].clone();
                    if self.has_attribute(contract_address, attribute.clone()) {
                        if let Some(index) = self.get_collection_attribute_index(contract_address, attribute.clone()) {
                            collection_attributes[index as usize] = (attribute.into(), value.into());
                        } else {
                            collection_attributes.push((attribute.into(), value.into()));
                        }
                    } else {
                        collection_attributes.push((attribute.into(), value.into()));
                    }
                }
                self.data::<Manager>().attributes.insert(&contract_address, &collection_attributes);
                Ok(())
            } else {
                Err(Error::CollectionOwnerAndAdmin)
            }
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    default fn has_attribute(&self, contract_address: AccountId, attribute_key: String) -> bool {
        if self.data::<Manager>().attributes.get(&contract_address).is_some() {
            return self.data::<Manager>().attributes.get(&contract_address).unwrap().iter().any(|attribute| attribute.0 == attribute_key.clone().into_bytes());
        }
        false
    }

    default fn get_collection_attribute_index(&self, contract_address: AccountId, attribute_key: String) -> Option<u64> {
        return Some(self.data::<Manager>().attributes.get(&contract_address).unwrap().iter().position(|attribute| attribute.0 == attribute_key.clone().into_bytes()).unwrap() as u64);
    }
}
