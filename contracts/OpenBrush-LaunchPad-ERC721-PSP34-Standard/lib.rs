#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::launchpad_psp34_nft_standard::{
    LaunchPadPsp34NftStandard,
    LaunchPadPsp34NftStandardRef,
};

#[brush::contract]
pub mod launchpad_psp34_nft_standard {
    use ink_prelude::string::String;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::psp34::extensions::enumerable::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            SpreadAllocate
        },
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_prelude::string::ToString;

    #[derive(
        Copy,
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        Default,
        PackedLayout,
        SpreadLayout,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Whitelist {
        whitelist_amount: u64,
        claimed_amount: u64,
        minting_fee: Balance
    }

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage, PSP34EnumerableStorage)]
    #[ink(storage)]
    pub struct LaunchPadPsp34NftStandard{
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,
        #[PSP34EnumerableStorageField]
        enumdata: PSP34EnumerableData,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        total_supply: u64,
        last_pharse_id: u64,
        whitelist_count: u64,
        pharses_code_by_id: Mapping<u64, Vec<u8>>,
        pharses_id_by_code: Mapping<Vec<u8>, u64>,
        pharse_whitelists_link: Mapping<(AccountId, u64), Whitelist)>,
        public_pharse: Mapping<u64, u8>,
        default_pharse_id: u64
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        NotOwner
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    impl Ownable for LaunchPadPsp34NftStandard {}
    #[brush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Burnable + PSP34Metadata;
    impl PSP34 for LaunchPadPsp34NftStandard {}
    impl PSP34Burnable for LaunchPadPsp34NftStandard {}
    impl PSP34Metadata for LaunchPadPsp34NftStandard {}
    impl PSP34Internal for LaunchPadPsp34NftStandard {}
    impl PSP34Enumerable for LaunchPadPsp34NftStandard {}

    #[brush::trait_definition]
    pub trait LaunchPadPsp34NftStandardTraits {
        #[ink(message)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error>;
        #[ink(message)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error>;
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String>;
        #[ink(message)]
        fn get_attribute_count(&self) -> u32;
        #[ink(message)]
        fn get_attribute_name(&self, index:u32) -> String;
        #[ink(message)]
        fn token_uri(&self,token_id: u64) -> String;
    }

    impl LaunchPadPsp34NftStandard {

        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, total_supply: u64) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.total_supply = total_supply;
                instance.last_pharse_id = 0;
                instance.default_pharse_id = 0;
            })
        }

        ///Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            Ok(())
        }

        /// Add new whitelist - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn add_whitelist(
            &mut self,
            account: AccountId,
            pharse_id: u64,
            whitelist_amount: u64,
            whitelist_price: Balance
        ) -> Result<(), Error> {
            //fee must less than total tokens
            if  whitelist_amount > self.total_supply ||
                whitelist_amount == 0 {
                return Err(Error::InvalidInput);
            }
            if  self.pharse_whitelists_link.get(&(account, pharse_id)).is_some() {
                return Err(Error::InvalidInput);
            }

            self.whitelist_count += 1;

            let whitelist = Whitelist {
                whitelist_amount: whitelist_amount,
                claimed_amount: 0,
                minting_fee: whitelist_price
            }

            self.pharse_whitelists_link.insert(&(account, pharse_id), &whitelist);

            Ok(())
            
        }

        /// Whitelisted User Creates multiple
        #[ink(message)]
        #[ink(payable)]
        pub fn whitelist_mint(&mut self, pharse_id: u64, mint_amount: u64) -> Result<(), Error> {
            if self.public_pharse.get(&pharse_id).is_none() {
                return Err(Error::PharseNotExist); 
            }
            if self.public_pharse.get(&pharse_id).unwrap() == 1 || pharse_id == self.default_pharse_id {
                if self.last_token_id >= self.total_supply {
                    return Err(Error::TokenLimitReached);
                }
                let caller = self.env().caller();
                
                if self.pharse_whitelists_link.get(&(caller, pharse_id)).is_none(){
                    return Err(Error::InvalidInput);
                }
    
                let mut caller_info = self.pharse_whitelists_link.get(&(caller, pharse_id)).unwrap();
                if caller_info.whitelist_amount <= caller_info.claimed_amount {
                    return Err(Error::ClaimedAll);
                }
    
                if  caller_info.minting_fee != self.env().transferred_value() {
                    return Err(Error::InvalidFee);
                }
    
                caller_info.claimed_amount = caller_info.claimed_amount.checked_add(mint_amount).unwrap();
                self.pharse_whitelists_link.insert(&(caller, pharse_id), &caller_info);
    
                for _i in 0..mint_amount {
                    self.last_token_id += 1;
                    assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
                }
    
                Ok(())
            } else {
                return Err(Error::PharseNotPublic);
            }
        }

        ///Add new phare - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn addNewPharse(
            &mut self, 
            pharse_code: String, 
            whitelist_accounts: Vec<AccountId>, 
            whitelist_amounts: Vec<u64>,
            whitelist_prices: Vec<Balance>
        ) -> Result<(), Error> {
            let pharse_id_by_code = self.pharses_id_by_code.get(&pharse_code.into_bytes());
            if pharse_id_by_code.is_some() {
                return Err(Error::Custom(String::from("This code is exist!")));
            }
            if whitelist_accounts.len() != whitelist_amounts.len() || whitelist_accounts.len() != whitelist_prices.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            let caller = self.env().caller();
            self.last_pharse_id += 1;
            let length = whitelist_accounts.len();
            self.pharses_id_by_code.insert(&pharse_code.into_bytes(), &self.last_pharse_id);
            self.pharses_code_by_id.insert(&self.last_pharse_id, &pharse_code.into_bytes());
            self.public_pharse.insert(&self.last_pharse_id, 0);
            for i in 0..length {
                let whitelist = Whitelist {
                    whitelist_amount: whitelist_amounts[i].clone(),
                    claimed_amount: 0,
                    minting_fee: whitelist_prices[i].clone()
                };
                self.whitelist_count += 1;
                self.pharse_whitelists_link.insert(&(whitelist_accounts[i].clone(), self.last_pharse_id), &whitelist);
            }
            Ok(())
        }

        /// Get whitelist information by pharse code
        #[ink(message)]
        pub fn get_whitelist_by_account_id(
            &self,
            account: AccountId,
            pharse_code: String
        ) -> Option<Whitelist> {
            if self.pharses_id_by_code.get(&pharse_code).is_none() {
                return None;
            }
            let pharse_id = self.pharses_id_by_code.get(&pharse_code).unwrap();
            if self.pharse_whitelists_link.get(&(account, pharse_id)).is_none() {
                return None;
            }
            return Some(self.pharse_whitelists_link.get(&(account, pharse_id)).unwrap());
        }

        /// Get Default Pharse Id
        #[ink(message)]
        pub fn get_default_pharse_id(
            &self
        ) -> u64 {
            return self.default_pharse_id;
        }

        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_public_pharse_status(
            &self,
            pharse_id: u64
        ) -> Option<u8> {
            if self.public_pharse.get(&pharse_id).is_none() {
                return None;
            }
            return self.public_pharse.get(&pharse_id).unwrap();
        }

        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_whitelist_count(
            &self
        ) -> u64 {
            return self.whitelist_count;
        }

        ///Get Pharse Count 
        #[ink(message)]
        pub fn get_last_pharse_id(&self) -> u64 {
            return self.last_pharse_id;
        }  

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        fn add_attribute_name(&mut self, attribute_input:Vec<u8>){
            let mut exist:bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&(index+1));
                if attribute_name.is_some(){
                    if attribute_name.unwrap() == attribute_input{
                        exist = true;
                        break;
                    }
                }
            }
            if !exist {
                self.attribute_count += 1;
                self.attribute_names.insert(&self.attribute_count, &attribute_input);
            }
        }

        /// Lock nft - Only owner token
        #[ink(message)]
        pub fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            let caller = self.env().caller();
            let token_owner = self.owner_of(token_id.clone()).unwrap();
            assert!(caller == token_owner);
            self.locked_token_count += 1;
            self.locked_tokens.insert(&token_id, &1);
            Ok(())
        }

        /// Check token is locked or not
        #[ink(message)]
        pub fn is_locked_nft(&self, token_id: Id) -> bool {
            if self.locked_tokens.get(&token_id).is_some() {
                return true;
            }
            return false;
        }

        ///Get Locked Token Count
        #[ink(message)]
        pub fn get_locked_token_count(&self) -> u64 {
            return self.locked_token_count;
        }

        ///Get Locked Token Count
        pub fn get_total_supply(&self) -> u64 {
            return self.total_supply;
        }

        /// Update public pharse status - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_public_pharse_status(
            &mut self, 
            pharse_id: u64, 
            status: u8
        ) -> Result<(), Error> {
            if self.public_pharse.get(&pharse_id).is_none() {
                return Err(Error::PharseNotExist); 
            }
            self.public_pharse.insert(&pharse_id, &status);
            Ok(())
        }

        /// Update default pharse id - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_default_pharse_id(
            &mut self, 
            pharse_id: u64
        ) -> Result<(), Error> {
            self.default_pharse_id = pharse_id;
            Ok(())
        }
    }

    impl LaunchPadPsp34NftStandardTraits for LaunchPadPsp34NftStandard {

        /// Change baseURI
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
            self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
            Ok(())
        }

        ///Only Owner can set multiple attributes to a token
        #[ink(message)]
        #[modifiers(only_owner)]
        fn set_multiple_attributes(&mut self, token_id:Id, attributes: Vec<String>, values: Vec<String>) -> Result<(),Error> {
            assert!(token_id != Id::U64(0));

            if self.is_locked_nft(token_id.clone()) {
                return Err(Error::Custom(String::from("Token is locked")));
            }

            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            //Check Duplication
            let mut sorted_attributes = attributes.clone();
            sorted_attributes.sort();
            let length = sorted_attributes.len();

            for i in 0..length {
                let attribute = sorted_attributes[i].clone();
                let byte_attribute = attribute.into_bytes();

                if i + 1 < length {
                    let next_attribute = sorted_attributes[i + 1].clone();
                    let byte_next_attribute = next_attribute.into_bytes();
                    if byte_attribute == byte_next_attribute{
                        return Err(Error::Custom(String::from("Duplicated Attributes")));
                    }
                }

                let unsorted_attribute = attributes[i].clone();
                let byte_unsorted_attribute = unsorted_attribute.into_bytes();
                let value = values[i].clone();

                self.add_attribute_name(byte_unsorted_attribute.clone());
                self._set_attribute(token_id.clone(),byte_unsorted_attribute.clone(), value.into_bytes());
            }

            Ok(())
        }

        /// Get multiple  attributes
        #[ink(message)]
        fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.get_attribute(token_id.clone(),attribute.into_bytes());
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                }
                else{
                    ret.push(String::from(""));
                }
            }
            ret
        }

        ///Get Attribute Count
        #[ink(message)]
        fn get_attribute_count(&self) -> u32 {
            self.attribute_count
        }
        ///Get Attribute Name
        #[ink(message)]
        fn get_attribute_name(&self, index:u32) -> String {
            let attribute = self.attribute_names.get(&index);
            if attribute.is_some() {
                String::from_utf8(attribute.unwrap()).unwrap()
            }
            else{
                String::from("")
            }
        }
        /// Get URI from token ID
        #[ink(message)]
        fn token_uri(
            &self,
            token_id: u64
        ) -> String {
            let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
            let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
            token_uri = token_uri + &token_id.to_string() + &String::from(".json");
            return token_uri;
        }

    }
}
