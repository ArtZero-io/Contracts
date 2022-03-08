#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

pub use self::Collection_Management::{
    Collection
};

#[ink::contract]
#[allow(non_snake_case)]
mod Collection_Management {


    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::string::String;
    use ink_storage::Mapping;

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        /// Returned if the address already exists upon registration.
        AddressAlreadyExists,
        CollectionNotExist,
        //OnlyOwner can do
        OnlyOwner,
        OnlyAdmin,
        InvalidCaller,
        InvalidFee,
        InvalidRoyalFee,
        NotEnoughBalance
    }

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(
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
    pub struct Collection {
        owner: AccountId,
        name: String,
        description: String,
        image: String,
        typeCollection: u8,
        isRoyalFee: bool,
        royalFee: u32,          //100 = 1% 10000 = 100%
        isActive: bool
    }

    #[ink(event)]
    pub struct AddNewCollectionEvent {
        contractAddress: AccountId,
        collection_count: u64,
        value: Collection
    }

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct CollectionManagement {
        owner_address: AccountId,
        admin_address: AccountId,
        collection_count: u64,
        adding_fee: Balance,
        collections: Mapping<AccountId, Collection>,    //save collection by contract address
        collections_by_id: Mapping<u64, AccountId>      //save contract address by id
    }

    impl CollectionManagement {

        /// Constructor
        #[ink(constructor)]
        pub fn new(_default_admin_address: AccountId,_default_owner_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|contract: &mut Self| {
                contract.collection_count = 0;
                contract.adding_fee = 1 * 10u128.pow(12);       // 1 AZERO = 1 000 000 000 000 pAZERO (smallest unit)
                contract.owner_address = _default_owner_address;
                contract.admin_address = _default_admin_address;
            })
        }

        /// Add new collection
        #[ink(message)]
        #[ink(payable)]
        pub fn addNewCollection(
            &mut self,
            _name: String,
            _description: String,
            _image: String,
            _typeCollection: u8,
            _contractAddress: AccountId,
            _isRoyalFee: bool,
            _royalFee:u32
        ) -> Result<()> {
            if self.adding_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            if self.collections.get(&_contractAddress).is_some(){
                return Err(Error::AddressAlreadyExists);
            }
            //fee must less than 100%
            if _royalFee >= 10000 {
                return Err(Error::InvalidRoyalFee);
            }
            //Increase collection_count and save the latest id with _contractAddress - for tracking purpose
            self.collection_count += 1;
            self.collections_by_id.insert(&self.collection_count, &_contractAddress);

            let newCollection = Collection {
                owner: self.env().caller(),
                name: _name,
                description: _description,
                image: _image,
                typeCollection: _typeCollection,
                isRoyalFee: _isRoyalFee,
                royalFee: _royalFee,
                isActive: false                 //When first created, isActive = false, only Admin can change to true
            };

            self.collections.insert(&_contractAddress, &newCollection);

            self.env().emit_event(AddNewCollectionEvent {
                contractAddress: _contractAddress,
                collection_count: self.collection_count,
                value: newCollection
            });
            Ok(())
        }

        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(
            &mut self
        ) -> AccountId {
            return self.admin_address;
        }

        /// Update Admin Address
        #[ink(message)]
        pub fn update_admin_address(
            &mut self,
            _admin_address: AccountId
        )  -> Result<()> {
            //Only Owner can update
            if self.env().caller() != self.owner_address{
                return Err(Error::OnlyOwner);
            }
            self.admin_address = _admin_address;
            Ok(())
        }

        /// Get Admin Address
        #[ink(message)]
        pub fn get_owner_address(
            &mut self
        ) -> AccountId {
            return self.owner_address;
        }

        /// Update Owner Address
        #[ink(message)]
        pub fn update_owner_address(
            &mut self,
            _owner_address: AccountId
        )  -> Result<()> {
            //Only Owner can update
            if self.env().caller() != self.owner_address{
                return Err(Error::OnlyOwner);
            }
            self.owner_address = _owner_address;
            Ok(())
        }
        /// Get Admin Address
        #[ink(message)]
        pub fn get_adding_fee(
            &mut self
        ) -> Balance {
            return self.adding_fee;
        }

        /// Update Adding Fee
        #[ink(message)]
        pub fn update_adding_fee(
            &mut self,
            _fee: Balance
        )  -> Result<()> {
            //Only Owner can update
            if self.env().caller() != self.owner_address{
                return Err(Error::OnlyOwner);
            }
            self.adding_fee = _fee;
            Ok(())
        }
        /// Withdraw Fees
        #[ink(message)]
        pub fn withdraw_fee(
            &mut self,
            _value: Balance
        )  -> Result<()> {
            //Only Owner can update
            if self.env().caller() != self.owner_address{
                return Err(Error::OnlyOwner);
            }
            if _value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            if self.env().transfer(self.env().caller(), _value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }

        /// Update Name
        #[ink(message)]
        pub fn update_name(
            &mut self,
            _contractAddress: AccountId,
            _name: String
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&_contractAddress).unwrap();

            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                return Err(Error::InvalidCaller);
             }

            collection.name = _name;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update Description
        #[ink(message)]
        pub fn update_description(
            &mut self,
            _contractAddress: AccountId,
            _description: String
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                return Err(Error::InvalidCaller);
             }
            collection.description = _description;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update Image
        #[ink(message)]
        pub fn update_image(
            &mut self,
            _contractAddress: AccountId,
            _image: String
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                 return Err(Error::InvalidCaller);
             }
            collection.image = _image;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update Type Collection
        #[ink(message)]
        pub fn update_type_collection(
            &mut self,
            _contractAddress: AccountId,
            _typeCollection: u8
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                 return Err(Error::InvalidCaller);
             }
            collection.typeCollection = _typeCollection;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update Is Royal Fee
        #[ink(message)]
        pub fn update_isRoyalFee(
            &mut self,
            _contractAddress: AccountId,
            _isRoyalFee: bool
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                 return Err(Error::InvalidCaller);
             }

            collection.isRoyalFee = _isRoyalFee;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update RoyalFee
        #[ink(message)]
        pub fn update_royal_fee(
            &mut self,
            _contractAddress: AccountId,
            _newFee: u32
        ) -> Result<()>  {
            //fee must less than 100%
            if _newFee >= 10000 {
                  return Err(Error::InvalidFee);
             }
            if self.collections.get(&_contractAddress).is_none(){
                 return Err(Error::CollectionNotExist);
             }

            let mut collection = self.collections.get(&_contractAddress).unwrap();

            if  self.env().caller() != collection.owner &&
                self.env().caller() != self.admin_address {
                return Err(Error::InvalidCaller);
             }

            collection.royalFee = _newFee;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Update Active Status
        #[ink(message)]
        pub fn update_isActive(
            &mut self,
            _contractAddress: AccountId,
            _isActive: bool
        ) -> Result<()>  {
            if self.collections.get(&_contractAddress).is_none(){
                return Err(Error::CollectionNotExist);
            }
            if  self.env().caller() != self.admin_address {
                 return Err(Error::OnlyAdmin);
             }
            let mut collection = self.collections.get(&_contractAddress).unwrap();
            collection.isActive = _isActive;
            self.collections.insert(&_contractAddress, &collection);
            Ok(())
        }

        /// Get Collection by Address
        #[ink(message)]
        pub fn get_colletion_by_address(
            &mut self,
            _contractAddress: AccountId
        ) -> Collection {
            return self.collections.get(&_contractAddress).unwrap();
        }

        /// Get Collection Contract by ID
        #[ink(message)]
        pub fn get_contract_by_id(
            &mut self,
            _id: u64
        ) -> AccountId {
            return self.collections_by_id.get(&_id).unwrap();
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_collection_count(
            &mut self
        ) -> u64 {
            return self.collection_count;
        }

    }

}
