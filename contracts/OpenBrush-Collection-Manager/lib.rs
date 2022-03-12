#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

#[brush::contract]
pub mod artzero_collection_manager {
    use ink_prelude::string::String;
    use brush::contracts::ownable::*;
    use brush::modifiers;
    use ink_storage::{
        traits::{
            PackedLayout,
            SpreadAllocate,
            SpreadLayout,
        }
    };
    use ink_prelude::vec::Vec;
    use ink_storage::Mapping;

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        /// Custom error type for cases if writer of traits added own restrictions
        Custom(String),
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

    impl From<OwnableError> for Error {
        fn from(ownable: OwnableError) -> Self {
            match ownable {
                OwnableError::CallerIsNotOwner => Error::Custom(String::from("O::CallerIsNotOwner")),
                OwnableError::NewOwnerIsZero => Error::Custom(String::from("O::NewOwnerIsZero")),
            }
        }
    }

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
        collection_owner: AccountId,                //to receive Royal Fee - OnlyAdmin can update
        nft_contract_address: AccountId,            //OnlyAdmin can update
        name: Vec<u8>,
        description: Vec<u8>,
        avatar_image: Vec<u8>,           //IPFS Hash
        header_image: Vec<u8>,           //IPFS Hash
        type_collection: u8,             //0 - PSP34 ERC721 ; 1 - ERC1155 PSP1155 OnlyAdmin
        is_collect_royal_fee: bool,      //OnlyAdmin can update
        royal_fee: u32,                  //100 = 1% 10000 = 100% OnlyAdmin
        is_active: bool                  // OnlyAdmin can update
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroCollectionManager{
        #[OwnableStorageField]
        ownable: OwnableData,

        admin_address: AccountId,
        collection_count: u64,
        adding_fee: Balance,
        collections: Mapping<AccountId, Collection>,    //save collection by contract address
        collections_by_id: Mapping<u64, AccountId>      //save contract address by id
    }

    impl Ownable for ArtZeroCollectionManager {}

