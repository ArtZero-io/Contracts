#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, Ownable, PSP34Metadata)]
#[openbrush::contract]
pub mod psp34_nft {
    use standard_contracts::traits::psp34_standard::*;
    use openbrush::{
        modifiers,
        storage::Mapping,
        traits::{
            Storage,
            String,
        },
    };

    use openbrush::{
        contracts::psp34::{
            extensions::{
                enumerable::*,
                metadata::*,
                burnable::*,
            },
            Internal,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Psp34Nft {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data
    }

    impl Psp34Traits for Psp34Nft {
        
    }

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            ownable::Internal::_init_with_owner(&mut instance, contract_owner);
            metadata::Internal::_set_attribute(
                &mut instance,
                Id::U8(0),
                String::from("name"),
                name
            );
            metadata::Internal::_set_attribute(
                &mut instance,
                Id::U8(0),
                String::from("symbol"),
                symbol
            );
            instance
        }
    }
}