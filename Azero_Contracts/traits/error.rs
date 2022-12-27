use openbrush::{
    contracts::ownable::*,
    contracts::access_control::*,
};
use ink_prelude::{
    string::String,
};


#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum Error {
    Custom(String),
    OnlyOwner,
    OnlyAdmin,
    InvalidCaller,
    TokenOwnerNotMatch,
    NotApproved,
    CannotTransfer,
    NotEnoughBalance,
    AlreadyInit,
    //use in psp34_standard
    NotOwner,
    //use in launchpad_manager
    ProjectNotExist,
    ProjectOwnerAndAdmin,
    InvalidStartTimeAndEndTime,
    //use in collection_manager
    CollectionOwnerAndAdmin,
    //use in psp34_launchpad_standard
    InvalidInput,
    ClaimedAll,
    TokenLimitReached,
    PhaseNotExist,
    PhaseExpired,
    WhitelistNotExist,

    //use in admin
    WithdrawFeeError,

    //use in marketplace
    NotListed,
    BidAlreadyExist,
    BidNotExist,

}

impl From<OwnableError> for Error {
    fn from(ownable: OwnableError) -> Self {
        match ownable {
            OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
            OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
        }
    }
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        match access {
            AccessControlError::MissingRole => Error::Custom(String::from("AC::MissingRole")),
            AccessControlError::RoleRedundant => {
                Error::Custom(String::from("AC::RoleRedundant"))
            }
            AccessControlError::InvalidCaller => {
                Error::Custom(String::from("AC::InvalidCaller"))
            }
        }
    }
}
