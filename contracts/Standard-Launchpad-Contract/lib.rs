#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::launchpad_psp34_nft_standard::{
    LaunchPadPsp34NftStandard,
    LaunchPadPsp34NftStandardRef,
};

#[openbrush::contract]
pub mod launchpad_psp34_nft_standard {
    use ink_prelude::string::String;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };

    use ink_prelude::vec::Vec;
    use ink_prelude::string::ToString;
    use openbrush::{
        contracts::access_control::*,
        contracts::ownable::*,
        contracts::psp34::extensions::{
            enumerable::*,
            metadata::*
        },
        storage::{
            MultiMapping,
            ValueGuard,
            Mapping,
            TypeGuard
        },
        traits::Storage,
        modifiers,
    };

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
        is_active: bool,
        title: Vec<u8>,
        is_public: bool,
        public_minting_fee: Balance,
        public_minting_amount: u64,
        public_max_minting_amount: u64,
        public_claimed_amount: u64,
        whitelist_amount: u64,
        claimed_amount: u64,
        total_amount: u64,
        start_time: Timestamp,
        end_time: Timestamp
    }

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct LaunchPadPsp34NftStandard{
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        admin_address: AccountId,
        last_token_id: u64,
        attribute_count: u32,
        attribute_names: Mapping<u32,Vec<u8>>,
        locked_tokens: Mapping<Id, u8>,
        locked_token_count: u64,
        total_supply: u64,
        last_phase_id: u8,
        whitelist_count: u64,
        phase_account_public_claimed_amount: Mapping<(AccountId, u8), u64, PhaseAccountPublicClaimedAmountKeys>,
        phase_whitelists_link: Mapping<(AccountId, u8), Whitelist, PhaseWhitelistsLinkKeys>,
        phases: Mapping<u8, Phase>,
        phase_account_link: MultiMapping<u8, AccountId, ValueGuard<u8>>,
        limit_phase_count: u8,
        launchpad_contract_address: AccountId,
        project_info: Vec<u8>,
        public_minted_count: u64,
        active_phase_count: u8,
        available_token_amount: u64,
        owner_claimed_amount: u64
    }

    pub struct PhaseAccountPublicClaimedAmountKeys;

    impl<'a> TypeGuard<'a> for PhaseAccountPublicClaimedAmountKeys {
        type Type = &'a (&'a AccountId, &'a u8);
    }

    pub struct PhaseWhitelistsLinkKeys;

    impl<'a> TypeGuard<'a> for PhaseWhitelistsLinkKeys {
        type Type = &'a (&'a AccountId, &'a u8);
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        InvalidInput,
        ClaimedAll,
        TokenLimitReached,
        PhaseNotExist,
        PhaseExpired,
        WhitelistNotExist
    }

    impl From<AccessControlError> for Error {
        fn from(access: AccessControlError) -> Self {
            match access {
                AccessControlError::MissingRole => Error::Custom(String::from("AC::MissingRole")),
                AccessControlError::RoleRedundant => {
                    Error::Custom(String::from("AC::RoleRedundant"))
                }
                AccessControlError::InvalidCaller => {
                    Error::Custom(String::from("AC::InvalidCaller"))
                }
            }
        }
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    const ADMINER: RoleType = ink_lang::selector_id!("ADMINER");

    #[openbrush::wrapper]
    pub type Psp34Ref = dyn PSP34 + PSP34Metadata;
    impl PSP34 for LaunchPadPsp34NftStandard {}
    impl PSP34Metadata for LaunchPadPsp34NftStandard {}
    impl PSP34Enumerable for LaunchPadPsp34NftStandard {}
    impl AccessControl for LaunchPadPsp34NftStandard {}
    impl Ownable for LaunchPadPsp34NftStandard {}

    #[openbrush::wrapper]
    pub type ArtZeroLaunchPadPSP34Ref = dyn CrossArtZeroLaunchPadPSP34;

    #[openbrush::trait_definition]
    pub trait CrossArtZeroLaunchPadPSP34 {
        #[ink(message)]
        fn get_project_mint_fee_rate(&self) -> u32;

        #[ink(message)]
        fn get_public_max_minting_amount(&self) -> u64;
    }

    #[openbrush::trait_definition]
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
        #[ink(message)]
        fn get_owner(&self) -> AccountId ;
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
            public_minting_amount_phases: Vec<u64>,
            public_max_minting_amount_phases: Vec<u64>,
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance._init_with_owner(contract_owner);
                instance._init_with_admin(contract_owner);
                instance.grant_role(ADMINER, contract_owner).expect("Should grant the role");
                instance.launchpad_contract_address = launchpad_contract_address;
                instance.total_supply = total_supply;
                instance.last_phase_id = 0;
                instance.active_phase_count = 0;
                instance.project_info = project_info.into_bytes();
                instance.limit_phase_count = limit_phase_count;
                instance.public_minted_count = 0;
                instance.owner_claimed_amount = 0;
                instance.available_token_amount = total_supply;
                if code_phases.len() == start_time_phases.len() &&
                    code_phases.len() == is_public_phases.len() &&
                    code_phases.len() == public_minting_fee_phases.len() &&
                    code_phases.len() == public_minting_amount_phases.len() &&
                    code_phases.len() == public_max_minting_amount_phases.len() &&
                    code_phases.len() == end_time_phases.len() && 
                    code_phases.len() as u8 <= limit_phase_count {
                    let phase_length = code_phases.len();
                    for i in 0..phase_length {
                        instance.add_new_phase(
                            code_phases[i].clone(),
                            is_public_phases[i].clone(),
                            public_minting_fee_phases[i].clone(),
                            public_minting_amount_phases[i].clone(),
                            public_max_minting_amount_phases[i].clone(),
                            start_time_phases[i].clone(), 
                            end_time_phases[i].clone()
                        );
                    }
                }
            })
        }

        ///Add new phare
        #[ink(message)]
        pub fn add_new_phase(
            &mut self, 
            phase_code: String,
            is_public: bool,
            public_minting_fee: Balance,
            public_minting_amount: u64,
            public_max_minting_amount: u64,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            assert!(self.has_role(ADMINER, self.env().caller()) || self.env().caller() == self.launchpad_contract_address);
            assert!(self.active_phase_count.checked_add(1).unwrap() <= self.limit_phase_count);
            assert!(self.validate_phase_schedule(start_time, end_time));
            
            let byte_phase_code = phase_code.into_bytes();
            self.last_phase_id += 1;
            self.active_phase_count += 1;
            if is_public {
                self.available_token_amount -= public_minting_amount;
                assert!(self.available_token_amount >= 0);
            }
            let phase = if is_public {
                Phase {
                    is_active: true,
                    title: byte_phase_code,
                    is_public: is_public,
                    public_minting_fee: public_minting_fee,
                    public_minting_amount: public_minting_amount,
                    public_max_minting_amount: public_max_minting_amount,
                    public_claimed_amount: 0,
                    whitelist_amount: 0,
                    claimed_amount: 0,
                    total_amount: public_minting_amount,
                    start_time: start_time, 
                    end_time: end_time                   
                }
            } else {
                Phase {
                    is_active: true,
                    title: byte_phase_code,
                    is_public: is_public,
                    public_minting_fee: 0,
                    public_minting_amount: 0,
                    public_max_minting_amount: 0,
                    public_claimed_amount: 0,
                    whitelist_amount: 0,
                    claimed_amount: 0,
                    total_amount: 0,
                    start_time: start_time, 
                    end_time: end_time                   
                }
            };
            self.phases.insert(&self.last_phase_id, &phase);
            Ok(())
        }

        /// Update whitelist - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_whitelist(
            &mut self,
            account: AccountId,
            phase_id: u8,
            whitelist_amount: u64,
            whitelist_price: Balance
        ) -> Result<(), Error> {
            if  self.phase_whitelists_link.get(&(&account, &phase_id)).is_none() {
                return Err(Error::WhitelistNotExist);
            }
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            let mut whitelist = self.phase_whitelists_link.get(&(&account, &phase_id)).unwrap();
            let old_whitelist_amount = whitelist.whitelist_amount;
            self.available_token_amount = self.available_token_amount.checked_add(old_whitelist_amount).unwrap().checked_sub(whitelist_amount).unwrap();
            assert!(self.available_token_amount >= 0);
            assert!((whitelist.claimed_amount..=self.total_supply).contains(&whitelist_amount));
            whitelist.whitelist_amount = whitelist_amount;
            whitelist.minting_fee = whitelist_price;
            self.phase_whitelists_link.insert(&(&account, &phase_id), &whitelist);
            let mut phase = self.phases.get(&phase_id).unwrap();
            phase.whitelist_amount = phase.whitelist_amount.checked_sub(old_whitelist_amount).unwrap().checked_add(whitelist_amount).unwrap();
            phase.total_amount = phase.total_amount.checked_sub(old_whitelist_amount).unwrap().checked_add(whitelist_amount).unwrap();
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        /// Add new whitelist - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn add_whitelist(
            &mut self,
            account: AccountId,
            phase_id: u8,
            whitelist_amount: u64,
            whitelist_price: Balance
        ) -> Result<(), Error> {
            assert!((0..=self.total_supply).contains(&whitelist_amount));
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            if  self.phase_whitelists_link.get(&(&account, &phase_id)).is_some() {
                return Err(Error::InvalidInput);
            }
            self.whitelist_count += 1;
            let whitelist = Whitelist {
                whitelist_amount: whitelist_amount,
                claimed_amount: 0,
                minting_fee: whitelist_price
            };
            self.available_token_amount = self.available_token_amount.checked_sub(whitelist_amount).unwrap();
            assert!(self.available_token_amount >= 0);
            self.phase_account_link.insert(phase_id, &account);
            self.phase_whitelists_link.insert(&(&account, &phase_id), &whitelist);
            let mut phase = self.phases.get(&phase_id).unwrap();
            phase.total_amount = phase.total_amount.checked_add(whitelist_amount).unwrap();
            phase.whitelist_amount = phase.whitelist_amount.checked_add(whitelist_amount).unwrap();
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        ///Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self, mint_amount: u64 ) -> Result<(), Error> {
            let caller = self.env().caller();
            assert!(self.last_token_id.checked_add(mint_amount).unwrap() <= self.total_supply);
            let current_time = Self::env().block_timestamp();
            let mut available_amount = 0;
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.is_active && current_time > phase.end_time {
                    available_amount += phase.total_amount.checked_sub(phase.claimed_amount.clone()).unwrap();
                }
            }  
            assert!(mint_amount <= available_amount.checked_sub(self.owner_claimed_amount).unwrap());
            for _i in 0..mint_amount {
                self.last_token_id += 1;
                assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
            }
            self.owner_claimed_amount += mint_amount;
            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn public_mint(&mut self, phase_id: u8, mint_amount: u64) -> Result<(), Error> {
            let caller = self.env().caller();
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist)
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            assert!(mint_amount <= phase.public_max_minting_amount, "PhasePublicMintLimitReached");
            let current_time = Self::env().block_timestamp();
            if (phase.start_time..=phase.end_time).contains(&current_time) {
                assert!(phase.is_public);
                assert!(self.last_token_id.checked_add(mint_amount).unwrap() <= self.total_supply);
                assert!(phase.public_claimed_amount.checked_add(mint_amount).unwrap() <= phase.public_minting_amount);
                assert!(phase.claimed_amount.checked_add(mint_amount).unwrap() <= phase.total_amount);
                assert!(phase.public_minting_fee.checked_mul(mint_amount as u128).unwrap() == self.env().transferred_value());
                let project_mint_fee_rate = ArtZeroLaunchPadPSP34Ref::get_project_mint_fee_rate(
                    &self.launchpad_contract_address
                );
                let project_mint_fee = phase.public_minting_fee
                    .checked_mul(project_mint_fee_rate as u128)
                    .unwrap()
                    .checked_mul(mint_amount as u128)
                    .unwrap()
                    .checked_div(10000)
                    .unwrap();
                if project_mint_fee > 0 {
                    assert!(self
                        .env()
                        .transfer(self.launchpad_contract_address, project_mint_fee)
                        .is_ok());
                }
                
                for _i in 0..mint_amount {
                    self.last_token_id += 1;
                    self.public_minted_count += 1;
                    assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
                }
                
                if self.phase_account_public_claimed_amount.get(&(&caller, &phase_id)).is_none() {
                    self.phase_account_public_claimed_amount.insert(&(&caller, &phase_id), &mint_amount);
                } else {
                    let mut phase_account_public_old_claimed_amount = self.phase_account_public_claimed_amount.get(&(&caller, &phase_id)).unwrap();
                    self.phase_account_public_claimed_amount.insert(&(&caller, &phase_id), &phase_account_public_old_claimed_amount.checked_add(mint_amount).unwrap());
                }

                phase.public_claimed_amount = phase.public_claimed_amount.checked_add(mint_amount).unwrap();
                phase.claimed_amount = phase.claimed_amount.checked_add(mint_amount).unwrap();
                self.phases.insert(&phase_id, &phase);
                Ok(())
            } else {
                return Err(Error::PhaseExpired);
            }
        }

        /// Whitelisted User eates multiple
        #[ink(message)]
        #[ink(payable)]
        pub fn whitelist_mint(&mut self, phase_id: u8, mint_amount: u64) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist)
            }
            let phase = self.phases.get(&phase_id).unwrap();
            let current_time = Self::env().block_timestamp();
            if (phase.start_time..=phase.end_time).contains(&current_time) {
                assert!(phase.claimed_amount.checked_add(mint_amount).unwrap() <= phase.total_amount);
                assert!(self.last_token_id < self.total_supply);
                let caller = self.env().caller();
                if self.phase_whitelists_link.get(&(&caller, &phase_id)).is_none(){
                    return Err(Error::InvalidInput);
                }
                let mut caller_info = self.phase_whitelists_link.get(&(&caller, &phase_id)).unwrap();
                assert!(caller_info.whitelist_amount >= caller_info.claimed_amount.checked_add(mint_amount).unwrap());
                assert!(caller_info.minting_fee.checked_mul(mint_amount as u128).unwrap() == self.env().transferred_value());
                let project_mint_fee_rate = ArtZeroLaunchPadPSP34Ref::get_project_mint_fee_rate(
                    &self.launchpad_contract_address
                );
                // Send minting fee to launchpad contract
                let project_mint_fee = caller_info.minting_fee
                    .checked_mul(project_mint_fee_rate as u128)
                    .unwrap()
                    .checked_mul(mint_amount as u128)
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
                self.phase_whitelists_link.insert(&(&caller, &phase_id), &caller_info);

                for _i in 0..mint_amount {
                    self.last_token_id += 1;
                    assert!(self._mint_to(caller, Id::U64(self.last_token_id)).is_ok());
                }
                let mut phase = self.phases.get(&phase_id).unwrap();
                phase.claimed_amount = phase.claimed_amount.checked_add(mint_amount).unwrap();
                self.phases.insert(&phase_id, &phase);
                Ok(())
            } else {
                return Err(Error::PhaseExpired);
            }
            
        }

        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            assert!(value <= self.env().balance());
            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }

        /// Update admin address
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(&mut self, admin_address: AccountId) -> Result<(), Error> {
            self.admin_address = admin_address;
            Ok(())
        }

        /// Deactive Phase - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn deactive_phase(
            &mut self, 
            phase_id: u8
        ) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            assert!(phase.claimed_amount == 0 && self.phase_account_link.count(phase_id) == 0 && phase.start_time > Self::env().block_timestamp() && !phase.is_active);
            phase.is_active = false;
            self.active_phase_count -= 1;
            if phase.is_public {
                self.available_token_amount = self.available_token_amount.checked_add(phase.public_minting_amount).unwrap();
            }
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        /// Update phase schedule - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_schedule_phase(
            &mut self, 
            phase_id: u8,
            phase_code: String,
            is_public: bool,
            public_minting_fee: Balance,
            public_minting_amount: u64,
            public_max_minting_amount: u64,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            if self.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            assert!(
                public_max_minting_amount <= ArtZeroLaunchPadPSP34Ref::get_public_max_minting_amount(&self.launchpad_contract_address) || 
                start_time < end_time
            );
            for index in 0..self.last_phase_id {
                if index.checked_add(1).unwrap() != phase_id {
                    let phase = self.phases.get(&(index+1)).unwrap();
                    if phase.is_active && (
                        (phase.start_time..=phase.end_time).contains(&start_time) || (phase.start_time..=phase.end_time).contains(&end_time)
                    ) {
                        return Err(Error::InvalidInput);
                    }
                }
            }
            let mut phase = self.phases.get(&phase_id).unwrap();
            assert!(phase.is_active && 
                phase.claimed_amount == 0 && 
                self.phase_account_link.count(phase_id) == 0 && phase.start_time > Self::env().block_timestamp()
            );
            phase.title = phase_code.clone().into_bytes();
            if phase.is_public && !is_public {
                self.available_token_amount = self.available_token_amount.checked_add(phase.public_minting_amount.clone()).unwrap();
                phase.total_amount = phase.total_amount.checked_sub(phase.public_minting_amount.clone()).unwrap();
            } else if !phase.is_public && is_public {
                self.available_token_amount = self.available_token_amount.checked_sub(public_minting_amount.clone()).unwrap();
                phase.total_amount = phase.total_amount.checked_add(public_minting_amount.clone()).unwrap();
            } else if phase.is_public && is_public {
                self.available_token_amount = self.available_token_amount.checked_add(phase.public_minting_amount.clone()).unwrap().checked_sub(public_minting_amount.clone()).unwrap();
                phase.total_amount = phase.total_amount.checked_sub(phase.public_minting_amount.clone()).unwrap().checked_add(public_minting_amount.clone()).unwrap();
            }
            assert!(self.available_token_amount >= 0);
            phase.is_public = is_public.clone();
            phase.start_time = start_time.clone();
            phase.end_time = end_time.clone();
            
            if is_public {
                phase.public_minting_fee = public_minting_fee.clone();
                phase.public_minting_amount = public_minting_amount.clone();
                phase.public_max_minting_amount = public_max_minting_amount.clone();
            } else {
                phase.public_minting_fee = 0;
                phase.public_minting_amount = 0;
                phase.public_max_minting_amount = 0;
            }
            self.phases.insert(&phase_id, &phase);
            Ok(())
        }

        // Update schedule phases - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_schedule_phases(
            &mut self, 
            id_phases: Vec<u8>,
            code_phases: Vec<String>,
            is_public_phases: Vec<bool>,
            public_minting_fee_phases: Vec<Balance>,
            public_minting_amount_phases: Vec<u64>,
            public_max_minting_amount_phases: Vec<u64>,
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Result<(), Error> {
            if id_phases.len() != code_phases.len() ||
                id_phases.len() != is_public_phases.len() ||
                id_phases.len() != public_minting_fee_phases.len() ||
                id_phases.len() != public_minting_amount_phases.len() ||
                id_phases.len() != public_max_minting_amount_phases.len() ||
                id_phases.len() != start_time_phases.len() || 
                id_phases.len() != end_time_phases.len() {
                return Err(Error::InvalidInput);
            }
            let phase_length = id_phases.len();
            for index in 0..phase_length {
                self.update_schedule_phase(
                    id_phases[index + 1].clone(),
                    code_phases[index + 1].clone(),
                    is_public_phases[index + 1].clone(),
                    public_minting_fee_phases[index + 1].clone(),
                    public_minting_amount_phases[index +1].clone(),
                    public_max_minting_amount_phases[index + 1].clone(),
                    start_time_phases[index + 1].clone(),
                    end_time_phases[index + 1].clone()
                );
            }
            Ok(())
        }

        /// Edit project information  - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn edit_project_information(
            &mut self,
            project_info: String
        ) -> Result<(), AccessControlError> {
            self.project_info = project_info.into_bytes();
            Ok(())
        }

        fn validate_phase_schedule(&self, start_time: Timestamp, end_time: Timestamp) -> bool {
            if start_time >= end_time {
                return false;
            }
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.is_active && (
                    (phase.start_time..=phase.end_time).contains(&start_time) || (phase.start_time..=phase.end_time).contains(&end_time)
                ) {
                    return false;
                }
            }
            return true;
        }

        /// Get owner claimed amount
        #[ink(message)]
        pub fn get_owner_claimed_amount(
            &self
        ) -> u64 {
            return self.owner_claimed_amount;
        }

        /// Get owner available amount
        #[ink(message)]
        pub fn get_owner_available_amount(
            &self
        ) -> u64 {
            let current_time = Self::env().block_timestamp();
            let mut available_amount = 0;
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.is_active && current_time > phase.end_time {
                    available_amount += phase.total_amount.checked_sub(phase.claimed_amount.clone()).unwrap();
                }
            }
            return available_amount;
        }
        
        /// Get limit phase count
        #[ink(message)]
        pub fn get_limit_phase_count(
            &self
        ) -> u8 {
            return self.limit_phase_count;
        }

        /// Get admin address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.admin_address;
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
            return Some(self.phases.get(&phase_id))?;
        }

        /// Get whitelist information by phase code
        #[ink(message)]
        pub fn get_whitelist_by_account_id(
            &self,
            account: AccountId,
            phase_id: u8
        ) -> Option<Whitelist> {
            return Some(self.phase_whitelists_link.get(&(&account, &phase_id)))?;
        }

        /// Get phase Account Link
        #[ink(message)]
        pub fn get_phase_account_link( 
            &self,
            phase_id: u8, 
            account_index: u64
        ) -> AccountId {
            return self.phase_account_link.get_value(phase_id, &(account_index as u128)).unwrap();
        }

        /// Get current phase
        #[ink(message)]
        pub fn get_current_phase(&self) -> Option<u8> {
            let current_time = Self::env().block_timestamp();
            for index in 0..self.last_phase_id {
                let phase = self.phases.get(&(index+1)).unwrap();
                if phase.is_active && (phase.start_time..=phase.end_time).contains(&current_time) {
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

        ///Get active phase count
        #[ink(message)]
        pub fn get_active_phase_count(&self) -> u8 {
            return self.active_phase_count;
        } 

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            return self.last_token_id;
        }

        ///Get Phase Account Public Claimed Amount
        #[ink(message)]
        pub fn get_phase_account_public_claimed_amount(&self, account_id: AccountId, phase_id: u8) -> Option<u64> {
            return Some(self.phase_account_public_claimed_amount.get(&(&account_id, &phase_id)))?;
        }

        ///Get Phase Account Last Index by Phase Id
        #[ink(message)]
        pub fn get_phase_account_last_index(&self, phase_id: u8) -> u64 {
            return self.phase_account_link.count(phase_id) as u64;
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
        #[modifiers(only_owner)]
        pub fn lock(&mut self, token_id: Id) -> Result<(), Error> {
            assert!(self.env().caller() == self.owner_of(token_id.clone()).unwrap());
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

        ///Get Total Supply
        #[ink(message)]
        pub fn get_total_supply(&self) -> u64 {
            return self.total_supply;
        }

        ///Get Available Token Amount
        #[ink(message)]
        pub fn get_available_token_amount(&self) -> u64 {
            return self.available_token_amount;
        }
    }

    impl Psp34Traits for LaunchPadPsp34NftStandard {

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
                if let Some(value) = self.get_attribute(token_id.clone(),attribute.into_bytes()) {
                    ret.push(String::from_utf8(value).unwrap());
                } else {
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

        /// Get owner address
        #[ink(message)]
        fn get_owner(&self) -> AccountId {
            return self.owner()
        }

    }
}
