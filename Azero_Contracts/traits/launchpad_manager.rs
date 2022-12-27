use openbrush::{
    contracts::ownable::*,
};
use ink_prelude::{
    string::String,
};

#[openbrush::wrapper]
pub type ArtZeroLaunchPadRef = dyn ArtZeroLaunchPadTrait;

#[openbrush::trait_definition]
pub trait ArtZeroLaunchPadTrait {
    #[ink(message)]
    fn get_project_mint_fee_rate(&self) -> u32;

    #[ink(message)]
    fn get_public_max_minting_amount(&self) -> u64;
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        Custom(String),
        OnlyOwner,
        OnlyAdmin,
        InvalidCaller,
        NotEnoughBalance,
        ProjectNotExist,
        ProjectOwnerAndAdmin,
        InvalidStartTimeAndEndTime
    }

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }
