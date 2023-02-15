#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#![allow(clippy::let_unit_value)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod artzero_collection_manager {
    use ink::ToAccountId;
    use ink::prelude::{
        string::{
            String,
        },
        vec,
        vec::Vec,
    };
    use openbrush::{
        contracts::access_control::*,
        contracts::ownable::*,
        modifiers,
        traits::{
            DefaultEnv,
            Storage,
        }
    };
    use artzero_project::{
        impls::collection_manager::*,
        traits::{
            admin::*,
            upgradable::*,
            error::Error,
        }
    };
    use psp34_nft::psp34_nft::Psp34NftRef;
    use artzero_project::traits::psp34_standard::*;

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroCollectionManager {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: artzero_project::impls::collection_manager::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
    }

    #[ink(event)]
    pub struct AddNewCollectionEvent {
        collection_owner: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        contract_type: CollectionType,
        is_active: bool,
        show_on_chain_metadata: bool,
    }

    impl AccessControl for ArtZeroCollectionManager {}
    impl Ownable for ArtZeroCollectionManager {}
    impl ArtZeroCollectionTrait for ArtZeroCollectionManager {}
    impl AdminTrait for ArtZeroCollectionManager {}
    impl UpgradableTrait for ArtZeroCollectionManager {}

    impl ArtZeroCollectionManager {
        /// Collection Contract Manager manages all collections on ArtZero platform. User can create in simple mode or in advanced mode
        /// In Simple mode, the contract will automatically create the standard NFT contract for the User
        /// In Advanced mode, user creates their own customized NFT Contract and add the contract address to the Collection Manager
        #[ink(constructor)]
        pub fn new(
            admin_address: AccountId,
            standard_nft_hash: Hash,
            simple_mode_adding_fee: Balance,
            advance_mode_adding_fee: Balance,
            max_royalty_fee_rate: u32,
        ) -> Self {
            assert!(simple_mode_adding_fee > 0 && advance_mode_adding_fee > 0);
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_owner(caller);
            instance
                .initialize(
                    standard_nft_hash,
                    simple_mode_adding_fee,
                    advance_mode_adding_fee,
                    max_royalty_fee_rate,
                    admin_address
                )
                .ok()
                .unwrap();
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            standard_nft_hash: Hash,
            simple_mode_adding_fee: Balance,
            advance_mode_adding_fee: Balance,
            max_royalty_fee_rate: u32,
            admin_address: AccountId
        ) -> Result<(), Error> {
            //Fee will never set to Zero to prevent spamming the contract so can be used for checking
            if self.manager.simple_mode_adding_fee > 0 || self.manager.advance_mode_adding_fee > 0 {
                return Err(Error::AlreadyInit);
            }
            self.manager.collection_count = 0;
            self.manager.active_collection_count = 0;
            self.manager.simple_mode_adding_fee = simple_mode_adding_fee;
            self.manager.advance_mode_adding_fee = advance_mode_adding_fee;
            self.manager.standard_nft_hash = standard_nft_hash;
            self.manager.max_royalty_fee_rate = max_royalty_fee_rate;
            self._init_with_admin(self.env().caller());
            self.grant_role(ADMINER, admin_address).expect("Should grant the role");
            Ok(())
        }

        /// Simple New Collection Creation - Auto create NFT Contract - Collection_Owner is owner of NFT contract and receive royalty fee
        #[ink(message)]
        #[ink(payable)]
        pub fn auto_new_collection(
            &mut self,
            nft_name: String,
            nft_symbol: String,
            attributes: Vec<String>,
            attribute_vals: Vec<String>,
            is_collect_royalty_fee: bool,
            royalty_fee: u32,
        ) -> Result<(), Error> {
            let collection_owner = self.env().caller();
            // Check if caller sends correct adding fee to the contract
            if self.manager.simple_mode_adding_fee != self.env().transferred_value(){
                return Err(Error::InvalidFee)
            }
            // Check if royalty fee is less than maximal value
            if royalty_fee > self.manager.max_royalty_fee_rate{
                return Err(Error::InvalidInput)
            }

            // Create PSP34 Standard Contract using cross call to psp34_standard contract
            let contract = Psp34NftRef::new(collection_owner, nft_name, nft_symbol)
                .endowment(0)
                .code_hash(self.manager.standard_nft_hash)
                .salt_bytes(self.manager.collection_count.to_le_bytes())
                .instantiate()
                .unwrap_or_else(|error| panic!("failed at instantiating the NFT contract: {error:?}"));
            let contract_account: AccountId = contract.to_account_id();

            self.manager.collection_count = self.manager.collection_count.checked_add(1).unwrap();
            self.manager.active_collection_count = self.manager.active_collection_count.checked_add(1).unwrap();

            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &contract_account);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            if let Some(mut collections) = collections_by_owner {
                collections.push(contract_account);
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            } else {
                let collections = vec![contract_account];
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                nft_contract_address: contract_account,
                contract_type: CollectionType::Psp34Auto,
                is_collect_royalty_fee,
                royalty_fee,
                is_active: false,
                show_on_chain_metadata: true,
            };
            self.manager.collections.insert(&contract_account, &new_collection);
            if self
                .set_multiple_attributes(contract_account, attributes, attribute_vals)
                .is_err()
            {
                return Err(Error::Custom(String::from("Cannot set attributes")))
            };
            // Emit AddNewCollectionEvent event for tracking purposes
            self.env().emit_event(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(contract_account),
                contract_type: CollectionType::Psp34Auto,
                is_active: false,
                show_on_chain_metadata: true,
            });
            Ok(())
        }

        /// Advanced Add new collection - with fee in AZERO 1% = 100 - anyone can add Existing contract - Collection_Owner is owner of NFT contract and receive royal fee
        #[ink(message)]
        #[ink(payable)]
        pub fn add_new_collection(
            &mut self,
            nft_contract_address: AccountId,
            attributes: Vec<String>,
            attribute_vals: Vec<String>,
            is_collect_royalty_fee: bool,
            royalty_fee: u32,
        ) -> Result<(), Error> {
            // Check if caller sends correct adding fee to the contract
            if self.manager.advance_mode_adding_fee != self.env().transferred_value(){
                return Err(Error::InvalidFee)
            }
            // Check if the contract already exists
            if self.manager.collections.get(&nft_contract_address).is_some(){
                return Err(Error::Custom(String::from("Collection exists")))
            }

            // Only the owner of the NFT contract collection can perform this call
            let collection_owner = Psp34Ref::get_owner(&nft_contract_address);
            if collection_owner != self.env().caller(){
                return Err(Error::NotOwner)
            }
            // Check if royalty fee is less than maximal value
            if royalty_fee > self.manager.max_royalty_fee_rate {
                return Err(Error::InvalidInput)
            }
            self.manager.collection_count = self.manager.collection_count.checked_add(1).unwrap();
            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &nft_contract_address);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            if let Some(mut collections) = collections_by_owner {
                collections.push(nft_contract_address);
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            } else {
                let collections = vec![nft_contract_address];
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                nft_contract_address,
                contract_type: CollectionType::Psp34Manual,
                is_collect_royalty_fee,
                royalty_fee,
                is_active: false,
                show_on_chain_metadata: false,
            };
            self.manager.collections.insert(&nft_contract_address, &new_collection);
            if self
                .set_multiple_attributes(nft_contract_address, attributes, attribute_vals)
                .is_err()
            {
                return Err(Error::Custom(String::from("Cannot set attributes")))
            };
            // Emit AddNewCollectionEvent event for tracking purposes
            self.env().emit_event(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(nft_contract_address),
                contract_type: CollectionType::Psp34Manual,
                is_active: false,
                show_on_chain_metadata: false,
            });
            Ok(())
        }

        // SETTERS
        /// Update Owner of Collecion - who receive royalty fee - Only Admin can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_collection_owner(
            &mut self,
            contract_address: AccountId,
            new_owner: AccountId,
        ) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.collection_owner = new_owner;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update NFT Contract Address of a collection - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_nft_contract_address(
            &mut self,
            contract_address: AccountId,
            nft_contract_address: AccountId,
        ) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.nft_contract_address = nft_contract_address;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Set attributes for the collections
        #[ink(message)]
        pub fn set_multiple_attributes(
            &mut self,
            contract_address: AccountId,
            attributes: Vec<String>,
            values: Vec<String>,
        ) -> Result<(), Error> {
            if attributes.len() != values.len(){
                return Err(Error::InvalidInput)
            }
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let collection = self.manager.collections.get(&contract_address).unwrap();
            if collection.collection_owner == self.env().caller() || self.has_role(ADMINER, self.env().caller()) {
                let mut collection_attributes = self.manager.attributes.get(&contract_address).unwrap_or_default();
                for i in 0..attributes.len() {
                    let attribute = attributes[i].clone();
                    let value = values[i].clone();
                    if self.has_attribute(contract_address, attribute.clone()) {
                        let index = self.get_collection_attribute_index(contract_address, attribute.clone()).unwrap();
                        collection_attributes[index as usize] = (attribute.into(), value.into());
                    } else {
                        collection_attributes.push((attribute.into(), value.into()));
                    }
                }
                self.manager.attributes.insert(&contract_address, &collection_attributes);
                Ok(())
            } else {
                Err(Error::CollectionOwnerAndAdmin)
            }
        }

        // Get attributes of an NFT Collection
        #[ink(message)]
        pub fn get_attributes(&self, contract_address: AccountId, attributes: Vec<String>) -> Vec<String> {
            let mut ret = Vec::<String>::new();
            for item in attributes.iter().take(attributes.len()) {
                ret.push(self.get_attribute(contract_address, item.clone()));
            }
            ret
        }

        // Find an attribute of an NFT Collection by key
        #[ink(message)]
        pub fn get_attribute(&self, contract_address: AccountId, attribute_key: String) -> String {
            if self.manager.attributes.get(&contract_address).is_none() {
                return String::from("");
            }
            let collection_attributes = self.manager.attributes.get(&contract_address).unwrap();
            for item in collection_attributes.iter().take(collection_attributes.len()) {
                if item.0 == attribute_key.clone().into_bytes() {
                    return String::from_utf8(item.1.clone()).unwrap();
                }
            }
            String::from("")
        }

        /// Check collection has an attribute
        #[ink(message)]
        pub fn has_attribute(&self, contract_address: AccountId, attribute_key: String) -> bool {
            if self.manager.attributes.get(&contract_address).is_some() {
                return self.manager.attributes.get(&contract_address).unwrap().iter().any(|attribute| attribute.0 == attribute_key.clone().into_bytes());
            }
            false
        }

        /// Get attribute index of collection
        #[ink(message)]
        pub fn get_collection_attribute_index(&self, contract_address: AccountId, attribute_key: String) -> Option<u64> {
            return Some(self.manager.attributes.get(&contract_address).unwrap().iter().position(|attribute| attribute.0 == attribute_key.clone().into_bytes()).unwrap() as u64);
        }

        /// Count attributes of collection
        #[ink(message)]
        pub fn get_collection_attribute_count(&self, contract_address: AccountId) -> Option<u64> {
            if self.manager.attributes.get(&contract_address).is_none() {
                return Some(0);
            }
            Some(self.manager.attributes.get(&contract_address).unwrap().len() as u64)
        }

        /// Update Type Collection - Only Admin Role can change - 1: Manual 2: Auto
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_contract_type(&mut self, contract_address: AccountId, contract_type: CollectionType) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.contract_type = contract_type;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update Is Royalty Fee - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_collect_royalty_fee(
            &mut self,
            contract_address: AccountId,
            is_collect_royalty_fee: bool,
        ) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.is_collect_royalty_fee = is_collect_royalty_fee;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update royalty fee of an NFT Collection - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_royalty_fee(&mut self, contract_address: AccountId, new_fee: u32) -> Result<(), Error> {
            if new_fee > self.manager.max_royalty_fee_rate{
                return Err(Error::InvalidFee)
            }
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.royalty_fee = new_fee;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update show_on_chain_metadata - admin and collection owner can change
        #[ink(message)]
        pub fn update_show_on_chain_metadata(
            &mut self,
            contract_address: AccountId,
            show_on_chain_metadata: bool,
        ) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            if self.env().caller() == collection.collection_owner || self.has_role(ADMINER, self.env().caller()) {
                collection.show_on_chain_metadata = show_on_chain_metadata;
                self.manager.collections.insert(&contract_address, &collection);
                Ok(())
            } else {
                Err(Error::OnlyAdmin)
            }
        }

        /// Update Active Status When its active, collection will be shown on the UI and will be tradable - only Admin can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_active(&mut self, contract_address: AccountId, is_active: bool) -> Result<(), Error> {
            if self.manager.collections.get(&contract_address).is_none(){
                return Err(Error::Custom(String::from("Collection not exist")))
            }
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            if is_active == collection.is_active{
                return Err(Error::InvalidInput)
            }
            collection.is_active = is_active;
            if is_active {
                self.manager.active_collection_count = self.manager.active_collection_count.checked_add(1).unwrap();
            } else {
                self.manager.active_collection_count = self.manager.active_collection_count.checked_sub(1).unwrap();
            }
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update Simple Mode Adding Collection Fee - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_simple_mode_adding_fee(&mut self, simple_mode_adding_fee: Balance) -> Result<(), Error> {
            if simple_mode_adding_fee == 0{
                return Err(Error::InvalidFee);
            }
            self.manager.simple_mode_adding_fee = simple_mode_adding_fee;
            Ok(())
        }

        /// Update Standard NFT Hash - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_standard_nft_hash(&mut self, standard_nft_hash: Hash) -> Result<(), Error> {
            self.manager.standard_nft_hash = standard_nft_hash;
            Ok(())
        }

        /// Update Advance Mode Adding Collection Fee - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_advance_mode_adding_fee(&mut self, advance_mode_adding_fee: Balance) -> Result<(), Error> {
            if advance_mode_adding_fee == 0{
                return Err(Error::InvalidFee);
            }
            self.manager.advance_mode_adding_fee = advance_mode_adding_fee;
            Ok(())
        }

        /// Update Max Royalty Fee Rate - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_max_royalty_fee_rate(&mut self, max_royalty_fee_rate: u32) -> Result<(), Error> {
            self.manager.max_royalty_fee_rate = max_royalty_fee_rate;
            Ok(())
        }

        // GETTERS
        /// Get Collection Information by Collection Address (NFT address)
        #[ink(message)]
        pub fn get_collection_by_address(&self, nft_contract_address: AccountId) -> Option<Collection> {
            self.manager.collections.get(&nft_contract_address)
        }

        /// Get All Collection Addresses by Owner Address
        #[ink(message)]
        pub fn get_collections_by_owner(&self, owner_address: AccountId) -> Option<Vec<AccountId>> {
            self.manager.collections_by_owner.get(&owner_address)
        }

        /// Get Standard Nft Hash
        #[ink(message)]
        pub fn get_standard_nft_hash(&self) -> Hash {
            self.manager.standard_nft_hash
        }

        /// Get Collection Contract by ID
        #[ink(message)]
        pub fn get_contract_by_id(&self, id: u64) -> Option<AccountId> {
            self.manager.collections_by_id.get(&id)
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_collection_count(&self) -> u64 {
            self.manager.collection_count
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_active_collection_count(&self) -> u64 {
            self.manager.active_collection_count
        }

        /// Get Simple Mode Adding Fee
        #[ink(message)]
        pub fn get_simple_mode_adding_fee(&self) -> Balance {
            self.manager.simple_mode_adding_fee
        }

        /// Get Advance Mode Adding Fee
        #[ink(message)]
        pub fn get_advance_mode_adding_fee(&self) -> Balance {
            self.manager.advance_mode_adding_fee
        }

        /// Get Royalty Max Fee
        #[ink(message)]
        pub fn get_max_royalty_fee_rate(&self) -> u32 {
            self.manager.max_royalty_fee_rate
        }
    }
}
