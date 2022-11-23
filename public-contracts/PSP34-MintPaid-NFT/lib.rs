#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[openbrush::contract]
pub mod psp34_nft {
    use ink_prelude::{
        string::{
            String,
            ToString,
        },
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::extensions::{
            enumerable::*,
            burnable::*,
            metadata::*
        },
        traits::Storage,
        modifiers,
    };

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32, Vec<u8>>,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        total_supply: u64,      
        mint_fee: Balance
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
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

    impl Ownable for Psp34Nft {}

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    impl PSP34 for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}
    impl PSP34Burnable for Psp34Nft {}

    #[openbrush::trait_definition]
    pub trait Psp34Traits {
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
        #[ink(message)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>
        ) -> Result<(), Error>;
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
        #[ink(message)]
        fn get_attribute_count(&self) -> u32;
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String;
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String;
        #[ink(message)]
        fn get_owner(&self) -> AccountId ;
    }

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(
            contract_owner: AccountId, 
            name: String, 
            symbol: String, 
            total_supply: u64,      
            mint_fee: Balance
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.initialize(name, symbol, total_supply, mint_fee).ok().unwrap();
            })
        }

        #[ink(message)]     
        #[modifiers(only_owner)]        
        pub fn initialize(      
            &mut self,      
            name: String,       
            symbol: String,     
            total_supply: u64,      
            mint_fee: Balance,      
        ) -> Result<(), OwnableError> {     
            self._set_attribute(        
                Id::U8(0),      
                String::from("name").into_bytes(),      
                name.into_bytes(),      
            );      
            self._set_attribute(        
                Id::U8(0),      
                String::from("symbol").into_bytes(),        
                symbol.into_bytes(),        
            );      
            self.mint_fee = mint_fee;       
            self.total_supply = total_supply;       
            Ok(())      
        }

        ///Everyone can Mint with Fee as defined in mint_fee        
        #[ink(message)]     
        #[ink(payable)]     
        pub fn public_mint(&mut self) -> Result<(), Error> {        
            assert!(self.mint_fee == self.env().transferred_value());
            assert!(self.last_token_id < self.total_supply); 
            let caller = self.env().caller();       
            self.last_token_id += 1;        
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());        
            Ok(())      
        }

        /// Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            assert!(self.last_token_id < self.total_supply);
            let caller = self.env().caller();
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            Ok(())
        }

        /// Only Owner can mint new token and add attributes for it
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), PSP34Error> {
            assert!(self.last_token_id < self.total_supply);
            let caller = self.env().caller();
            self.last_token_id += 1;
            self._mint_to(caller, Id::U64(self.last_token_id))?;
            self.set_multiple_attributes(Id::U64(self.last_token_id), metadata);
            Ok(())
        }

        fn add_attribute_name(&mut self, attribute_input: Vec<u8>) {
            let mut exist: bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&(index + 1));
                if attribute_name.is_some() {
                    if attribute_name.unwrap() == attribute_input {
                        exist = true;
                        break
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

        /// Change mint fee - Only Owner        
        #[ink(message)]     
        #[modifiers(only_owner)]        
        pub fn set_mint_fee(&mut self, mint_fee: Balance) -> Result<(), Error> {        
            self.mint_fee = mint_fee;       
            Ok(())      
        }

        /// Check token is locked or not
        #[ink(message)]
        pub fn is_locked_nft(&self, token_id: Id) -> bool {
            if self.locked_tokens.get(&token_id).is_some() {
                return true
            }
            return false
        }

        /// Get Locked Token Count
        #[ink(message)]
        pub fn get_locked_token_count(&self) -> u64 {
            return self.locked_token_count
        }

        /// Burn NFT
        #[ink(message)]
        pub fn burn(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(id.clone()).unwrap();
            assert!(self.locked_tokens.get(&id).is_none());
            assert!(caller == token_owner);
            self._burn_from(caller, id)
        }

         /// Get total supply   
        #[ink(message)]     
        pub fn get_total_supply(&self) -> u64 {     
            return self.total_supply;       
        }   
        /// Get Mint Fee        
        #[ink(message)]     
        pub fn get_mint_fee(&self) -> Balance {     
            return self.mint_fee;       
        }

        /// Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id
        }
    }

    impl Psp34Traits for Psp34Nft {
        /// Change baseURI
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
            Ok(())
        }

        /// Only Owner can set multiple attributes to a token
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>
        ) -> Result<(), Error> {
            assert!(token_id != Id::U64(0));
            if self.is_locked_nft(token_id.clone()) {
                return Err(Error::Custom(String::from("Token is locked")))
            }
            for (attribute, value) in &metadata {
                self.add_attribute_name(attribute.clone().into_bytes());
                self._set_attribute(token_id.clone(), attribute.clone().into_bytes(), value.clone().into_bytes());
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
                let value = self.get_attribute(token_id.clone(), attribute.into_bytes());
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }

        /// Get Attribute Count
        #[ink(message)]
        fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }
        /// Get Attribute Name
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String {
            let attribute = self.attribute_names.get(&index);
            if attribute.is_some() {
                String::from_utf8(attribute.unwrap()).unwrap()
            } else {
                String::from("")
            }
        }

        /// Get URI from token ID
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            return token_uri
        }

        /// Get owner address
        #[ink(message)]
        fn get_owner(&self) -> AccountId {
            return self.owner()
        }

    }
}