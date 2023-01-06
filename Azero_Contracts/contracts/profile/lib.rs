#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod profile_manager {
    use ink_prelude::{
        string::{
            String,
        },
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate,
    };
    use openbrush::{
        contracts::ownable::*,
        traits::{
            Storage
        },
        storage::{
            Mapping
        },
    };
    use artzero_project::{
        traits::{
            admin::*,
            error::Error,
        }
    };

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

    #[derive(Default, Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Manager {
        attributes: Mapping<(AccountId, Vec<u8>), Vec<u8>>
    }

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct ProfileManager {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        manager: Manager,
    }

    impl Ownable for ProfileManager {}
    impl AdminTrait for ProfileManager {}

    impl ProfileManager {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
            })
        }

        /// Set multiple profile attributes
        #[ink(message)]
        pub fn set_multiple_attributes(&mut self, attributes: Vec<String>, values: Vec<String>) -> Result<(), Error> {
            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")))
            }
            let length = attributes.len();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = values[i].clone();
                self._set_attribute(self.env().caller(), attribute.into_bytes(), value.into_bytes());
            }
            Ok(())
        }

        // Get multiple profile attributes
        #[ink(message)]
        pub fn get_attributes(&self, account: AccountId, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.manager.attributes.get(&(account, attribute.into_bytes()));
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                } else {
                    ret.push(String::from(""));
                }
            }
            ret
        }

        fn _set_attribute(&mut self, account: AccountId, key: Vec<u8>, value: Vec<u8>) {
            self.manager.attributes.insert(&(account, key), &value);
        }
    }
}
