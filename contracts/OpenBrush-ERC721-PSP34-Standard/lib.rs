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
    use brush::contracts::psp34::extensions::enumerable::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            SpreadAllocate
        },
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_prelude::string::ToString;

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage, PSP34EnumerableStorage)]
    #[ink(storage)]
    pub struct Psp34Nft{
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,
        #[PSP34EnumerableStorageField]
        enumdata: PSP34EnumerableData,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64
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
    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    impl PSP34 for Psp34Nft {}
    impl PSP34Burnable for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Internal for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}

    #[brush::trait_definition]
    pub trait Psp34Traits {
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
        #[ink(message)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error>;
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
        #[ink(message)]
        fn get_attribute_count(&self) -> u32;
        #[ink(message)]
        fn get_attribute_name(&self, index:u32) -> String;
        #[ink(message)]
        fn token_uri(&self,token_id: u64) -> String;
    }

    impl Psp34Nft {

        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance._init_with_owner(contract_owner);
            })
        }

        ///Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            Ok(())
        }

        ///Only Owner can mint new token and add attributes for it
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, attributes: Vec<String>, values: Vec<String>) -> Result<(), Error> {
            let caller = self.env().caller();
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            if self.set_multiple_attributes(Id::U64(self.last_token_id), attributes, values).is_err() {
                panic!(
                    "error set_multiple_attributes"
                )
            };
            Ok(())
        }

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        fn add_attribute_name(&mut self, attribute_input:Vec<u8>){
            let mut exist:bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&(index+1));
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

        /// Lock nft - Only owner token
        #[ink(message)]
        pub fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(token_id.clone()).unwrap();
            assert!(caller == token_owner);
            self.locked_token_count += 1;
            self.locked_tokens.insert(&token_id, &1);
            Ok(())
        }

        /// Check token is locked or not
        #[ink(message)]
        pub fn is_locked_nft(&self, token_id: Id) -> bool {
            if self.locked_tokens.get(&token_id).is_some() {
                return true;
            }
            return false;
        }

        ///Get Locked Token Count
        #[ink(message)]
        pub fn get_locked_token_count(&self) -> u64 {
            return self.locked_token_count;
        }

    }

    impl Psp34Traits for Psp34Nft{

        /// Change baseURI
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
            Ok(())
        }

        ///Only Owner can set multiple attributes to a token
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error> {
            assert!(token_id != Id::U64(0));

            if self.is_locked_nft(token_id.clone()) {
                return Err(Error::Custom(String::from("Token is locked")));
            }

            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            //Check Duplication
            let mut sorted_attributes = attributes.clone();
            sorted_attributes.sort();
            let length = sorted_attributes.len();

            for i in 0..length {
                let attribute = sorted_attributes[i].clone();
                let byte_attribute = attribute.into_bytes();

                if i + 1 < length {
                    let next_attribute = sorted_attributes[i + 1].clone();
                    let byte_next_attribute = next_attribute.into_bytes();
                    if byte_attribute == byte_next_attribute{
                        return Err(Error::Custom(String::from("Duplicated Attributes")));
                    }
                }

                let unsorted_attribute = attributes[i].clone();
                let byte_unsorted_attribute = unsorted_attribute.into_bytes();
                let value = values[i].clone();

                self.add_attribute_name(byte_unsorted_attribute.clone());
                self._set_attribute(token_id.clone(),byte_unsorted_attribute.clone(), value.into_bytes());
            }

            Ok(())
        }

        /// Get multiple  attributes
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
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
        fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }
        ///Get Attribute Name
        #[ink(message)]
        fn get_attribute_name(&self, index:u32) -> String {
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
        fn token_uri(
            &self,
            token_id: u64
        ) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            return token_uri;
        }

    }
}
