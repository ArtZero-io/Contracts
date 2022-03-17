#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[brush::contract]
pub mod psp34_nft {
    use ink_prelude::string::String;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            SpreadAllocate
        },
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct Psp34Nft{
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        token_count: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        NotOwner
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    impl Ownable for Psp34Nft {}
    impl PSP34 for Psp34Nft {}
    impl PSP34Burnable for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Internal for Psp34Nft {}

    impl Psp34Nft {

        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance._init_with_owner(contract_owner);
            })
        }

        /// Change baseURI
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
            Ok(())
        }

        ///Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.token_count += 1;
            assert!(self._mint_to(caller, Id::U64(self.token_count)).is_ok());
            Ok(())
        }

        ///Only Owner can set attribute to a token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_attribute(&mut self, token_id:Id, attribute: String, value: String) -> Result<(),Error> {
            assert!(token_id != Id::U64(0));
            let byte_attribute = attribute.into_bytes();
            self.add_attribute_name(byte_attribute.clone());
            self._set_attribute(token_id.clone(),byte_attribute.clone(), value.into_bytes());
            Ok(())
        }
        ///Only Owner can set multiple attributes to a token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error> {
            assert!(token_id != Id::U64(0));
            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            let length = attributes.len();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = values[i].clone();
                let byte_attribute = attribute.into_bytes();
                self.add_attribute_name(byte_attribute.clone());
                self._set_attribute(token_id.clone(),byte_attribute.clone(), value.into_bytes());

            }

            Ok(())
        }

        /// Get multiple  attributes
        #[ink(message)]
        pub fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.get_attribute(token_id.clone(),attribute.into_bytes());
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                }
                else{
                    ret.push(String::from(""));
                }
            }
            ret
        }

        ///Get Attribute Count
        #[ink(message)]
        pub fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }
        ///Get Attribute Count
        #[ink(message)]
        pub fn get_attribute_name(&self, index:u32) -> String {
            let attribute = self.attribute_names.get(&index);
            if attribute.is_some() {
                String::from_utf8(attribute.unwrap()).unwrap()
            }
            else{
                String::from("")
            }
        }
        /// Get URI from token ID
        #[ink(message)]
        pub fn token_uri(
            &self,
            token_id: u64
        ) -> Vec<u8> {
            let mut token_uri = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes()).unwrap();
            token_uri.extend(&token_id.to_be_bytes());
            token_uri.extend(String::from(".json").into_bytes());

            return token_uri;
        }

        fn add_attribute_name(&mut self, attribute_input:Vec<u8>){
            let mut exist:bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&index);
                if attribute_name.is_some(){
                    if attribute_name.unwrap() == attribute_input{
                        exist = true;
                        break;
                    }
                }
            }
            if !exist {
                self.attribute_count += 1;
                self.attribute_names.insert(&self.attribute_count, &attribute_input);
            }
        }

    }
}
