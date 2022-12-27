
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
    traits::{
        Storage,
    }
};

impl<T: Storage<Manager>> ArtZeroLaunchPadTrait for T {

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
}
