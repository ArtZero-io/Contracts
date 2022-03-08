#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::Vendor_Management::{
    Vendor
};

#[ink::contract]
mod Vendor_Management {

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
    pub struct Vendor {
        userName: String,
        description: String,
        title: String,
        profileImage: String,
        twitter: String,
        facebook: String,
        telegram: String,
        instagram: String,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct VendorManagement {
        vendors: Mapping<AccountId, Vendor>
    }

    impl VendorManagement {
        #[ink(constructor)]
        pub fn new() -> Self {
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
            })
        }

        /// Add new vendor
        #[ink(message)]
        pub fn add_new_vendor(
            &mut self,
            _userName: String,
            _description: String,
            _title: String,
            _profileImage: String,
            _twitter: String,
            _facebook: String,
            _telegram: String,
            _instagram: String
        ) -> Result<()> {
            let caller = self.env().caller();
            
            if self.vendors.get(&caller).is_some() {
                return Err(Error::AddressAlreadyExists)
            }

            let new_vendor = Vendor {
                userName: _userName,
                description: _description,
                title: _title,
                profileImage: _profileImage,
                twitter: _twitter,
                facebook: _facebook,
                telegram: _telegram,
                instagram: _instagram
            };
            
            self.vendors.insert(&caller, &new_vendor);
            Ok(())
        }
    
        /// Update username
        #[ink(message)]
        pub fn update_username(
            &mut self,
            _userName: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.userName = _userName;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update description
        #[ink(message)]
        pub fn update_description(
            &mut self,
            _description: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.description = _description;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update title
        #[ink(message)]
        pub fn update_title(
            &mut self,
            _title: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.title = _title;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update profileImage
        #[ink(message)]
        pub fn update_profileImage(
            &mut self,
            _profileImage: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.profileImage = _profileImage;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update twitter
        #[ink(message)]
        pub fn update_twitter(
            &mut self,
            _twitter: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.twitter = _twitter;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update facebook
        #[ink(message)]
        pub fn update_facebook(
            &mut self,
            _facebook: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.facebook = _facebook;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update telegram
        #[ink(message)]
        pub fn update_telegram(
            &mut self,
            _telegram: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.telegram = _telegram;
            self.vendors.insert(&caller, &vendor);
        }

        /// Update instagram
        #[ink(message)]
        pub fn update_instagram(
            &mut self,
            _instagram: String
        ) {
            let caller = self.env().caller();
            let mut vendor = self.vendors.get(&caller).unwrap();
            vendor.instagram = _instagram;
            self.vendors.insert(&caller, &vendor);
        }

        /// Get vendor by address
        #[ink(message)]
        pub fn get_vendor_by_address(&self, address: AccountId) -> Vendor {
            return self.vendors
                .get(&address)
                .unwrap()
        }
    }
}
