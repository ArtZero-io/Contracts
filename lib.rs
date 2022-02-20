#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod marketplace {
    use ink_env::call::{
        build_call,
        utils::ReturnType,
        ExecutionInput,
    };
    use ink_prelude::vec::Vec;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        },
        Mapping,
    };
    use scale::Output;

    type VendorId = u128;
    type TokenId = u128;

    pub struct Vendor {
        userName: String,
        isActive: bool,
        walletAddress: AccountId
    }

    pub struct Token {
        ownerId: u128,
        producerId: u128,
        isForSale: bool,
        price: u128,
        typeId: u8
    }

    pub struct Vendors {
        /// Just store all vendor ids packed.
        vendors: Vec<VendorId>,
        next_id: VendorId,
    }

    pub struct Tokens {
        /// Just store all token ids packed.
        tokens: Vec<TokenId>,
        next_id: TokenId,
    }

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Marketplace {
        /// Stores a single `bool` value on the storage.
        vendors: Mapping<VendorId, Vendor>,
        vendor_list: Vendors
    }

    impl Marketplace {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new() -> Self {
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        #[ink(message)]
        pub fn add_vendor(
            &mut self,
            vendor: Vendor
        ) {
            self.ensure_caller_is_owner();
            let vendor_id = self.vendor_list.next_id;
            self.vendor_list.next_id =
                vendor_id.checked_add(1).expect("Vendor ids exhausted.");
            self.vendors.insert(vendor_id, &vendor);
            self.vendor_list.vendors.push(vendor_id);
        }

    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let marketplace = Marketplace::default();
            assert_eq!(marketplace.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut marketplace = Marketplace::new(false);
            assert_eq!(marketplace.get(), false);
            marketplace.flip();
            assert_eq!(marketplace.get(), true);
        }
    }
}
