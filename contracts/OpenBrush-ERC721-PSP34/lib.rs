#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_psp34 {
    use ink_prelude::string::String;
    use brush::contracts::psp34::*;
    use brush::contracts::psp34::extensions::metadata::*;
    use brush::contracts::psp34::extensions::burnable::*;
    use brush::contracts::psp34::extensions::enumerable::*;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
        Mapping,
    };
    use ink_prelude::vec::Vec;
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
        claimed_amount: u64
    }

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage, PSP34EnumerableStorage)]
    #[ink(storage)]
    pub struct ArtZeroNFT{
        #[PSP34StorageField]
        psp34: PSP34Data,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        #[PSP34EnumerableStorageField]
        enumdata: PSP34EnumerableData,

        //Max Total Token number to Mint
        total_supply: u64,
        token_count: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,

        //Who got free mint
        whitelists: Mapping<AccountId,Whitelist>,
        whitelist_count: u64,
        whitelist_accounts: Mapping<u64,AccountId>,
        whitelist_mint_total_amount: u64,
        //Pre_launch Minting Fee
        fee_1: Balance,
        //Launch Minting Fee
        fee_2: Balance,
        //To what amount the fee_1 is applied, after that fee_2
        amount_1: u64,
        //is Minting started
        // 0: not started
        // 1: started until amount_1 reached
        // 2: started until total_supply reached
        mint_mode: u8
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
        InvalidInput,
        OnlyAdmin,
        ClaimedAll,
        TokenLimitReached,
        TokenLimitReachedMode1,
        InvalidFee,
        NotMintTime,
        NotEnoughBalance,
        InvalidMintAmount
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    impl Ownable for ArtZeroNFT {}
    impl PSP34 for ArtZeroNFT {}
    impl PSP34Burnable for ArtZeroNFT {}
    impl PSP34Metadata for ArtZeroNFT {}
    impl PSP34Internal for ArtZeroNFT {}
    impl PSP34Enumerable for ArtZeroNFT {}

    #[brush::trait_definition]
    pub trait Psp34Traits {
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

    impl ArtZeroNFT {
        /// fee_1: Pre_launch Minting Fee
        /// fee_2: Launch Minting Fee
        /// amount_1: To what amount the fee_1 is applied, after that fee_2
        /// mint_mode 0: not started
        /// mint_mode 1: started until amount_1 reached
        /// mint_mode 2: started until total_supply reached
        /// total_supply: total_supply
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String, total_supply: u64, fee_1: Balance, fee_2: Balance, amount_1: u64) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
                instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
                instance.total_supply = total_supply;
                instance._init_with_owner(contract_owner);

                instance.fee_1 = fee_1;
                instance.fee_2 = fee_2;
                instance.amount_1 = amount_1;
            })
        }
        /*
            WHITELIST FUNCTIONS =============
        */
        /// Add new whitelist - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn add_whitelist(
            &mut self,
            account: AccountId,
            whitelist_amount: u64
        ) -> Result<(), Error> {

            //fee must less than total tokens
            if  whitelist_amount > self.total_supply ||
                whitelist_amount == 0{
                return Err(Error::InvalidInput);
            }
            if  self.whitelists.get(&account).is_some() ||
                self.whitelist_mint_total_amount.checked_add(whitelist_amount).unwrap() > self.amount_1{
                return Err(Error::InvalidInput);
            }

            self.whitelist_count += 1;
            self.whitelist_accounts.insert(&self.whitelist_count, &account);

            let whitelist = Whitelist {
                whitelist_amount: whitelist_amount,
                claimed_amount: 0
            };

            self.whitelists.insert(&account, &whitelist);
            self.whitelist_mint_total_amount = self.whitelist_mint_total_amount.checked_add(whitelist_amount).unwrap();

            Ok(())
        }

        /// Update Whitelist Amount - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_whitelist_amount(
            &mut self,
            account: AccountId,
            whitelist_amount: u64
        ) -> Result<(), Error>  {

            if self.whitelists.get(&account).is_none(){
                 return Err(Error::InvalidInput);
             }

            let mut whitelist = self.whitelists.get(&account).unwrap();
            if whitelist_amount >= whitelist.whitelist_amount {
                self.whitelist_mint_total_amount = self.whitelist_mint_total_amount.checked_add(whitelist_amount - whitelist.whitelist_amount).unwrap();
            }
            else{
                self.whitelist_mint_total_amount = self.whitelist_mint_total_amount.checked_sub(whitelist.whitelist_amount - whitelist_amount).unwrap();
            }
            if self.whitelist_mint_total_amount > self.amount_1{
                return Err(Error::InvalidInput);
            }

            whitelist.whitelist_amount = whitelist_amount;
            self.whitelists.insert(&account, &whitelist);
            Ok(())

        }

        /* SETTERS */
        /// Set mint_mode - Only Owner - mint_mode 0: not started - mint_mode 1: started until amount_1 reached - mint_mode 2: started until total_supply reached
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_mint_mode(
            &mut self,
            mint_mode: u8
        ) -> Result<(), Error> {
            self.mint_mode = mint_mode;
            Ok(())
        }

        /// set fee_1: Pre_launch Minting Fee - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_fee_1(
            &mut self,
            fee_1: Balance
        ) -> Result<(), Error> {
            self.fee_1 = fee_1;
            Ok(())
        }
        /// set fee_2: Launch Minting Fee - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_fee_2(
            &mut self,
            fee_2: Balance
        ) -> Result<(), Error> {
            self.fee_2 = fee_2;
            Ok(())
        }
        /// set amount_1: To what amount the fee_1 is applied, after that fee_2 - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_amount_1(
            &mut self,
            amount_1: u64
        ) -> Result<(), Error> {
            if amount_1 < self.whitelist_mint_total_amount{
                return Err(Error::InvalidInput);
            }
            self.amount_1 = amount_1;
            Ok(())
        }

        /*
            END OF WHITELIST FUNCTIONS =============
        */
        /*
            MINT FUNCTIONS =============
        */

        /// Whitelisted User Creates multiple
        #[ink(message)]
        pub fn whitelist_mint(&mut self, mint_amount: u64) -> Result<(), Error> {

            if self.mint_mode == 0 {
                return Err(Error::NotMintTime);
            }

            let caller = self.env().caller();

            if self.whitelists.get(&caller).is_none(){
                 return Err(Error::InvalidInput);
             }

             if self.token_count >= self.total_supply {
                 return Err(Error::TokenLimitReached);
             }

            let mut caller_info = self.whitelists.get(&caller).unwrap();
            if caller_info.whitelist_amount <= caller_info.claimed_amount {
                return Err(Error::ClaimedAll);
            }
            if caller_info.whitelist_amount < caller_info.claimed_amount.checked_add(mint_amount).unwrap() {
                return Err(Error::InvalidMintAmount);
            }
            caller_info.claimed_amount = caller_info.claimed_amount.checked_add(mint_amount).unwrap();
            self.whitelists.insert(&caller, &caller_info);

            for _i in 0..mint_amount {
                self.token_count += 1;
                assert!(self._mint_to(caller, Id::U64(self.token_count)).is_ok());
            }

            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn paid_mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            //Mint is disabled
            if self.mint_mode == 0 {
                return Err(Error::NotMintTime);
            }
            //Mode 1 - mint till amount_1 reached
            if  self.mint_mode == 1 &&
                self.token_count >= self.amount_1
            {
                return Err(Error::TokenLimitReachedMode1);
            }

            if  self.mint_mode == 1 &&
                self.fee_1 != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            //Mode 2 - mint till total_supply reached
            if  self.mint_mode == 2 &&
                self.fee_2 != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }

            if self.token_count >= self.total_supply {
                return Err(Error::TokenLimitReached);
            }
            self.token_count += 1;
            assert!(self._mint_to(caller, Id::U64(self.token_count)).is_ok());

            Ok(())
        }
        /*
            END OF MINT FUNCTIONS =============
        */

        /* GETTERS */
        /// mint_mode 0: not started - mint_mode 1: started until amount_1 reached - mint_mode 2: started until total_supply reached
        #[ink(message)]
        pub fn get_mint_mode(
            &self
        ) -> u8 {
            return self.mint_mode;
        }
        /// fee_1: Pre_launch Minting Fee
        #[ink(message)]
        pub fn get_fee_1(
            &self
        ) -> Balance {
            return self.fee_1;
        }
        /// fee_2: Launch Minting Fee
        #[ink(message)]
        pub fn get_fee_2(
            &self
        ) -> Balance {
            return self.fee_2;
        }
        /// token_count: get token count
        #[ink(message)]
        pub fn get_token_count(
            &self
        ) -> u64 {
            return self.token_count;
        }
        /// amount_1: To what amount the fee_1 is applied, after that fee_2
        #[ink(message)]
        pub fn get_amount_1(
            &self
        ) -> u64 {
            return self.amount_1;
        }

        /// Get Whitelist Account by ID
        #[ink(message)]
        pub fn get_whitelist_account(
            &self,
            id: u64
        ) -> AccountId {
            return self.whitelist_accounts.get(&id).unwrap();
        }

        /// Get Whitelist Information by AccountID
        #[ink(message)]
        pub fn get_whitelist(
            &self,
            account: AccountId
        ) -> Whitelist {
            return self.whitelists.get(&account).unwrap();
        }
        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_whitelist_count(
            &self
        ) -> u64 {
            return self.whitelist_count;
        }

        ///Get total tokens can be mint by whitelisted accounts
        #[ink(message)]
        pub fn get_whitelist_mint_total_amount(
            &self
        ) -> u64 {
            return self.whitelist_mint_total_amount;
        }

        ///Get total supply
        #[ink(message)]
        pub fn get_total_supply(
            &self
        ) -> u64 {
            return self.total_supply;
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            if value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }

        fn add_attribute_name(&mut self, attribute_input:Vec<u8>){
            let mut exist:bool = false;
            for index in 0..self.attribute_count {
                let attribute_name = self.attribute_names.get(&index);
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

    }

    impl Psp34Traits for ArtZeroNFT {
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
                let value = values[i].clone();

                self.add_attribute_name(byte_attribute.clone());
                self._set_attribute(token_id.clone(),unsorted_attribute.into_bytes().clone(), value.into_bytes());
            }

            Ok(())
        }
        // Get multiple  attributes
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
