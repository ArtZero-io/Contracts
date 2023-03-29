#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#![allow(clippy::let_unit_value)]
#![allow(clippy::inline_fn_without_body)]
#![allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod artzero_launchpad_psp34 {
    use ink::ToAccountId;
    use ink::prelude::{
        string::{
            String,
        },
        vec,
        vec::Vec,
    };
    use openbrush::{
        contracts::{
            access_control::{
                extensions::enumerable
            },
        },
        contracts::access_control::extensions::enumerable::*,
        contracts::ownable::*,
        modifiers,
        traits::{
            Storage,
            DefaultEnv
        }
    };
    use artzero_project::{
        impls::launchpad_manager::*,
        traits::{
            admin::*,
            upgradable::*,
            error::Error,
        }
    };
    use launchpad_psp34_nft_standard::launchpad_psp34_nft_standard::LaunchPadPsp34NftStandardRef;
    use ink::{codegen::EmitEvent, reflect::ContractEventBase};

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct ArtZeroLaunchPadPSP34 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access_control: access_control::Data<enumerable::Members>,
        #[storage_field]
        manager: artzero_project::impls::launchpad_manager::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
    }
    // ADMINER RoleType = 3739740293
    const ADMINER: RoleType = ink::selector_id!("ADMINER");

    impl AccessControl for ArtZeroLaunchPadPSP34 {}
    impl Ownable for ArtZeroLaunchPadPSP34 {}
    impl ArtZeroLaunchPadTrait for ArtZeroLaunchPadPSP34 {}
    impl AccessControlEnumerable for ArtZeroLaunchPadPSP34 {}
    impl AdminTrait for ArtZeroLaunchPadPSP34 {}
    impl UpgradableTrait for ArtZeroLaunchPadPSP34 {}
    
    #[ink(event)]
    pub struct AddNewProjectEvent {
        project_id: u64,
        nft_contract_address: Option<AccountId>
    }

    pub type Event = <ArtZeroLaunchPadPSP34 as ContractEventBase>::Type;

    impl ArtZeroLaunchPadPSP34 {
        #[ink(constructor)]
        pub fn new(
            max_phases_per_project: u8,
            admin_address: AccountId,
            standard_nft_hash: Hash,
            project_adding_fee: Balance,
            project_mint_fee_rate: u32, //1% = 100
            public_max_minting_amount: u64
        ) -> Self {
            assert!(project_mint_fee_rate < 10000);
            assert!(project_adding_fee > 0);
            let mut instance = Self::default();
            let caller = <Self as DefaultEnv>::env().caller();
            instance._init_with_owner(caller);
            instance.initialize(
                max_phases_per_project,
                standard_nft_hash,
                project_adding_fee,
                project_mint_fee_rate,
                public_max_minting_amount,
                admin_address
            ).ok().unwrap();
            instance
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            max_phases_per_project: u8,
            standard_nft_hash: Hash,
            project_adding_fee: Balance,
            project_mint_fee_rate: u32,
            public_max_minting_amount: u64,
            admin_address: AccountId
        ) -> Result<(), Error> {
            //Project Adding Fee will not be set to Zero to prevent spamming the contract
            if self.manager.project_adding_fee > 0 {
                return Err(Error::AlreadyInit);
            }
            self.manager.standard_nft_hash = standard_nft_hash;
            self.manager.project_count = 0;
            self.manager.active_project_count = 0;
            self.manager.max_phases_per_project = max_phases_per_project;
            self.manager.project_adding_fee = project_adding_fee;
            self.manager.project_mint_fee_rate = project_mint_fee_rate;
            self.manager.public_max_minting_amount = public_max_minting_amount;
            self._init_with_admin(self.env().caller());
            self.grant_role(ADMINER, admin_address).expect("Should grant the role");
            Ok(())
        }

        /* EXECUTE FUNCTION */

        fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        /// Add new project
        #[ink(message)]
        #[ink(payable)]
        pub fn add_new_project(
            &mut self,
            total_supply: u64,
            start_time: Timestamp,
            end_time: Timestamp,
            project_info: String,
            code_phases: Vec<String>,
            is_public_phases: Vec<bool>,
            public_minting_fee_phases: Vec<Balance>,
            public_minting_amount_phases: Vec<u64>,
            public_max_minting_amount_phases: Vec<u64>,
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Result<(), Error> {
            if start_time >= end_time || self.manager.project_adding_fee != self.env().transferred_value() {
                return Err(Error::InvalidInput);
            }
            let project_owner = self.env().caller();
            let contract = LaunchPadPsp34NftStandardRef::new(
                <ArtZeroLaunchPadPSP34 as DefaultEnv>::env().account_id(),
                self.manager.max_phases_per_project,
                project_owner,
                total_supply,
                project_info,
                code_phases,
                is_public_phases,
                public_minting_fee_phases,
                public_minting_amount_phases,
                public_max_minting_amount_phases,
                start_time_phases,
                end_time_phases
            ).endowment(0)
                .code_hash(self.manager.standard_nft_hash)
                .salt_bytes(self.manager.project_count.to_le_bytes())
                .instantiate();
            let contract_account:AccountId = contract.to_account_id();
            if let Some(project_count) = self.manager.project_count.checked_add(1) {
                self.manager.project_count = project_count;
            } else {
                return Err(Error::CheckedOperations);
            }
            self.manager.projects_by_id.insert(&self.manager.project_count, &contract_account);
            let projects_by_owner = self.manager.projects_by_owner.get(&project_owner);
            if let Some(mut projects) = projects_by_owner {
                projects.push(contract_account);
                self.manager.projects_by_owner.insert(&project_owner, &projects);
            } else {
                let projects = vec![contract_account];
                self.manager.projects_by_owner.insert(&project_owner, &projects);
            }

            let new_project = Project {
                is_active: false,
                project_owner,
                total_supply,
                start_time,
                end_time
            };
            self.manager.projects.insert(&contract_account, &new_project);
            Self::emit_event(self.env(), Event::AddNewProjectEvent(AddNewProjectEvent {
                project_id: self.manager.project_count,
                nft_contract_address: Some(contract_account),
            }));
            Ok(())
        }
    }
}
