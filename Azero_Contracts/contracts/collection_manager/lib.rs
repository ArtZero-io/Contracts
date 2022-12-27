#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_collection_manager {
    use ink_lang::ToAccountId;
    use ink_prelude::{
        string::{
            String,
        },
        vec,
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate
    };
    use openbrush::{
        contracts::access_control::*,
        contracts::ownable::*,
        modifiers,
        traits::{
            Storage,
            ZERO_ADDRESS
        }
    };
    use artzero_project::{
        impls::collection_manager::*,
        traits::{
            admin::*,
            error::Error,
        }
    };
    use psp34_nft::psp34_nft::Psp34NftRef;
    use artzero_project::traits::psp34_standard::*;

    #[derive(Default, SpreadAllocate, Storage)]
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
    }

    #[ink(event)]
    pub struct AddNewCollectionEvent {
        collection_owner: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        contract_type: u8,
        is_active: bool,
        show_on_chain_metadata: bool,
    }

    impl AccessControl for ArtZeroCollectionManager {}
    impl Ownable for ArtZeroCollectionManager {}
    impl ArtZeroCollectionTrait for ArtZeroCollectionManager {}
    impl ArtZeroAdminTrait for ArtZeroCollectionManager {}

    impl ArtZeroCollectionManager {
        /// Collection Contract Manager manages all collections on ArtZero platform. User can create in simple mode or in advanced mode
        /// In Simple mode, the contract will automatically create the standard NFT contract for the User
        /// In Advanced mode, user creates their own customized NFT Contract and add the contract address to the Collection Manager
        #[ink(constructor)]
        pub fn new(
            admin_address: AccountId,
            owner_address: AccountId,
            standard_nft_hash: Hash,
            simple_mode_adding_fee: Balance,
            advance_mode_adding_fee: Balance,
            max_royal_fee_rate: u32,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(owner_address);
                instance._init_with_admin(instance.env().caller());
                instance.grant_role(ADMINER, admin_address).expect("Should grant the role");
                instance
                    .initialize(
                        admin_address,
                        standard_nft_hash,
                        simple_mode_adding_fee,
                        advance_mode_adding_fee,
                        max_royal_fee_rate,
                    )
                    .ok()
                    .unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            admin_address: AccountId,
            standard_nft_hash: Hash,
            simple_mode_adding_fee: Balance,
            advance_mode_adding_fee: Balance,
            max_royal_fee_rate: u32,
        ) -> Result<(), Error> {
            if self.manager.admin_address != ZERO_ADDRESS.into(){
                return Err(Error::AlreadyInit);
            }
            self.manager.collection_count = 0;
            self.manager.active_collection_count = 0;
            self.manager.simple_mode_adding_fee = simple_mode_adding_fee;
            self.manager.advance_mode_adding_fee = advance_mode_adding_fee;
            self.manager.admin_address = admin_address;
            self.manager.standard_nft_hash = standard_nft_hash;
            self.manager.max_royal_fee_rate = max_royal_fee_rate;
            Ok(())
        }

        /// Simple New Collection Creation - Auto create NFT Contract - Collection_Owner is owner of NFT contract and receive royalty fee
        #[ink(message)]
        #[ink(payable)]
        pub fn auto_new_collection(
            &mut self,
            nft_name: String,
            nft_symbol: String,
            collection_owner: AccountId,
            attributes: Vec<String>,
            attribute_vals: Vec<String>,
            is_collect_royal_fee: bool,
            royal_fee: u32,
        ) -> Result<(), Error> {
            // Check if caller sends correct adding fee to the contract
            assert!(
                self.manager.simple_mode_adding_fee == self.env().transferred_value(),
                "invalid fee"
            );
            // Check if royalty fee is less than maximal value
            assert!(royal_fee <= self.manager.max_royal_fee_rate, "invalid royal fee");

            // Create PSP34 Standard Contract using cross call to psp34_standard contract
            let contract = Psp34NftRef::new(collection_owner, nft_name, nft_symbol)
                .endowment(0)
                .code_hash(self.manager.standard_nft_hash)
                .salt_bytes(self.manager.collection_count.to_le_bytes())
                .instantiate()
                .unwrap_or_else(|error| panic!("failed at instantiating the NFT contract: {:?}", error));
            let contract_account: AccountId = contract.to_account_id();

            self.manager.collection_count += 1;
            self.manager.active_collection_count += 1;

            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &contract_account);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            if collections_by_owner.is_none() {
                let mut collections = Vec::<AccountId>::new();
                collections.push(contract_account);
                self.manager
                    .collections_by_owner
                    .insert(&collection_owner, &collections);
            } else {
                let mut collections = collections_by_owner.unwrap();
                collections.push(contract_account);
                self.manager
                    .collections_by_owner
                    .insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                nft_contract_address: contract_account,
                contract_type: 2,
                is_collect_royal_fee,
                royal_fee,
                is_active: true,
                show_on_chain_metadata: true,
            };
            self.manager.collections.insert(&contract_account, &new_collection);
            if self
                .set_multiple_attributes(contract_account, attributes, attribute_vals)
                .is_err()
            {
                panic!("error set_multiple_attributes")
            };
            // Emit AddNewCollectionEvent event for tracking purposes
            self.env().emit_event(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(contract_account),
                contract_type: 2,
                is_active: true,
                show_on_chain_metadata: true,
            });
            Ok(())
        }

        /// Advanced Add new collection - with fee in AZERO 1% = 100 - anyone can add Existing contract - Collection_Owner is owner of NFT contract and receive royal fee
        #[ink(message)]
        #[ink(payable)]
        pub fn add_new_collection(
            &mut self,
            collection_owner: AccountId,
            nft_contract_address: AccountId,
            attributes: Vec<String>,
            attribute_vals: Vec<String>,
            is_collect_royal_fee: bool,
            royal_fee: u32,
        ) -> Result<(), Error> {
            // Check if caller sends correct adding fee to the contract
            assert!(
                self.manager.advance_mode_adding_fee == self.env().transferred_value(),
                "invalid fee"
            );
            // Check if the contract already exists
            assert!(
                self.manager.collections.get(&nft_contract_address).is_none(),
                "address exists"
            );

            // Only the owner of the NFT contract collection can perform this call
            let collection_owner = Psp34Ref::get_owner(&nft_contract_address);
            assert!(collection_owner == self.env().caller());
            // Check if royalty fee is less than maximal value
            assert!(royal_fee <= self.manager.max_royal_fee_rate, "invalid royal fee");
            self.manager.collection_count += 1;
            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &nft_contract_address);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            if collections_by_owner.is_none() {
                let collections = vec![nft_contract_address];
                self.manager
                    .collections_by_owner
                    .insert(&collection_owner, &collections);
            } else {
                let mut collections = collections_by_owner.unwrap();
                collections.push(nft_contract_address);
                self.manager
                    .collections_by_owner
                    .insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                nft_contract_address,
                contract_type: 1,
                is_collect_royal_fee,
                royal_fee,
                is_active: false,
                show_on_chain_metadata: false,
            };
            self.manager.collections.insert(&nft_contract_address, &new_collection);
            if self
                .set_multiple_attributes(nft_contract_address, attributes, attribute_vals)
                .is_err()
            {
                panic!("error set_multiple_attributes")
            };
            // Emit AddNewCollectionEvent event for tracking purposes
            self.env().emit_event(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(nft_contract_address),
                contract_type: 1,
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
        ) -> Result<(), AccessControlError> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
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
        ) -> Result<(), AccessControlError> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
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
            assert!(attributes.len() == values.len(), "Inputs not same length");
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let collection = self.manager.collections.get(&contract_address).unwrap();
            if collection.collection_owner == self.env().caller() || self.has_role(ADMINER, self.env().caller()) {
                let length = attributes.len();
                for i in 0..length {
                    let attribute = attributes[i].clone();
                    let value = values[i].clone();
                    self._set_attribute(contract_address, attribute.into_bytes(), value.into_bytes());
                }
                Ok(())
            } else {
                return Err(Error::CollectionOwnerAndAdmin)
            }
        }

        // Get attributes of an NFT Collection
        #[ink(message)]
        pub fn get_attributes(&self, account: AccountId, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                if let Some(value) = self.manager.attributes.get(&(&account, &attribute.into_bytes())) {
                    ret.push(String::from_utf8(value).unwrap());
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }
        /// Internal function to add attributes to mapping
        fn _set_attribute(&mut self, account: AccountId, key: Vec<u8>, value: Vec<u8>) {
            self.manager.attributes.insert(&(&account, &key), &value);
        }

        /// Update Type Collection - Only Admin Role can change - 1: Manual 2: Auto
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_contract_type(&mut self, contract_address: AccountId, contract_type: u8) -> Result<(), AccessControlError> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.contract_type = contract_type;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update Is Royalty Fee - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_collect_royal_fee(
            &mut self,
            contract_address: AccountId,
            is_collect_royal_fee: bool,
        ) -> Result<(), AccessControlError> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.is_collect_royal_fee = is_collect_royal_fee;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update royalty fee of an NFT Collection - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_royal_fee(&mut self, contract_address: AccountId, new_fee: u32) -> Result<(), AccessControlError> {
            assert!(new_fee <= self.manager.max_royal_fee_rate, "invalid fee");
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            collection.royal_fee = new_fee;
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
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            if self.env().caller() == collection.collection_owner || self.has_role(ADMINER, self.env().caller()) {
                collection.show_on_chain_metadata = show_on_chain_metadata;
                self.manager.collections.insert(&contract_address, &collection);
                Ok(())
            } else {
                return Err(Error::OnlyAdmin)
            }
        }

        /// Update Active Status When its active, collection will be shown on the UI and will be tradable - only Admin can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_active(&mut self, contract_address: AccountId, is_active: bool) -> Result<(), AccessControlError> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(is_active != collection.is_active);
            collection.is_active = is_active;
            if is_active == true {
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
            self.manager.advance_mode_adding_fee = advance_mode_adding_fee;
            Ok(())
        }

        /// Update Max Royal Fee Rate - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_max_royal_fee_rate(&mut self, max_royal_fee_rate: u32) -> Result<(), Error> {
            self.manager.max_royal_fee_rate = max_royal_fee_rate;
            Ok(())
        }

        /// Update Admin Address - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(&mut self, admin_address: AccountId) -> Result<(), Error> {
            self.manager.admin_address = admin_address;
            Ok(())
        }

        // GETTERS
        /// Get Collection Information by Collection Address (NFT address)
        #[ink(message)]
        pub fn get_collection_by_address(&self, nft_contract_address: AccountId) -> Option<Collection> {
            return self.manager.collections.get(&nft_contract_address)
        }

        /// Get All Collection Addresses by Owner Address
        #[ink(message)]
        pub fn get_collections_by_owner(&self, owner_address: AccountId) -> Option<Vec<AccountId>> {
            return self.manager.collections_by_owner.get(&owner_address)
        }

        /// Get Standard Nft Hash
        #[ink(message)]
        pub fn get_standard_nft_hash(&self) -> Hash {
            return self.manager.standard_nft_hash
        }

        /// Get Collection Contract by ID
        #[ink(message)]
        pub fn get_contract_by_id(&self, id: u64) -> Option<AccountId> {
            return self.manager.collections_by_id.get(&id)
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_collection_count(&self) -> u64 {
            return self.manager.collection_count
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_active_collection_count(&self) -> u64 {
            return self.manager.active_collection_count
        }

        /// Get Simple Mode Adding Fee
        #[ink(message)]
        pub fn get_simple_mode_adding_fee(&self) -> Balance {
            return self.manager.simple_mode_adding_fee
        }

        /// Get Advance Mode Adding Fee
        #[ink(message)]
        pub fn get_advance_mode_adding_fee(&self) -> Balance {
            return self.manager.advance_mode_adding_fee
        }

        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(&self) -> AccountId {
            return self.manager.admin_address
        }

        /// Get Royal Max Fee
        #[ink(message)]
        pub fn get_max_royal_fee_rate(&self) -> u32 {
            return self.manager.max_royal_fee_rate
        }
    }
}
