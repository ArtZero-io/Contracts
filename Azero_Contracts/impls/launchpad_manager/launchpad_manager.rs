use crate::traits::launchpad_manager::*;
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
use ink_lang::ToAccountId;
use ink_prelude::{
    vec,
    string::String,
    vec::Vec,
};
use ink_storage::{
    traits::{
        PackedLayout,
        SpreadAllocate,
        SpreadLayout,
    },
};
use openbrush::{
    contracts::access_control::*,
    contracts::ownable::*,
    modifiers,
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        Storage,
        AccountId,
        Hash,
        Balance,
        StorageAsRef,
        StorageAsMut
    }
};

// ADMINER RoleType = 3739740293
const ADMINER: RoleType = ink_lang::selector_id!("ADMINER");

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
