
## Introduction

This is the location of all files to test ArtZero Contract using typechain-polkadot

Examples and information can be found at

https://github.com/Supercolony-net/typechain-polkadot (examples folder)
https://github.com/727-Ventures/openbrush-contracts/tree/main/tests/e2e


## Quick Guide:

1. Copy **.contract and .json** files from targe/ink folders for each contract into **artifacts** folder
2. Rename **metadata.json** to **<contract_name>.json** to match .contract file
3. Run **npx @727-ventures/typechain-polkadot --in artifacts --out typed_contracts** to let typechain-polkadot convert ABI to typescript files
4. Import the files to .ts and start using
For example:
import Contract from "./typed_contracts/contracts/psp34_nft";
import Constructors from "./typed_contracts/constructors/psp34_nft";
5. Create test file in .ts
6. Run **npm install**
7. Run all tests **npm run test:mocha**

## Test Files

1.profile_test.ts  
2.psp34_standard_test.ts  
3.launchpad_manager_test.ts  
4.collection_manager_test.ts  
5.marketplace_test.ts  
6.psp34_launchpad_standard_test.ts  
7.staking_test.ts  

## Notes

The testing time may take **4-5 minutes** because some testcases such as unstaking PMP NFT or Psp34 launchpad standard public mint... needs to wait for changing phase or state. Please kindly wait until tests are completely run.