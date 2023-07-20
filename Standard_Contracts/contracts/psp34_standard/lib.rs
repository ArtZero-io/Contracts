#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[openbrush::implementation(PSP34, Ownable, PSP34Metadata, PSP34Enumerable)]
#[openbrush::contract]
pub mod psp34_nft {
    use standard_contracts::impls::psp34_standard::*;
    use standard_contracts::traits::psp34_standard::*;
    use openbrush::{
        modifiers,
        storage::Mapping,
        traits::{
            Storage,
            String,
        },
    };
    use openbrush::traits::DefaultEnv;
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
        #[storage_field]
        enumerable: enumerable::Data,
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

    impl Psp34TraitsImpl for Psp34Nft {}

    impl Psp34Traits for Psp34Nft {
        /// This function sets the baseURI for the NFT contract. Only Contract Owner can perform this function. baseURI is the location of the metadata files if the NFT collection use external source to keep their NFT artwork. ArtZero uses IPFS by default, the baseURI can have format like this: ipfs://<hash_ID>/
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            Psp34TraitsImpl::set_base_uri(self, uri)
        }

        /// This function set the attributes to each NFT. Only Contract Owner can perform this function. The metadata input is an array of [(attribute, value)]. The attributes in ArtZero platform are the NFT traits.
        #[ink(message)]
        fn set_multiple_attributes(
            &mut self,
            token_id: Id,
            metadata: Vec<(String, String)>
        ) -> Result<(), Error> {
            Psp34TraitsImpl::set_multiple_attributes(self, token_id, metadata)
        }
        /// This function returns all available attributes of each NFT
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
            Psp34TraitsImpl::get_attributes(self, token_id, attributes)
        }
        /// This function return how many unique attributes in the contract
        #[ink(message)]
        fn get_attribute_count(&self) -> u32 {
            Psp34TraitsImpl::get_attribute_count(self)
        }
        /// This function return the attribute name using attribute index. Beacause attributes of an NFT can be set to anything by Contract Owner, AztZero uses this function to get all attributes of an NFT
        #[ink(message)]
        fn get_attribute_name(&self, index: u32) -> String {
            Psp34TraitsImpl::get_attribute_name(self, index)
        }
        /// This function return the metadata location of an NFT. The format is baseURI/<token_id>.json
        #[ink(message)]
        fn token_uri(&self, token_id: u64) -> String {
            Psp34TraitsImpl::token_uri(self, token_id)
        }
        /// This function return the owner of the NFT Contract
        #[ink(message)]
        fn get_owner(&self) -> Option<AccountId> {
            Psp34TraitsImpl::get_owner(self)
        }
        /// This function return the latest token ID, everytime new NFT is mint, last_token_id is increased by 1 in mint function. Note: This is not the same as the total supply return by the psp34 function as NFT can be burnt.
        #[ink(message)]
        fn get_last_token_id(&self) -> u64 {
            Psp34TraitsImpl::get_last_token_id(self)
        }
        /// This function lets NFT owner to lock their NFT. Once locked, the NFT traits (attributes) can not be changed
        #[ink(message)]
        fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            Psp34TraitsImpl::lock(self, token_id)
        }
        /// This function check if an NFT is locked or not
        #[ink(message)]
        fn is_locked_nft(&self, token_id: Id) -> bool {
            Psp34TraitsImpl::is_locked_nft(self, token_id)
        }
        /// This function returns how many NFTs have been locked by its owners
        #[ink(message)]
        fn get_locked_token_count(&self) -> u64 {
            Psp34TraitsImpl::get_locked_token_count(self)
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