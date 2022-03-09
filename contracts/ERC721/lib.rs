//! # ERC-721
//!
//! This is an ERC-721 Token implementation.
//!

#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc721 {
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
        Mapping,
    };

    use scale::{
        Decode,
        Encode,
    };

    /// A token ID.
    pub type TokenId = u32;

    #[derive(
        Copy,
        Clone,
        Debug,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        Default,
        PackedLayout,
        SpreadLayout,
        scale::Encode,
        scale::Decode,
    )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct Whitelist {
        whitelist_amount: u32,
        claimed_amount: u32
    }

    #[ink(storage)]
    #[derive(Default, SpreadAllocate)]
    pub struct Erc721 {
        /// Mapping from token to owner.
        token_owner: Mapping<TokenId, AccountId>,
        /// Mapping from token to approvals users.
        token_approvals: Mapping<TokenId, AccountId>,
        /// Mapping from owner to number of owned token.
        owned_tokens_count: Mapping<AccountId, u32>,
        /// Mapping from owner to operator approvals.
        operator_approvals: Mapping<(AccountId, AccountId), ()>,
        //===== ADDED
        //Total Token number to Mint
        total_tokens: u32,
        //To manage 10K minting
        token_count: TokenId,
        //Who got free mint
        whitelists: Mapping<AccountId,Whitelist>,
        whitelist_count: u32,
        whitelist_accounts: Mapping<u32,AccountId>,
        //administractor account
        admin: AccountId,
        //Pre_launch Minting Fee - 99 AZERO
        fee_1: Balance,
        //Launch Minting Fee - 199 AZERO
        fee_2: Balance,
        //To what amount the fee_1 is applied, after that fee_2 - first 5K
        amount_1: u32,
        //is Minting started
        // 0: not started
        // 1: started until amount_1 reached
        // 2: started until total_tokens reached
        mint_mode: u8
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        NotApproved,
        TokenExists,
        TokenNotFound,
        CannotInsert,
        CannotFetchValue,
        NotAllowed,
        InvalidInput,
        OnlyAdmin,
        ClaimedAll,
        TokenLimitReached,
        TokenLimitReachedMode1,
        InvalidFee,
        NotMintTime
    }

    /// Event emitted when a token transfer occurs.
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event emitted when a token approve occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        id: TokenId,
    }

    /// Event emitted when an operator is enabled or disabled for an owner.
    /// The operator can manage all NFTs of the owner.
    #[ink(event)]
    pub struct ApprovalForAll {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        operator: AccountId,
        approved: bool,
    }

    impl Erc721 {
        /// Creates a new ERC-721 token contract.
        #[ink(constructor)]
        pub fn new(_admin: AccountId,_fee_1: Balance, _fee_2: Balance, _amount_1: u32) -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                contract.token_count = 0;
                contract.whitelist_count = 0;
                contract.admin = _admin;
                contract.total_tokens = 10000;
                contract.fee_1 = _fee_1;
                contract.fee_2 = _fee_2;
                contract.amount_1 = _amount_1;
                contract.mint_mode = 0;
            })
        }
        /*
            WHITELIST FUNCTIONS =============
        */
        /// Add new whitelist
        #[ink(message)]
        pub fn add_whitelist(
            &mut self,
            _account: AccountId,
            _whitelist_amount: u32
        ) -> Result<(), Error> {
            //Only Owner can update
            if self.env().caller() != self.admin{
                return Err(Error::OnlyAdmin);
            }

            //fee must less than total tokens
            if _whitelist_amount > self.total_tokens {
                return Err(Error::InvalidInput);
            }
            if self.whitelists.get(&_account).is_some(){
                return Err(Error::InvalidInput);
            }

            self.whitelist_count += 1;
            self.whitelist_accounts.insert(&self.whitelist_count, &_account);

            let whitelist = Whitelist {
                whitelist_amount: _whitelist_amount,
                claimed_amount: 0
            };

            self.whitelists.insert(&_account, &whitelist);

            Ok(())
        }

        /// Update Whitelist Amount - Only Admin
        #[ink(message)]
        pub fn update_whitelist_amount(
            &mut self,
            _account: AccountId,
            _whitelist_amount: u32
        ) -> Result<(), Error>  {

            if self.whitelists.get(&_account).is_none(){
                 return Err(Error::InvalidInput);
             }

            let mut whitelist = self.whitelists.get(&_account).unwrap();

            if  self.env().caller() == self.admin {
                    whitelist.whitelist_amount = _whitelist_amount;
                    self.whitelists.insert(&_account, &whitelist);
                    Ok(())
             }
             else{
                 return Err(Error::InvalidInput);
             }
        }

        /// Get Whitelist by AccountID
        #[ink(message)]
        pub fn get_whitelist_account(
            &mut self,
            _id: u32
        ) -> AccountId {
            return self.whitelist_accounts.get(&_id).unwrap();
        }

        /// Get Whitelist by AccountID
        #[ink(message)]
        pub fn get_whitelist(
            &mut self,
            _account: AccountId
        ) -> Whitelist {
            return self.whitelists.get(&_account).unwrap();
        }
        /// Get Whitelist Count
        #[ink(message)]
        pub fn get_whitelist_count(
            &mut self
        ) -> u32 {
            return self.whitelist_count;
        }

        /// Set mint_mode
        #[ink(message)]
        pub fn set_mint_mode(
            &mut self,
            _mint_mode: u8
        ) -> Result<(), Error> {
            //Only Owner can update
            if self.env().caller() != self.admin{
                return Err(Error::OnlyAdmin);
            }
            self.mint_mode = _mint_mode;
            Ok(())
        }
        /// Get mint_mode
        #[ink(message)]
        pub fn get_mint_mode(
            &mut self
        ) -> u8 {
            return self.mint_mode;
        }
        /// Get fee_1
        #[ink(message)]
        pub fn get_fee_1(
            &mut self
        ) -> Balance {
            return self.fee_1;
        }
        /// Get fee_2
        #[ink(message)]
        pub fn get_fee_2(
            &mut self
        ) -> Balance {
            return self.fee_2;
        }
        /// Get amount_1
        #[ink(message)]
        pub fn get_amount_1(
            &mut self
        ) -> u32 {
            return self.amount_1;
        }
        /*
            END OF WHITELIST FUNCTIONS =============
        */
        /*
            MINT FUNCTIONS =============
        */
        /// Creates a new token.
        #[ink(message)]
        pub fn whitelist_mint(&mut self) -> Result<(), Error> {

            if self.mint_mode == 0 {
                return Err(Error::NotMintTime);
            }

            let caller = self.env().caller();

            if self.whitelists.get(&caller).is_none(){
                 return Err(Error::InvalidInput);
             }

            let mut caller_info = self.whitelists.get(&caller).unwrap();
            if caller_info.whitelist_amount <= caller_info.claimed_amount {
                return Err(Error::ClaimedAll);
            }
            caller_info.claimed_amount += 1;
            self.token_count += 1;

            self.add_token_to(&caller, self.token_count)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                id: self.token_count,
            });
            Ok(())
        }

        #[ink(message)]
        #[ink(payable)]
        pub fn paid_mint(&mut self) -> Result<(), Error> {
            let caller = self.env().caller();
            //Mint is disabled
            if self.mint_mode == 0 {
                return Err(Error::NotMintTime);
            }
            //Mode 1 - mint till amount_1 reached
            if  self.mint_mode == 1 &&
                self.token_count >= self.amount_1
            {
                return Err(Error::TokenLimitReachedMode1);
            }

            if  self.mint_mode == 1 &&
                self.fee_1 != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            //Mode 2 - mint till total_tokens reached
            if  self.mint_mode == 2 &&
                self.fee_2 != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }

            if self.token_count >= self.total_tokens {
                return Err(Error::TokenLimitReached);
            }
            self.token_count += 1;

            self.add_token_to(&caller, self.token_count)?;
            self.env().emit_event(Transfer {
                from: Some(AccountId::from([0x0; 32])),
                to: Some(caller),
                id: self.token_count,
            });
            Ok(())
        }
        /*
            END OF MINT FUNCTIONS =============
        */
        /// Returns the balance of the owner.
        ///
        /// This represents the amount of unique tokens the owner has.
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> u32 {
            self.balance_of_or_zero(&owner)
        }

        /// Returns the owner of the token.
        #[ink(message)]
        pub fn owner_of(&self, id: TokenId) -> Option<AccountId> {
            self.token_owner.get(&id)
        }

        /// Returns the approved account ID for this token if any.
        #[ink(message)]
        pub fn get_approved(&self, id: TokenId) -> Option<AccountId> {
            self.token_approvals.get(&id)
        }

        /// Returns `true` if the operator is approved by the owner.
        #[ink(message)]
        pub fn is_approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.approved_for_all(owner, operator)
        }

        /// Approves or disapproves the operator for all tokens of the caller.
        #[ink(message)]
        pub fn set_approval_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            self.approve_for_all(to, approved)?;
            Ok(())
        }

        /// Approves the account to transfer the specified token on behalf of the caller.
        #[ink(message)]
        pub fn approve(&mut self, to: AccountId, id: TokenId) -> Result<(), Error> {
            self.approve_for(&to, id)?;
            Ok(())
        }

        /// Transfers the token from the caller to the given destination.
        #[ink(message)]
        pub fn transfer(
            &mut self,
            destination: AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            self.transfer_token_from(&caller, &destination, id)?;
            Ok(())
        }

        /// Transfer approved or owned token.
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            self.transfer_token_from(&from, &to, id)?;
            Ok(())
        }

        /// Deletes an existing token. Only the owner can burn the token.
        #[ink(message)]
        pub fn burn(&mut self, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;

            let owner = token_owner.get(&id).ok_or(Error::TokenNotFound)?;
            if owner != caller {
                return Err(Error::NotOwner)
            };

            let count = owned_tokens_count
                .get(&caller)
                .map(|c| c - 1)
                .ok_or(Error::CannotFetchValue)?;
            owned_tokens_count.insert(&caller, &count);
            token_owner.remove(&id);

            self.env().emit_event(Transfer {
                from: Some(caller),
                to: Some(AccountId::from([0x0; 32])),
                id,
            });

            Ok(())
        }

        /// Transfers token `id` `from` the sender to the `to` `AccountId`.
        fn transfer_token_from(
            &mut self,
            from: &AccountId,
            to: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if !self.exists(id) {
                return Err(Error::TokenNotFound)
            };
            if !self.approved_or_owner(Some(caller), id) {
                return Err(Error::NotApproved)
            };
            self.clear_approval(id);
            self.remove_token_from(from, id)?;
            self.add_token_to(to, id)?;
            self.env().emit_event(Transfer {
                from: Some(*from),
                to: Some(*to),
                id,
            });
            Ok(())
        }

        /// Removes token `id` from the owner.
        fn remove_token_from(
            &mut self,
            from: &AccountId,
            id: TokenId,
        ) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;

            if token_owner.get(&id).is_none() {
                return Err(Error::TokenNotFound)
            }

            let count = owned_tokens_count
                .get(&from)
                .map(|c| c - 1)
                .ok_or(Error::CannotFetchValue)?;
            owned_tokens_count.insert(&from, &count);
            token_owner.remove(&id);

            Ok(())
        }

        /// Adds the token `id` to the `to` AccountID.
        fn add_token_to(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let Self {
                token_owner,
                owned_tokens_count,
                ..
            } = self;

            if token_owner.get(&id).is_some() {
                return Err(Error::TokenExists)
            }

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };

            let count = owned_tokens_count.get(to).map(|c| c + 1).unwrap_or(1);

            owned_tokens_count.insert(to, &count);
            token_owner.insert(&id, to);

            Ok(())
        }

        /// Approves or disapproves the operator to transfer all tokens of the caller.
        fn approve_for_all(
            &mut self,
            to: AccountId,
            approved: bool,
        ) -> Result<(), Error> {
            let caller = self.env().caller();
            if to == caller {
                return Err(Error::NotAllowed)
            }
            self.env().emit_event(ApprovalForAll {
                owner: caller,
                operator: to,
                approved,
            });

            if approved {
                self.operator_approvals.insert((&caller, &to), &());
            } else {
                self.operator_approvals.remove((&caller, &to));
            }

            Ok(())
        }

        /// Approve the passed `AccountId` to transfer the specified token on behalf of the message's sender.
        fn approve_for(&mut self, to: &AccountId, id: TokenId) -> Result<(), Error> {
            let caller = self.env().caller();
            let owner = self.owner_of(id);
            if !(owner == Some(caller)
                || self.approved_for_all(owner.expect("Error with AccountId"), caller))
            {
                return Err(Error::NotAllowed)
            };

            if *to == AccountId::from([0x0; 32]) {
                return Err(Error::NotAllowed)
            };

            if self.token_approvals.get(&id).is_some() {
                return Err(Error::CannotInsert)
            } else {
                self.token_approvals.insert(&id, to);
            }

            self.env().emit_event(Approval {
                from: caller,
                to: *to,
                id,
            });

            Ok(())
        }

        /// Removes existing approval from token `id`.
        fn clear_approval(&mut self, id: TokenId) {
            self.token_approvals.remove(&id);
        }

        // Returns the total number of tokens from an account.
        fn balance_of_or_zero(&self, of: &AccountId) -> u32 {
            self.owned_tokens_count.get(of).unwrap_or(0)
        }

        /// Gets an operator on other Account's behalf.
        fn approved_for_all(&self, owner: AccountId, operator: AccountId) -> bool {
            self.operator_approvals.get((&owner, &operator)).is_some()
        }

        /// Returns true if the `AccountId` `from` is the owner of token `id`
        /// or it has been approved on behalf of the token `id` owner.
        fn approved_or_owner(&self, from: Option<AccountId>, id: TokenId) -> bool {
            let owner = self.owner_of(id);
            from != Some(AccountId::from([0x0; 32]))
                && (from == owner
                    || from == self.token_approvals.get(&id)
                    || self.approved_for_all(
                        owner.expect("Error with AccountId"),
                        from.expect("Error with AccountId"),
                    ))
        }

        /// Returns true if token `id` exists or false if it does not.
        fn exists(&self, id: TokenId) -> bool {
            self.token_owner.get(&id).is_some()
        }
    }

}
