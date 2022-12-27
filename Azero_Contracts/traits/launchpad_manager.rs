
#[openbrush::wrapper]
pub type ArtZeroLaunchPadRef = dyn ArtZeroLaunchPadTrait;

#[openbrush::trait_definition]
pub trait ArtZeroLaunchPadTrait {
    #[ink(message)]
    fn get_project_mint_fee_rate(&self) -> u32;

    #[ink(message)]
    fn get_public_max_minting_amount(&self) -> u64;
}
