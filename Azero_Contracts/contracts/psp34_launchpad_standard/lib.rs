#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::launchpad_psp34_nft_standard::{
    LaunchPadPsp34NftStandard,
    LaunchPadPsp34NftStandardRef,
};

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod launchpad_psp34_nft_standard {
    use ink::prelude::{
        string::{
            String,
        },
        vec::Vec,
    };
    use ink::codegen::{
        EmitEvent,
        Env,
    };
    use openbrush::{
        contracts::access_control::*,
        contracts::ownable::*,
        contracts::psp34::extensions::{
            enumerable::*,
            metadata::*,
            burnable::*,
        },
        storage::{
            MultiMapping,
            ValueGuard,
            Mapping,
            TypeGuard
        },
        traits::{
            Storage,
            DefaultEnv,
            ZERO_ADDRESS
        },
        modifiers,
    };
    use artzero_project::{
        traits::{
            psp34_standard::*,
            launchpad_manager::ArtZeroLaunchPadRef,
            admin::*,
            error::Error,
        }
    };

    #[cfg(feature = "std")]
    use ink::storage::traits::StorageLayout;

    #[derive(
        Copy,
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        Default,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
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
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
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

    pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

    #[derive(Debug)]
    #[openbrush::upgradeable_storage(STORAGE_KEY)]
    pub struct Manager {
        pub total_supply: u64,
        pub last_phase_id: u8,
        pub whitelist_count: u32,
        pub phase_account_public_claimed_amount: Mapping<(AccountId, u8), u64, PhaseAccountPublicClaimedAmountKeys>,
        pub phase_whitelists_link: Mapping<(AccountId, u8), Whitelist, PhaseWhitelistsLinkKeys>,
        pub phases: Mapping<u8, Phase>,
        pub phase_account_link: MultiMapping<u8, AccountId, ValueGuard<u8>>,
        pub limit_phase_count: u8,
        pub launchpad_contract_address: AccountId,
        pub project_info: Vec<u8>,
        pub public_minted_count: u64,
        pub active_phase_count: u8,
        pub available_token_amount: u64,
        pub owner_claimed_amount: u64,
        pub _reserved: Option<()>,
    }

    impl Default for Manager {
        fn default() -> Self {
            Self {
                total_supply: Default::default(),
                last_phase_id: Default::default(),
                whitelist_count: Default::default(),
                phase_account_public_claimed_amount: Default::default(),
                phase_whitelists_link: Default::default(),
                phases: Default::default(),
                phase_account_link: Default::default(),
                limit_phase_count: Default::default(),
                launchpad_contract_address: ZERO_ADDRESS.into(),
                project_info: Default::default(),
                public_minted_count: Default::default(),
                active_phase_count: Default::default(),
                available_token_amount: Default::default(),
                owner_claimed_amount: Default::default(),
                _reserved: Default::default(),
            }
        }
    }

    pub struct PhaseAccountPublicClaimedAmountKeys;

    impl<'a> TypeGuard<'a> for PhaseAccountPublicClaimedAmountKeys {
        type Type = &'a (&'a AccountId, &'a u8);
    }

    pub struct PhaseWhitelistsLinkKeys;

    impl<'a> TypeGuard<'a> for PhaseWhitelistsLinkKeys {
        type Type = &'a (&'a AccountId, &'a u8);
    }

    #[derive(Default, Storage)]
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
        #[storage_field]
        manager: Manager,
        #[storage_field]
        manager_psp34_standard: artzero_project::impls::psp34_standard::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
    }

    /// Event emitted when withdrawing fee
    #[ink(event)]
    pub struct WithdrawFeeEvent {
        #[ink(topic)]
        value: Balance,
        #[ink(topic)]
        receiver: AccountId
    }

    #[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(StorageLayout, scale_info::TypeInfo))]
    pub enum MintingMode {
        Public         = 0,
        Whitelist     = 1
    }

    /// Event emitted when minting
    #[ink(event)]
    pub struct LaunchpadMintingEvent {
        mode: MintingMode,
        minter: AccountId,
        phase_id: u8,
        mint_amount: u64,
        minting_fee: Balance,
        project_mint_fee: Balance
    }

    const ADMINER: RoleType = ink::selector_id!("ADMINER");

    impl Ownable for LaunchPadPsp34NftStandard {}
    impl PSP34 for LaunchPadPsp34NftStandard {}
    impl PSP34Metadata for LaunchPadPsp34NftStandard {}
    impl PSP34Enumerable for LaunchPadPsp34NftStandard {}
    impl Psp34Traits for LaunchPadPsp34NftStandard {}
    impl AccessControl for LaunchPadPsp34NftStandard {}

    impl AdminTrait for LaunchPadPsp34NftStandard {
        fn _emit_withdraw_fee(&self, _value: Balance, _receiver: AccountId) {
            self.env().emit_event(WithdrawFeeEvent {
                value: _value, 
                receiver: _receiver
            });
        }
    }

    impl PSP34Burnable for LaunchPadPsp34NftStandard {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();
            
            if let Some(token_owner) = self.owner_of(id.clone()) {
                if token_owner != account {
                    return Err(PSP34Error::Custom(String::from("not token owner").into_bytes()))
                }
    
                let allowance = self.allowance(account,caller,Some(id.clone()));
    
                if caller == account || allowance {
                    self._burn_from(account, id)
                } else{
                    Err(PSP34Error::Custom(String::from("caller is not token owner or approved").into_bytes()))
                }
            } else {
                return Err(PSP34Error::Custom(String::from("Cannot find token owner").into_bytes()));
            }            
        }
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
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_owner(contract_owner);
            access_control::Internal::_init_with_admin(&mut instance, caller);
            access_control::Internal::_init_with_admin(&mut instance, contract_owner);
            instance.grant_role(ADMINER, contract_owner).expect("Should grant the role");
            instance.manager.launchpad_contract_address = launchpad_contract_address;
            instance.manager.total_supply = total_supply;
            instance.manager.last_phase_id = 0;
            instance.manager.active_phase_count = 0;
            instance.manager.project_info = project_info.into_bytes();
            instance.manager.limit_phase_count = limit_phase_count;
            instance.manager.public_minted_count = 0;
            instance.manager.owner_claimed_amount = 0;
            instance.manager.available_token_amount = total_supply;
            if !code_phases.is_empty() &&
                code_phases.len() == start_time_phases.len() &&
                code_phases.len() == is_public_phases.len() &&
                code_phases.len() == public_minting_fee_phases.len() &&
                code_phases.len() == public_minting_amount_phases.len() &&
                code_phases.len() == public_max_minting_amount_phases.len() &&
                code_phases.len() == end_time_phases.len() &&
                code_phases.len() as u8 <= limit_phase_count {
                let phase_length = code_phases.len();
                for i in 0..phase_length {
                    assert!(instance.add_new_phase(
                        code_phases[i].clone(),
                        is_public_phases[i],
                        public_minting_fee_phases[i],
                        public_minting_amount_phases[i],
                        public_max_minting_amount_phases[i],
                        start_time_phases[i],
                        end_time_phases[i]
                    ).is_ok());
                }
            }
            instance
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
            if !self.has_role(ADMINER, self.env().caller()) && self.env().caller() != self.manager.launchpad_contract_address {
                return Err(Error::InvalidCaller);
            }
            if let Some(active_phase_count) = self.manager.active_phase_count.checked_add(1) {
                if active_phase_count > self.manager.limit_phase_count {
                    return Err(Error::InvalidPhaseCount);
                }
                if !self.validate_phase_schedule(&start_time, &end_time){
                    return Err(Error::InvalidStartTimeAndEndTime);
                }
                let byte_phase_code = phase_code.into_bytes();
                if let Some(last_phase_id) = self.manager.last_phase_id.checked_add(1) {
                    self.manager.last_phase_id = last_phase_id;

                    if let Some(active_phase_count) = self.manager.active_phase_count.checked_add(1) {
                        self.manager.active_phase_count = active_phase_count;
                        if is_public {
                            if let Some(available_token_amount) = self.manager.available_token_amount.checked_sub(public_minting_amount) {
                                self.manager.available_token_amount = available_token_amount;
                            } else {
                                return Err(Error::Custom(String::from("Cannot subtract available token amount")));
                            }                            
                        }
                        let phase = if is_public {
                            Phase {
                                is_active: true,
                                title: byte_phase_code,
                                is_public,
                                public_minting_fee,
                                public_minting_amount,
                                public_max_minting_amount,
                                public_claimed_amount: 0,
                                whitelist_amount: 0,
                                claimed_amount: 0,
                                total_amount: public_minting_amount,
                                start_time,
                                end_time
                            }
                        } else {
                            Phase {
                                is_active: true,
                                title: byte_phase_code,
                                is_public,
                                public_minting_fee: 0,
                                public_minting_amount: 0,
                                public_max_minting_amount: 0,
                                public_claimed_amount: 0,
                                whitelist_amount: 0,
                                claimed_amount: 0,
                                total_amount: 0,
                                start_time,
                                end_time
                            }
                        };
                        self.manager.phases.insert(&self.manager.last_phase_id, &phase);
                        Ok(())
                    } else {
                        return Err(Error::Custom(String::from("Cannot add active phase count")));
                    }                    
                } else {
                    return Err(Error::Custom(String::from("Cannot add last phase id")));
                }
            } else {
                return Err(Error::Custom(String::from("Cannot add active phase count")));
            }            
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
            if let Some(mut whitelist) = self.manager.phase_whitelists_link.get(&(&account, &phase_id)) {
                if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                    let old_whitelist_amount = whitelist.whitelist_amount;
                    
                    if let Some(value_added_old_whitelist_amount) = self.manager.available_token_amount.checked_add(old_whitelist_amount) {
                        if let Some(available_token_amount) = value_added_old_whitelist_amount.checked_sub(whitelist_amount) {
                            self.manager.available_token_amount = available_token_amount;
                            if !(whitelist.claimed_amount..=self.manager.total_supply).contains(&whitelist_amount) {
                                return Err(Error::InvalidInput);
                            }
                            whitelist.whitelist_amount = whitelist_amount;
                            whitelist.minting_fee = whitelist_price;
                            self.manager.phase_whitelists_link.insert(&(&account, &phase_id), &whitelist);
                            if let Some(value_subbed_old_whitelist_amount) = phase.whitelist_amount.checked_sub(old_whitelist_amount) {
                                if let Some(wl_amount) = value_subbed_old_whitelist_amount.checked_add(whitelist_amount) {
                                    phase.whitelist_amount = wl_amount;
                                    if let Some(total_amount_subbed_old_whitelist_amount) = phase.total_amount.checked_sub(old_whitelist_amount) {
                                        if let Some(total_amount) = total_amount_subbed_old_whitelist_amount.checked_add(whitelist_amount) {
                                            phase.total_amount = total_amount;
                                            self.manager.phases.insert(&phase_id, &phase);
                                            Ok(())
                                        } else {
                                            return Err(Error::Custom(String::from("Cannot add to phase total amount"))); 
                                        }                                
                                    } else {
                                        return Err(Error::Custom(String::from("Cannot subtract from phase total amount"))); 
                                    }                                    
                                } else {
                                    return Err(Error::Custom(String::from("Cannot add to phase whitelist amount"))); 
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot subtract from phase whitelist amount"))); 
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Cannot subtract from available token amount")));
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to available token amount"))); 
                    }                    
                } else {
                    return Err(Error::PhaseNotExist);
                }                
            } else {
                return Err(Error::WhitelistNotExist);
            }            
        }

        /// Add multi whitelists - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn add_multi_whitelists(
            &mut self,
            phase_id: u8,
            accounts: Vec<AccountId>,
            whitelist_amounts: Vec<u64>,
            whitelist_prices: Vec<Balance>
        ) -> Result<(), Error> {           
            if !accounts.is_empty() &&
                accounts.len() == whitelist_amounts.len() && 
                accounts.len() == whitelist_prices.len() {
                let whitelist_count = accounts.len();
                if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                    for i in 0..whitelist_count {
                        let whitelist_account = accounts[i].clone();
                        let whitelist_amount = whitelist_amounts[i].clone();
                        let whitelist_price = whitelist_prices[i].clone();
                        let whitelist = Whitelist {
                            whitelist_amount,
                            claimed_amount: 0,
                            minting_fee: whitelist_price
                        };
                        if let Some(available_token_amount) = self.manager.available_token_amount.checked_sub(whitelist_amount) {
                            self.manager.available_token_amount = available_token_amount;
                            self.manager.phase_account_link.insert(phase_id, &whitelist_account);
                            self.manager.phase_whitelists_link.insert(&(&whitelist_account, &phase_id), &whitelist);
                            if let Some(total_amount) = phase.total_amount.checked_add(whitelist_amount) {
                                phase.total_amount = total_amount;
                                if let Some(wl_amount) = phase.whitelist_amount.checked_add(whitelist_amount) {
                                    phase.whitelist_amount = wl_amount;
                                } else {
                                    return Err(Error::Custom(String::from("Cannot add to phase whitelist amount")));
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot add to phase token amount")));
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Cannot subtract from available token amount"))); 
                        }                        
                    }
                    self.manager.phases.insert(&phase_id, &phase);
                    if let Some(wl_count) = self.manager.whitelist_count.checked_add(whitelist_count as u32) {
                        self.manager.whitelist_count = wl_count;
                        Ok(())
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to whitelist count")));
                    }                    
                } else {
                    return Err(Error::PhaseNotExist);
                }                
            } else {
                return Err(Error::InvalidInput);
            }
        }

        /// Add new whitelist - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn clear_whitelist_phase(
            &mut self,
            phase_id: u8,
            accounts: Vec<AccountId>,
        ) -> Result<(), Error> {
            if let Some(phase) = self.manager.phases.get(&phase_id) {
                let current_time = self.env().block_timestamp();
                if current_time > phase.end_time {
                    if !accounts.is_empty() {
                        for i in 0..accounts.len() {
                            let remove_account = accounts[i].clone();
                            if self.manager.phase_whitelists_link.get(&(&remove_account, &phase_id)).is_some() &&  
                            self.manager.phase_account_link.get_index(phase_id, &remove_account).is_some() {
                                self.manager.phase_whitelists_link.remove(&(&remove_account, &phase_id));
                                self.manager.phase_account_link.remove_value(phase_id, &remove_account);
                            } else {
                                return Err(Error::InvalidInput);
                            }
                        }
                        if let Some(whitelist_count) = self.manager.whitelist_count.checked_sub(accounts.len() as u32) {
                            self.manager.whitelist_count = whitelist_count;
                            Ok(())
                        } else {
                            return Err(Error::Custom(String::from("Cannot subtract from whitelist count")));
                        }                        
                    } else {
                        return Err(Error::InvalidInput);
                    }
                } else {
                    return Err(Error::InvalidInput);
                }
            } else {
                return Err(Error::PhaseNotExist);
            }              
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
            if self.manager.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            if  self.manager.phase_whitelists_link.get(&(&account, &phase_id)).is_some() {
                return Err(Error::InvalidInput);
            }
            if let Some(whitelist_count) = self.manager.whitelist_count.checked_add(1) {
                self.manager.whitelist_count = whitelist_count;
                let whitelist = Whitelist {
                    whitelist_amount,
                    claimed_amount: 0,
                    minting_fee: whitelist_price
                };
                if let Some(available_token_amount) = self.manager.available_token_amount.checked_sub(whitelist_amount) {
                    self.manager.available_token_amount = available_token_amount;
                    self.manager.phase_account_link.insert(phase_id, &account);
                    self.manager.phase_whitelists_link.insert(&(&account, &phase_id), &whitelist);
                    if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                        if let Some(total_amount) = phase.total_amount.checked_add(whitelist_amount) {
                            phase.total_amount = total_amount;
                            if let Some(wl_amount) = phase.whitelist_amount.checked_add(whitelist_amount) {
                                phase.whitelist_amount = wl_amount;
                                self.manager.phases.insert(&phase_id, &phase);
                                Ok(())
                            } else {
                                return Err(Error::Custom(String::from("Cannot add to phase whitelist amount")));
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Cannot add to phase total amount")));
                        }                        
                    } else {
                        return Err(Error::PhaseNotExist);
                    }                    
                } else {
                    return Err(Error::Custom(String::from("Cannot subtract from available token amount")));
                }                
            } else {
                return Err(Error::Custom(String::from("Cannot add to whitelist count")));
            }            
        }

        ///Only Owner can mint new token
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self, mint_amount: u64 ) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager_psp34_standard.last_token_id.checked_add(mint_amount) {
                if last_token_id > self.manager.total_supply {
                    return Err(Error::InvalidInput);
                }
                let current_time = self.env().block_timestamp();
                let mut available_amount: u64 = 0;
                for index in 0..self.manager.last_phase_id {
                    if let Some(phase) = self.manager.phases.get(&(index+1)) {
                        if phase.is_active && current_time > phase.end_time {
                            if let Some(total_amount) = phase.total_amount.checked_sub(phase.claimed_amount) {
                                if let Some (aval_amount) = available_amount.checked_add(total_amount) {
                                    available_amount = aval_amount;
                                } else {
                                    return Err(Error::Custom(String::from("Cannot add to available amount")));
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot subtract from phase total amount")));
                            }                            
                        }
                    } else {
                        return Err(Error::PhaseNotExist);        
                    }                    
                }
                if let Some(owner_claimed_amount) = available_amount.checked_sub(self.manager.owner_claimed_amount) {
                    if mint_amount > owner_claimed_amount {
                        return Err(Error::InvalidInput);
                    }
                    for _i in 0..mint_amount {
                        if let Some(last_tkn_id) = self.manager_psp34_standard.last_token_id.checked_add(1) {
                            self.manager_psp34_standard.last_token_id = last_tkn_id;
                            if self._mint_to(caller, Id::U64(self.manager_psp34_standard.last_token_id)).is_err() {
                                return Err(Error::CannotMint);
                            }
                        } else {
                            return Err(Error::Custom(String::from("Cannot add to last token id")));
                        }                        
                    }
                    if let Some(owner_claimed_amnt) = self.manager.owner_claimed_amount.checked_add(mint_amount) {
                        self.manager.owner_claimed_amount = owner_claimed_amnt;
                        Ok(())
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to owner claimed amount")));
                    }                    
                } else {
                    return Err(Error::Custom(String::from("Cannot subtract from owner claimed amount")));
                }                
            } else {
                return Err(Error::Custom(String::from("Cannot add to last token id")));
            }            
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn public_mint(&mut self, phase_id: u8, mint_amount: u64) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                if !phase.is_active {
                    return Err(Error::PhaseDeactivate);
                }
                if mint_amount > phase.public_max_minting_amount {
                    return Err(Error::InvalidInput);
                }
                let current_time = self.env().block_timestamp();
                if (phase.start_time..=phase.end_time).contains(&current_time) {
                    if !phase.is_public {
                        return Err(Error::NotPublicMint);
                    }
                    if let Some(last_token_id) = self.manager_psp34_standard.last_token_id.checked_add(mint_amount) {
                        if last_token_id > self.manager.total_supply {
                            return Err(Error::InvalidInput);
                        }
                        if let Some(public_claimed_amount) = phase.public_claimed_amount.checked_add(mint_amount) {
                            if public_claimed_amount > phase.public_minting_amount {
                                return Err(Error::InvalidInput);
                            }
                            if let Some(claimed_amount) = phase.claimed_amount.checked_add(mint_amount) {
                                if claimed_amount > phase.total_amount {
                                    return Err(Error::InvalidInput);
                                }
                                if let Some(public_minting_fee) = phase.public_minting_fee.checked_mul(mint_amount as u128) {
                                    if public_minting_fee != self.env().transferred_value() {
                                        return Err(Error::InvalidInput);
                                    }
                                    let project_mint_fee_rate = ArtZeroLaunchPadRef::get_project_mint_fee_rate(
                                        &self.manager.launchpad_contract_address
                                    );
                                    if let Some(mul_project_mint_fee_rate) = phase.public_minting_fee.checked_mul(project_mint_fee_rate as u128) {
                                        if let Some(mul_mint_amount) = mul_project_mint_fee_rate.checked_mul(mint_amount as u128) {
                                            if let Some(project_mint_fee) = mul_mint_amount.checked_div(10000) {
                                                if project_mint_fee > 0 {
                                                    if project_mint_fee > self.env().balance() {
                                                        return Err(Error::NotEnoughBalance);
                                                    }
                                                    if self
                                                        .env()
                                                        .transfer(self.manager.launchpad_contract_address, project_mint_fee)
                                                        .is_err(){
                                                            return Err(Error::CannotTransfer);
                                                        }
                                                }
                                
                                                for _i in 0..mint_amount {
                                                    if let Some(last_tkn_id) = self.manager_psp34_standard.last_token_id.checked_add(1) {
                                                        self.manager_psp34_standard.last_token_id = last_tkn_id;
                                                        if let Some(public_minted_count) = self.manager.public_minted_count.checked_add(1) {
                                                            self.manager.public_minted_count = public_minted_count;
                                                            if self._mint_to(caller, Id::U64(self.manager_psp34_standard.last_token_id)).is_err() {
                                                                return Err(Error::CannotMint);
                                                            }
                                                        } else {
                                                            return Err(Error::Custom(String::from("Cannot add to public minted count"))); 
                                                        }                                            
                                                    } else {
                                                        return Err(Error::Custom(String::from("Cannot add to last token id"))); 
                                                    }                                        
                                                }
                                
                                                if self.manager.phase_account_public_claimed_amount.get(&(&caller, &phase_id)).is_none() {
                                                    self.manager.phase_account_public_claimed_amount.insert(&(&caller, &phase_id), &mint_amount);
                                                } else {
                                                    if let Some(phase_account_public_old_claimed_amount) = self.manager.phase_account_public_claimed_amount.get(&(&caller, &phase_id)) {
                                                        if let Some(public_claimed_amount_added_mint_amount) = phase_account_public_old_claimed_amount.checked_add(mint_amount) {
                                                            self.manager.phase_account_public_claimed_amount.insert(&(&caller, &phase_id), &public_claimed_amount_added_mint_amount);
                                                        } else {
                                                            return Err(Error::Custom(String::from("Cannot add to phase account public old claimed amount"))); 
                                                        }                                            
                                                    } else {
                                                        return Err(Error::Custom(String::from("Phase account public claimed amount does not exist"))); 
                                                    }                            
                                                }
                                
                                                if let Some(public_claimed_amnt) = phase.public_claimed_amount.checked_add(mint_amount) {
                                                    phase.public_claimed_amount = public_claimed_amnt;
                                                    if let Some(claimed_amnt) = phase.claimed_amount.checked_add(mint_amount) {
                                                        phase.claimed_amount = claimed_amnt;
                                                        self.manager.phases.insert(&phase_id, &phase);
                                                        self.env().emit_event(LaunchpadMintingEvent {
                                                            mode: MintingMode::Public,
                                                            minter: caller,
                                                            phase_id,
                                                            mint_amount,
                                                            minting_fee: self.env().transferred_value(),
                                                            project_mint_fee
                                                        });
                                                        Ok(())
                                                    } else {
                                                        return Err(Error::Custom(String::from("Cannot add to phase claimed amount")));
                                                    }                                        
                                                } else {
                                                    return Err(Error::Custom(String::from("Cannot add to phase public claimed amount"))); 
                                                }
                                            } else {
                                                return Err(Error::Custom(String::from("Cannot div with 10000")));
                                            }                         
                                        } else {
                                            return Err(Error::Custom(String::from("Cannot mul with mint amount")));
                                        }                                        
                                    } else {
                                        return Err(Error::Custom(String::from("Cannot mul with project mint fee rate")));
                                    }                                                                        
                                } else {
                                    return Err(Error::Custom(String::from("Cannot add to phase public minting fee"))); 
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot add to phase claimed amount"))); 
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Cannot add to phase public claimed amount"))); 
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to last token id"))); 
                    }                    
                } else {
                    Err(Error::PhaseExpired)
                }
            } else {
                return Err(Error::PhaseNotExist)
            }            
        }

        /// Whitelisted User eates multiple
        #[ink(message)]
        #[ink(payable)]
        pub fn whitelist_mint(&mut self, phase_id: u8, mint_amount: u64) -> Result<(), Error> {
            if let Some(phase) = self.manager.phases.get(&phase_id) {
                if !phase.is_active {
                    return Err(Error::PhaseDeactivate);
                }
                let current_time = self.env().block_timestamp();
                if (phase.start_time..=phase.end_time).contains(&current_time) {
                    if let Some(claimed_amount) = phase.claimed_amount.checked_add(mint_amount) {
                        if claimed_amount > phase.total_amount {
                            return Err(Error::InvalidInput);
                        }
                        if self.manager_psp34_standard.last_token_id >= self.manager.total_supply {
                            return Err(Error::MaxSupply);
                        }
                        let caller = self.env().caller();
                        if self.manager.phase_whitelists_link.get(&(&caller, &phase_id)).is_none(){
                            return Err(Error::WhitelistNotExist);
                        }
                        if let Some(mut caller_info) = self.manager.phase_whitelists_link.get(&(&caller, &phase_id)) {
                            if let Some(caller_info_claimed_amount) = caller_info.claimed_amount.checked_add(mint_amount) {
                                if caller_info.whitelist_amount < caller_info_claimed_amount {
                                    return Err(Error::InvalidInput);
                                }
                                if let Some(minting_fee) = caller_info.minting_fee.checked_mul(mint_amount as u128) {
                                    if minting_fee != self.env().transferred_value() {
                                        return Err(Error::InvalidFee);
                                    }
                                    let project_mint_fee_rate = ArtZeroLaunchPadRef::get_project_mint_fee_rate(
                                        &self.manager.launchpad_contract_address
                                    );
                                    // Send minting fee to launchpad contract
                                    if let Some(mul_project_mint_fee_rate) = caller_info.minting_fee.checked_mul(project_mint_fee_rate as u128) {
                                        if let Some(mul_mint_amount) = mul_project_mint_fee_rate.checked_mul(mint_amount as u128) {
                                            if let Some(project_mint_fee) = mul_mint_amount.checked_div(10000) {
                                                if project_mint_fee > 0 {
                                                    if project_mint_fee > self.env().balance() {
                                                        return Err(Error::NotEnoughBalance);
                                                    }
                                                    if self
                                                        .env()
                                                        .transfer(self.manager.launchpad_contract_address, project_mint_fee)
                                                        .is_err() {
                                                            return Err(Error::CannotTransfer);
                                                        }
                                                }
                                                if let Some(caller_info_claimed_amount) = caller_info.claimed_amount.checked_add(mint_amount) {
                                                    caller_info.claimed_amount = caller_info_claimed_amount;
                                                    self.manager.phase_whitelists_link.insert(&(&caller, &phase_id), &caller_info);
                                    
                                                    for _i in 0..mint_amount {
                                                        if let Some(last_token_id) = self.manager_psp34_standard.last_token_id.checked_add(1) {
                                                            self.manager_psp34_standard.last_token_id = last_token_id;
                                                            if self._mint_to(caller, Id::U64(self.manager_psp34_standard.last_token_id)).is_err() {
                                                                return Err(Error::CannotMint);
                                                            }
                                                        } else {
                                                            return Err(Error::Custom(String::from("Cannot add to last token id"))); 
                                                        }                                            
                                                    }
                                                    if let Some(mut phs) = self.manager.phases.get(&phase_id) {
                                                        if let Some(claimed_amnt) = phs.claimed_amount.checked_add(mint_amount) {
                                                            phs.claimed_amount = claimed_amnt;
                                                            self.manager.phases.insert(&phase_id, &phs);
                                                            self.env().emit_event(LaunchpadMintingEvent {
                                                                mode: MintingMode::Whitelist,
                                                                minter: caller,
                                                                phase_id,
                                                                mint_amount,
                                                                minting_fee: self.env().transferred_value(),
                                                                project_mint_fee
                                                            });
                                                            Ok(())
                                                        } else {
                                                            return Err(Error::Custom(String::from("Cannot add to phase claimed amount"))); 
                                                        }                                        
                                                    } else {
                                                        return Err(Error::PhaseNotExist); 
                                                    } 
                                                } else {
                                                    return Err(Error::Custom(String::from("Cannot add to caller info claimed amount")));  
                                                }
                                            } else {
                                                return Err(Error::Custom(String::from("Cannot div with 10000")));
                                            }                                      
                                        } else {
                                            return Err(Error::Custom(String::from("Cannot mul with mint amount"))); 
                                        }                                            
                                    } else {
                                        return Err(Error::Custom(String::from("Cannot mul with project mint fee rate"))); 
                                    }                                                                                                     
                                } else {
                                    return Err(Error::Custom(String::from("Cannot mul with caller info minting fee"))); 
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot add to caller info claimed amount"))); 
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Caller info does not exist"))); 
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to phase claimed amount"))); 
                    }                    
                } else {
                    Err(Error::PhaseExpired)
                }
            } else {
                return Err(Error::PhaseNotExist);
            }          
        }

        /// Deactive Phase - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn deactive_phase(
            &mut self,
            phase_id: u8
        ) -> Result<(), Error> {
            if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                if phase.claimed_amount != 0 || self.manager.phase_account_link.count(phase_id) != 0 || phase.start_time <= self.env().block_timestamp() || !phase.is_active{
                    return Err(Error::Custom(String::from("Cannot deactivate phase")))
                }
                phase.is_active = false;
                if let Some(active_phase_count) = self.manager.active_phase_count.checked_sub(1) {
                    self.manager.active_phase_count = active_phase_count;
                    if phase.is_public {
                        if let Some(available_token_amount) = self.manager.available_token_amount.checked_add(phase.public_minting_amount) {
                            self.manager.available_token_amount = available_token_amount;
                        } else {
                            return Err(Error::Custom(String::from("Cannot add to available token amount"))); 
                        }                        
                    }
                    self.manager.phases.insert(&phase_id, &phase);
                    Ok(())
                } else {
                    return Err(Error::Custom(String::from("Cannot subtract from active phase count"))); 
                }                
            } else {
                return Err(Error::PhaseNotExist);
            }            
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
            if self.manager.phases.get(&phase_id).is_none() {
                return Err(Error::PhaseNotExist);
            }
            if public_max_minting_amount > ArtZeroLaunchPadRef::get_public_max_minting_amount(&self.manager.launchpad_contract_address) {
                return Err(Error::InvalidInput);
            }
            if start_time >= end_time {
                return Err(Error::InvalidStartTimeAndEndTime);
            }
            for index in 0..self.manager.last_phase_id {
                if let Some(idx) = index.checked_add(1) {
                    if idx != phase_id {
                        if let Some(phs) = self.manager.phases.get(&(index+1)) {
                            if phs.is_active && (
                                (phs.start_time..=phs.end_time).contains(&start_time) || (phs.start_time..=phs.end_time).contains(&end_time)
                            ) {
                                return Err(Error::InvalidInput);
                            }
                        } else {
                            return Err(Error::PhaseNotExist); 
                        }                        
                    }
                } else {
                    return Err(Error::Custom(String::from("Cannot add to index"))); 
                }                
            }
            if let Some(mut phase) = self.manager.phases.get(&phase_id) {
                if !phase.is_active ||
                phase.claimed_amount != 0 ||
                self.manager.phase_account_link.count(phase_id) != 0 || phase.start_time <= self.env().block_timestamp() {
                    return Err(Error::Custom(String::from("cannot update phase")))
                }

                phase.title = phase_code.clone().into_bytes();
                if phase.is_public && !is_public {
                    if let Some(available_token_amount) = self.manager.available_token_amount.checked_add(phase.public_minting_amount) {
                        self.manager.available_token_amount = available_token_amount;
                        if let Some(total_amount) = phase.total_amount.checked_sub(phase.public_minting_amount) {
                            phase.total_amount = total_amount;
                        } else {
                            return Err(Error::Custom(String::from("Cannot subtract from phase total amount")));
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to available token amount"))); 
                    }                    
                } else if !phase.is_public && is_public {
                    if let Some(available_token_amount) = self.manager.available_token_amount.checked_sub(public_minting_amount) {
                        self.manager.available_token_amount = available_token_amount;
                        if let Some(total_amount) = phase.total_amount.checked_add(public_minting_amount) {
                            phase.total_amount = total_amount;
                        } else {
                            return Err(Error::Custom(String::from("Cannot add to phase total amount"))); 
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot subtract from available token amount")));
                    }                    
                } else if phase.is_public && is_public {
                    if let Some(available_token_amount_added_public_minting_amount) = self.manager.available_token_amount.checked_add(phase.public_minting_amount) {
                        if let Some(available_token_amount) = available_token_amount_added_public_minting_amount.checked_sub(public_minting_amount) {
                            self.manager.available_token_amount = available_token_amount;
                            if let Some(total_amount_subbed_public_minting_amount) = phase.total_amount.checked_sub(phase.public_minting_amount) {
                                if let Some(total_amount) = total_amount_subbed_public_minting_amount.checked_add(public_minting_amount) {
                                    phase.total_amount = total_amount;
                                } else {
                                    return Err(Error::Custom(String::from("Cannot add to phase total amount")));
                                }                                
                            } else {
                                return Err(Error::Custom(String::from("Cannot subtract from phase total amount")));  
                            }                            
                        } else {
                            return Err(Error::Custom(String::from("Cannot subtract from available token amount")));  
                        }                        
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to available token amount")));  
                    }                    
                }
                phase.is_public = is_public;
                phase.start_time = start_time;
                phase.end_time = end_time;

                if is_public {
                    phase.public_minting_fee = public_minting_fee;
                    phase.public_minting_amount = public_minting_amount;
                    phase.public_max_minting_amount = public_max_minting_amount;
                } else {
                    phase.public_minting_fee = 0;
                    phase.public_minting_amount = 0;
                    phase.public_max_minting_amount = 0;
                }
                self.manager.phases.insert(&phase_id, &phase);
                Ok(())
            } else {
                return Err(Error::PhaseNotExist);
            }            
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
                if self.update_schedule_phase(
                    id_phases[index + 1],
                    code_phases[index + 1].clone(),
                    is_public_phases[index + 1],
                    public_minting_fee_phases[index + 1],
                    public_minting_amount_phases[index +1],
                    public_max_minting_amount_phases[index + 1],
                    start_time_phases[index + 1],
                    end_time_phases[index + 1]
                ).is_err(){
                    return Err(Error::UpdatePhase);
                }

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
            self.manager.project_info = project_info.into_bytes();
            Ok(())
        }

        fn validate_phase_schedule(&self, start_time: &Timestamp, end_time: &Timestamp) -> bool {
            if *start_time >= *end_time {
                return false;
            }
            for index in 0..self.manager.last_phase_id {
                if let Some(phase) = self.manager.phases.get(&(index+1)) {
                    if phase.is_active && (
                        (phase.start_time..=phase.end_time).contains(start_time) || (phase.start_time..=phase.end_time).contains(end_time)
                    ) {
                        return false;
                    }
                }             
            }
            true
        }

        /// Get owner claimed amount
        #[ink(message)]
        pub fn get_owner_claimed_amount(
            &self
        ) -> u64 {
            self.manager.owner_claimed_amount
        }

        /// Get owner available amount
        #[ink(message)]
        pub fn get_owner_available_amount(
            &self
        ) -> u64 {
            let current_time = self.env().block_timestamp();
            let mut available_amount: u64 = 0;
            for index in 0..self.manager.last_phase_id {
                if let Some(phase) = self.manager.phases.get(&(index+1)) {
                    if phase.is_active && current_time > phase.end_time {
                        if let Some(total_amount) = phase.total_amount.checked_sub(phase.claimed_amount) {
                            if let Some(avail_amount) = available_amount.checked_add(total_amount) {
                                available_amount = avail_amount;
                            }                            
                        }                        
                    }
                }                
            }
            available_amount
        }

        /// Get limit phase count
        #[ink(message)]
        pub fn get_limit_phase_count(
            &self
        ) -> u8 {
            self.manager.limit_phase_count
        }

        /// Get public minted count
        #[ink(message)]
        pub fn get_public_minted_count(
            &self
        ) -> u64 {
            self.manager.public_minted_count
        }

        /// Get limit phase count
        #[ink(message)]
        pub fn get_project_info(
            &self
        ) -> Vec<u8> {
            self.manager.project_info.clone()
        }

        /// Get Phase Schedule by Phase Id
        #[ink(message)]
        pub fn get_phase_schedule_by_id(
            &self,
            phase_id: u8
        ) -> Option<Phase> {
            self.manager.phases.get(&phase_id)
        }

        /// Get whitelist information by phase code
        #[ink(message)]
        pub fn get_whitelist_by_account_id(
            &self,
            account: AccountId,
            phase_id: u8
        ) -> Option<Whitelist> {
            self.manager.phase_whitelists_link.get(&(&account, &phase_id))
        }

        /// Get phase Account Link
        #[ink(message)]
        pub fn get_phase_account_link(
            &self,
            phase_id: u8,
            account_index: u64
        ) -> Option<AccountId> {
            self.manager.phase_account_link.get_value(phase_id, &(account_index as u128))
        }

        /// Get current phase
        #[ink(message)]
        pub fn get_current_phase(&self) -> Option<u8> {
            let current_time = self.env().block_timestamp();
            for index in 0..self.manager.last_phase_id {
                if let Some(phase) = self.manager.phases.get(&(index+1)) {
                    if phase.is_active && (phase.start_time..=phase.end_time).contains(&current_time) {
                        return Some(index + 1);
                    }
                }                
            }
            None
        }

        /// Check time in a phase
        #[ink(message)]
        pub fn is_in_schedule_phase(&self, time: Timestamp) -> Option<u8> {
            for index in 0..self.manager.last_phase_id {
                if let Some(phase) = self.manager.phases.get(&(index+1)) {
                    if phase.start_time <= time && phase.end_time >= time {
                        return Some(index);
                    }
                }                
            }
            None
        }

        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_whitelist_count(
            &self
        ) -> u32 {
            self.manager.whitelist_count
        }

        ///Get phase Count
        #[ink(message)]
        pub fn get_last_phase_id(&self) -> u8 {
            self.manager.last_phase_id
        }

        ///Get active phase count
        #[ink(message)]
        pub fn get_active_phase_count(&self) -> u8 {
            self.manager.active_phase_count
        }

        ///Get Token Count
        #[ink(message)]
        pub fn get_last_token_id(&self) -> u64 {
            self.manager_psp34_standard.last_token_id
        }

        ///Get Phase Account Public Claimed Amount
        #[ink(message)]
        pub fn get_phase_account_public_claimed_amount(&self, account_id: AccountId, phase_id: u8) -> Option<u64> {
            Some(self.manager.phase_account_public_claimed_amount.get(&(&account_id, &phase_id)))?
        }

        ///Get Phase Account Last Index by Phase Id
        #[ink(message)]
        pub fn get_phase_account_last_index(&self, phase_id: u8) -> u64 {
            self.manager.phase_account_link.count(phase_id) as u64
        }

        ///Get Total Supply
        #[ink(message)]
        pub fn get_total_supply(&self) -> u64 {
            self.manager.total_supply
        }

        ///Get Available Token Amount
        #[ink(message)]
        pub fn get_available_token_amount(&self) -> u64 {
            self.manager.available_token_amount
        }
    }

}
