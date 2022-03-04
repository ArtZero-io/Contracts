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
        },
        Lazy,
    };
    use scale::Output;
    use ink_prelude::string::String;
    use ink_storage::lazy::Mapping;

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
        type: bool,
        isRoyalFee: bool,
        isActive: bool
    }

    #[ink(storage)]
    pub struct CollectionManagement {
        numberCollection: u64,
        collections: Mapping<AccountId, Collection>
    }

    impl CollectionManagement {

        #[ink(event)]
        pub struct AddNewCollectionEvent {
            contractAddress: AccountId,
            numberCollection: u64,
            value: Collection
        }

        /// Constructor
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            self.numberCollection = 0;
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
            })
        }

        /// Add new collection
        #[ink(message)]
        pub fn addNewCollection -> Self(
            &mut self,
            _name: String,
            _description: String,
            _image: String,
            _type: bool,
            _contractAddress: AccountId,
            _isRoyalFee: bool,
            _isActive: bool
        ) {
            if self.collections.get(&_contractAddress).is_some() {
                return Err(Error::AddressAlreadyExists)
            }

            let newCollection = Collection {
                name: _name,
                description: _description,
                image: _image,
                type: _type,
                isRoyalFee: _isRoyalFee,
                isActive: _isActive
            };

            self.collections.insert(&_contractAddress, &newCollection);
            self.numberCollection++;
            self.env().emit_event(AddNewCollectionEvent {
                contractAddress: _contractAddress,
                numberCollection: self.numberCollection,
                value: newCollection
            });            
            Ok(())
        }

        /// Update description
        #[ink(message)]

        /// Update description
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

        /// Update description
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

        /// Update description
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

        /// Update description
        #[ink(message)]
        pub fn update_type(
            &mut self,
            _contractAddress: AccountId,
            _type: bool
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.type = _type;
            self.collections.insert(&_contractAddress, &collection);
        }

        /// Update description
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
        
        /// Update description
        #[ink(message)]
        pub fn update_isActive(
            &mut self,
            _contractAddress: AccountId,
            _isActive: bool
        ) {
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.isActive = _isActive;
            self.collections.insert(&_contractAddress, &collection);
        }
        
    }

}
