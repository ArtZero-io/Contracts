#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_launchpad_psp34 {
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use psp34_nft::psp34_nft::Psp34NftRef;
    use ink_lang::ToAccountId;

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, OwnableStorage)]
    pub struct ArtZeroLaunchPadPSP34 {
        #[OwnableStorageField]
        ownable: OwnableData,
        admin_address: AccountId,
        project_count: u64,
        attributes: Mapping<(AccountId,Vec<u8>), Vec<u8>>,
    }

    impl Ownable for ArtZeroLaunchPadPSP34 {}

    impl ArtZeroLaunchPadPSP34 {
        #[ink(constructor)]
        pub fn new(
            admin_address: AccountId, 
            owner_address: AccountId,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                instance._init_with_owner(owner_address);
                instance.initialize(admin_address).ok().unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            admin_address: AccountId
        ) -> Result<(), OwnableError> {
            self.admin_address = admin_address;
            self.project_count = 0;
            Ok(())
        }

        /* SETTERS */

        /// Update admin address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(
            &mut self,
            admin_address: AccountId
        ) -> Result<(), Error>  {
            self.admin_address = admin_address;
            Ok(())
        }

        /* END SETTERS */
        
        /* GETTERS */

        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.admin_address;
        }

        /// Get Project Count
        #[ink(message)]
        pub fn get_project_count(   
            &self
        ) -> u64 {
            return self.project_count;
        }

        /* END GETTERS*/
    }
}