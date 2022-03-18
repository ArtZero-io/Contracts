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
    use psp34_nft::psp34_nft::Psp34NftRef;
    use ink_lang::ToAccountId;


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
        contract_type: u8,               //1 - PSP34 ERC721 Manual; 2 - PSP34 ERC721 Auto
        is_collect_royal_fee: bool,      //OnlyAdmin can update
        royal_fee: u32,                  //100 = 1% 10000 = 100% OnlyAdmin
        is_active: bool,                 // OnlyAdmin can update
        show_on_chain_metadata: bool
    }

    #[derive(Default, SpreadAllocate, OwnableStorage)]
    #[ink(storage)]
    pub struct ArtZeroCollectionManager{
        #[OwnableStorageField]
        ownable: OwnableData,

        standard_nft_hash: Hash,

        admin_address: AccountId,
        collection_count: u64,
        adding_fee: Balance,
        collections: Mapping<AccountId, Collection>,    //save collection by contract address
        collections_by_id: Mapping<u64, AccountId>,      //save contract address by id
        collections_by_owner: Mapping<AccountId, Vec<AccountId>>,      //save contract address by owner ID
        max_royal_fee_rate: u32
    }

    impl Ownable for ArtZeroCollectionManager {}

    #[brush::trait_definition]
    pub trait CrossArtZeroCollection {
        #[ink(message)]
        fn get_royal_fee(&self,nft_contract_address: AccountId) -> u32;
        #[ink(message)]
        fn is_active(&self,nft_contract_address: AccountId) -> bool;
        #[ink(message)]
        fn get_contract_type(&self,nft_contract_address: AccountId) -> u8;
        #[ink(message)]
        fn get_collection_owner(&self,nft_contract_address: AccountId) -> Option<AccountId>;
    }

    impl ArtZeroCollectionManager {
        #[ink(constructor)]
        pub fn new(admin_address: AccountId,owner_address: AccountId, standard_nft_hash: Hash) -> Self {
            ink_lang::codegen::initialize_contract(|instance: &mut Self| {
                instance.collection_count = 0;
                instance.adding_fee = 1 * 10u128.pow(12);       // 1 AZERO = 1 000 000 000 000 pAZERO (smallest unit)
                instance._init_with_owner(owner_address);
                instance.admin_address = admin_address;
                instance.standard_nft_hash = standard_nft_hash;
                instance.max_royal_fee_rate = 500;
            })
        }

        ///Simple New Collection Creation - Auto create NFT Contract - Collection_Owner is owner of NFT contract and receive royal fee
        #[ink(message)]
        #[ink(payable)]
        pub fn auto_new_collection(
            &mut self,
            nft_name: String,
            nft_symbol: String,
            collection_owner: AccountId,
            name: Vec<u8>,
            description: Vec<u8>,
            avatar_image: Vec<u8>,
            header_image: Vec<u8>,
            is_collect_royal_fee: bool,
            royal_fee:u32
        ) -> Result<(), Error> {

            if self.adding_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            //fee must equal or less than 5%
            if royal_fee > self.max_royal_fee_rate {
                return Err(Error::InvalidRoyalFee);
            }

            let (hash, _) =
                ink_env::random::<ink_env::DefaultEnvironment>(nft_name.as_bytes()).expect("Failed to get salt");
            let hash = hash.as_ref();
            let contract = Psp34NftRef::new(collection_owner,nft_name,nft_symbol)
                .endowment(0)
                .code_hash(self.standard_nft_hash)
                .salt_bytes(&hash[..4])
                .instantiate()
                .unwrap_or_else(|error| {
                    panic!(
                        "failed at instantiating the NFT contract: {:?}",
                        error
                    )
                });
            let contract_account:AccountId = contract.to_account_id();

            //Increase collection_count and save the latest id with nft_contract_address - for tracking purpose
            self.collection_count += 1;
            self.collections_by_id.insert(&self.collection_count, &contract_account);

            let collections_by_owner = self.collections_by_owner.get(&collection_owner);
            if collections_by_owner.is_none(){
                let mut collections = Vec::<AccountId>::new();
                collections.push(contract_account);
                self.collections_by_owner.insert(&collection_owner, &collections);
            }
            else{
                let mut collections = collections_by_owner.unwrap();
                collections.push(contract_account);
                self.collections_by_owner.insert(&collection_owner, &collections);
            }

            let new_collection = Collection {
                collection_owner,
                nft_contract_address:contract_account,
                name,
                description,
                avatar_image,
                header_image,
                contract_type:2,
                is_collect_royal_fee,
                royal_fee,
                is_active: true,
                show_on_chain_metadata: true
            };

            self.collections.insert(&contract_account, &new_collection);

            Ok(())
        }

        /// Advanced Add new collection - with fee in AZERO 1% = 100 - anyone can add Existing contract - Collection_Owner is owner of NFT contract and receive royal fee
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
            is_collect_royal_fee: bool,
            royal_fee:u32
        ) -> Result<(), Error> {
            if self.adding_fee != self.env().transferred_value() {
                return Err(Error::InvalidFee);
            }
            if self.collections.get(&nft_contract_address).is_some(){
                return Err(Error::AddressAlreadyExists);
            }
            //fee must equal or less than 5%
            if royal_fee > self.max_royal_fee_rate {
                return Err(Error::InvalidRoyalFee);
            }
            //Increase collection_count and save the latest id with nft_contract_address - for tracking purpose
            self.collection_count += 1;
            self.collections_by_id.insert(&self.collection_count, &nft_contract_address);

            let collections_by_owner = self.collections_by_owner.get(&collection_owner);
            if collections_by_owner.is_none(){
                let mut collections = Vec::<AccountId>::new();
                collections.push(nft_contract_address);
                self.collections_by_owner.insert(&collection_owner, &collections);
            }
            else{
                let mut collections = collections_by_owner.unwrap();
                collections.push(nft_contract_address);
                self.collections_by_owner.insert(&collection_owner, &collections);
            }

            let new_collection = Collection {
                collection_owner,
                nft_contract_address,
                name,
                description,
                avatar_image,
                header_image,
                contract_type:1,
                is_collect_royal_fee,
                royal_fee,
                is_active: false,
                show_on_chain_metadata: false
            };

            self.collections.insert(&nft_contract_address, &new_collection);

            Ok(())
        }
        /* SETTERS */
        /// Update Owner of Collecion - who receive royal fee - Only Admin can change
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
        pub fn update_contract_type(
            &mut self,
            contract_address: AccountId,
            contract_type: u8
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                return Err(Error::CollectionNotExist);
            }
            let mut collection = self.collections.get(&contract_address).unwrap();
            if  self.env().caller() == self.admin_address {
                    collection.contract_type = contract_type;
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
            //fee must equal or less than 5%
            if new_fee > self.max_royal_fee_rate {
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

        /// Update show_on_chain_metadata - admin and collection owner can change
        #[ink(message)]
        pub fn update_show_on_chain_metadata(
            &mut self,
            contract_address: AccountId,
            show_on_chain_metadata: bool
        ) -> Result<(), Error>  {
            if self.collections.get(&contract_address).is_none(){
                 return Err(Error::CollectionNotExist);
             }

            let mut collection = self.collections.get(&contract_address).unwrap();

            if  self.env().caller() == collection.collection_owner ||
                self.env().caller() == self.admin_address {
                    collection.show_on_chain_metadata = show_on_chain_metadata;
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
        /// Update Max Royal Fee Rate - only Owner
        #[ink(message)]
        #[modifiers(only_owner)]
        pub fn update_max_royal_fee_rate(&mut self,max_royal_fee_rate: u32)  -> Result<(), Error> {
            self.max_royal_fee_rate = max_royal_fee_rate;
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

        /// Get Collection Information by Collection Address (NFT address)
        #[ink(message)]
        pub fn get_collection_by_address(&self,nft_contract_address: AccountId) -> Option<Collection> {
            return self.collections.get(&nft_contract_address);
        }
        /// Get All Collection Addresses by Owner Address
        #[ink(message)]
        pub fn get_collections_by_owner(&self,owner_address: AccountId) -> Option<Vec<AccountId>> {
            return self.collections_by_owner.get(&owner_address);
        }

        /// Get Collection Contract by ID
        #[ink(message)]
        pub fn get_contract_by_id(
            &self,
            id: u64
        ) -> Option<AccountId> {
            return self.collections_by_id.get(&id);
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

    impl CrossArtZeroCollection for ArtZeroCollectionManager{
        ///Get royal fee of the Collection
        #[ink(message)]
        fn get_royal_fee(&self,nft_contract_address: AccountId) -> u32 {
            if self.collections.get(&nft_contract_address).is_none(){
                 return 0;
             }

            let collection = self.collections.get(&nft_contract_address).unwrap();
            if  !collection.is_collect_royal_fee ||
                !collection.is_active{
                return 0;
            }
            else{
                collection.royal_fee
            }
        }
        ///Check if the Collection is active not
        #[ink(message)]
        fn is_active(&self,nft_contract_address: AccountId) -> bool {
            if self.collections.get(&nft_contract_address).is_none(){
                 return false;
             }

            let collection = self.collections.get(&nft_contract_address).unwrap();
            return collection.is_active;
        }

        ///Get NFT Contract Type 1 or 2 for PSP34
        #[ink(message)]
        fn get_contract_type(&self,nft_contract_address: AccountId) -> u8 {
            if self.collections.get(&nft_contract_address).is_none(){
                 return 0;
             }

            let collection = self.collections.get(&nft_contract_address).unwrap();
            return collection.contract_type;
        }

        /// Get Collection Owner by Collection Address (NFT address)
        #[ink(message)]
        fn get_collection_owner(&self,nft_contract_address: AccountId) -> Option<AccountId> {
            if self.collections.get(&nft_contract_address).is_none(){
                 return None;
             }
            let collection = self.collections.get(&nft_contract_address).unwrap();
            Some(collection.collection_owner)
        }
    }
}
