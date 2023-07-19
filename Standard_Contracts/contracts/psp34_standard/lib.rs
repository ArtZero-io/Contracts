#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

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
    use ink::codegen::Env;
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
    use standard_contracts::traits::error::Error;
    
    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        manager: standard_contracts::impls::psp34_standard::data::Manager,
    }

    impl PSP34Burnable for Psp34Nft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();

            if let Some(token_owner) = psp34::PSP34Impl::owner_of(self, id.clone()) {
                if token_owner != account {
                    return Err(PSP34Error::Custom(String::from("not token owner")));
                }

                let allowance = psp34::PSP34Impl::allowance(self, account, caller, Some(id.clone()));

                if caller == account || allowance {
                    self.manager.locked_tokens.remove(&id);
                    if let Some(locked_token_count) = self.manager.locked_token_count.checked_sub(1) {
                        self.manager.locked_token_count = locked_token_count;
                        psp34::Internal::_burn_from(self, account, id)
                    } else {
                        return Err(PSP34Error::Custom(String::from("Locked token count error")));
                    }
                } else {
                    return Err(PSP34Error::Custom(String::from("caller is not token owner or approved")));
                }
            } else {
                return Err(PSP34Error::Custom(String::from("No token owner found")));
            }
        }
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

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                self.manager.last_token_id = last_token_id;
                if psp34::Internal::_mint_to(self, caller, Id::U64(self.manager.last_token_id)).is_err(){
                    return Err(Error::Custom(String::from("Cannot mint")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }

        // /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        // #[ink(message)]
        // #[modifiers(only_owner)]
        // pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), Error> {
        //     let caller = self.env().caller();
        //     if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
        //         self.manager.last_token_id = last_token_id;
        //         if psp34::Internal::_mint_to(self, caller, Id::U64(self.manager.last_token_id)).is_err(){
        //             return Err(Error::Custom(String::from("Cannot mint")));
        //         }
        //         if self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata).is_err(){
        //             return Err(Error::Custom(String::from("Cannot set attributes")));
        //         }
        //         return Ok(());
        //     } else {
        //         return Err(Error::Custom(String::from("Cannot increase last token id")));
        //     }
        // }
    }
}