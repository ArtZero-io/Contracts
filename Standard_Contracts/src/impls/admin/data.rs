pub const STORAGE_KEY: u32 = openbrush::storage_unique_key!(Data);

#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct Data {
    pub _reserved: Option<()>,
}
