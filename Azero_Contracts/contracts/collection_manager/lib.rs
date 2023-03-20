#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#![allow(clippy::let_unit_value)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod artzero_collection_manager {
    use ink::prelude::{
        string::{
            String,
        },
        vec,
        vec::Vec,
    };
    use openbrush::{
        contracts::{
            access_control::{
                extensions::enumerable,
                members,
            },
        },
        contracts::access_control::extensions::enumerable::*,
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
    use ink::ToAccountId;
    use ink::{codegen::EmitEvent, reflect::ContractEventBase};
    
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroCollectionManager {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access_control: access_control::Data<enumerable::Members>,
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

    pub type Event = <ArtZeroCollectionManager as ContractEventBase>::Type;

    impl AccessControl for ArtZeroCollectionManager {}
    impl AccessControlEnumerable for ArtZeroCollectionManager {}
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

        fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
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
            // Check if caller sends correct adding fee to the contract and royalty fee is less than maximal value
            if self.manager.simple_mode_adding_fee != self.env().transferred_value() || royalty_fee > self.manager.max_royalty_fee_rate {
                return Err(Error::InvalidFee)
            }

            // Create PSP34 Standard Contract using cross call to psp34_standard contract
            let contract = Psp34NftRef::new(collection_owner, nft_name, nft_symbol)
                .endowment(0)
                .code_hash(self.manager.standard_nft_hash)
                .salt_bytes(self.manager.collection_count.to_le_bytes())
                .instantiate();
            let contract_account: AccountId = contract.to_account_id();
            
            if let Some(collection_count) = self.manager.collection_count.checked_add(1) {
                self.manager.collection_count = collection_count;
            } else {
                return Err(Error::CheckedOperations)
            }
            
            if let Some(active_collection_count) = self.manager.active_collection_count.checked_add(1) {
                self.manager.active_collection_count = active_collection_count;
            } else {
                return Err(Error::CheckedOperations)
            }

            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &contract_account);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            let mut collection_id = 0;
            if let Some(mut collections) = collections_by_owner {
                collections.push(contract_account);
                collection_id = (collections.len() as u32) - 1;
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            } else {
                let collections = vec![contract_account];
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                collection_id,
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
            Self::emit_event(self.env(), Event::AddNewCollectionEvent(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(contract_account),
                contract_type: CollectionType::Psp34Auto,
                is_active: false,
                show_on_chain_metadata: true,
            }));
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
            
            if let Some(collection_count) = self.manager.collection_count.checked_add(1) {
                self.manager.collection_count = collection_count;
            } else {
                return Err(Error::CheckedOperations)
            }
            // Add collection contract to collections_by_owner for Front-end use purpose
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &nft_contract_address);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            let mut collection_id = 0;
            if let Some(mut collections) = collections_by_owner {
                collections.push(nft_contract_address);
                collection_id = (collections.len() as u32) - 1;
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            } else {
                let collections = vec![nft_contract_address];
                self.manager.collections_by_owner.insert(&collection_owner, &collections);
            }
            // Add collection information to collections mapping
            let new_collection = Collection {
                collection_owner,
                collection_id,
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
            Self::emit_event(self.env(), Event::AddNewCollectionEvent(AddNewCollectionEvent {
                collection_owner: Some(collection_owner),
                nft_contract_address: Some(nft_contract_address),
                contract_type: CollectionType::Psp34Manual,
                is_active: false,
                show_on_chain_metadata: false,
            }));
            Ok(())
        }
    }
}
