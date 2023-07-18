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
    modifier_definition,
    contracts::ownable::*,
    contracts::psp34::extensions::{
        enumerable::*,
        metadata::*,
    },
    traits::{
        AccountId,
        Storage,
    },
};
use ink::prelude::{
    string::{
        String,
        ToString,
    },
    vec::Vec,
};
use crate::traits::error::Error;

#[modifier_definition]
pub fn only_token_owner<T, F, R, E>(instance: &mut T, body: F, token_owner: AccountId) -> Result<R, E>
where
    T: Storage<Manager>,
    F: FnOnce(&mut T) -> Result<R, E>,
    E: From<Error>,
{
    if token_owner != T::env().caller() {
        return Err(From::from(Error::NotTokenOwner))
    }
    body(instance)
}

pub trait Psp34Traits: 
    Storage<Manager> + 
    PSP34 + psp34::Internal +
    psp34::extensions::metadata::Internal +
    psp34::extensions::metadata::PSP34MetadataImpl +
    openbrush::contracts::ownable::OwnableImpl
{
    /// Get Token Count
    fn get_last_token_id(&self) -> u64 {
        return self.data::<Manager>().last_token_id
    }

    /// Lock nft - Only owner token
    #[modifiers(only_token_owner(self.owner_of(token_id.clone()).unwrap()))]
    fn lock(&mut self, token_id: Id) -> Result<(), Error> {
        if let Some(locked_token_count) = self.data::<Manager>().locked_token_count.checked_add(1) {
            self.data::<Manager>().locked_token_count = locked_token_count;
            self.data::<Manager>().locked_tokens.insert(&token_id, &true);
            return Ok(());
        } else {
            return Err(Error::Custom(String::from("Cannot increase locked token count")));
        }
    }

    /// Check token is locked or not
    fn is_locked_nft(&self, token_id: Id) -> bool {
        if self.data::<Manager>().locked_tokens.get(&token_id).is_some() {
            return true;
        }
        return false;
    }

    /// Get Locked Token Count
    fn get_locked_token_count(&self) -> u64 {
        self.data::<Manager>().locked_token_count
    }

    /// Change baseURI
    #[modifiers(only_owner)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), Error> {
        self._set_attribute(Id::U8(0), String::from("baseURI"), uri);
        Ok(())
    }

    /// Only Owner can set multiple attributes to a token
    #[modifiers(only_owner)]
    fn set_multiple_attributes(
        &mut self,
        token_id: Id,
        metadata: Vec<(String, String)>
    ) -> Result<(), Error> {
        if token_id == Id::U64(0){
            return Err(Error::InvalidInput)
        }
        if self.is_locked_nft(token_id.clone()) {
            return Err(Error::Custom(String::from("Token is locked")))
        }
        for (attribute, value) in &metadata {
            add_attribute_name(self, &attribute.clone().into_bytes());
            self._set_attribute(token_id.clone(), attribute.clone(), value.clone());
        }
        Ok(())
    }

    /// Get multiple  attributes
    fn get_attributes(&self, token_id: Id, attributes: Vec<String>) -> Vec<String> {
        let length = attributes.len();
        let mut ret = Vec::<String>::new();
        for i in 0..length {
            let attribute = attributes[i].clone();
            let value = self.get_attribute(token_id.clone(), attribute);
            
            if let Some(value_in_bytes) = value {
                if let Ok(value_in_string) = String::from_utf8(value_in_bytes.into()) {
                    ret.push(value_in_string);
                } else {
                    ret.push(String::from(""));    
                }                   
            } else {
                ret.push(String::from(""));
            }
        }
        ret
    }

    /// Get Attribute Count
    fn get_attribute_count(&self) -> u32 {
        self.data::<Manager>().attribute_count
    }
    /// Get Attribute Name
    fn get_attribute_name(&self, index: u32) -> String {
        let attribute = self.data::<Manager>().attribute_names.get(&index);

        if let Some(value_in_bytes) = attribute {
            if let Ok(value_in_string) = String::from_utf8(value_in_bytes) {
                return value_in_string;
            } else {
                return String::from("");    
            }                   
        } else {
            return String::from(""); 
        }
    }

    /// Get URI from token ID
    fn token_uri(&self, token_id: u64) -> String {
        let value = self.get_attribute(Id::U8(0), String::from("baseURI"));
        let mut token_uri = String::from("");

        if let Some(value_in_bytes) = value {
            if let Ok(value_in_string) = String::from_utf8(value_in_bytes.into()) {
                token_uri = value_in_string;
            }                 
        }

        token_uri = token_uri + &token_id.to_string() + &String::from(".json");
        token_uri
    }

    /// Get owner address
    fn get_owner(&self) -> Option<AccountId> {
        self.owner()
    }
}

fn add_attribute_name<T: Storage<Manager>>(
    instance: &mut T,
    attribute_input: &Vec<u8>
) -> Result<(), Error> {
    if let Ok(attr_input) = String::from_utf8((*attribute_input).clone()) {
        let exist: bool = instance.data::<Manager>().is_attribute.get(&attr_input).is_some();

        if !exist {
            if let Some(attribute_count) = instance.data::<Manager>().attribute_count.checked_add(1) {
                instance.data::<Manager>().attribute_count = attribute_count;
                let data = &mut instance.data::<Manager>();
                data.attribute_names.insert(&data.attribute_count, &attribute_input);
                data.is_attribute.insert(&attr_input, &true);
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Fail to increase attribute count"))); 
            }
        } else {
            return Err(Error::Custom(String::from("Attribute input exists"))); 
        } 
    } else {
        return Err(Error::Custom(String::from("Attribute input error")));
    }
}
