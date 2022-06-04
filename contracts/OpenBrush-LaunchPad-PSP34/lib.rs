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
        InvalidFee,
        InvalidRoyalFee,
        NotEnoughBalance
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct RoadMap {
        name: AccountId,
        description: Vec<u8>,
        estimated_time: Timestamp
    }

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct TeamMemberInformation {
        name: AccountId,
        avatar: Vec<u8>,
        social_links: Vec<u8>
    }

    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Project {
        project_type: u8, // 1 is Live Project, 2 is Ended Project
        project_owner: AccountId,
        total_supply: u64,
        start_time: Timestamp,
        end_time: Timestamp,
        attributes: Mapping<Vec<u8>, Vec<u8>>
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
        projects_by_owner: Mapping<AccountId, Vec<AccountId>>
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
            start_time: Timestamp,
            end_time: Timestamp,
            attributes: Vec<String>,
            attribute_vals: Vec<String> 
        ) -> Result<(), Error> {
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

            let projects_by_owner = self.projects_by_owner.get(&collection_owner);
            if projects_by_owner.is_none() {
                let mut projects = Vec::<AccountId>::new();
                projects.push(contract_account);
                self.projects_by_owner.insert(&project_owner, &projects);
            } else {
                let mut projects = projects_by_owner.unwrap();
                projects.push(contract_account);
                self.projects_by_owner.insert(&project_owner, &collections);
            }

            let new_project = Project {
                project_type: 1, // 1 is Live Project, 2 is Ended Project
                project_owner: project_owner,
                total_supply: total_supply,
                start_time: start_time,
                end_time: end_time,
                attributes: Mapping<Vec<u8>, Vec<u8>>
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

        /* END GETTERS*/
    }
}