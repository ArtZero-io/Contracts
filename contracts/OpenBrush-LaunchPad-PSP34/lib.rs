#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_launchpad_psp34 {
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::modifiers;
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
        roadmaps: Vec<u8>,
        team_members: Vec<u8>,
        start_time: Timestamp,
        end_time: Timestamp
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate, OwnableStorage)]
    pub struct ArtZeroLaunchPadPSP34 {
        #[OwnableStorageField]
        ownable: OwnableData,
        admin_address: AccountId,
        standard_nft_hash: Hash,
        project_count: u64,
        projects: Mapping<AccountId, Project>,
        projects_by_id: Mapping<u64, AccountId>,
        projects_by_owner: Mapping<AccountId, Vec<AccountId>>,
        attributes: Mapping<(AccountId,Vec<u8>), Vec<u8>>,
        active_project_count: u64
    }

    impl Ownable for ArtZeroLaunchPadPSP34 {}

    impl ArtZeroLaunchPadPSP34 {
        #[ink(constructor)]
        pub fn new(
            admin_address: AccountId, 
            owner_address: AccountId,
            standard_nft_hash: Hash
        ) -> Self {
            ink_lang::codegen::initialize_contract(|_instance: &mut Self| {
                _instance._init_with_owner(owner_address);
                _instance.initialize(admin_address, standard_nft_hash).ok().unwrap();
            })
        }

        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn initialize(
            &mut self,
            admin_address: AccountId,
            standard_nft_hash: Hash
        ) -> Result<(), OwnableError> {
            self.admin_address = admin_address;
            self.standard_nft_hash = standard_nft_hash;
            self.project_count = 0;
            self.active_project_count = 0;
            Ok(())
        }

        /* EXECUTE FUNCTION */

        /// Add new project
        #[ink(message)]
        pub fn add_new_project(
            &mut self,
            project_owner: AccountId,
            name: String,
            description: String,
            total_supply: u64,
            roadmaps: String,
            team_members: String,
            start_time: Timestamp,
            end_time: Timestamp,
            attributes: Vec<String>,
            attribute_vals: Vec<String> 
        ) -> Result<(), Error> {
            if start_time > end_time {
                return Err(Error::InvalidStartTimeAndEndTime);
            }

            let (hash, _) =
                ink_env::random::<ink_env::DefaultEnvironment>(name.as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract = LaunchPadPsp34NftStandardRef::new(project_owner)
                .endowment(0)
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

            let mut project_type = 1;

            if end_time <= Self::env().block_timestamp() {
                project_type = 2;
            }

            let new_project = Project {
                is_active: false,
                project_type: project_type, // 1 is Live Project, 2 is Ended Project
                project_owner: project_owner,
                total_supply: total_supply,
                roadmaps: roadmaps.into_bytes(),
                team_members: team_members.into_bytes(),
                start_time: start_time,
                end_time: end_time
            };

            self.projects.insert(&contract_account, &new_project);

            if self.set_multiple_attributes(contract_account, attributes, attribute_vals).is_err() {
                panic!(
                    "error set_multiple_attributes"
                )
            };

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

        /// Update standard nft hash - Only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_standard_nft_hash(
            &mut self,
            standard_nft_hash: Hash
        ) -> Result<(), Error>  {
            self.standard_nft_hash = standard_nft_hash;
            Ok(())
        }

        fn _set_attribute(
            &mut self, 
            account: AccountId,
            key: Vec<u8>, 
            value: Vec<u8>
        ) {
            self.attributes.insert(&(account,key), &value);
        }

        /// Set multiple attributes type string - Only admin - project owner
        #[ink(message)]
        pub fn set_multiple_attributes(
            &mut self, 
            contract_address: AccountId, 
            attributes: Vec<String>, 
            values: Vec<String>
        ) -> Result<(),Error> {
            if attributes.len() != values.len() {
                return Err(Error::Custom(String::from("Inputs not same length")));
            }
            if self.projects.get(&contract_address).is_none() {
                return Err(Error::ProjectNotExist);
            }
            let project = self.projects.get(&contract_address).unwrap();
            if project.project_owner == self.env().caller() || self.admin_address == self.env().caller() {
                let length = attributes.len();
                for i in 0..length {
                    let attribute = attributes[i].clone();
                    let value = values[i].clone();
                    self._set_attribute(contract_address, attribute.into_bytes(), value.into_bytes());
                }

                Ok(())
            } else {
                return Err(Error::ProjectOwnerAndAdmin);
            }
        }

        /* END SETTERS */
        
        /* GETTERS */

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

        // Get multi attributes
        #[ink(message)]
        pub fn get_attributes(
            &self, 
            account: AccountId, 
            attributes: Vec<String>
        ) -> Vec<String> {
            let length = attributes.len();
            let mut ret = Vec::<String>::new();
            for i in 0..length {
                let attribute = attributes[i].clone();
                let value = self.attributes.get(&(account,attribute.into_bytes()));
                if value.is_some() {
                    ret.push(String::from_utf8(value.unwrap()).unwrap());
                } else{
                    ret.push(String::from(""));
                }
            }
            ret
        }

        // Get project by id
        #[ink(message)]
        pub fn get_project_by_id(
            &self,
            id: u64
        ) -> Option<AccountId> {
            return self.projects_by_id.get(&id);
        }

        /* END GETTERS*/
    }
}