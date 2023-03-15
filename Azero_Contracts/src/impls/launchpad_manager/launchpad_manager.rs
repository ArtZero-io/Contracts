
use crate::impls::launchpad_manager::data::Manager;
pub use crate::{
    impls::launchpad_manager::{
        data,
        data::*,
        launchpad_manager,
        *,
    },
    traits::launchpad_manager::*,
};
use openbrush::{
    modifiers,
    contracts::access_control::*,
    contracts::ownable::*,
    traits::{
        Storage,
        AccountId,
        Balance,
        Hash
    }
};
use ink::prelude::{
    string::{
        String,
    },
    vec,
    vec::Vec,
};
use crate::traits::error::Error;

// ADMINER RoleType = 3739740293
pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T: Storage<Manager> + 
    Storage<access_control::Data> + 
    Storage<ownable::Data>
> ArtZeroLaunchPadTrait for T {

   /// Get project mint fee
    default fn get_project_mint_fee_rate(
        &self
    ) -> u32 {
        return self.data::<Manager>().project_mint_fee_rate;
    }

    /// Get public max minting amount
    default fn get_public_max_minting_amount(
        &self
    ) -> u64 {
        return self.data::<Manager>().public_max_minting_amount;
    }

    #[modifiers(only_role(ADMINER))]
    default fn update_project_adding_fee(
        &mut self,
        project_adding_fee: Balance
    ) -> Result<(), Error> {
        if project_adding_fee == 0 {
            return Err(Error::InvalidFee);
        }
        self.data::<Manager>().project_adding_fee = project_adding_fee;
        Ok(())
    }
    
    #[modifiers(only_role(ADMINER))]
    default fn update_public_max_minting_amount(
        &mut self,
        public_max_minting_amount: u64
    ) -> Result<(), AccessControlError> {
        self.data::<Manager>().public_max_minting_amount = public_max_minting_amount;
        Ok(())
    }
    
    #[modifiers(only_role(ADMINER))]
    default fn update_project_mint_fee_rate(
        &mut self,
        project_mint_fee_rate: u32
    ) -> Result<(), AccessControlError>  {
        self.data::<Manager>().project_mint_fee_rate = project_mint_fee_rate;
        Ok(())
    }
    
    #[modifiers(only_owner)]
    default fn update_standard_nft_hash(
        &mut self,
        standard_nft_hash: Hash
    ) -> Result<(), Error> {
        self.data::<Manager>().standard_nft_hash = standard_nft_hash;
        Ok(())
    }
    
    #[modifiers(only_role(ADMINER))]
    default fn update_is_active_project(
        &mut self,
        is_active: bool,
        contract_address: AccountId
    ) -> Result<(), Error>  {
        if let Some(mut project) = self.data::<Manager>().projects.get(&contract_address) {
            if is_active == project.is_active{
                return Err(Error::InvalidInput);
            }
            project.is_active = is_active;
            if is_active {
                if let Some(active_project_count) = self.data::<Manager>().active_project_count.checked_add(1) {
                    self.data::<Manager>().active_project_count = active_project_count;
                } else {
                    return Err(Error::CheckedOperations);
                }
            } else {
                if let Some(active_project_count) = self.data::<Manager>().active_project_count.checked_sub(1) {
                    self.data::<Manager>().active_project_count = active_project_count;
                } else {
                    return Err(Error::CheckedOperations);
                }
            }
            self.data::<Manager>().projects.insert(&contract_address, &project);
            Ok(())
        } else {
            return Err(Error::ProjectNotExist);
        }
    }
    
    /* END SETTERS */
    
    /* GETTERS */
    
    default fn get_project_adding_fee(
        &self
    ) -> Balance {
        self.data::<Manager>().project_adding_fee
    }
    
    default fn get_active_project_count(
        &self
    ) -> u64 {
        self.data::<Manager>().active_project_count
    }
    
    default fn get_project_count(
        &self
    ) -> u64 {
        self.data::<Manager>().project_count
    }
    
    default fn get_standard_nft_hash(
        &self
    ) -> Hash {
        self.data::<Manager>().standard_nft_hash
    }
    
    default fn get_project_by_id(
        &self,
        id: u64
    ) -> Option<AccountId> {
        self.data::<Manager>().projects_by_id.get(&id)
    }
    
    default fn get_projects_by_owner(
        &self,
        owner_address: AccountId
    ) -> Vec<AccountId> {
        self.data::<Manager>().projects_by_owner.get(&owner_address).unwrap_or_default()
    }
    
    default fn get_project_by_nft_address(
        &self,
        nft_contract_address: AccountId
    ) -> Option<Project> {
        self.data::<Manager>().projects.get(&nft_contract_address)
    }
    
    default fn get_max_phases_per_project(
        &self
    ) -> u8 {
        self.data::<Manager>().max_phases_per_project
    }
}
