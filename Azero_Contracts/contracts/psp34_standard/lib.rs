#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[openbrush::contract]
pub mod psp34_nft {
    // use artzero_project::impls::psp34_standard::psp34_standard::Internal;
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
            metadata::*,
            burnable::*
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
    impl PSP34Burnable for Psp34Nft {}
    impl Psp34Traits for Psp34Nft {}
    // impl artzero_project::impls::psp34_standard::Internal for Psp34Nft {}

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance._init_with_owner(contract_owner);
            })
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.manager.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_ok());
            Ok(())
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), PSP34Error> {
            let caller = self.env().caller();
            self.manager.last_token_id += 1;
            self._mint_to(caller, Id::U64(self.manager.last_token_id))?;
            self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata);
            Ok(())
        }
    }
}
