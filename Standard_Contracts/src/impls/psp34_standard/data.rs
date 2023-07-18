use openbrush::{
    storage::{
        Mapping,
    },
};

use ink::prelude::{
    vec::Vec,
    string::String,
};
use openbrush::{
    contracts::psp34::extensions::{
        enumerable::*,
    },
};

pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Manager);

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Manager {
    pub last_token_id: u64,
    pub attribute_count: u32,
    pub attribute_names: Mapping<u32, Vec<u8>>,
    pub is_attribute: Mapping<String, bool>,
    pub locked_tokens: Mapping<Id, bool>,
    pub locked_token_count: u64,
    pub _reserved: Option<()>
}
