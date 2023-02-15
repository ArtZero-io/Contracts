
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


