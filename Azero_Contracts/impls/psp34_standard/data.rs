use crate::traits::psp34_standard::*;
use openbrush::{
    storage::{
        Mapping,
        TypeGuard,
    },
    traits::{
        AccountId,
        Balance,
        Hash,
    },
};

use ink_prelude::{
    string::{
        String,
        ToString,
    },
    vec::Vec,
};
use ink_storage::{
    traits::SpreadAllocate
};
use openbrush::{
    contracts::ownable::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*
    },
    traits::Storage,
    modifiers,
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    last_token_id: u64,
    attribute_count: u32,
    attribute_names: Mapping<u32, Vec<u8>>,
    locked_tokens: Mapping<Id, u8>,
    locked_token_count: u64,
}
