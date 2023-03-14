
#[openbrush::wrapper]
pub type ArtZeroLaunchPadRef = dyn ArtZeroLaunchPadTrait;

#[openbrush::trait_definition]
pub trait ArtZeroLaunchPadTrait {
    /// This function returns the rate in % that the launchpad will collect for each NFT minting
    #[ink(message)]
    fn get_project_mint_fee_rate(&self) -> u32;
    /// This function returns the maximal amount of NFT that one can mint each time
    #[ink(message)]
    fn get_public_max_minting_amount(&self) -> u64;
}
