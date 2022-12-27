#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_launchpad_psp34 {
    use ink_lang::ToAccountId;
    use ink_prelude::{
        string::{
            String,
        },
        vec,
        vec::Vec,
    };
    use ink_storage::{
        traits::SpreadAllocate
    };
    use openbrush::{
        contracts::access_control::*,
        contracts::ownable::*,
        modifiers,
        traits::Storage,
    };
    use artzero_project::{
        impls::launchpad_manager::*,
        traits::{
            psp34_standard::{
                psp34traits_external::Psp34Traits
            },
            launchpad_manager::{
                *
            },
            admin::*,
            error::Error,
        }
    };
    use launchpad_psp34_nft_standard::launchpad_psp34_nft_standard::LaunchPadPsp34NftStandardRef;
    use artzero_project::traits::psp34_standard::*;

    #[derive(Default, SpreadAllocate, Storage)]
    #[ink(storage)]
    pub struct ArtZeroLaunchPadPSP34 {
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        access: access_control::Data,
        #[storage_field]
        manager: artzero_project::impls::launchpad_manager::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
    }
    // ADMINER RoleType = 3739740293
    const ADMINER: RoleType = ink_lang::selector_id!("ADMINER");

    impl AccessControl for ArtZeroLaunchPadPSP34 {}
    impl Ownable for ArtZeroLaunchPadPSP34 {}
    impl ArtZeroLaunchPadTrait for ArtZeroLaunchPadPSP34 {}
    impl ArtZeroAdminTrait for ArtZeroLaunchPadPSP34 {}

    #[ink(event)]
    pub struct AddNewProjectEvent {
        project_id: u64,
        nft_contract_address: Option<AccountId>
    }

    impl ArtZeroLaunchPadPSP34 {
        #[ink(constructor)]
        pub fn new(
            max_phases_per_project: u8,
            admin_address: AccountId,
            owner_address: AccountId,
            standard_nft_hash: Hash,
            project_adding_fee: Balance,
            project_mint_fee_rate: u32, //1% = 100
            public_max_minting_amount: u64
        ) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                assert!(project_mint_fee_rate < 10000);
                _instance._init_with_owner(owner_address);
                _instance._init_with_admin(_instance.env().caller());
                _instance.grant_role(ADMINER, admin_address).expect("Should grant the role");
                _instance.initialize(
                    max_phases_per_project,
                    admin_address,
                    standard_nft_hash,
                    project_adding_fee,
                    project_mint_fee_rate,
                    public_max_minting_amount
                ).ok().unwrap()
            })
        }

        #[ink(message)]
        pub fn initialize(
            &mut self,
            max_phases_per_project: u8,
            admin_address: AccountId,
            standard_nft_hash: Hash,
            project_adding_fee: Balance,
            project_mint_fee_rate: u32,
            public_max_minting_amount: u64
        ) -> Result<(), OwnableError> {
            self.manager.admin_address = admin_address;
            self.manager.standard_nft_hash = standard_nft_hash;
            self.manager.project_count = 0;
            self.manager.active_project_count = 0;
            self.manager.max_phases_per_project = max_phases_per_project;
            self.manager.project_adding_fee = project_adding_fee;
            self.manager.project_mint_fee_rate = project_mint_fee_rate;
            self.manager.public_max_minting_amount = public_max_minting_amount;
            Ok(())
        }

        /* EXECUTE FUNCTION */

        /// Add new project
        #[ink(message)]
        #[ink(payable)]
        pub fn add_new_project(
            &mut self,
            project_owner: AccountId,
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
            // assert!(start_time > Self::env().block_timestamp());
            assert!(start_time < end_time && self.manager.project_adding_fee == self.env().transferred_value(), "invalid fee");
            let contract = LaunchPadPsp34NftStandardRef::new(
                Self::env().account_id(),
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
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the NFT contract: {:?}",
                        error
                    )
                });
            let contract_account:AccountId = contract.to_account_id();
            self.manager.project_count += 1;
            self.manager.projects_by_id.insert(&self.manager.project_count, &contract_account);
            let projects_by_owner = self.manager.projects_by_owner.get(&project_owner);
            if projects_by_owner.is_none() {
                let mut projects = Vec::<AccountId>::new();
                projects.push(contract_account);
                self.manager.projects_by_owner.insert(&project_owner, &projects);
            } else {
                let mut projects = projects_by_owner.unwrap();
                projects.push(contract_account);
                self.manager.projects_by_owner.insert(&project_owner, &projects);
            }

            let new_project = Project {
                is_active: false,
                project_owner: project_owner,
                total_supply: total_supply,
                start_time: start_time,
                end_time: end_time
            };
            self.manager.projects.insert(&contract_account, &new_project);
            self.env().emit_event(AddNewProjectEvent {
                project_id: self.manager.project_count,
                nft_contract_address: Some(contract_account),
            });
            Ok(())
        }

        /// Edit a project - Only Admin Role can change
        #[ink(message)]
        pub fn edit_project(
            &mut self,
            contract_address: AccountId,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            if self.manager.projects.get(&contract_address).is_none(){
                return Err(Error::ProjectNotExist);
            }
            assert!(start_time < end_time);
            let mut project = self.manager.projects.get(&contract_address).unwrap();
            assert!(self.has_role(ADMINER, self.env().caller()) && project.end_time > Self::env().block_timestamp());
            project.end_time = end_time;
            project.start_time = start_time;
            self.manager.projects.insert(&contract_address, &project);
            Ok(())
        }

        /* END EXECUTE FUNCTION*/

        /* SETTERS */
        /// Update admin address - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(
            &mut self,
            admin_address: AccountId
        ) -> Result<(), Error>  {
            self.manager.admin_address = admin_address;
            Ok(())
        }

        /// Update project adding fee - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_project_adding_fee(
            &mut self,
            project_adding_fee: Balance
        ) -> Result<(), AccessControlError> {
            self.manager.project_adding_fee = project_adding_fee;
            Ok(())
        }

        /// Update public max minting amount - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_public_max_minting_amount(
            &mut self,
            public_max_minting_amount: u64
        ) -> Result<(), AccessControlError> {
            self.manager.public_max_minting_amount = public_max_minting_amount;
            Ok(())
        }

        /// Update project mint fee rate - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_project_mint_fee_rate(
            &mut self,
            project_mint_fee_rate: u32
        ) -> Result<(), AccessControlError>  {
            self.manager.project_mint_fee_rate = project_mint_fee_rate;
            Ok(())
        }

        /// Update standard nft hash - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_standard_nft_hash(
            &mut self,
            standard_nft_hash: Hash
        ) -> Result<(), Error> {
            self.manager.standard_nft_hash = standard_nft_hash;
            Ok(())
        }

        /// Update is active project - Only Admin Role can change
        #[ink(message)]
        #[modifiers(only_role(ADMINER))]
        pub fn update_is_active_project(
            &mut self,
            is_active: bool,
            contract_address: AccountId
        ) -> Result<(), AccessControlError>  {
            assert!(self.manager.projects.get(&contract_address).is_some());
            let mut project = self.manager.projects.get(&contract_address).unwrap();
            assert!(is_active != project.is_active);
            project.is_active = is_active;

            if is_active {
                self.manager.active_project_count = self.manager.active_project_count.checked_add(1).unwrap();
            } else {
                self.manager.active_project_count = self.manager.active_project_count.checked_sub(1).unwrap();
            }
            self.manager.projects.insert(&contract_address, &project);
            Ok(())
        }



        /* END SETTERS */

        /* GETTERS */

        /// Get Project Adding Fee
        #[ink(message)]
        pub fn get_project_adding_fee(
            &self
        ) -> Balance {
            return self.manager.project_adding_fee;
        }

        /// Get active project count
        #[ink(message)]
        pub fn get_active_project_count(
            &self
        ) -> u64 {
            return self.manager.active_project_count;
        }

        /// Get admin address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.manager.admin_address;
        }

        /// Get project count
        #[ink(message)]
        pub fn get_project_count(
            &self
        ) -> u64 {
            return self.manager.project_count;
        }

        /// Get standard nft hash
        #[ink(message)]
        pub fn get_standard_nft_hash(
            &self
        ) -> Hash {
            return self.manager.standard_nft_hash;
        }

        // Get project by id
        #[ink(message)]
        pub fn get_project_by_id(
            &self,
            id: u64
        ) -> Option<AccountId> {
            return self.manager.projects_by_id.get(&id);
        }

        /// Get projects by owner address
        #[ink(message)]
        pub fn get_projects_by_owner(
            &self,
            owner_address: AccountId
        ) -> Vec<AccountId> {
            return self.manager.projects_by_owner.get(&owner_address).unwrap();
        }

        /// Get project by NFT address
        #[ink(message)]
        pub fn get_project_by_nft_address(
            &self,
            nft_contract_address: AccountId
        ) -> Option<Project> {
            return Some(self.manager.projects.get(&nft_contract_address))?;
        }

        /* END GETTERS*/
    }

}
