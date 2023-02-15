#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod psp34_nft {
    use ink::prelude::{
        string::{
            String,
        },
        vec::Vec,
    };
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::extensions::{
            enumerable::*,
            metadata::*,
            burnable::*,
        },
        traits::{
            Storage,
            DefaultEnv
        },
        modifiers,
    };
    use artzero_project::{
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
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        manager: artzero_project::impls::psp34_standard::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
    }

    impl Ownable for Psp34Nft {}
    impl PSP34 for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}
    impl Psp34Traits for Psp34Nft {}
    impl AdminTrait for Psp34Nft {}

    impl PSP34Burnable for Psp34Nft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();
            let token_owner = self.owner_of(id.clone()).unwrap();
            if token_owner != account {
                return Err(PSP34Error::Custom(String::from("not token owner").into_bytes()))
            }

            let allowance = self.allowance(account,caller,Some(id.clone()));

            if caller == account || allowance {
                self._burn_from(account, id)
            } else{
                Err(PSP34Error::Custom(String::from("caller is not token owner or approved").into_bytes()))
            }
        }
    }

    impl Psp34Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(contract_owner);
            instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
            instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
            instance
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.manager.last_token_id = self.manager.last_token_id.checked_add(1).unwrap();
            if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                return Err(Error::Custom(String::from("Cannot mint")))
            }
            Ok(())
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), Error> {
            let caller = self.env().caller();
            self.manager.last_token_id = self.manager.last_token_id.checked_add(1).unwrap();
            if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                return Err(Error::Custom(String::from("Cannot mint")))
            }
            if self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata).is_err(){
                return Err(Error::Custom(String::from("Cannot set attributes")))
            }
            Ok(())
        }
    }
}
