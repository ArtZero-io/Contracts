- [Contract deployment](#contract-deployment)
  - [Build contracts](#build-contracts)
  - [Deloy contracts](#deloy-contracts)
- [Update addresses and abis for FE](#update-addresses-and-abis-for-fe)
- [Update addresses and abis for BE](#update-addresses-and-abis-for-be)
  - [Files for api](#files-for-api)
  - [Files for jobs](#files-for-jobs)
- [Create PMP project and update its address in staking contract and PMP abi in FE, BE](#create-pmp-project-and-update-its-address-in-staking-contract-and-pmp-abi-in-fe-be)
- [Update art location and cache data for PMP NFT](#update-art-location-and-cache-data-for-pmp-nft)
  - [Update art location for PMP NFT](#update-art-location-for-pmp-nft)
  - [Cache data for PMP NFT](#cache-data-for-pmp-nft)

# Contract deployment

## Build contracts 

1. Go to each contract folder (i.e. Azero_Contracts/contracts/profile)

2. Run the below command to build a contract

```
cargo +nightly contract build --release
```
3. After building, we will have .contract and .json files in target/ink folder for each contract. We will use these file to deploy contracts, update addresses and abis in next steps. 
   
## Deloy contracts

1. Some contracts are related to other contracts, **use an account as a deloyment account (it will also be the admin account)** to deploy contracts in the following order:  
    
   -  profile contract
   -  psp34_standard contract
   -  collection_manager contract
   -  launchpad_manager contract
   -  psp34_launchpad_standard contract
   -  staking contract
   -  marketplace contract 

2. profile contract

   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select profile_manager.contract file in Upload Contract Bundle part
   - Click Next / Next / Upload and Instantiate to upload the contract
   - Record contract address in the part "You instantiated this contract ... from"
  
3. psp34_standard contract
   
   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select psp34_nft.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
      - **contractOwner**: input deloyment account
      - **name**: input any name, i.e. PMP
      - **symbol**: input any symbol, i.e. PMP
   - Click Next / Upload and Instantiate to upload the contract
   - Record contract address in the part "You instantiated this contract ... from"
   - Open psp34_nft.json file with any editor and record its hash at "source": {"hash": ...}  

4. collection_manager contract
   
   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select artzero_collection_manager.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
     - **adminAddress**: input deloyment account
     - **standardNftHash**: input the hash of psp34_standard recorded in step 3 above
     - **simpleModeAddingFee**: input fee in TZERO, i.e. 100
     - **advanceModeAddingFee**: input fee in TZERO, i.e. 100
     - **maxRoyaltyFeeRate**: input max royalty fee rate, it is scaled 10000 to support fixed point number, i.e. 500 for 5% (0.05)
     - Click Next / Upload and Instantiate to upload the contract
     - Record contract address in the part "You instantiated this contract ... from"
  
5. launchpad_manager contract

   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select artzero_launchpad_psp34.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
     - **maxPhasesPerProject**: input maximum number of phases for any project, i.e. 10
     - **adminAddress**: input deloyment account
     - **standardNftHash**: input the hash of psp34_launchpad_standard contract by opening the launchpad_psp34_nft_standard.json file with any editor and searching its hash at "source": {"hash": ...}
     - **projectAddingFee**: input fee in TZERO, i.e. 100
     - **projectMintFeeRate**: it is scaled 10000 to support fixed point number, i.e. 500 for 5% (0.05)
     - **publicMaxMintingAmount**: input max amount of nft for each public mint, i.e. 10
   - Click Next / Upload and Instantiate to upload the contract
   - Record contract address in the part "You instantiated this contract ... from" 

6. psp34_launchpad_standard contract
   
   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select launchpad_psp34_nft_standard.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
     - **launchpadContractAddress**: input the launchpad_manager address above 
     - **limitPhaseCount**: should equal to maxPhasesPerProject in the launchpad_manager above. i.e. 10
     - **contractOwner**: input deloyment account
     - **totalSupply**: input total Supply, i.e. 10000
     - **projectInfo**: can type any info about project, i.e. This is the launchpad project
     - **codePhases**: Vector of phase codes, i.e. 2 phases ["Phase 1", "Phase 2"]
     - **isPublicPhases**: allow for public phases or not, i.e. [false, true]
     - **publicMintingFeePhases**: public minting fee for each phases, i.e. [50000000000000, 90000000000000] means 50 & 90 TZERO for phase 1 and phase 2
     - **publicMintingAmountPhases**: the amount of public minting for each phase, i.e. [1000, 5000]
     - **publicMaxMintingAmountPhases**: maximum of public minting amount each time in each phase, i.e. [5, 10]
     - **startTimePhases**: Start time for each phase, time in milliseconds, i.e.  currentTime in ms = 1678769679000, phase2 = phase 1 + 3 days (259200000 ms), [currentTime, currentTime + 3 days] equals to  [1678769679000, 1679028879000]
     - **endTimePhases**: End time for each phase and must be greater than start time and there is no [startTimePhase, endTimePhase] overlapped each other., i.e, assume to continue example with startTimePhases above [currentTime + 2 days, currentTime + 8 days] equals to [1678942479000, 1679460879000]
   - Click Next / Upload and Instantiate to upload the contract
   - Record contract address in the part "You instantiated this contract ... from" 

7. staking contract
   
   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select artzero_staking_nft.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
     - **adminAddress**: input deloyment account
     - **artzeroNftContract**: the contract for PMP NFT to stake. Just temporarily input any address, we will update the real PMP address later in "Create PMP project and update its address ..." part below. i.e. use psp34_launchpad_standard address above for this contract.
     - **limitUnstakeTime**: Time wait for unstaking in minutes, i.e. 5
   - Click Next / Upload and Instantiate to upload the contract
   - Record contract address in the part "You instantiated this contract ... from" 
  
8. marketplace contract  

   - In contract UI, go to Add New Contract / Upload New Contract Code, choose deployment account for Account part and select artzero_marketplace_psp34.contract file in Upload Contract Bundle part then click Next
   - In Upload and Instantiate Contract
     - **collectionContractAddress**: input the address of collection_manager contract above
     - **stakingContractAddress**: input the address of staking contract above
     - **platformFee**: it is scaled 10000 to support fixed point number, i.e. 500 for 5% (0.05)

# Update addresses and abis for FE

Use json files and contract addresses in the contract deployment part to create abi files for FE  

1. profile.js  

   - Copy profile_manager.json to a place and change name to profile.js
   - Open profile.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const profile = {
         CONTRACT_ADDRESS: "Address of the profile contract",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const profile = {
         CONTRACT_ADDRESS: "5FYc5C2RiiZrvRA2Pfr1u753NjwEkwiXGCJ4c8aNzjX1aauE",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of profile.js 
     ```
      };

      export default profile;
     ```   

2. nft721-psp34-standard.js  

   - Copy psp34_nft.json to a place and change name to nft721-psp34-standard.js
   - Open nft721-psp34-standard.js, modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const nft721_psp34_standard = {
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add the end of nft721_psp34_standard.js 
     ```
      };

      export default nft721_psp34_standard;
     ```

3. collection-manager.js  

   - Copy artzero_collection_manager.json to a place and change name to collection-manager.js
   - Open collection-manager.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const collection_manager = {
         CONTRACT_ADDRESS: "Address of the collection manager contract",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const collection_manager = {
         CONTRACT_ADDRESS: "5GHA36iM5xKeUAjTsJizmrAPvpyUTpixnciL91k34FEoGGte",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of collection-manager.js 
     ```
      };

      export default collection_manager;
     ```   

4. launchpad-manager.js  

   - Copy artzero_launchpad_psp34.json to a place and change name to launchpad-manager.js
   - Open launchpad-manager.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const launchpad_manager = {
         CONTRACT_ADDRESS: "Address of the launchpad manager contract",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const launchpad_manager = {
         CONTRACT_ADDRESS: "5HhS5Gum1cm2ZAWxHzwHUbFQLFB4KKGUQNaHPbt9toLCcA81",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of launchpad-manager.js 
     ```
      };

      export default launchpad_manager;
     ``` 

5. launchpad-psp34-nft-standard.js

   - Copy launchpad_psp34_nft_standard.json to a place and change name to launchpad-psp34-nft-standard.js
   - Open launchpad-psp34-nft-standard.js, modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const launchpad_psp34_nft_standard = {
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add the end of launchpad-psp34-nft-standard.js 
     ```
      };

      export default launchpad_psp34_nft_standard;
     ```

6. staking.js  

   - Copy artzero_staking_nft.json to a place and change name to staking.js
   - Open staking.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const staking = {
         CONTRACT_ADDRESS: "Address of the staking contract",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const staking = {
         CONTRACT_ADDRESS: "5FzenH2c2bLywe1bnKoguovgeHqsaeNMHiXQivbPdxPV1ukW",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of staking.js 
     ```
      };

      export default staking;
     ```  

7. marketplace.js  

   - Copy artzero_marketplace_psp34.json to a place and change name to marketplace.js
   - Open marketplace.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const marketplace = {
         CONTRACT_ADDRESS: "Address of the marketplace contract",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const marketplace = {
         CONTRACT_ADDRESS: "5FJMs5vjMzsE4SMiXqZdWjRqZYqwi5EfC23yQRsiuVooUBsb",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of marketplace.js 
     ```
      };

      export default marketplace;
     ```  

8. artzero-nft.js (contains address and abi for PMP contract - the PRAYING MANTIS PREDATORS NFT created by Artzero team) 

   - It is noted that we will temporarily input any address for CONTRACT_ADDRESS below and will update the real PMP contract address later in the step "Create PMP project and update its address ..." 
  
   - Copy launchpad_psp34_nft_standard.json to a place and change name to artzero-nft.js
   - Open artzero-nft.js and modify:
     ```
      {
         "source": {
     ```
     to 
     ```
      const artzero_nft = {
         CONTRACT_ADDRESS: "Put any address here",
         CONTRACT_ABI: {
            "source": {
     ``` 

     i.e.

     ```
      const artzero_nft = {
         CONTRACT_ADDRESS: "5C6fC67LnWu2WgZqz79EFasmzwUL3UqKaz3eL7X2mDRVo2t1",
         CONTRACT_ABI: {
            "source": {
     ``` 
   - Add at the end of artzero-nft.js 
     ```
      };

      export default artzero_nft;
     ```  
   
9. Copy all js files we've created to src/utils/blockchain of FE code
   
# Update addresses and abis for BE

Abi files for BE are stored in Api/src/contracts folder for api and in Contracts folder for jobs. They are nearly the same as js files for FE, we just do small adjustments as below.  

## Files for api
1. profile.ts

   - Copy profile.js from FE to a place and rename to profile.ts 
   - Modify the file:
     ```
      const profile = {
         ...
      };

      export default profile;
     ```
     to 
     ```
      export const profile = {
         ...
      };
     ```

2. nft721_psp34_standard.ts

   - Copy nft721-psp34-standard.js from FE to a place and rename to nft721_psp34_standard.ts 
   - Modify the file:
     ```
      const nft721_psp34_standard = {
         ...
      };

      export default nft721_psp34_standard;
     ```
     to 
     ```
      export const nft721_psp34_standard = {
         ...
      };
     ```

3. collection_manager.ts

   - Copy collection-manager.js from FE to a place and rename to collection_manager.ts 
   - Modify the file:
     ```
      const collection_manager = {
         ...
      };

      export default collection_manager;
     ```
     to 
     ```
      export const collection_manager = {
         ...
      };
     ```

4. launchpad_manager.ts

   - Copy launchpad-manager.js from FE to a place and rename to launchpad_manager.ts 
   - Modify the file:
     ```
      const launchpad_manager = {
         ...
      };

      export default launchpad_manager;
     ```
     to 
     ```
      export const launchpad_manager = {
         ...
      };
     ```

5. launchpad_psp34_nft_standard.ts

   - Copy launchpad-psp34-nft-standard.js from FE to a place and rename to launchpad_psp34_nft_standard.ts 
   - Modify the file:
     ```
      const launchpad_psp34_nft_standard = {
         ...
      };

      export default launchpad_psp34_nft_standard;
     ```
     to 
     ```
      export const launchpad_psp34_nft_standard = {
         ...
      };
     ```

6. staking.ts

   - Copy staking.js from FE to a place and rename to staking.ts 
   - Modify the file:
     ```
      const staking = {
         ...
      };

      export default staking;
     ```
     to 
     ```
      export const staking = {
         ...
      };
     ```

7. marketplace.ts

   - Copy marketplace.js from FE to a place and rename to marketplace.ts 
   - Modify the file:
     ```
      const marketplace = {
         ...
      };

      export default marketplace;
     ```
     to 
     ```
      export const marketplace = {
         ...
      };
     ```

8. artzero_nft.ts

   - Copy artzero_nft.js from FE to a place and rename to artzero_nft.ts 
   - Modify the file:
     ```
      const artzero_nft = {
         ...
      };

      export default artzero_nft;
     ```
     to 
     ```
      export const artzero_nft = {
         ...
      };
     ```
   - It is noted that we temporarily input any address for CONTRACT_ADDRESS and will update the real PMP contract address later in the step "Create PMP project and update its address ..."

9. Copy all ts files we've created to Api/src/contracts of BE code

## Files for jobs

1. Copy ts files in the api part above to a place, change their extension to .js files.
   
2. With each .js file, open and modify as follows:

   - Change "export const module_name" to "const module_name"  
      i.e.
      ```
      export const collection_manager
      ```
      to 
      ```
      const collection_manager
      ```
   - At the end of file add module export with the template
      ```
      module.exports = {
         module_name:module_name
      };
      ```

      i.e.
      ```
      module.exports = {
         collection_manager:collection_manager
      };
      ```  
3. After all .js files are modified, copy them to Contracts folder of BE code
   
4. Move to backend folder and start api and jobs through pm2 for BE
   ```
   pm2 start az_bids_monitor.js
   pm2 start az_cache_image.js
   pm2 start az_cloudflare_sync_monitor.js
   pm2 start az_collection_monitor.js
   pm2 start az_events_collector.js
   pm2 start az_nft_monitor.js
   pm2 start az_project_monitor.js
   pm2 start az_telegram_bot.js
   cd Api
   pm2 start npm --name=az_api_loopback -- run start
   ```

# Create PMP project and update its address in staking contract and PMP abi in FE, BE  

Assuming that BE and FE are setup and deployed.

This part is to use FE to create a launchpad project for PMP NFT and then use the launchpad_manager contract to get the psp34 launchpad standard contract address for PMP NFT.     

Then we will update this PMP contract address for artzero_nft.js and artzero-nft.js in BE, artzero-nft.js in FE and in the staking contract.

It is noted that **the deployment account is the original admin** and we will use this account for FE to create the launchpad project for PMP NFT.  

1. Start FE, in the homepage click Connect Wallet button and select supporting wallet, then make sure to choose the deployment account
   
2. Go to Launchpad/Live projects, click Create Project
   
3. Select avatar image, featured image, header image, input at least required fields (are tagged with *), i.e.
   - Project Info
     - Project name: PRAYING MANTIS PREDATORS
     - Start time - End time: 16/3/2023 12:00 AM - 31/3/2023 11:59 PM
     - Project description: PRAYING MANTIS PREDATORS
     - Enable Collect Royalty Fee
     - Milestone name: Q1 - 2023
     - Milestone description: Launching the Launchpad with PMP Public Sale & Marketplace
     - PROJECT TEAM MEMBER
       - Upload member image
       - Name: Brian
       - Title: Founder & CEO
   - NFT INFO
     - NFT Name: PMP
     - NFT Symbol: PMP
     - Total Supply: 10000 
   - SALE PHASE INFORMATION
     - Phase name: Phase 1
     - Start time - End time: 17/3/2023 12:00 AM - 30/3/2023 11:59 PM
     - Enable Allow public mint
       - Public minting fee: 79 
       - Total Mint Amount: 1000
       - Max per mint: 5
   - CONTACT INFO
     - Email: your email
   - Tick "Create new project you will pay...", "I agree to pay...", "I agree to ArtZero's Terms of Service" then click "Create Project" and approve for transactons to create PMP project   

4. Go to contracts UI, select Launchpad psp34 contract 
   - Caller: input deloyment account
   - Message to Send: select getProjectById
     - id: input 1
   - The Outcome/ Return value will be the PMP contract address of the project we've created.
   - Record this PMP contract address

5. Update PMP contract address in staking contract 
   - In contracts UI, select Staking contract
     - Caller: input deloyment account
     - Message to Send: select setArtzeroNftContract
       - artzeroNftContract: input the PMP contract address we saved in the previous step
     - Click Call contract, the PMP contract address will be updated for staking contract

6. Update PMP contract address for artzero_nft.js and artzero-nft.js in BE, artzero-nft.js in FE
   - BE
     - Open Api/src/contracts/artzero_nft.ts and Contracts/artzero_nft.js 
     - Replace the contract at CONTRACT_ADDRESS field by the PMP contract address, i.e.
       ```
       CONTRACT_ADDRESS: "5EmvscC7Nz293txpLow7NDYTi1k926wyr1XTmJyyGyUh7jb4", 
       ```
   - FE
     - Open src/utils/blockchain/artzero-nft.js 
     - Replace the contract at CONTRACT_ADDRESS field by the PMP contract address.

7. Restart all service for BE
   ```
   pm2 restart all   
   ```
8. Rerun FE
   ```
   yarn start
   ```
# Update art location and cache data for PMP NFT

## Update art location for PMP NFT

 The step we create PMP project above also creates the launchpad psp34 for PMP NFT but its data (images info and their metadata including image paths, their attributes... in JSON format) is off-chain (not stored directly in smart contract) and is often pushed to ipfs. We need to indicate for the system knowing where its data is located by using FE. 
 
 For the purpose of testing, we uploaded the images info and metadata for PMP project into the ifps links: 
   - image info: ipfs://bafybeicuufb2745l5g5zm2fa32fydao76iur72lwaqrt7dmhzkdupmzxue/
   - metadata info: ipfs://bafybeibw7yco54n24gieazxya2zt3gfujvrv5twed4xheruv3izyuar2ry/ 

 Actually, if we have the metadata ipfs link then we can extract the image path, thus, knowing the image info through this path as well. So the system needs to store the metadata info only.   
 
 Just follow below steps to add metadata ipfs link 
 
 1. Use deployment account
 2. Go to MY ACCOUNT / MY PROJECTS
 3. Select PRAYING MANTIS PREDATORS project  
 4. Select UPDATE ART LOCATION
 5. Put metadata ipfs link ipfs://bafybeibw7yco54n24gieazxya2zt3gfujvrv5twed4xheruv3izyuar2ry/ and click submit

## Cache data for PMP NFT

Loading data in ipfs is actually slow, especially, when we have a large number of NFTs. Thus, even we put the ipfs link to the system, we still cache their data to improve the performance to process or show them on both FE and BE side.

For PMP NFT, we have 10000 NFT but for testing purpose, to save time, we just demo to cache data of the first 200 NFTs by following the below steps and using our scripts.

1. Preparation
   
   - Link ipfs for images and their metadata as in part "Update art location for PMP NFT"
     - image info: ipfs://bafybeicuufb2745l5g5zm2fa32fydao76iur72lwaqrt7dmhzkdupmzxue/
     - metadata info: ipfs://bafybeibw7yco54n24gieazxya2zt3gfujvrv5twed4xheruv3izyuar2ry/

   - Data and script link: https://drive.google.com/file/d/19spVnhbz6GynoesjMMOihqd0axvlyviV/view?usp=sharing . This is the PMP.zip file containing: 
     - images.tar.gz: First 200 PMP images, index from 1 to 200
     - jsons.zip: Metadata 
     - index.js: script to cache  

2. Cache

We will perform the cache in the backend. The script will cache image links, metadata and add cached NFT token id to NftQueue in DB so that the BE will check and get their cached data (image link and metadata) to update into DB. 

Assume that we already have BE code, kindly follow below steps: 
   
   - Create Cache folder which same level with the Api folder of the BE code. And then create PMP folder inside Cache folder.   
   - Download PMP.zip and extract it to Cache/PMP folder, we will have 3 files
     -  images.tar.gz
     -  jsons.zip
     -  index.js 
   - Extract images.tar.gz and jsons.zip files so the images and jsons folders will appear in the Cache/PMP folder 
   - Make sure that mongodb of BE is running
   - Open the script index.js, find and customize config parameters for the cache: 
      - from (should be in line 15): the starting nft id wants to cache, default value 1 
      - total_supply (line 16): total supply of NFT, for the demo, we can set to 200 or whatever we like, let's say 10...
      - image_ipfs_link (line 17): input the ipfs link above
      - json_ipfs_link (line 18): input the metadata link above   
      - collection_address (line 19): the collection address, replace it by the PMP contract address recorded in step 4 of the part "Create PMP project and update its address in staking contract and PMP abi in FE, BE" and saved the script file. i.e.
         ```
         let collection_address = "5EmvscC7Nz293txpLow7NDYTi1k926wyr1XTmJyyGyUh7jb4";
         ```
      - collection_name (line 20): The name of the collection, input "PMP" 
   - Save, go to the Cache/PMP folder and run the script to cache:
      ```
      node index.js    
      ```
   - It will display logs into console. For each NFT, after putting token id to NFTQueue, it will display the message 
      ```
      added to NFT queue token_id
      ```
      i.e.
      ```
      added to NFT queue 10
      ```
   -  Wait until all NFTs are processed (means that the last token with id of total supply will be notified "added to NFT queue")
   - **Note:** if in any case the script is stopped in the middle of the process, kindly check log to see the last token id has the issue and then we can reopen the script to reconfig the from parameter to this last-processed token id. The script will cache at this token id instead of running from the first token id.  
   - Press Ctrl-C to exit the process when it is done.

3. Check PMP collection on FE

In this step, we try to mint some NFTs from PMP launchpad project and check PMP collection to see if it is displayed smoothly after caching.

- Open FE and use deployment account
- Enable the PRAYING MANTIS PREDATORS project
   - Access to the admin page with the template
      ```
      hostname:port/#/admin
      ```
      i.e.
      ```
      http://localhost:8001/#/admin
      ```
   - Select PROJECT MANAGEMENT tab
   - Find the PRAYING MANTIS PREDATORS project in the table and enable it by clicking the Enable button at the Action column 
   - Wait to see the Status column changing from Inactive to Active and Action column changing from ENABLE to DISABLE, which means that the PMP project is enable.
- Do steps to mint some PMP NFTs
   - Click to LAUNCHPAD part at the header, we can see the PMP showing up in the launchpad 
   - In the launchpad, select PRAYING MANTIS PREDATORS project
   - Go to a phase which has public mint, type the number of NFTs you want to mint and click PUBLIC MINT. I.e. In my case, in phase 1, i public mint 3 NFTs
- Go to collection to check nft list  
   - Go to MY ACCOUNT / MY COLLECTIONS
   - Select PRAYING MANTIS PREDATORS collection
   - Select tab ALL and check if NFTs can display their images. If they can this means images are cached succesfully
   - Click to one of NFT and check if it can show its attributes. If can, the metada for this NFT is cached