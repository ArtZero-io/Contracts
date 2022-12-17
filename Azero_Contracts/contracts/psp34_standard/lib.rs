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
    use artzero_project::traits::psp34::*;
    use artzero_project::impls::psp34::*;

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
        manager: psp34::data::Manager,
    }

    impl Ownable for Psp34Nft {}
    impl PSP34 for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance._init_with_owner(contract_owner);
                instance.manager.last_token_id = 0;
                instance.manager.attribute_count = 0;
                instance.manager.locked_token_count = 0;
            })
        }
    }
}