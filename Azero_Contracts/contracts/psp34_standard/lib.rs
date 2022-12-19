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
            metadata::*
        },
        traits::Storage,
        modifiers,
    };
    use artzero_project::traits::psp34_standard::*;
    use artzero_project::impls::psp34_standard::*;

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        manager: artzero_project::impls::psp34_standard::data::Manager,
    }

    impl Ownable for Psp34Nft {}
    impl PSP34 for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}
    impl Psp34Traits for Psp34Nft {}
    
    impl artzero_project::impls::psp34_standard::Internal for Psp34Nft {
        /// Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            Ok(())
        }

        /// Only Owner can mint new token and add attributes for it
        #[ink(message)]
        #[modifiers(only_owner)]
        fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            self.last_token_id += 1;
            self._mint_to(caller, Id::U64(self.last_token_id))?;
            self.set_multiple_attributes(Id::U64(self.last_token_id), metadata);
            Ok(())
        }

        /// Get Token Count
        #[ink(message)]
        fn get_last_token_id(&self) -> u64 {
            return self.last_token_id
        }

        /// Lock nft - Only owner token
        #[ink(message)]
        fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(token_id.clone()).unwrap();
            assert!(caller == token_owner);
            self.locked_token_count += 1;
            self.locked_tokens.insert(&token_id, &1);
            Ok(())
        }

        /// Check token is locked or not
        #[ink(message)]
        fn is_locked_nft(&self, token_id: Id) -> bool {
            if self.locked_tokens.get(&token_id).is_some() {
                return true
            }
            return false
        }

        /// Get Locked Token Count
        #[ink(message)]
        fn get_locked_token_count(&self) -> u64 {
            return self.locked_token_count
        }

        #[ink(message)]
        fn burn(&mut self, id: Id) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(id.clone()).unwrap();
            assert!(caller == token_owner);
            self._burn_from(caller, id)
        }
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
    }
}