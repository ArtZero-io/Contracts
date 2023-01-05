use openbrush::{
    traits::{
        Balance,
        AccountId
    },
    contracts::{
        traits::psp34::{
            Id
        },
        traits::psp22::*,
    },
};
use crate::traits::error::Error;

#[openbrush::wrapper]
pub type ArtZeroAdminRef = dyn AdminTrait;
#[openbrush::wrapper]
pub type Psp22Ref = dyn PSP22;

#[openbrush::trait_definition]
pub trait AdminTrait {
    /// This function allows contract owner to withdraw contract balance to his account.
    #[ink(message)]
    fn withdraw_fee(&mut self, value: Balance, receiver: AccountId) -> Result<(), Error>;
    /// This function allow contract owner withdraw NFT to an account in case there is any NFT sent to contract by mistake
    #[ink(message)]
    fn tranfer_nft(&mut self, nft_contract_address: AccountId, token_id: Id, receiver: AccountId) -> Result<(), Error>;
    /// This function allow contract owner withdraw PSP22 to an account in case there is any token sent to contract by mistake
    #[ink(message)]
    fn tranfer_psp22(&mut self, psp22_contract_address: AccountId, amount: Balance, receiver: AccountId) -> Result<(), Error>;
}
