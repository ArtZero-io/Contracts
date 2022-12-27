
use openbrush::{
    storage::{
        Mapping,
    },
};

use ink_prelude::{
    vec::Vec,
};
use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default, Debug)]
#[openbrush::upgradeable_storage(STORAGE_KEY)]
pub struct Manager {
    pub last_token_id: u64,
    pub attribute_count: u32,
    pub attribute_names: Mapping<u32, Vec<u8>>,
    pub locked_tokens: Mapping<Id, u8>,
    pub locked_token_count: u64,
    pub _reserved: Option<()>
}
