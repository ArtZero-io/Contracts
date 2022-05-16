#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod psp1155_nft {
    use brush::contracts::psp1155::*;
    use brush::contracts::psp1155::extensions::metadata::*;
    use brush::contracts::ownable::*;

    use ink_prelude::{
        string::String,
        vec,
    };
    use ink_storage::{
        traits::SpreadAllocate,
        Mapping,
    };

    #[derive(Default, SpreadAllocate, PSP1155Storage, PSP1155MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct Psp1155Nft {
        #[PSP1155StorageField]
        psp1155: PSP1155Data,
        #[PSP1155MetadataStorageField]
        metadata: PSP1155MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData
    }

    impl PSP1155 for Psp1155Nft {}
    impl PSP1155Metadata for Psp1155Nft {}
    impl Ownable for Psp1155Nft {}

    impl Psp1155Nft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, uri: Option<String>) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.metadata.uri = uri;
                instance._init_with_owner(contract_owner);
            })
        }

        #[ink(message)]
        pub fn mint_tokens(&mut self, id: Id, amount: Balance) -> Result<(), PSP1155Error> {
            self._mint_to(Self::env().caller(), vec![(id, amount)])
        }
    }
}
