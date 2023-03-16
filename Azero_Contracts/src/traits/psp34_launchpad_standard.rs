use ink::prelude::{
    vec::Vec,
    string::{
        String,
    },
};
use openbrush::{
    contracts::access_control::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*
    },
    traits::{
        AccountId,
        Balance,
        Timestamp
    }
};
use crate::traits::error::Error;
use crate::impls::psp34_launchpad_standard::Phase;
use crate::impls::psp34_launchpad_standard::Whitelist;

#[openbrush::wrapper]
pub type Psp34LaunchPadRef = dyn Psp34LaunchPadTraits + PSP34 + PSP34Metadata;

#[openbrush::trait_definition]
pub trait Psp34LaunchPadTraits {
    #[ink(message)]
    fn deactive_phase(
        &mut self,
        phase_id: u8
    ) -> Result<(), Error>;

    #[ink(message)]
    fn update_schedule_phase(
        &mut self,
        phase_id: u8,
        phase_code: String,
        is_public: bool,
        public_minting_fee: Balance,
        public_minting_amount: u64,
        public_max_minting_amount: u64,
        start_time: Timestamp,
        end_time: Timestamp
    ) -> Result<(), Error>;

    #[ink(message)]
    fn update_schedule_phases(
        &mut self,
        id_phases: Vec<u8>,
        code_phases: Vec<String>,
        is_public_phases: Vec<bool>,
        public_minting_fee_phases: Vec<Balance>,
        public_minting_amount_phases: Vec<u64>,
        public_max_minting_amount_phases: Vec<u64>,
        start_time_phases: Vec<Timestamp>,
        end_time_phases: Vec<Timestamp>
    ) -> Result<(), Error>;

    #[ink(message)]
    fn edit_project_information(
        &mut self,
        project_info: String
    ) -> Result<(), AccessControlError>;

    #[ink(message)]
    fn get_owner_claimed_amount(
        &self
    ) -> u64;

    #[ink(message)]
    fn get_owner_available_amount(
        &self
    ) -> u64;

    #[ink(message)]
    fn get_limit_phase_count(
        &self
    ) -> u8;

    #[ink(message)]
    fn get_public_minted_count(
        &self
    ) -> u64;

    #[ink(message)]
    fn get_project_info(
        &self
    ) -> Vec<u8>;

    #[ink(message)]
    fn get_phase_schedule_by_id(
        &self,
        phase_id: u8
    ) -> Option<Phase>;

    #[ink(message)]
    fn get_whitelist_by_account_id(
        &self,
        account: AccountId,
        phase_id: u8
    ) -> Option<Whitelist>;

    #[ink(message)]
    fn get_phase_account_link(
        &self,
        phase_id: u8,
        account_index: u64
    ) -> Option<AccountId>;

    #[ink(message)]
    fn get_current_phase(&self) -> Option<u8>;

    #[ink(message)]
    fn is_in_schedule_phase(&self, time: Timestamp) -> Option<u8>;

    #[ink(message)]
    fn get_whitelist_count(
        &self
    ) -> u32;

    #[ink(message)]
    fn get_last_phase_id(&self) -> u8;

    #[ink(message)]
    fn get_active_phase_count(&self) -> u8;
    
    #[ink(message)]
    fn get_phase_account_public_claimed_amount(&self, account_id: AccountId, phase_id: u8) -> Option<u64>;

    #[ink(message)]
    fn get_phase_account_last_index(&self, phase_id: u8) -> u64;

    #[ink(message)]
    fn get_total_supply(&self) -> u64;

    #[ink(message)]
    fn get_available_token_amount(&self) -> u64;
}
