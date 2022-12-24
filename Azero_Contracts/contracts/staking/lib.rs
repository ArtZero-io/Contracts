#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_staking_nft {
    use ink_lang::ToAccountId;
    use ink_env::CallFlags;
    use ink_prelude::{
        string::String,
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate
    };
    use openbrush::{
        contracts::{
            ownable::*,
            access_control::*,
            traits::psp34::{
                extensions::{
                    burnable::*,
                    metadata::*,
                },
                *,
            },
        },
        storage::{
            Mapping,
            TypeGuard,
            MultiMapping,
            ValueGuard,
        },
        traits::Storage,
        modifiers,
    };
    use artzero_project::{
        impls::staking::*,
        traits::{
            psp34_standard::{
                psp34traits_external::Psp34Traits
            },
            staking::{
                Error,
                *
            }
        }
    };
    use psp34_nft::psp34_nft::Psp34NftRef;
    use artzero_project::traits::psp34_standard::*;
    
    impl AccessControl for ArtZeroStakingNFT {}
    impl Ownable for ArtZeroStakingNFT {}
    impl ArtZeroStakingTrait for ArtZeroStakingNFT {}
    
    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct ArtZeroStakingNFT {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: artzero_project::impls::staking::data::Manager,
    }

    impl ArtZeroStakingNFT {
        #[ink(constructor)]
        pub fn new(
            contract_owner: AccountId,
            admin_address: AccountId,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance._init_with_admin(instance.env().caller());
                instance.grant_role(ADMINER, admin_address).expect("Should grant the role");
                instance
                    .initialize(artzero_nft_contract, limit_unstake_time, admin_address)
                    .ok()
                    .unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            artzero_nft_contract: AccountId,
            limit_unstake_time: u64,
            admin_address: AccountId,
        ) -> Result<(), OwnableError> {
            self.manager.nft_contract_address = artzero_nft_contract;
            self.manager.limit_unstake_time = limit_unstake_time;
            self.manager.admin_address = admin_address;
            self.manager.is_locked = false;
            Ok(())
        }
    }

}