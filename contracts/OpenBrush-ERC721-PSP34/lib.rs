#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_psp34 {
    use ink_prelude::string::String;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::traits::SpreadAllocate;
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroNFT{
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,

        //Max Total Token number to Mint
        max_token_number: u32,
        token_count: u32,

    }
    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        MaxTokenMint
    }

    impl Ownable for ArtZeroNFT {}
    impl PSP34 for ArtZeroNFT {}
    impl PSP34Burnable for ArtZeroNFT {}
    impl PSP34Metadata for ArtZeroNFT {}
    impl PSP34Internal for ArtZeroNFT {}


    impl ArtZeroNFT {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String, max_token_number: u32) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance.max_token_number = max_token_number;
                instance._init_with_owner(contract_owner);
            })
        }
        /// Mint method which mints a token and updates the id of next token
        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), Error> {
            if self.token_count >= self.max_token_number {
                return Err(Error::MaxTokenMint);
            }
            self._mint_to(Self::env().caller(), Id::U32(self.token_count));
            self.token_count += 1;
            Ok(())
        }
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
            Ok(())
        }
    }
}