    impl ArtZeroCollectionManager {
        #[ink(constructor)]
        pub fn new(admin_address: AccountId,owner_address: AccountId) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.collection_count = 0;
                instance.adding_fee = 1 * 10u128.pow(12);       // 1 AZERO = 1 000 000 000 000 pAZERO (smallest unit)
                instance._init_with_owner(owner_address);
                instance.admin_address = admin_address;
            })
        }

        /// Add new collection - with fee in AZERO 1% = 100 - anyone can add
        #[ink(message)]
        #[ink(payable)]
        pub fn add_new_collection(
            &mut self,
            collection_owner: AccountId,
            nft_contract_address: AccountId,
            name: Vec<u8>,
            description: Vec<u8>,
            avatar_image: Vec<u8>,
            header_image: Vec<u8>,
            type_collection: u8,
            is_collect_royal_fee: bool,
            royal_fee:u32
        ) -> Result<(), Error> {
            if self.adding_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            if self.collections.get(&nft_contract_address).is_some(){
                return Err(Error::AddressAlreadyExists);
            }
            //fee must less than 100%
            if royal_fee >= 10000 {
                return Err(Error::InvalidRoyalFee);
            }
            //Increase collection_count and save the latest id with nft_contract_address - for tracking purpose
            self.collection_count += 1;
            self.collections_by_id.insert(&self.collection_count, &nft_contract_address);

            let new_collection = Collection {
                collection_owner,
                nft_contract_address,
                name,
                description,
                avatar_image,
                header_image,
                type_collection,
                is_collect_royal_fee,
                royal_fee,
                is_active: false
            };

            self.collections.insert(&nft_contract_address, &new_collection);

            Ok(())
        }
        /* SETTERS */
        /// Update Owner of Collecion - Only Admin can change
        #[ink(message)]
        pub fn update_collection_owner(
            &mut self,
            contract_address: AccountId,
            new_owner: AccountId
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();

            if  self.env().caller() == self.admin_address {
                    collection.collection_owner = new_owner;
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else{
                 return Err(Error::OnlyAdmin);
             }
        }
        /// Update nft_contract_address - Only Admin can change
        #[ink(message)]
        pub fn update_nft_contract_address(
            &mut self,
            contract_address: AccountId,
            nft_contract_address: AccountId
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();

            if  self.env().caller() == self.admin_address {
                collection.nft_contract_address = nft_contract_address;
                self.collections.insert(&contract_address, &collection);
                Ok(())
             }
             else{
                 return Err(Error::OnlyAdmin);
             }
        }
        /// Update Name - admin and collection owner can change
        #[ink(message)]
        pub fn update_name(
            &mut self,
            contract_address: AccountId,
            name: String
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();

            if  self.env().caller() == collection.collection_owner ||
                self.env().caller() == self.admin_address {
                    collection.name = name.into_bytes();
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else{
                 return Err(Error::InvalidCaller);
             }
        }

        /// Update Description - admin and collection owner can change
        #[ink(message)]
        pub fn update_description(
            &mut self,
            contract_address: AccountId,
            description: String
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == collection.collection_owner ||
                self.env().caller() == self.admin_address {
                    collection.description = description.into_bytes();
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else
             {
                return Err(Error::InvalidCaller);
             }

        }

        /// Update Avatar Image - admin and collection owner can change
        #[ink(message)]
        pub fn update_avatar_image(
            &mut self,
            contract_address: AccountId,
            avatar_image: String
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == collection.collection_owner ||
                self.env().caller() == self.admin_address {
                    collection.avatar_image = avatar_image.into_bytes();
                    self.collections.insert(&contract_address, &collection);
                    Ok(())

             }
             else{
                 return Err(Error::InvalidCaller);
             }

        }

        /// Update header Image - admin and collection owner can change
        #[ink(message)]
        pub fn update_header_image(
            &mut self,
            contract_address: AccountId,
            header_image: String
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == collection.collection_owner ||
                self.env().caller() == self.admin_address {
                    collection.header_image = header_image.into_bytes();
                    self.collections.insert(&contract_address, &collection);
                    Ok(())

             }
             else{
                 return Err(Error::InvalidCaller);
             }

        }

        /// Update Type Collection - only Admin can change - 0 - PSP34 ERC721 ; 1 - ERC1155 PSP1155
        #[ink(message)]
        pub fn update_type_collection(
            &mut self,
            contract_address: AccountId,
            type_collection: u8
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == self.admin_address {
                    collection.type_collection = type_collection;
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else{
                 return Err(Error::OnlyAdmin);
             }

        }

        /// Update Is Royal Fee - Only Admin can change
        #[ink(message)]
        pub fn update_is_collect_royal_fee(
            &mut self,
            contract_address: AccountId,
            is_collect_royal_fee: bool
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == self.admin_address {
                    collection.is_collect_royal_fee = is_collect_royal_fee;
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else{
                 return Err(Error::OnlyAdmin);
             }
        }

        /// Update royal_fee - Only Admin can change
        #[ink(message)]
        pub fn update_royal_fee(
            &mut self,
            contract_address: AccountId,
            new_fee: u32
        ) -> Result<(), Error>  {
            //fee must less than 100%
            if new_fee >= 10000 {
                  return Err(Error::InvalidFee);
             }
            if self.collections.get(&contract_address).is_none(){
                 return Err(Error::CollectionNotExist);
             }

            let mut collection = self.collections.get(&contract_address).unwrap();

            if  self.env().caller() == self.admin_address {
                    collection.royal_fee = new_fee;
                    self.collections.insert(&contract_address, &collection);
                    Ok(())
             }
             else{
                 return Err(Error::OnlyAdmin);
             }
        }

        /// Update Active Status - only Admin can change
        /// When its active, collection will be shown on the UI and will be tradable
        #[ink(message)]
        pub fn update_is_active(
            &mut self,
            contract_address: AccountId,
            is_active: bool
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            if  self.env().caller() != self.admin_address {
                 return Err(Error::OnlyAdmin);
             }
            let mut collection = self.collections.get(&contract_address).unwrap();
            collection.is_active = is_active;
            self.collections.insert(&contract_address, &collection);
            Ok(())
        }

        /* OWNER FUNCTIONS */
        /// Update Adding Collection Fee - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_adding_fee(&mut self,new_fee: Balance)  -> Result<(), Error> {
            self.adding_fee = new_fee;
            Ok(())
        }
        /// Update Admin Address - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_admin_address(
            &mut self,
            admin_address: AccountId
        )  -> Result<(), Error> {
            self.admin_address = admin_address;
            Ok(())
        }
        /// Withdraw Fees - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn withdraw_fee(&mut self,value: Balance)  -> Result<(), Error> {
            if value > self.env().balance() {
                return Err(Error::NotEnoughBalance);
            }
            if self.env().transfer(self.env().caller(), value).is_err() {
                panic!(
                    "error withdraw_fee"
                )
            }
            Ok(())
        }

        /* GETTERS */
        /// Get Collection Information by Address
        #[ink(message)]
        pub fn get_colletion_by_address(&self,nft_contract_address: AccountId) -> Collection {
            return self.collections.get(&nft_contract_address).unwrap();
        }

        /// Get Collection Contract by ID
        #[ink(message)]
        pub fn get_contract_by_id(
            &self,
            id: u64
        ) -> AccountId {
            return self.collections_by_id.get(&id).unwrap();
        }

        /// Get Collection Count
        #[ink(message)]
        pub fn get_collection_count(
            &self
        ) -> u64 {
            return self.collection_count;
        }
        ///Get Adding Fee
        #[ink(message)]
        pub fn get_adding_fee(
            &self
        ) -> Balance {
            return self.adding_fee;
        }
        /// Get Admin Address
        #[ink(message)]
        pub fn get_admin_address(
            &self
        ) -> AccountId {
            return self.admin_address;
        }



    }
}
