#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::implementation(PSP34, PSP34Burnable, Ownable, PSP34Metadata)]
#[openbrush::contract]
pub mod psp34_nft {
    use ink::{
        codegen::{Env, EmitEvent},
        reflect::ContractEventBase
    };
    use ink::prelude::{
        string::{
            String,
        }
    };
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::{
            extensions::{
                enumerable::*,
                metadata::*,
                burnable::*,
            },
            Internal,
        },
        
        traits::{
            Storage,
            DefaultEnv
        },
        modifiers,
    };
    use standard_contracts::{
        traits::{
            psp34_standard::*,
            admin::*,
            error::Error,
        }
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        manager: standard_contracts::impls::psp34_standard::data::Manager,
    }

    impl Psp34Traits for Psp34Nft {}

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(contract_owner);
            instance._set_attribute(Id::U8(0), String::from("name"), name);
            instance._set_attribute(Id::U8(0), String::from("symbol"), symbol);
            instance
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                self.manager.last_token_id = last_token_id;
                if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                    return Err(Error::Custom(String::from("Cannot mint")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                self.manager.last_token_id = last_token_id;
                if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                    return Err(Error::Custom(String::from("Cannot mint")));
                }
                if self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata).is_err(){
                    return Err(Error::Custom(String::from("Cannot set attributes")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }
    }
}
