#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_psp34 {
    use ink_prelude::string::String;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use ink_storage::traits::SpreadAllocate;
    use scale::{
        Decode,
        Encode,
    };

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage)]
    #[ink(storage)]
    pub struct ArtZeroNFT{
        #[PSP34StorageField]
        psp34: PSP34Data,
        next_id: u32,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,

    }
    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {

    }

    impl PSP34 for ArtZeroNFT {}
    impl PSP34Burnable for ArtZeroNFT {}
    impl PSP34Metadata for ArtZeroNFT {}
    impl PSP34Internal for ArtZeroNFT {}

    impl ArtZeroNFT {
        #[ink(constructor)]
        pub fn new(name: String, symbol: String) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
            })
        }
        /// Mint method which mints a token and updates the id of next token
        #[ink(message)]
        pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
            self._mint_to(Self::env().caller(), Id::U32(self.next_id))?;
            self.next_id += 1;
            Ok(())
        }
        #[ink(message)]
        //TOTO: Only Owner
        pub fn set_base_uri(&mut self, uri: String) {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
        }
    }
}
