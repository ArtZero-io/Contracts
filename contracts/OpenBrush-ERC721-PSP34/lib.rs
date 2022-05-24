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
        admin_address: AccountId,
        //Max Total Token number to Mint
        total_supply: u64,
        last_token_id: u64,
        whitelist_minted_count: u64,
        public_sale_minted_count: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,

        //Who got free mint
        whitelists: Mapping<AccountId,Whitelist>,
        whitelist_count: u64,
        whitelist_accounts: Mapping<u64,AccountId>,
        whitelist_mint_total_amount: u64,
        //Public Sale Minting Fee
        minting_fee: Balance,
        public_sale_amount: u64,
        //is Minting started
        // 0: not started
        // 1: allow whitelist and public sale mint
        // 2: just allow whitelist mint
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
        /// mint_mode 0: not started
        /// mint_mode 1: allow whitelist and public sale mint
        /// mint_mode 2: just allow whitelist mint
        /// total_supply: total_supply
        /// minting_fee: Public Sale Minting Fee
        /// public_sale_amount: Public Sale Amount
        #[ink(constructor)]
        pub fn new(
            contract_owner: AccountId,
            admin_address: AccountId, 
            name: String, 
            symbol: String, 
            total_supply: u64,
            minting_fee: Balance,
            public_sale_amount: u64
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.initialize(admin_address, name, symbol, total_supply, minting_fee, public_sale_amount).ok().unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            admin_address: AccountId, 
            name: String, 
            symbol: String, 
            total_supply: u64,
            minting_fee: Balance,
            public_sale_amount: u64
        ) -> Result<(), OwnableError> {
            self._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
            self._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
            self.total_supply = total_supply;
            self.admin_address = admin_address;
            self.minting_fee = minting_fee;
            self.public_sale_amount = public_sale_amount;
            Ok(())
        }

        /*
            WHITELIST FUNCTIONS =============
        */
        /// Add new whitelist - Only Admin
        #[ink(message)]
        pub fn add_whitelist(
            &mut self,
            account: AccountId,
            whitelist_amount: u64
        ) -> Result<(), Error> {
            if  self.env().caller() == self.admin_address {
                //fee must less than total tokens
                if  whitelist_amount > self.total_supply ||
                    whitelist_amount == 0{
                    return Err(Error::InvalidInput);
                }
                if  self.whitelists.get(&account).is_some() {
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
            } else{
                return Err(Error::OnlyAdmin);
            }
        }

        /// Update Whitelist Amount - Only Owner
        #[ink(message)]
        pub fn update_whitelist_amount(
            &mut self,
            account: AccountId,
            whitelist_amount: u64
        ) -> Result<(), Error>  {
            if  self.env().caller() == self.admin_address {
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
   
               whitelist.whitelist_amount = whitelist_amount;
               self.whitelists.insert(&account, &whitelist);
               Ok(())
            } else{
                return Err(Error::OnlyAdmin);
            }

        }

        /* SETTERS */
        /// Set mint_mode - Only Owner - mint_mode 0: not started - mint_mode 1: allow whitelist and public sale mint - mint_mode 2: just allow whitelist mint
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_mint_mode(
            &mut self,
            mint_mode: u8
        ) -> Result<(), Error> {
            self.mint_mode = mint_mode;
            Ok(())
        }

        /// Update Admin Address - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(
            &mut self,
            admin_address: AccountId
        )  -> Result<(), Error> {
            self.admin_address = admin_address;
            Ok(())
        }

        /// set minting_fee: Public sale minting  - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_minting_fee(
            &mut self,
            minting_fee: Balance
        ) -> Result<(), Error> {
            self.minting_fee = minting_fee;
            Ok(())
        }        

        /// set public_sale_amount: public sale amount limit - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn set_public_sale_amount(
            &mut self,
            public_sale_amount: u64
        ) -> Result<(), Error> {
            if public_sale_amount > self.total_supply{
                return Err(Error::InvalidInput);
            }
            self.public_sale_amount = public_sale_amount;
            Ok(())
        }
        
        /*
            END OF WHITELIST FUNCTIONS =============
        */
        /*
            MINT FUNCTIONS =============
        */

        /// Standard mint only owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn standard_mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();

            if self.last_token_id >= self.total_supply {
                return Err(Error::TokenLimitReached);
            }
            
            self.last_token_id += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());

            Ok(())
        }

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

            if self.last_token_id >= self.total_supply {
                return Err(Error::TokenLimitReached);
            }
            
            if self.whitelist_minted_count.checked_add(mint_amount).unwrap() > self.whitelist_mint_total_amount {
                return Err(Error::InvalidMintAmount);
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
                self.last_token_id += 1;
                assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            }
            self.whitelist_minted_count = self.whitelist_minted_count.checked_add(mint_amount).unwrap();

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
            //Mode 2 - just allow whitelist mint
            if self.mint_mode == 2 {
                return Err(Error::NotMintTime);
            }
            //Mode 1 - allow whitelist and pulic mint
            if  self.mint_mode == 1 &&
                self.public_sale_minted_count >= self.public_sale_amount
            {
                return Err(Error::TokenLimitReachedMode1);
            }

            if  self.mint_mode == 1 &&
                self.minting_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }

            if self.last_token_id >= self.total_supply {
                return Err(Error::TokenLimitReached);
            }
            
            self.last_token_id += 1;
            self.public_sale_minted_count += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());

            Ok(())
        }
        /*
            END OF MINT FUNCTIONS =============
        */

        /* GETTERS */
        /// minting_fee: Public sale minting fee
        #[ink(message)]
        pub fn get_minting_fee(
            &self
        ) -> Balance {
            return self.minting_fee;
        }

        /// mint_mode 0: not started - mint_mode 1: allow whitelist and public sale mint - mint_mode 2: just allow whitelist mint
        #[ink(message)]
        pub fn get_mint_mode(
            &self
        ) -> u8 {
            return self.mint_mode;
        }

        /// last_token_id: get last token id
        #[ink(message)]
        pub fn get_last_token_id(
            &self
        ) -> u64 {
            return self.last_token_id;
        }

        /// whitelist_minted_count: get the whitelist minted amount
        #[ink(message)]
        pub fn get_whitelist_minted_count(
            &self
        ) -> u64 {
            return self.whitelist_minted_count;
        }

        /// public_sale_minted_count: get the public sale minted amount
        #[ink(message)]
        pub fn get_public_sale_minted_count(
            &self
        ) -> u64 {
            return self.public_sale_minted_count;
        }

        /// public_sale_amount: get public sale limit amount
        #[ink(message)]
        pub fn get_public_sale_amount(
            &self
        ) -> u64 {
            return self.public_sale_amount;
        }

        /// Get Whitelist Account by ID
        #[ink(message)]
        pub fn get_whitelist_account(
            &self,
            id: u64
        ) -> AccountId {
            return self.whitelist_accounts.get(&id).unwrap();
        }

        /// Get Whitelist Information by AccountId
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

        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.admin_address;
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
                let byte_unsorted_attribute = unsorted_attribute.into_bytes();
                let value = values[i].clone();

                self.add_attribute_name(byte_unsorted_attribute.clone());
                self._set_attribute(token_id.clone(),byte_unsorted_attribute.clone(), value.into_bytes());
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
