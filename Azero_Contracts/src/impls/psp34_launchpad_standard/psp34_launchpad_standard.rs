
pub use crate::{
    impls::psp34_launchpad_standard::{
        data,
        data::*,
        psp34_launchpad_standard,
        *,
    },
    traits::psp34_launchpad_standard::*,
};
use openbrush::{
    modifiers,
    modifier_definition,
    contracts::ownable::*,
    contracts::access_control::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*,
    },
    traits::{
        AccountId,
        Storage,
        DefaultEnv,
        ZERO_ADDRESS,
        Timestamp,
        Balance,
        OccupiedStorage
    },
};
use ink::prelude::{
    string::{
        String,
        ToString,
    },
    vec::Vec,
};
use ink::{
    storage::traits::{
        AutoStorableHint,
        ManualKey,
        Storable,
        StorableHint,
    },
};
use crate::traits::error::Error;
use crate::traits::launchpad_manager::ArtZeroLaunchPadRef;

pub const ADMINER: RoleType = ink::selector_id!("ADMINER");

impl<T, M> Psp34LaunchPadTraits for T
where
    M: access_control::members::MembersManager,
    M: Storable
        + StorableHint<ManualKey<{ access_control::STORAGE_KEY }>>
        + AutoStorableHint<ManualKey<3218979580, ManualKey<{ access_control::STORAGE_KEY }>>, Type = M>,
    T: PSP34 + psp34::Internal,
    T: Storage<psp34::extensions::metadata::Data>,
    T: Storage<psp34::Data<psp34::extensions::enumerable::Balances>>,
    T: Storage<openbrush::contracts::ownable::Data>,
    T: Storage<ownable::Data>,
    T: Storage<access_control::Data<M>>,
    T: OccupiedStorage<{ access_control::STORAGE_KEY }, WithData = access_control::Data<M>>,
    T: Storage<Manager> 
{
    #[modifiers(only_role(ADMINER))]
    default fn deactive_phase(
        &mut self,
        phase_id: u8
    ) -> Result<(), Error> {
        if let Some(mut phase) = self.data::<Manager>().phases.get(&phase_id) {
            if phase.claimed_amount != 0 || self.data::<Manager>().phase_account_link.count(phase_id) != 0 || phase.start_time <= T::env().block_timestamp() || !phase.is_active{
                return Err(Error::Custom(String::from("Cannot deactivate phase")))
            }
            phase.is_active = false;
            if let Some(active_phase_count) = self.data::<Manager>().active_phase_count.checked_sub(1) {
                self.data::<Manager>().active_phase_count = active_phase_count;
                if phase.is_public {
                    if let Some(available_token_amount) = self.data::<Manager>().available_token_amount.checked_add(phase.public_minting_amount) {
                        self.data::<Manager>().available_token_amount = available_token_amount;
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to available token amount"))); 
                    }                        
                }
                self.data::<Manager>().phases.insert(&phase_id, &phase);
                Ok(())
            } else {
                return Err(Error::Custom(String::from("Cannot subtract from active phase count"))); 
            }                
        } else {
            return Err(Error::PhaseNotExist);
        }            
    }

    #[modifiers(only_role(ADMINER))]
    default fn update_schedule_phase(
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
        if self.data::<Manager>().phases.get(&phase_id).is_none() {
            return Err(Error::PhaseNotExist);
        }
        if public_max_minting_amount > ArtZeroLaunchPadRef::get_public_max_minting_amount(&self.data::<Manager>().launchpad_contract_address) {
            return Err(Error::InvalidInput);
        }
        if start_time >= end_time {
            return Err(Error::InvalidStartTimeAndEndTime);
        }
        for index in 0..self.data::<Manager>().last_phase_id {
            if let Some(idx) = index.checked_add(1) {
                if idx != phase_id {
                    if let Some(phs) = self.data::<Manager>().phases.get(&(index+1)) {
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
        if let Some(mut phase) = self.data::<Manager>().phases.get(&phase_id) {
            if !phase.is_active ||
            phase.claimed_amount != 0 ||
            self.data::<Manager>().phase_account_link.count(phase_id) != 0 || phase.start_time <= T::env().block_timestamp() {
                return Err(Error::Custom(String::from("cannot update phase")))
            }

            phase.title = phase_code.clone().into_bytes();
            if phase.is_public && !is_public {
                if let Some(available_token_amount) = self.data::<Manager>().available_token_amount.checked_add(phase.public_minting_amount) {
                    self.data::<Manager>().available_token_amount = available_token_amount;
                    if let Some(total_amount) = phase.total_amount.checked_sub(phase.public_minting_amount) {
                        phase.total_amount = total_amount;
                    } else {
                        return Err(Error::Custom(String::from("Cannot subtract from phase total amount")));
                    }                        
                } else {
                    return Err(Error::Custom(String::from("Cannot add to available token amount"))); 
                }                    
            } else if !phase.is_public && is_public {
                if let Some(available_token_amount) = self.data::<Manager>().available_token_amount.checked_sub(public_minting_amount) {
                    self.data::<Manager>().available_token_amount = available_token_amount;
                    if let Some(total_amount) = phase.total_amount.checked_add(public_minting_amount) {
                        phase.total_amount = total_amount;
                    } else {
                        return Err(Error::Custom(String::from("Cannot add to phase total amount"))); 
                    }                        
                } else {
                    return Err(Error::Custom(String::from("Cannot subtract from available token amount")));
                }                    
            } else if phase.is_public && is_public {
                if let Some(available_token_amount_added_public_minting_amount) = self.data::<Manager>().available_token_amount.checked_add(phase.public_minting_amount) {
                    if let Some(available_token_amount) = available_token_amount_added_public_minting_amount.checked_sub(public_minting_amount) {
                        self.data::<Manager>().available_token_amount = available_token_amount;
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
            self.data::<Manager>().phases.insert(&phase_id, &phase);
            Ok(())
        } else {
            return Err(Error::PhaseNotExist);
        }            
    }

    #[modifiers(only_role(ADMINER))]
    default fn update_schedule_phases(
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

    #[modifiers(only_role(ADMINER))]
    default fn edit_project_information(
        &mut self,
        project_info: String
    ) -> Result<(), AccessControlError> {
        self.data::<Manager>().project_info = project_info.into_bytes();
        Ok(())
    }

    default fn get_owner_claimed_amount(
        &self
    ) -> u64 {
        self.data::<Manager>().owner_claimed_amount
    }

    default fn get_owner_available_amount(
        &self
    ) -> u64 {
        let current_time = T::env().block_timestamp();
        let mut available_amount: u64 = 0;
        for index in 0..self.data::<Manager>().last_phase_id {
            if let Some(phase) = self.data::<Manager>().phases.get(&(index+1)) {
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

    default fn get_limit_phase_count(
        &self
    ) -> u8 {
        self.data::<Manager>().limit_phase_count
    }

    default fn get_public_minted_count(
        &self
    ) -> u64 {
        self.data::<Manager>().public_minted_count
    }

    default fn get_project_info(
        &self
    ) -> Vec<u8> {
        self.data::<Manager>().project_info.clone()
    }

    default fn get_phase_schedule_by_id(
        &self,
        phase_id: u8
    ) -> Option<Phase> {
        self.data::<Manager>().phases.get(&phase_id)
    }

    default fn get_whitelist_by_account_id(
        &self,
        account: AccountId,
        phase_id: u8
    ) -> Option<Whitelist> {
        self.data::<Manager>().phase_whitelists_link.get(&(&account, &phase_id))
    }

    default fn get_phase_account_link(
        &self,
        phase_id: u8,
        account_index: u64
    ) -> Option<AccountId> {
        self.data::<Manager>().phase_account_link.get_value(phase_id, &(account_index as u128))
    }

    default fn get_current_phase(&self) -> Option<u8> {
        let current_time = T::env().block_timestamp();
        for index in 0..self.data::<Manager>().last_phase_id {
            if let Some(phase) = self.data::<Manager>().phases.get(&(index+1)) {
                if phase.is_active && (phase.start_time..=phase.end_time).contains(&current_time) {
                    return Some(index + 1);
                }
            }                
        }
        None
    }

    default fn is_in_schedule_phase(&self, time: Timestamp) -> Option<u8> {
        for index in 0..self.data::<Manager>().last_phase_id {
            if let Some(phase) = self.data::<Manager>().phases.get(&(index+1)) {
                if phase.start_time <= time && phase.end_time >= time {
                    return Some(index);
                }
            }                
        }
        None
    }

    default fn get_whitelist_count(
        &self
    ) -> u32 {
        self.data::<Manager>().whitelist_count
    }

    default fn get_last_phase_id(&self) -> u8 {
        self.data::<Manager>().last_phase_id
    }

    default fn get_active_phase_count(&self) -> u8 {
        self.data::<Manager>().active_phase_count
    }

    default fn get_phase_account_public_claimed_amount(&self, account_id: AccountId, phase_id: u8) -> Option<u64> {
        Some(self.data::<Manager>().phase_account_public_claimed_amount.get(&(&account_id, &phase_id)))?
    }

    default fn get_phase_account_last_index(&self, phase_id: u8) -> u64 {
        self.data::<Manager>().phase_account_link.count(phase_id) as u64
    }

    default fn get_total_supply(&self) -> u64 {
        self.data::<Manager>().total_supply
    }

    default fn get_available_token_amount(&self) -> u64 {
        self.data::<Manager>().available_token_amount
    }
}

fn validate_phase_schedule<T: Storage<Manager>>(
    instance: &mut T,
    start_time: &Timestamp, 
    end_time: &Timestamp
) -> bool {

    if *start_time >= *end_time {
        return false;
    }

    for index in 0..instance.data::<Manager>().last_phase_id {
        if let Some(phase) = instance.data::<Manager>().phases.get(&(index+1)) {
            if phase.is_active && (
                (phase.start_time..=phase.end_time).contains(start_time) || (phase.start_time..=phase.end_time).contains(end_time)
            ) {
                return false;
            }
        }             
    }
    true
}

