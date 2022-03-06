#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::Collection_Management::{
    Collection
};

#[ink::contract]
mod Collection_Management {

    use ink_env::call::{
        build_call,
        utils::ReturnType,
        ExecutionInput,
    };
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use scale::Output;
    use ink_prelude::string::String;
    use ink_storage::Mapping;
    
    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the address already exists upon registration.
        AddressAlreadyExists
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        Default,
        PackedLayout,
        SpreadLayout,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Collection {
        name: String,
        description: String,
        image: String,
        typeCollection: u8,
        isRoyalFee: bool,
        isActive: bool
    }

    #[ink(event)]
    pub struct AddNewCollectionEvent {
        contractAddress: AccountId,
        numberCollection: u64,
        value: Collection
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct CollectionManagement {
        admin_address: AccountId,
        numberCollection: u64,
        collections: Mapping<AccountId, Collection>
    }

    
    impl CollectionManagement {

        /// Constructor
        #[ink(constructor)]
        pub fn new(_default_admin_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                contract.numberCollection = 0;
                contract.admin_address = _default_admin_address;
            })
        }

        /// Add new collection
        #[ink(message)]
        pub fn addNewCollection(
            &mut self,
            _name: String,
            _description: String,
            _image: String,
            _typeCollection: u8,
            _contractAddress: AccountId,
            _isRoyalFee: bool,
            _isActive: bool
        ) -> Result<()> {
            if self.collections.get(&_contractAddress).is_some() {
                return Err(Error::AddressAlreadyExists)
            }

            let newCollection = Collection {
                name: _name,
                description: _description,
                image: _image,
                typeCollection: _typeCollection,
                isRoyalFee: _isRoyalFee,
                isActive: _isActive
            };

            self.collections.insert(&_contractAddress, &newCollection);
            self.numberCollection = self.numberCollection + 1;
            self.env().emit_event(AddNewCollectionEvent {
                contractAddress: _contractAddress,
                numberCollection: self.numberCollection,
                value: newCollection
            });            
            Ok(())
        }

        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(
            &mut self
        ) -> AccountId {
            return self.admin_address;
        }

        /// Update Admin Address
        #[ink(message)]
        pub fn update_admin_address(
            &mut self,
            _admin_address: AccountId
        ) {
            self.ensure_from_wallet();
            self.admin_address = _admin_address;
            
        }

        /// Update Name
        #[ink(message)]
        pub fn update_name(
            &mut self,
            _contractAddress: AccountId,
            _name: String
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.name = _name;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Update Description
        #[ink(message)]
        pub fn update_description(
            &mut self,
            _contractAddress: AccountId,
            _description: String
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.description = _description;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Update Image
        #[ink(message)]
        pub fn update_image(
            &mut self,
            _contractAddress: AccountId,
            _image: String
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.image = _image;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Update Type Collection
        #[ink(message)]
        pub fn update_type_collection(
            &mut self,
            _contractAddress: AccountId,
            _typeCollection: u8
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.typeCollection = _typeCollection;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Update Is Royal Fee
        #[ink(message)]
        pub fn update_isRoyalFee(
            &mut self,
            _contractAddress: AccountId,
            _isRoyalFee: bool
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.isRoyalFee = _isRoyalFee;
            self.collections.insert(&_contractAddress, &collection);
        }
        
        /// Update Active Status
        #[ink(message)]
        pub fn update_isActive(
            &mut self,
            _contractAddress: AccountId,
            _isActive: bool
        ) {
            self.ensure_admin();
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.isActive = _isActive;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Get Collection by Address
        #[ink(message)]
        pub fn get_colletion_by_address(
            &mut self,
            _contractAddress: AccountId
        ) -> Collection {
            return self.collections.get(&_contractAddress).unwrap();
        }

        /// Check address is owner
        #[ink(message)]
        pub fn ensure_from_wallet(&self) {
            assert_eq!(self.env().caller(), self.env().account_id());
        }

        fn ensure_admin(&self) {
            assert_eq!(self.env().caller(), self.admin_address);
        }
        
    }

}
