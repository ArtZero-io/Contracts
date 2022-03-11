#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_profile_manager {
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::traits::SpreadAllocate;
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String)
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroProfileManager{
        #[OwnableStorageField]
        ownable: OwnableData,
        attributes: Mapping<Vec<u8>, Vec<u8>>
    }

    impl Ownable for ArtZeroProfileManager {}

    impl ArtZeroProfileManager {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_profile_attribute(&mut self, attribute: String, value: String) -> Result<(),Error> {
            self._set_attribute(attribute.into_bytes(), value.into_bytes());
            Ok(())
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_multiple_attributes(&mut self, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error> {
            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            let length = attributes.len();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = values[i].clone();
                self._set_attribute(attribute.into_bytes(), value.into_bytes());
            }

            Ok(())
        }

        #[ink(message)]
        pub fn get_attribute(&self, key: Vec<u8>) -> Option<Vec<u8>> {
            self.attributes.get(&key)
        }

        fn _set_attribute(&mut self, key: Vec<u8>, value: Vec<u8>) {
            self.attributes.insert(&key, &value);
        }
    }
}
