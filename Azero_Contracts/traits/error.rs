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
    InvalidFee,
    TokenOwnerNotMatch,
    NotApproved,
    CannotTransfer,
    NotEnoughBalance,
    AlreadyInit,
    NotOwner,
    NotTokenOwner,
    ProjectNotExist,
    ProjectOwnerAndAdmin,
    InvalidStartTimeAndEndTime,
    CollectionOwnerAndAdmin,
    CollectionNotActive,
    InvalidInput,
    InvalidType,
    ClaimedAll,
    TokenLimitReached,
    UpdatePhase,
    PhaseNotExist,
    PhaseExpired,
    WhitelistNotExist,
    WithdrawFeeError,
    WithdrawNFTError,
    WithdrawPSP22Error,
    NotListed,
    BidAlreadyExist,
    BidNotExist,
    NotInMarket,
    NotForSale,
    NotInSaleList,
    InvalidBidLength,
    InvalidCollectionOwner,
    OwnableError(OwnableError),
    AccessControlError(AccessControlError)
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum LockError {
    NotLocked,
    Locked
}

impl From<OwnableError> for Error {
    fn from(ownable: OwnableError) -> Self {
        Error::OwnableError(ownable)
    }
}
impl From<LockError> for Error {
    fn from(locked: LockError) -> Self {
        match locked {
            LockError::Locked => Error::Custom(String::from("O::Locked")),
            LockError::NotLocked => Error::Custom(String::from("O::NotLocked")),
        }
    }
}

impl From<AccessControlError> for Error {
    fn from(access: AccessControlError) -> Self {
        Error::AccessControlError(access)
    }
}
