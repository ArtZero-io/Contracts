#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_collection_manager {
    use ink_lang::ToAccountId;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
        Mapping,
    };
    use openbrush::{
        contracts::ownable::*,
        modifiers,
        traits::Storage,
    };
    use psp34_nft::psp34_nft::Psp34NftRef;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        CollectionOwnerAndAdmin,
        OnlyOwner,
        OnlyAdmin,
        InvalidCaller,
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[derive(
        Clone, Debug, Ord, PartialOrd, Eq, PartialEq, PackedLayout, SpreadLayout, scale::Encode, scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Collection {
        collection_owner: AccountId,     // to receive Royal Fee - OnlyAdmin can update
        nft_contract_address: AccountId, // OnlyAdmin can update
        contract_type: u8,               // 1 - PSP34 ERC721 Manual; 2 - PSP34 ERC721 Auto
        is_collect_royal_fee: bool,      // OnlyAdmin can update
        royal_fee: u32,                  // 100 = 1% 10000 = 100% OnlyAdmin
        is_active: bool,                 // OnlyAdmin can update
        show_on_chain_metadata: bool,
    }

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(ArtZeroCollectionManager);

    #[derive(Default)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    struct Manager {
        standard_nft_hash: Hash,
        admin_address: AccountId,
        collection_count: u64,
        simple_mode_adding_fee: Balance,
        advance_mode_adding_fee: Balance,
        collections: Mapping<AccountId, Collection>, // save collection by contract address
        collections_by_id: Mapping<u64, AccountId>,  // save contract address by id
        collections_by_owner: Mapping<AccountId, Vec<AccountId>>, // save contract address by owner ID
        max_royal_fee_rate: u32,
        active_collection_count: u64,
        attributes: Mapping<(AccountId, Vec<u8>), Vec<u8>>,
        _reserved: Option<()>,
    }

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct ArtZeroCollectionManager {
        #[storage_field]
        ownable: ownable::Data,
        manager: Manager,
    }

    impl Ownable for ArtZeroCollectionManager {}

    #[openbrush::trait_definition]
    pub trait CrossArtZeroCollection {
        #[ink(message)]
        fn get_royal_fee(&self, nft_contract_address: AccountId) -> u32;
        #[ink(message)]
        fn is_active(&self, nft_contract_address: AccountId) -> bool;
        #[ink(message)]
        fn get_contract_type(&self, nft_contract_address: AccountId) -> u8;
        #[ink(message)]
        fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId>;
    }

    #[ink(event)]
    pub struct AddNewCollectionEvent {
        collection_owner: Option<AccountId>,
        nft_contract_address: Option<AccountId>,
        contract_type: u8,
        is_active: bool,
        show_on_chain_metadata: bool,
    }

    impl ArtZeroCollectionManager {
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
        ) -> Result<(), OwnableError> {
            self.manager.collection_count = 0;
            self.manager.active_collection_count = 0;
            self.manager.simple_mode_adding_fee = simple_mode_adding_fee;
            self.manager.advance_mode_adding_fee = advance_mode_adding_fee;
            self.manager.admin_address = admin_address;
            self.manager.standard_nft_hash = standard_nft_hash;
            self.manager.max_royal_fee_rate = max_royal_fee_rate;
            Ok(())
        }

        /// Simple New Collection Creation - Auto create NFT Contract - Collection_Owner is owner of NFT contract and receive royal fee
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
            assert!(
                self.manager.simple_mode_adding_fee == self.env().transferred_value(),
                "invalid fee"
            );
            assert!(royal_fee <= self.manager.max_royal_fee_rate, "invalid royal fee");

            let (hash, _) =
                ink_env::random::<ink_env::DefaultEnvironment>(nft_name.as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract = Psp34NftRef::new(collection_owner, nft_name, nft_symbol)
                .endowment(0)
                .code_hash(self.manager.standard_nft_hash)
                .salt_bytes(&hash[..4])
                .instantiate()
                .unwrap_or_else(|error| panic!("failed at instantiating the NFT contract: {:?}", error));
            let contract_account: AccountId = contract.to_account_id();

            self.manager.collection_count += 1;
            self.manager.active_collection_count += 1;
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
            assert!(
                self.manager.advance_mode_adding_fee == self.env().transferred_value(),
                "invalid fee"
            );
            assert!(
                self.manager.collections.get(&nft_contract_address).is_none(),
                "address exists"
            );
            assert!(royal_fee <= self.manager.max_royal_fee_rate, "invalid royal fee");
            self.manager.collection_count += 1;
            self.manager
                .collections_by_id
                .insert(&self.manager.collection_count, &nft_contract_address);
            let collections_by_owner = self.manager.collections_by_owner.get(&collection_owner);
            if collections_by_owner.is_none() {
                let mut collections = Vec::<AccountId>::new();
                collections.push(nft_contract_address);
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
        /// Update Owner of Collecion - who receive royal fee - Only Admin can change
        #[ink(message)]
        pub fn update_collection_owner(
            &mut self,
            contract_address: AccountId,
            new_owner: AccountId,
        ) -> Result<(), Error> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(self.env().caller() == self.manager.admin_address);
            collection.collection_owner = new_owner;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }
        /// Update nft_contract_address - Only Admin can change
        #[ink(message)]
        pub fn update_nft_contract_address(
            &mut self,
            contract_address: AccountId,
            nft_contract_address: AccountId,
        ) -> Result<(), Error> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(self.env().caller() == self.manager.admin_address);
            collection.nft_contract_address = nft_contract_address;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Set multiple profile attributes
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
            if collection.collection_owner == self.env().caller() || self.manager.admin_address == self.env().caller() {
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

        // Get multiple profile attributes
        #[ink(message)]
        pub fn get_attributes(&self, account: AccountId, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.manager.attributes.get(&(account, attribute.into_bytes()));
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }

        fn _set_attribute(&mut self, account: AccountId, key: Vec<u8>, value: Vec<u8>) {
            self.manager.attributes.insert(&(account, key), &value);
        }

        /// Update Type Collection - only Admin can change - 1: Manual 2: Auto
        #[ink(message)]
        pub fn update_contract_type(&mut self, contract_address: AccountId, contract_type: u8) -> Result<(), Error> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(self.env().caller() == self.manager.admin_address);
            collection.contract_type = contract_type;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update Is Royal Fee - Only Admin can change
        #[ink(message)]
        pub fn update_is_collect_royal_fee(
            &mut self,
            contract_address: AccountId,
            is_collect_royal_fee: bool,
        ) -> Result<(), Error> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(self.env().caller() == self.manager.admin_address);
            collection.is_collect_royal_fee = is_collect_royal_fee;
            self.manager.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /// Update royal_fee - Only Admin can change
        #[ink(message)]
        pub fn update_royal_fee(&mut self, contract_address: AccountId, new_fee: u32) -> Result<(), Error> {
            assert!(new_fee <= self.manager.max_royal_fee_rate, "invalid fee");
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            let mut collection = self.manager.collections.get(&contract_address).unwrap();
            assert!(self.env().caller() == self.manager.admin_address);
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
            if self.env().caller() == collection.collection_owner || self.env().caller() == self.manager.admin_address {
                collection.show_on_chain_metadata = show_on_chain_metadata;
                self.manager.collections.insert(&contract_address, &collection);
                Ok(())
            } else {
                return Err(Error::OnlyAdmin)
            }
        }

        /// Update Active Status When its active, collection will be shown on the UI and will be tradable - only Admin can change
        #[ink(message)]
        pub fn update_is_active(&mut self, contract_address: AccountId, is_active: bool) -> Result<(), Error> {
            assert!(
                self.manager.collections.get(&contract_address).is_some(),
                "collection not exist"
            );
            assert!(self.env().caller() == self.manager.admin_address, "only admin");
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

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self, value: Balance) -> Result<(), Error> {
            assert!(value <= self.env().balance(), "not enough balance");
            assert!(
                self.env().transfer(self.env().caller(), value).is_ok(),
                "error withdraw_fee"
            );
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

    impl CrossArtZeroCollection for ArtZeroCollectionManager {
        /// Get royal fee of the Collection
        #[ink(message)]
        fn get_royal_fee(&self, nft_contract_address: AccountId) -> u32 {
            if self.manager.collections.get(&nft_contract_address).is_none() {
                return 0
            }
            let collection = self.manager.collections.get(&nft_contract_address).unwrap();
            if !collection.is_collect_royal_fee || !collection.is_active {
                return 0
            } else {
                collection.royal_fee
            }
        }

        /// Check if the Collection is active not
        #[ink(message)]
        fn is_active(&self, nft_contract_address: AccountId) -> bool {
            if self.manager.collections.get(&nft_contract_address).is_none() {
                return false
            }
            let collection = self.manager.collections.get(&nft_contract_address).unwrap();
            return collection.is_active
        }

        /// Get NFT Contract Type 1 or 2 for PSP34
        #[ink(message)]
        fn get_contract_type(&self, nft_contract_address: AccountId) -> u8 {
            if self.manager.collections.get(&nft_contract_address).is_none() {
                return 0
            }
            let collection = self.manager.collections.get(&nft_contract_address).unwrap();
            return collection.contract_type
        }

        /// Get Collection Owner by Collection Address (NFT address)
        #[ink(message)]
        fn get_collection_owner(&self, nft_contract_address: AccountId) -> Option<AccountId> {
            if self.manager.collections.get(&nft_contract_address).is_none() {
                return None
            }
            let collection = self.manager.collections.get(&nft_contract_address).unwrap();
            Some(collection.collection_owner)
        }
    }
}
