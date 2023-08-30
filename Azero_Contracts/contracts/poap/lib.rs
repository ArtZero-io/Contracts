#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::poap_nft::{
    POAPNft,
    POAPNftRef,
};

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod poap_nft {
    use ink::{
        codegen::{Env, EmitEvent},
        reflect::ContractEventBase
    };
    use ink::prelude::{
        string::{
            String,
        },
        vec::Vec,
    };
    use openbrush::{
        contracts::ownable::*,
        contracts::psp34::{
            extensions::{
                enumerable::*,
                metadata::*,
                burnable::*,
            },
            Internal,
        },
        
        traits::{
            Storage,
            DefaultEnv
        },
        modifiers,
    };
    use artzero_project::{
        traits::{
            psp34_standard::*,
            admin::*,
            upgradable::*,
            error::Error,
        }
    };

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct POAPNft {
        #[storage_field]
        psp34: psp34::Data<enumerable::Balances>,
        #[storage_field]
        metadata: metadata::Data,
        #[storage_field]
        ownable: ownable::Data,
        #[storage_field]
        manager: artzero_project::impls::psp34_standard::data::Manager,
        #[storage_field]
        admin_data: artzero_project::impls::admin::data::Data,
        #[storage_field]
        upgradable_data: artzero_project::impls::upgradable::data::Data,
    }

    /// - Specify transfer event.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
    }

    /// - Specify approval event.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        id: Id,
        approved: bool,
    }

    pub type Event = <POAPNft as ContractEventBase>::Type;

    impl Ownable for POAPNft {}
    impl PSP34 for POAPNft {}
    impl PSP34Metadata for POAPNft {}
    impl PSP34Enumerable for POAPNft {}
    impl Psp34Traits for POAPNft {}
    impl AdminTrait for POAPNft {}
    impl UpgradableTrait for POAPNft {}

    impl Internal for POAPNft {

        /// - Impliment Transfer emit event because Openbrush doesn't.
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
        ) {
            POAPNft::emit_event(
                self.env(),
                Event::Transfer(Transfer {
                    from: _from,
                    to: _to,
                    id: _id,
                }),
            );
        }

        /// - Impliment Approval emit event because Openbrush doesn't.
        fn _emit_approval_event(
            &self,
            _from: AccountId,
            _to: AccountId,
            _id: Option<Id>,
            _approved: bool,
        ) {
            POAPNft::emit_event(
                self.env(),
                Event::Approval(Approval {
                    from: Some(_from),
                    to: Some(_to),
                    id: _id.unwrap(),
                    approved: _approved,
                }),
            );
        }
    }

    impl PSP34Burnable for POAPNft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();

            if let Some(token_owner) = self.owner_of(id.clone()) {
                if token_owner != account {
                    return Err(PSP34Error::Custom(String::from("not token owner").into_bytes()));
                }

                let allowance = self.allowance(account,caller,Some(id.clone()));

                if caller == account || allowance {
                    self.manager.locked_tokens.remove(&id);
                    if let Some(locked_token_count) = self.manager.locked_token_count.checked_sub(1) {
                        self.manager.locked_token_count = locked_token_count;
                        self._burn_from(account, id)
                    } else {
                        return Err(PSP34Error::Custom(String::from("Locked token count error").into_bytes()));
                    }
                } else {
                    return Err(PSP34Error::Custom(String::from("caller is not token owner or approved").into_bytes()));
                }
            } else {
                return Err(PSP34Error::Custom(String::from("No token owner found").into_bytes()));
            }
        }
    }

    impl POAPNft {
        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String, total_amount: u64) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(contract_owner);
            instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
            instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
            instance.manager.total_amount = total_amount;
            instance
        }

        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        #[ink(message)]
        pub fn get_total_amount(&self) -> u64 {
            self.manager.total_amount
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                if last_token_id > self.manager.total_amount {
                    return Err(Error::Custom(String::from("Reached Total Supply")));
                }
                self.manager.last_token_id = last_token_id;
                if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                    return Err(Error::Custom(String::from("Cannot mint")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_amount(&mut self, receiver: AccountId, amount: u64) -> Result<(), Error> {
            for i in 0..amount {
                if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                    if last_token_id > self.manager.total_amount {
                        return Err(Error::Custom(String::from("Reached Total Supply")));
                    }
                    if self._mint_to(receiver, Id::U64(self.manager.last_token_id)).is_err(){
                        return Err(Error::Custom(String::from("Cannot mint")));
                    }
                    self.manager.last_token_id = last_token_id;
                } else {
                    return Err(Error::Custom(String::from("Cannot increase last token id")));
                }
            }
            return Ok(());
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), Error> {
            let caller = self.env().caller();
            if let Some(last_token_id) = self.manager.last_token_id.checked_add(1) {
                if last_token_id > self.manager.total_amount {
                    return Err(Error::Custom(String::from("Reached Total Supply")));
                }
                self.manager.last_token_id = last_token_id;
                if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                    return Err(Error::Custom(String::from("Cannot mint")));
                }
                if self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata).is_err(){
                    return Err(Error::Custom(String::from("Cannot set attributes")));
                }
                return Ok(());
            } else {
                return Err(Error::Custom(String::from("Cannot increase last token id")));
            }
        }
    }
}
