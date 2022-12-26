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
        Balance
    }
};
use ink_prelude::{
    vec,
    string::String,
    vec::Vec,
};

#[openbrush::wrapper]
pub type ArtZeroStakingRef = dyn ArtZeroStakingTrait;

#[openbrush::trait_definition]
pub trait ArtZeroStakingTrait {
    #[ink(message)]
    fn get_total_staked_by_account(&self, account: AccountId) -> u64;
    #[ink(message)]
    fn get_total_pending_unstaked_by_account(&self, account: AccountId) -> u64;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
    TokenOwnerNotMatch,
    NotApproved,
    CannotTransfer,
    OnlyOwner,
    OnlyAdmin,
}

impl From<OwnableError> for Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}
