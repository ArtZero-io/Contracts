#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp1155_nft::{
    Psp1155Nft,
    Psp1155NftRef,
};

#[brush::contract]
pub mod psp1155_nft {
    use brush::contracts::psp1155::*;
    use brush::contracts::psp1155::extensions::metadata::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;

    use ink_prelude::{
        string::String,
        vec,
    };
    use ink_storage::{
        traits::{
            SpreadAllocate
        },
    };

    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Default, SpreadAllocate, PSP1155Storage, PSP1155MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct Psp1155Nft {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        token_count: u64,
        attributes: Mapping<(Id, Vec<u8>), Vec<u8>>,
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
    
    impl Ownable for Psp1155Nft {}
    impl PSP1155 for Psp1155Nft {}
    impl PSP1155Metadata for Psp1155Nft {}
    

    #[brush::trait_definition]
    pub trait Psp1155Traits {
        #[ink(message)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error>;
    }

    impl Psp1155Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, uri: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.uri = uri;
                instance._init_with_owner(contract_owner);
            })
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn _set_attribute(&mut self, id: Id, key: Vec<u8>, value: Vec<u8>) -> Result<(), Error> {
            self.attributes.insert((&id, &key), &value);
            Ok(())
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
    }

    impl Psp1155Traits for Psp1155Nft {
        
        ///Only Owner can set multiple attributes to a token
        #[modifiers(only_owner)]
        #[ink(message)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error> {
            
            //TODO: PSP1155 using enum for ID
            // assert!(token_id != Id::U64(0));

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
    }
}
