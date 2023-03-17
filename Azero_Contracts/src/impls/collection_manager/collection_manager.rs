
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
    modifier_definition,
    modifiers,
    storage::Mapping,
    contracts::ownable::*,
    contracts::access_control::*,
    traits::{
        AccountId,
        Hash,
        Balance,
        OccupiedStorage,
        Storage,
        Timestamp,
        ZERO_ADDRESS,
    },
};
use ink::prelude::{
    string::{
        String,
    },
    vec,
    vec::Vec,
};
use crate::traits::error::Error;
use ink::{
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T, M> ArtZeroCollectionTrait for T
where
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>, Type = M>,
    T: Storage<Manager>,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    T: Storage<ownable::Data>
{
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

    default fn get_attributes(&self, contract_address: AccountId, attributes: Vec<String>) -> Vec<String> {
        let mut ret = Vec::<String>::new();
        for item in attributes.iter().take(attributes.len()) {
            ret.push(self.get_attribute(contract_address, item.clone()));
        }
        ret
    }

    default fn get_attribute(&self, contract_address: AccountId, attribute_key: String) -> String {
        if let Some(collection_attributes) = self.data::<Manager>().attributes.get(&contract_address) {
            for item in collection_attributes.iter().take(collection_attributes.len()) {
                if item.0 == attribute_key.clone().into_bytes() {
                    return String::from_utf8(item.1.clone()).unwrap();
                }
            }
            String::from("")
        } else {
            String::from("")
        }
    }

    /// Count attributes of collection
    default fn get_collection_attribute_count(&self, contract_address: AccountId) -> Option<u64> {
        if self.data::<Manager>().attributes.get(&contract_address).is_none() {
            return Some(0);
        }
        Some(self.data::<Manager>().attributes.get(&contract_address).unwrap().len() as u64)
    }

    /// Update Type Collection - Only Admin Role can change - 1: Manual 2: Auto
    #[modifiers(only_role(ADMINER))]
    default fn update_contract_type(&mut self, contract_address: AccountId, contract_type: CollectionType) -> Result<(), Error> {
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            collection.contract_type = contract_type;
            self.data::<Manager>().collections.insert(&contract_address, &collection);
            Ok(())
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    /// Update Is Royalty Fee - Only Admin Role can change
    #[modifiers(only_role(ADMINER))]
    default fn update_is_collect_royalty_fee(
        &mut self,
        contract_address: AccountId,
        is_collect_royalty_fee: bool,
    ) -> Result<(), Error> {
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            collection.is_collect_royalty_fee = is_collect_royalty_fee;
            self.data::<Manager>().collections.insert(&contract_address, &collection);
            Ok(())
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    /// Update royalty fee of an NFT Collection - Only Admin Role can change
    #[modifiers(only_role(ADMINER))]
    default fn update_royalty_fee(&mut self, contract_address: AccountId, new_fee: u32) -> Result<(), Error> {
        if new_fee > self.data::<Manager>().max_royalty_fee_rate{
            return Err(Error::InvalidFee)
        }
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            collection.royalty_fee = new_fee;
            self.data::<Manager>().collections.insert(&contract_address, &collection);
            Ok(())
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    /// Update show_on_chain_metadata - admin and collection owner can change
    default fn update_show_on_chain_metadata(
        &mut self,
        contract_address: AccountId,
        show_on_chain_metadata: bool,
    ) -> Result<(), Error> {
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            if T::env().caller() == collection.collection_owner || self.has_role(ADMINER, T::env().caller()) {
                collection.show_on_chain_metadata = show_on_chain_metadata;
                self.data::<Manager>().collections.insert(&contract_address, &collection);
                Ok(())
            } else {
                Err(Error::OnlyAdmin)
            }
        } else {
            return Err(Error::CollectionNotExist)
        }
    }

    /// Update Active Status When its active, collection will be shown on the UI and will be tradable - only Admin can change
    #[modifiers(only_role(ADMINER))]
    default fn update_is_active(
        &mut self, 
        contract_address: AccountId, 
        is_active: bool
    ) -> Result<(), Error> {
        if let Some(mut collection) = self.data::<Manager>().collections.get(&contract_address) {
            if is_active == collection.is_active{
                return Err(Error::InvalidInput)
            }
            collection.is_active = is_active;
            if is_active {
                if let Some(active_collection_count) = self.data::<Manager>().active_collection_count.checked_add(1) {
                    self.data::<Manager>().active_collection_count = active_collection_count;
                } else {
                    return Err(Error::CheckedOperations)
                }
            } else {
                if let Some(active_collection_count) = self.data::<Manager>().active_collection_count.checked_sub(1) {
                    self.data::<Manager>().active_collection_count = active_collection_count;
                } else {
                    return Err(Error::CheckedOperations)
                }
                
            }
            self.data::<Manager>().collections.insert(&contract_address, &collection);
            Ok(())
        } else {
            return Err(Error::CollectionNotExist)
        }
        
    }

    /// Update Simple Mode Adding Collection Fee - only Owner
    #[modifiers(only_owner)]
    default fn update_simple_mode_adding_fee(&mut self, simple_mode_adding_fee: Balance) -> Result<(), Error> {
        if simple_mode_adding_fee == 0{
            return Err(Error::InvalidFee);
        }
        self.data::<Manager>().simple_mode_adding_fee = simple_mode_adding_fee;
        Ok(())
    }

    /// Update Standard NFT Hash - only Owner
    #[modifiers(only_owner)]
    default fn update_standard_nft_hash(&mut self, standard_nft_hash: Hash) -> Result<(), Error> {
        self.data::<Manager>().standard_nft_hash = standard_nft_hash;
        Ok(())
    }

    /// Update Advance Mode Adding Collection Fee - only Owner
    #[modifiers(only_owner)]
    default fn update_advance_mode_adding_fee(&mut self, advance_mode_adding_fee: Balance) -> Result<(), Error> {
        if advance_mode_adding_fee == 0{
            return Err(Error::InvalidFee);
        }
        self.data::<Manager>().advance_mode_adding_fee = advance_mode_adding_fee;
        Ok(())
    }

    /// Update Max Royalty Fee Rate - only Owner
    #[modifiers(only_owner)]
    default fn update_max_royalty_fee_rate(&mut self, max_royalty_fee_rate: u32) -> Result<(), Error> {
        self.data::<Manager>().max_royalty_fee_rate = max_royalty_fee_rate;
        Ok(())
    }

    // GETTERS
    /// Get Collection Information by Collection Address (NFT address)
    default fn get_collection_by_address(&self, nft_contract_address: AccountId) -> Option<Collection> {
        self.data::<Manager>().collections.get(&nft_contract_address)
    }

    /// Get All Collection Addresses by Owner Address
    default fn get_collections_by_owner(&self, owner_address: AccountId) -> Option<Vec<AccountId>> {
        self.data::<Manager>().collections_by_owner.get(&owner_address)
    }

    /// Get Standard Nft Hash
    default fn get_standard_nft_hash(&self) -> Hash {
        self.data::<Manager>().standard_nft_hash
    }

    /// Get Collection Contract by ID
    default fn get_contract_by_id(&self, id: u64) -> Option<AccountId> {
        self.data::<Manager>().collections_by_id.get(&id)
    }

    /// Get Collection Count
    default fn get_collection_count(&self) -> u64 {
        self.data::<Manager>().collection_count
    }

    /// Get Collection Count
    default fn get_active_collection_count(&self) -> u64 {
        self.data::<Manager>().active_collection_count
    }

    /// Get Simple Mode Adding Fee
    default fn get_simple_mode_adding_fee(&self) -> Balance {
        self.data::<Manager>().simple_mode_adding_fee
    }

    /// Get Advance Mode Adding Fee
    default fn get_advance_mode_adding_fee(&self) -> Balance {
        self.data::<Manager>().advance_mode_adding_fee
    }

    /// Get Royalty Max Fee
    default fn get_max_royalty_fee_rate(&self) -> u32 {
        self.data::<Manager>().max_royalty_fee_rate
    }
}
