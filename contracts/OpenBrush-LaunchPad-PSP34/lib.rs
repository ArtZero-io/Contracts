#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod artzero_launchpad_psp34 {
    use ink_prelude::string::String;
    use openbrush::contracts::ownable::*;
    use openbrush::modifiers;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;
    use ink_lang::ToAccountId;
    use launchpad_psp34_nft_standard::launchpad_psp34_nft_standard::LaunchPadPsp34NftStandardRef;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        OnlyOwner,
        OnlyAdmin,
        InvalidCaller,
        NotEnoughBalance,
        ProjectNotExist,
        ProjectOwnerAndAdmin,
        InvalidStartTimeAndEndTime
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
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
    pub struct Project {
        is_active: bool,
        project_type: u8, // 1 is Live Project, 2 is Ended Project
        project_owner: AccountId,
        total_supply: u64,
        start_time: Timestamp,
        end_time: Timestamp
    }

    
    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroLaunchPadPSP34 {
        #[OwnableStorageField]
        ownable: OwnableData,
        admin_address: AccountId,
        standard_nft_hash: Hash,
        project_count: u64,
        projects: Mapping<AccountId, Project>,
        projects_by_id: Mapping<u64, AccountId>,
        projects_by_owner: Mapping<AccountId, Vec<AccountId>>,
        active_project_count: u64,
        max_phases_per_project: u8,
        project_adding_fee: Balance,
        project_mint_fee_rate: u32
    }

    impl Ownable for ArtZeroLaunchPadPSP34 {}

    #[openbrush::trait_definition]
    pub trait CrossArtZeroLaunchPadPSP34 {
        #[ink(message)]
        fn get_project_mint_fee_rate(&self) -> u32;
    }

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
        ) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                assert!(project_mint_fee_rate < 10000);
                _instance._init_with_owner(owner_address);
                _instance.initialize(max_phases_per_project, admin_address, standard_nft_hash, project_adding_fee, project_mint_fee_rate).ok().unwrap()
            })
        }

        #[ink(message)]
        pub fn initialize(
            &mut self,
            max_phases_per_project: u8,
            admin_address: AccountId,
            standard_nft_hash: Hash,
            project_adding_fee: Balance,
            project_mint_fee_rate: u32
        ) -> Result<(), OwnableError> {
            self.admin_address = admin_address;
            self.standard_nft_hash = standard_nft_hash;
            self.project_count = 0;
            self.active_project_count = 0;
            self.max_phases_per_project = max_phases_per_project;
            self.project_adding_fee = project_adding_fee;
            self.project_mint_fee_rate = project_mint_fee_rate;
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
            start_time_phases: Vec<Timestamp>,
            end_time_phases: Vec<Timestamp>
        ) -> Result<u64, Error> {
            if start_time >= end_time || end_time <= Self::env().block_timestamp() {
                return Err(Error::InvalidStartTimeAndEndTime);
            }
            assert!(self.project_adding_fee == self.env().transferred_value(), "invalid fee");
            let (hash, _) =
                ink_env::random::<ink_env::DefaultEnvironment>(&project_info[..4].as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract = LaunchPadPsp34NftStandardRef::new(
                Self::env().account_id(), 
                self.max_phases_per_project,
                project_owner,
                total_supply,
                project_info,
                code_phases,
                start_time_phases, 
                end_time_phases
            ).endowment(0)
                .code_hash(self.standard_nft_hash)
                .salt_bytes(&hash[..4])
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the NFT contract: {:?}",
                        error
                    )
                });
            let contract_account:AccountId = contract.to_account_id();
            self.project_count += 1;
            self.projects_by_id.insert(&self.project_count, &contract_account);
            let projects_by_owner = self.projects_by_owner.get(&project_owner);
            if projects_by_owner.is_none() {
                let mut projects = Vec::<AccountId>::new();
                projects.push(contract_account);
                self.projects_by_owner.insert(&project_owner, &projects);
            } else {
                let mut projects = projects_by_owner.unwrap();
                projects.push(contract_account);
                self.projects_by_owner.insert(&project_owner, &projects);
            }

            let new_project = Project {
                is_active: false,
                project_type: 1, // 1 is Live Project, 2 is Ended Project
                project_owner: project_owner,
                total_supply: total_supply,
                start_time: start_time,
                end_time: end_time
            };
            self.projects.insert(&contract_account, &new_project);
            self.env().emit_event(AddNewProjectEvent {
                project_id: self.project_count,
                nft_contract_address: Some(contract_account),
            });
            Ok(self.project_count)
        }

        /// Edit a project - Only project owner and admin
        #[ink(message)]
        pub fn edit_project(
            &mut self,
            contract_address: AccountId,
            start_time: Timestamp,
            end_time: Timestamp
        ) -> Result<(), Error> {
            if start_time > end_time {
                return Err(Error::InvalidStartTimeAndEndTime);
            }
            if self.projects.get(&contract_address).is_none(){
                return Err(Error::ProjectNotExist);
            }            
            let mut project = self.projects.get(&contract_address).unwrap();
            if  project.project_owner == self.env().caller() ||
                self.admin_address == self.env().caller() {
                    assert!(project.project_type != 2);
                    if  project.end_time <= Self::env().block_timestamp() {
                        return Err(Error::InvalidStartTimeAndEndTime);
                    } else {
                        project.end_time = end_time;
                        project.start_time = start_time;
                        self.projects.insert(&contract_address, &project);
                    }
            } else {
                return Err(Error::InvalidCaller);
            }
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
            self.admin_address = admin_address;
            Ok(())
        }

        /// update project adding fee - Only Admin
        #[ink(message)]
        pub fn update_project_adding_fee(
            &mut self,
            project_adding_fee: Balance
        ) -> Result<(), Error> {
            if  self.env().caller() != self.admin_address {
                return Err(Error::OnlyAdmin);
            }
            
            self.project_adding_fee = project_adding_fee;
            Ok(())
        }

        /// Update project mint fee rate - Only Admin
        #[ink(message)]
        pub fn update_project_mint_fee_rate(
            &mut self,
            project_mint_fee_rate: u32
        ) -> Result<(), Error>  {
            if  self.env().caller() != self.admin_address {
                return Err(Error::OnlyAdmin);
            }

            self.project_mint_fee_rate = project_mint_fee_rate;
            Ok(())
        }
        
        /// Update standard nft hash - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_standard_nft_hash(
            &mut self,
            standard_nft_hash: Hash
        ) -> Result<(), Error> {
            self.standard_nft_hash = standard_nft_hash;
            Ok(())
        }

        /// Update is active project - Only Admin
        #[ink(message)]
        pub fn update_is_active_project(
            &mut self,
            is_active: bool,
            contract_address: AccountId
        ) -> Result<(), Error>  {
            if self.projects.get(&contract_address).is_none(){
                return Err(Error::ProjectNotExist);
            }

            if  self.env().caller() != self.admin_address {
                return Err(Error::OnlyAdmin);
            }

            let mut project = self.projects.get(&contract_address).unwrap();
            assert!(is_active != project.is_active);
            project.is_active = is_active;

            if is_active == true {
                self.active_project_count = self.active_project_count.checked_add(1).unwrap();
            } else {
                self.active_project_count = self.active_project_count.checked_sub(1).unwrap();
            }
            self.projects.insert(&contract_address, &project);
            Ok(())
        }

        /* END SETTERS */
        
        /* GETTERS */

        /// Get Project Adding Fee
        #[ink(message)]
        pub fn get_project_adding_fee(
            &self
        ) -> Balance {
            return self.project_adding_fee;
        }
        
        /// Get active project count
        #[ink(message)]
        pub fn get_active_project_count(
            &self
        ) -> u64 {
            return self.active_project_count;
        }

        /// Get admin address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.admin_address;
        }

        /// Get project count
        #[ink(message)]
        pub fn get_project_count(   
            &self
        ) -> u64 {
            return self.project_count;
        }

        /// Get standard nft hash
        #[ink(message)]
        pub fn get_standard_nft_hash(   
            &self
        ) -> Hash {
            return self.standard_nft_hash;
        }

        // Get project by id
        #[ink(message)]
        pub fn get_project_by_id(
            &self,
            id: u64
        ) -> Option<AccountId> {
            return self.projects_by_id.get(&id);
        }

        /// Get projects by owner address
        #[ink(message)]
        pub fn get_projects_by_owner(
            &self,
            owner_address: AccountId
        ) -> Vec<AccountId> {
            return self.projects_by_owner.get(&owner_address).unwrap();
        }

        /// Get project by NFT address
        #[ink(message)]
        pub fn get_project_by_nft_address(
            &self,
            nft_contract_address: AccountId
        ) -> Option<Project> {
            if self.projects.get(&nft_contract_address).is_none(){
                return None;
            }
            
            Some(self.projects.get(&nft_contract_address).unwrap())
        }

        /* END GETTERS*/
    }

    impl CrossArtZeroLaunchPadPSP34 for ArtZeroLaunchPadPSP34 {
        /// Get project mint fee
        #[ink(message)]
        fn get_project_mint_fee_rate(
            &self
        ) -> u32 {
            return self.project_mint_fee_rate;
        }
    }
}