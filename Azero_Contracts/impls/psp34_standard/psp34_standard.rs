use crate::traits::psp34_standard::*;
pub use crate::{
    impls::psp34_standard::{
        data,
        data::*,
        psp34_standard,
        *,
    },
    traits::psp34_standard::*,
};
use openbrush::{
    modifiers,
    contracts::ownable::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*,
        mintable::*
    },
    traits::{
        AccountId,
        Balance,
        OccupiedStorage,
        Storage,
        ZERO_ADDRESS,
    },
};
use ink_prelude::{
    string::{
        String,
        ToString,
    },
    vec::Vec,
};

impl<T: Storage<Manager>> Psp34Traits for T 
where
    T: Internal +
    Storage<psp34::extensions::metadata::Data> + 
    Storage<psp34::Data<psp34::extensions::enumerable::Balances>> + 
    Storage<openbrush::contracts::ownable::Data>
{
    /// Change baseURI
    #[modifiers(only_owner)]
    default fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
        self._set_attribute(Id::U8(0), String::from("baseURI").into_bytes(), uri.into_bytes());
        Ok(())
    }

    /// Only Owner can set multiple attributes to a token
    #[modifiers(only_owner)]
    default fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>
    ) -> Result<(), Error> {
        assert!(token_id != Id::U64(0));
        if self.is_locked_nft(token_id.clone()) {
            return Err(Error::Custom(String::from("Token is locked")))
        }
        for (attribute, value) in &metadata {
            add_attribute_name(self, attribute.clone().into_bytes());
            self._set_attribute(token_id.clone(), attribute.clone().into_bytes(), value.clone().into_bytes());
        }
        Ok(())
    }

    /// Get multiple  attributes
    default fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
        let length = attributes.len();
        let mut ret = Vec::<String>::new();
        for i in 0..length {
            let attribute = attributes[i].clone();
            let value = self.get_attribute(token_id.clone(), attribute.into_bytes());
            if value.is_some() {
                ret.push(String::from_utf8(value.unwrap()).unwrap());
            } else {
                ret.push(String::from(""));
            }
        }
        ret
    }

    /// Get Attribute Count
    default fn get_attribute_count(&self) -> u32 {
        self.data::<Manager>().attribute_count
    }
    /// Get Attribute Name
    default fn get_attribute_name(&self, index: u32) -> String {
        let attribute = self.data::<Manager>().attribute_names.get(&index);
        if attribute.is_some() {
            String::from_utf8(attribute.unwrap()).unwrap()
        } else {
            String::from("")
        }
    }

    /// Get URI from token ID
    default fn token_uri(&self, token_id: u64) -> String {
        let value = self.get_attribute(Id::U8(0), String::from("baseURI").into_bytes());
        let mut token_uri = String::from_utf8(value.unwrap()).unwrap();
        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        return token_uri
    }

    /// Get owner address
    default fn get_owner(&self) -> AccountId {
        return self.owner()
    }
}

impl<T: Storage<Manager>> Internal for T 
where
    T : PSP34 + 
    Storage<Manager> +
    Storage<psp34::extensions::metadata::Data> + 
    Storage<psp34::Data<psp34::extensions::enumerable::Balances>> + 
    Storage<openbrush::contracts::ownable::Data>
{
    /// Only Owner can mint new token
    #[modifiers(only_owner)]
    default fn mint(&mut self) -> Result<(), Error> {
        let caller =  T::env().caller();
        self.data::<Manager>().last_token_id += 1;
        assert!(self._mint_to(caller, Id::U64(self.data::<Manager>().last_token_id)).is_ok());
        Ok(())
    }

    /// Only Owner can mint new token and add attributes for it
    #[modifiers(only_owner)]
    default fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), PSP34Error> {
        let caller = T::env().caller();
        self.data::<Manager>().last_token_id += 1;
        self._mint_to(caller, Id::U64(self.data::<Manager>().last_token_id))?;
        self.set_multiple_attributes(Id::U64(self.data::<Manager>().last_token_id), metadata);
        Ok(())
    }

    /// Get Token Count
    default fn get_last_token_id(&self) -> u64 {
        return self.data::<Manager>().last_token_id
    }

    /// Lock nft - Only owner token
    default fn lock(&mut self, token_id: Id) -> Result<(), Error> {
        let caller = T::env().caller();
        let token_owner = self.owner_of(token_id.clone()).unwrap();
        assert!(caller == token_owner);
        self.data::<Manager>().locked_token_count += 1;
        self.data::<Manager>().locked_tokens.insert(&token_id, &1);
        Ok(())
    }

    /// Check token is locked or not
    default fn is_locked_nft(&self, token_id: Id) -> bool {
        if self.data::<Manager>().locked_tokens.get(&token_id).is_some() {
            return true
        }
        return false
    }

    /// Get Locked Token Count
    default fn get_locked_token_count(&self) -> u64 {
        return self.data::<Manager>().locked_token_count
    }

    /// Burn NFT
    default fn burn(&mut self, id: Id) -> Result<(), PSP34Error> {
        let caller = T::env().caller();
        let token_owner = self.owner_of(id.clone()).unwrap();
        assert!(caller == token_owner);
        self._burn_from(caller, id)
    }
}

pub trait Internal {
    fn mint(&mut self) -> Result<(), Error>;
    fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), PSP34Error>;
    fn get_last_token_id(&self) -> u64;
    fn lock(&mut self, token_id: Id) -> Result<(), Error>;
    fn is_locked_nft(&self, token_id: Id) -> bool;
    fn get_locked_token_count(&self) -> u64;
    fn burn(&mut self, id: Id) -> Result<(), PSP34Error>;
}

fn add_attribute_name<T: Storage<Manager>>(
    instance: &mut T,
    attribute_input: Vec<u8>
) {
    let mut exist: bool = false;
    for index in 0..instance.data::<Manager>().attribute_count {
        let attribute_name = instance.data::<Manager>().attribute_names.get(&(index + 1));
        if attribute_name.is_some() {
            if attribute_name.unwrap() == attribute_input {
                exist = true;
                break
            }
        }
    }
    if !exist {
        instance.data::<Manager>().attribute_count += 1;
        let data = &mut instance.data::<Manager>();
        data.attribute_names.insert(&data.attribute_count, &attribute_input);
    }
}


