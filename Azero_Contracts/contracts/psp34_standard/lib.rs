#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
pub use self::psp34_nft::{
    Psp34Nft,
    Psp34NftRef,
};

#[allow(clippy::let_unit_value)]
#[allow(clippy::inline_fn_without_body)]
#[allow(clippy::too_many_arguments)]
#[openbrush::contract]
pub mod psp34_nft {
    use ink::{
            codegen::{Env, EmitEvent},
            reflect::ContractEventBase,
            prelude::{
            string::{
                String,
            },
            vec::Vec,
        },
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
            error::Error,
        }
    };

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

    #[derive(Default, Storage)]
    #[ink(storage)]
    pub struct Psp34Nft {
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
    }

    impl Ownable for Psp34Nft {}
    impl PSP34 for Psp34Nft {}
    impl PSP34Metadata for Psp34Nft {}
    impl PSP34Enumerable for Psp34Nft {}
    impl Psp34Traits for Psp34Nft {}
    impl AdminTrait for Psp34Nft {}
    /// - Needed for Openbrush internal event emission implementations.
    pub type Event = <Psp34Nft as ContractEventBase>::Type;

    impl Internal for Psp34Nft {

        /// - Impliment Transfer emit event because Openbrush doesn't.
        fn _emit_transfer_event(
            &self,
            _from: Option<AccountId>,
            _to: Option<AccountId>,
            _id: Id,
        ) {
            Psp34Nft::emit_event(
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
            Psp34Nft::emit_event(
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

    impl PSP34Burnable for Psp34Nft {
        #[ink(message)]
        fn burn(&mut self, account: AccountId, id: Id) -> Result<(), PSP34Error> {
            let caller = Self::env().caller();
            let token_owner = self.owner_of(id.clone()).unwrap();
            if token_owner != account {
                return Err(PSP34Error::Custom(String::from("not token owner").into_bytes()))
            }

            let allowance = self.allowance(account,caller,Some(id.clone()));

            if caller == account || allowance {
                self.manager.locked_tokens.remove(&id);
                self.manager.locked_token_count = self.manager.locked_token_count.checked_sub(1).unwrap();
                self._burn_from(account, id)
            } else{
                Err(PSP34Error::Custom(String::from("caller is not token owner or approved").into_bytes()))
            }
        }
    }

    impl Psp34Nft {

        /// - Function for internal _emit_event implementations.
        pub fn emit_event<EE: EmitEvent<Self>>(emitter: EE, event: Event) {
            emitter.emit_event(event);
        }

        #[ink(constructor)]
        pub fn new(contract_owner: AccountId, name: String, symbol: String) -> Self {
            let mut instance = Self::default();
            instance._init_with_owner(contract_owner);
            instance._set_attribute(Id::U8(0), String::from("name").into_bytes(), name.into_bytes());
            instance._set_attribute(Id::U8(0), String::from("symbol").into_bytes(), symbol.into_bytes());
            instance
        }

        /// This function let NFT Contract Owner to mint a new NFT without providing NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            self.manager.last_token_id = self.manager.last_token_id.checked_add(1).unwrap();
            if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                return Err(Error::Custom(String::from("Cannot mint")))
            }
            Ok(())
        }

        /// This function let NFT Contract Owner to mint a new NFT with NFT Traits/Attributes
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn mint_with_attributes(&mut self, metadata: Vec<(String, String)>) -> Result<(), Error> {
            let caller = self.env().caller();
            self.manager.last_token_id = self.manager.last_token_id.checked_add(1).unwrap();
            if self._mint_to(caller, Id::U64(self.manager.last_token_id)).is_err(){
                return Err(Error::Custom(String::from("Cannot mint")))
            }
            if self.set_multiple_attributes(Id::U64(self.manager.last_token_id), metadata).is_err(){
                return Err(Error::Custom(String::from("Cannot set attributes")))
            }
            Ok(())
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {

        use super::*;
        use crate::psp34_nft::PSP34Error::Custom;
        use ink_e2e::{
            build_message,
        };
        use openbrush::contracts::psp34::psp34_external::PSP34;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// HAPPY TRANSFER
        /// - Test if customized transfer function works correctly.
        /// - When transfer, credentials are revoked.
        /// - Test that register function works correctly.
        /// - Test that transfer events are properly emitted.
        /// - Test that get_credential() and get_collection() works..
        #[ink_e2e::test]
        async fn happye2e_mint_register_transfer(
            mut client: ink_e2e::Client<C, E>,
        ) -> E2EResult<()> {


            let alice_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
            let bob_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);
            let charlie_account = ink_e2e::account_id(ink_e2e::AccountKeyring::Charlie);

            let constructor = Psp34NftRef::new(
                alice_account.clone(),
                "Interlock Network Universal Access NFT".to_string(),
                "ILOCK-UANFT".to_string(),
            );
            let uanft_contract_acct_id = client
                .instantiate("psp34_nft", &ink_e2e::alice(), constructor, 0, None)
                .await.expect("instantiate failed").account_id;
        
            let mint_msg = build_message::<Psp34NftRef>(uanft_contract_acct_id.clone())
                .call(|contract| contract.mint());
            let mint_response = client
                .call(&ink_e2e::alice(), mint_msg, 0, None).await.expect("mint fail");
            
            // filter for transfer mint event
            let contract_emitted_transfer = mint_response
                .events
                .iter()
                .find(|event| {
                    event
                        .as_ref()
                        .expect("expected event")
                        .event_metadata()
                        .event()
                        == "ContractEmitted" &&
                        String::from_utf8_lossy(
                            event.as_ref().expect("bad event").bytes()).to_string()
                       .contains("Psp34Nft::Transfer")
                })
                .expect("Expect ContractEmitted event")
                .unwrap();
            println!("chirp");

            // decode to the expected event type (skip field_context)
            let transfer_event = contract_emitted_transfer.field_bytes();
            let decoded_transfer =
                <Transfer as scale::Decode>::decode(&mut &transfer_event[34..]).expect("invalid data");

            // destructor decoded eapproval
            let Transfer { from, to, id } = decoded_transfer;

            // assert with the expected value
            assert_eq!(from, None, "encountered invalid Transfer.to");
            assert_eq!(to, Some(alice_account), "encountered invalid Transfer.from");
            assert_eq!(id, Id::U64(1), "encountered invalid Transfer.id");  
            
            Ok(())
        }
    }
}

