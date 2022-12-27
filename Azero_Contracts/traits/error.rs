use openbrush::{
    contracts::ownable::*,
};
use ink_prelude::{
    string::String,
};


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
