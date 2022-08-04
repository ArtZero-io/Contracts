#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::launchpad_psp34_nft_standard::{
    LaunchPadPsp34NftStandard,
    LaunchPadPsp34NftStandardRef,
};

#[openbrush::contract]
pub mod launchpad_psp34_nft_standard {
    use ink_prelude::string::String;
    use openbrush::contracts::psp34::*;
    use openbrush::contracts::psp34::extensions::metadata::*;
    use openbrush::contracts::psp34::extensions::enumerable::*;
    use openbrush::contracts::ownable::*;
    use openbrush::modifiers;
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
        claimed_amount: u64,
        minting_fee: Balance
    }

    #[derive(
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
    pub struct Phase {
        title: Vec<u8>,
        is_public: bool,
        public_minting_fee: Balance,
        public_minting_amout: u64,
        whitelist_amount: u64,
        claimed_amount: u64,
        start_time: Timestamp,
        end_time: Timestamp
    }

    #[derive(Default, Debug, ink_storage::traits::SpreadLayout, ink_storage::traits::SpreadAllocate)]
    #[cfg_attr(feature = "std", derive(ink_storage::traits::StorageLayout))]
    pub struct EnumerablePhaseAccountMapping {
        /// Phase id is key
        phase_id_to_account: Mapping<(u64, u8), AccountId>,
        account_to_phase_id: Mapping<(u64, AccountId), u8>
    }

    impl EnumerablePhaseAccountMapping {
        pub fn insert(&mut self, account: &AccountId, phase_id: &u8, account_index: &u64) {
            self.phase_id_to_account.insert((account_index, phase_id), account);
            self.account_to_phase_id.insert((account_index, account), phase_id);
        }

        pub fn get_by_account(&self, account: &AccountId, account_index: &u64) -> Result<u8, Error> {
            self.account_to_phase_id.get((account_index, account)).ok_or(Error::InvalidInput)
        }

        pub fn get_by_phase_id(&self, phase_id: &u8, account_index: &u64) -> Result<AccountId, Error> {
            self.phase_id_to_account.get((account_index, phase_id)).ok_or(Error::InvalidInput)
        }
    }

    #[derive(Default, SpreadAllocate, PSP34Storage, PSP34MetadataStorage, OwnableStorage)]
    #[ink(storage)]
    pub struct LaunchPadPsp34NftStandard{
        #[PSP34StorageField]
        psp34: PSP34Data<EnumerableBalances>,
        #[PSP34MetadataStorageField]
        metadata: PSP34MetadataData,
        #[OwnableStorageField]
        ownable: OwnableData,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        total_supply: u64,
        last_phase_id: u8,
        whitelist_count: u64,
        phase_whitelists_link: Mapping<(AccountId, u8), Whitelist>,
        phases: Mapping<u8, Phase>,
        phase_account_link: EnumerablePhaseAccountMapping,
        phase_account_last_index: Mapping<u8, u64>,
        limit_phase_count: u8,
        launchpad_contract_address: AccountId,
        project_info: Vec<u8>,
        public_minted_count: u64,
        total_public_minting_amount: u64,
        public_minting_fee: Balance,
        public_minting_phase_id: u8
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
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
        InvalidMintAmount,
        PhaseNotExist,
        PhaseExpired,
        PhaseNotPublicSale,
        PhaseCodeNotExist,
        PhaseExisted,
        PhaseLimitReached,
        WhitelistNotExist,
        PhasePublicSale,
        PhaseWhiteList
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
    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;
    impl PSP34 for LaunchPadPsp34NftStandard {}
    impl PSP34Metadata for LaunchPadPsp34NftStandard {}
    impl PSP34Internal for LaunchPadPsp34NftStandard {}
    impl PSP34Enumerable for LaunchPadPsp34NftStandard {}

    #[openbrush::wrapper]
    pub type ArtZeroLaunchPadPSP34Ref = dyn CrossArtZeroLaunchPadPSP34;

    #[openbrush::trait_definition]
    pub trait CrossArtZeroLaunchPadPSP34 {
        #[ink(message)]
        fn get_project_mint_fee_rate(&self) -> u32;
    }

    #[openbrush::trait_definition]
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
        pub fn new(
            launchpad_contract_address: AccountId,
            limit_phase_count: u8, 
            contract_owner: AccountId, 
            total_supply: u64,
            project_info: String,
            code_phases: Vec<String>,
            is_public_phases: Vec<bool>,
            public_minting_fee_phases: Vec<Balance>,
            public_minting_amout_phases: Vec<u64>,
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance.launchpad_contract_address = launchpad_contract_address;
                instance.total_supply = total_supply;
                instance.last_phase_id = 0;
                instance.project_info = project_info.into_bytes();
                instance.limit_phase_count = limit_phase_count;
                instance.public_minted_count = 0;

                if code_phases.len() == start_time_phases.len() &&
                    code_phases.len() == is_public_phases.len() &&
                    code_phases.len() == public_minting_fee_phases.len() &&
                    code_phases.len() == public_minting_amout_phases.len() &&
                    code_phases.len() == end_time_phases.len() && 
                    code_phases.len() as u8 <= limit_phase_count {
                    let phase_length = code_phases.len();
                    for i in 0..phase_length {
                        instance.add_new_phase(
                            code_phases[i].clone(),
                            is_public_phases[i].clone(),
                            public_minting_fee_phases[i].clone(),
                            public_minting_amout_phases[i].clone(),
                            start_time_phases[i].clone(), 
                            end_time_phases[i].clone()
                        );
                    }
                }
            })
        }

        ///Add new phare - Only Owner
        #[ink(message)]
        pub fn add_new_phase(
            &mut self, 
            phase_code: String,
            is_public: bool,
            public_minting_fee: Balance,
            public_minting_amout: u64,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            if self.last_phase_id == self.limit_phase_count {
                return Err(Error::PhaseLimitReached);
            }
            if self.validate_phase_schedule(start_time, end_time) == true {
                let byte_phase_code = phase_code.into_bytes();
                self.last_phase_id += 1;
                let phase = Phase {
                    title: byte_phase_code,
                    is_public: is_public,
                    public_minting_fee: public_minting_fee,
                    public_minting_amout: public_minting_amout,
                    whitelist_amount: 0,
                    claimed_amount: 0,
                    start_time: start_time, 
                    end_time: end_time                   
                };
                self.phases.insert(&self.last_phase_id, &phase);
                Ok(())
            } else {
                return Err(Error::InvalidInput);
            }
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

        /// Update whitelist - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_whitelist(
            &mut self,
            account: AccountId,
            phase_id: u8,
            whitelist_amount: u64,
            whitelist_price: Balance
        ) -> Result<(), Error> {
            if  self.phase_whitelists_link.get(&(account, phase_id)).is_none() {
                return Err(Error::WhitelistNotExist);
            }
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            let mut whitelist = self.phase_whitelists_link.get(&(account, phase_id)).unwrap();
            let old_whitelist_amount = whitelist.whitelist_amount;
            //Whitelist amount must less than total supply or greater than zero
            if  whitelist_amount > self.total_supply ||
                whitelist_amount == 0 || 
                whitelist_amount <= whitelist.claimed_amount {
                return Err(Error::InvalidInput);
            }

            whitelist.whitelist_amount = whitelist_amount;
            whitelist.minting_fee = whitelist_price;
            self.phase_whitelists_link.insert(&(account, phase_id), &whitelist);
            let mut phase = self.phases.get(&phase_id).unwrap();
            let whitelist_amount_tmp = phase.whitelist_amount.checked_sub(old_whitelist_amount).unwrap().checked_add(whitelist_amount).unwrap();
            phase.whitelist_amount = whitelist_amount_tmp;
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        /// Add new whitelist - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn add_whitelist(
            &mut self,
            account: AccountId,
            phase_id: u8,
            whitelist_amount: u64,
            whitelist_price: Balance
        ) -> Result<(), Error> {
            //Whitelist amount must less than total supply or greater than zero
            if  whitelist_amount > self.total_supply ||
                whitelist_amount == 0 {
                return Err(Error::InvalidInput);
            }
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            if  self.phase_whitelists_link.get(&(account, phase_id)).is_some() {
                return Err(Error::InvalidInput);
            }

            self.whitelist_count += 1;

            let whitelist = Whitelist {
                whitelist_amount: whitelist_amount,
                claimed_amount: 0,
                minting_fee: whitelist_price
            };
            
            let mut phase_account_last_index_tmp = 1;
            if self.phase_account_last_index.get(&phase_id).is_some() {
                phase_account_last_index_tmp = self.phase_account_last_index.get(&phase_id).unwrap() + 1;
            }
            self.phase_account_last_index.insert(&phase_id, &phase_account_last_index_tmp);
            self.phase_account_link.insert(&account, &phase_id, &phase_account_last_index_tmp);

            self.phase_whitelists_link.insert(&(account, phase_id), &whitelist);
            let mut phase = self.phases.get(&phase_id).unwrap();
            let whitelist_amount_tmp = phase.whitelist_amount.checked_add(whitelist_amount).unwrap();
            phase.whitelist_amount = whitelist_amount_tmp;
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn public_mint(&mut self, phase_id: u8) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist)
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            let current_time = Self::env().block_timestamp();
            if phase.start_time >= current_time && phase.end_time <= current_time {
                return Err(Error::PhaseExpired);
            }
            if phase.is_public == false {
                return Err(Error::PhaseNotPublicSale)
            }
            if self.last_token_id >= self.total_supply || phase.claimed_amount >= phase.public_minting_amout {
                return Err(Error::TokenLimitReached)
            }
            if phase.public_minting_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee)
            }
            let claimed_amount_tmp = phase.claimed_amount.checked_add(1).unwrap();
            phase.claimed_amount = claimed_amount_tmp;
            self.phases.insert(&phase_id, &phase);
            self.last_token_id += 1;
            self.public_minted_count += 1;
            assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            Ok(())
        }

        /// Whitelisted User eates multiple
        #[ink(message)]
        #[ink(payable)]
        pub fn whitelist_mint(&mut self, phase_id: u8, mint_amount: u64) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist)
            }
            let phase = self.phases.get(&phase_id).unwrap();
            if phase.is_public == true {
                return Err(Error::PhasePublicSale)
            }
            let current_time = Self::env().block_timestamp();
            if phase.start_time >= current_time && phase.end_time <= current_time {
                return Err(Error::PhaseExpired);
            }
            if self.last_token_id >= self.total_supply {
                return Err(Error::TokenLimitReached);
            }
            let caller = self.env().caller();
            
            if self.phase_whitelists_link.get(&(caller, phase_id)).is_none(){
                return Err(Error::InvalidInput);
            }

            let mut caller_info = self.phase_whitelists_link.get(&(caller, phase_id)).unwrap();
            if caller_info.whitelist_amount <= caller_info.claimed_amount {
                return Err(Error::ClaimedAll);
            }

            if caller_info.whitelist_amount < caller_info.claimed_amount.checked_add(mint_amount).unwrap()  {
                return Err(Error::InvalidInput);
            }
            
            if  caller_info.minting_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            let project_mint_fee_rate = ArtZeroLaunchPadPSP34Ref::get_project_mint_fee_rate(
                &self.launchpad_contract_address
            );
            // Send minting fee to launchpad contract
            let project_mint_fee = caller_info.minting_fee
                .checked_mul(project_mint_fee_rate as u128)
                .unwrap()
                .checked_div(10000)
                .unwrap();
            if project_mint_fee > 0 {
                assert!(self
                    .env()
                    .transfer(self.launchpad_contract_address, project_mint_fee)
                    .is_ok());
            }
            caller_info.claimed_amount = caller_info.claimed_amount.checked_add(mint_amount).unwrap();
            self.phase_whitelists_link.insert(&(caller, phase_id), &caller_info);

            for _i in 0..mint_amount {
                self.last_token_id += 1;
                assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            let claimed_amount_tmp = phase.claimed_amount.checked_add(mint_amount).unwrap();
            phase.claimed_amount = claimed_amount_tmp;
            self.phases.insert(&phase_id, &phase);
            Ok(())
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

        /// Update public phase
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_public_phase(
            &mut self, 
            phase_id: u8,
            is_public: bool
        ) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            if phase.whitelist_amount > 0 {
                return Err(Error::PhaseWhiteList);
            }
            phase.is_public = is_public.clone();
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        /// Update phase schedule
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_schedule_phase(
            &mut self, 
            phase_id: u8,
            phase_code: String,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            if start_time >= end_time || phase_id > self.last_phase_id {
                return Err(Error::InvalidInput);
            }
            for index in 0..self.last_phase_id {
                if index != phase_id {
                    let phase = self.phases.get(&(index+1)).unwrap();
                    if phase.start_time >= start_time && phase.end_time <= start_time {
                        return Err(Error::InvalidInput);
                    }
                    if phase.start_time >= end_time && phase.end_time <= end_time {
                        return Err(Error::InvalidInput);
                    }
                }
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            phase.title = phase_code.clone().into_bytes();
            phase.start_time = start_time.clone();
            phase.end_time = end_time.clone();
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        // Update schedule phases
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_schedule_phases(
            &mut self, 
            id_phases: Vec<u8>,
            code_phases: Vec<String>,
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Result<(), Error> {
            if id_phases.len() != code_phases.len() ||
                id_phases.len() != start_time_phases.len() || 
                start_time_phases.len() != end_time_phases.len() {
                return Err(Error::InvalidInput);
            }
            let phase_length = id_phases.len();
            for index in 0..phase_length {
                self.update_schedule_phase(
                    id_phases[index + 1].clone(),
                    code_phases[index + 1].clone(),
                    start_time_phases[index + 1].clone(),
                    end_time_phases[index + 1].clone()
                );
            }
            Ok(())
        }

        /// Update limit phase count
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_limit_phase_count(&mut self, limit_phase_count: u8) -> Result<(), Error> {
            self.limit_phase_count = limit_phase_count;
            Ok(())
        }

        /// Update public minting phase
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_public_minting_phase_id(&mut self, phase_id: u8) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist)
            }
            let phase = self.phases.get(&phase_id).unwrap();
            if phase.whitelist_amount > 0 {
                return Err(Error::InvalidInput)
            }
            self.public_minting_phase_id = phase_id;
            Ok(())
        }

        /// Update public minting fee
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_public_minting_fee(&mut self, public_minting_fee: Balance) -> Result<(), Error> {
            self.public_minting_fee = public_minting_fee;
            Ok(())
        }

        /// Edit project information
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn edit_project_information(
            &mut self,
            project_info: String
        ) -> Result<(), Error> {
            self.project_info = project_info.into_bytes();
            Ok(())
        }

        fn validate_phase_schedule(&self, start_time: Timestamp, end_time: Timestamp) -> bool {
            if start_time >= end_time {
                return false;
            }
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.start_time >= start_time && phase.end_time <= start_time {
                    return false;
                }
                if phase.start_time >= end_time && phase.end_time <= end_time {
                    return false;
                }
            }
            return true;
        }
        
        /// Get limit phase count
        #[ink(message)]
        pub fn get_limit_phase_count(
            &self
        ) -> u8 {
            return self.limit_phase_count;
        }

        /// Get public minting phase id
        #[ink(message)]
        pub fn get_public_minting_phase_id(
            &self
        ) -> u8 {
            return self.public_minting_phase_id;
        }

        /// Get total public minting amount
        #[ink(message)]
        pub fn get_total_public_minting_amount(
            &self
        ) -> u64 {
            return self.total_public_minting_amount;
        }

        /// Get public minted count
        #[ink(message)]
        pub fn get_public_minted_count(
            &self
        ) -> u64 {
            return self.public_minted_count;
        }

        /// Get limit phase count
        #[ink(message)]
        pub fn get_project_info(
            &self
        ) -> Vec<u8> {
            return self.project_info.clone();
        }

        /// Get Phase Schedule by Phase Id
        #[ink(message)]
        pub fn get_phase_schedule_by_id(
            &self,
            phase_id: u8
        ) -> Option<Phase> {
            if self.phases.get(&phase_id).is_none() {
                return None;
            }
            return Some(self.phases.get(&phase_id).unwrap());
        }

        /// Get whitelist information by phase code
        #[ink(message)]
        pub fn get_whitelist_by_account_id(
            &self,
            account: AccountId,
            phase_id: u8
        ) -> Option<Whitelist> {
            if self.phase_whitelists_link.get(&(account, phase_id)).is_none() {
                return None;
            }
            return Some(self.phase_whitelists_link.get(&(account, phase_id)).unwrap());
        }

        /// Get Public Minting Fee
        #[ink(message)]
        pub fn get_public_minting_fee(
            &self
        ) -> Balance {
            return self.public_minting_fee;
        }

        /// Get phase Account Link by phase Id
        #[ink(message)]
        pub fn get_phase_account_link_by_phase_id( 
            &self,
            phase_id: u8, 
            account_index: u64
        ) -> AccountId {
            return self.phase_account_link.get_by_phase_id(&phase_id, &account_index).unwrap();
        }

        /// Get phase Account Link by Account
        #[ink(message)]
        pub fn get_phase_account_link_by_account( 
            &self,
            account: AccountId, 
            account_index: u64
        ) -> u8 {
            return self.phase_account_link.get_by_account(&account, &account_index).unwrap();
        }

        /// Get current phase
        #[ink(message)]
        pub fn get_current_phase(&self) -> Option<u8> {
            let current_time = Self::env().block_timestamp();
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.start_time <= current_time && phase.end_time >= current_time {
                    return Some(index + 1);
                }
            }
            return None;
        }

        /// Check time in a phase
        #[ink(message)]
        pub fn is_in_schedule_phase(&self, time: Timestamp) -> Option<u8> {
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.start_time <= time && phase.end_time >= time {
                    return Some(index);
                }
            }
            return None;
        }

        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_whitelist_count(
            &self
        ) -> u64 {
            return self.whitelist_count;
        }

        ///Get phase Count 
        #[ink(message)]
        pub fn get_last_phase_id(&self) -> u8 {
            return self.last_phase_id;
        }  

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        ///Get Phase Account Last Index by Phase Id
        #[ink(message)]
        pub fn get_phase_account_last_index(&self, phase_id: u8) -> Option<u64> {
            if self.phase_account_last_index.get(&phase_id).is_none() {
                return None;
            }
            return Some(self.phase_account_last_index.get(&phase_id).unwrap());
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
        #[ink(message)]
        pub fn get_total_supply(&self) -> u64 {
            return self.total_supply;
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
