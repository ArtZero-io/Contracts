# ArtZero Contracts Repo

## Introduction

The ArtZero platform aims to be a decentralized NFT marketplace on the AlephZero blockchain. It aims to allow the users to list their NFT collections to be tradeable on the platform for a fee and to create their NFT collection via the ArtZero contracts. The users can create the collections as standard NFT collections or in an advanced mode, which also serves as a launchpad for such projects. The platform also comes with its native NFT Collection, which owners can stake for platform fees and other perks.

## Contracts

The main implementation of ArtZero smart contracts are in /contracts folder and contains following contracts:
- Collection Manager (audited)
- LaunchPad Manager (audited)
- Marketplace PSP34 (audited)
- PMP Staking Contract (audited)
- Standard Launchpad Contract (audited)
- Standard NFT Contract (audited)
- Proxy contract (clone from OpenBrush)
- Profile Contract
The traits and implemenation of traits locate in **impls** and **traits** folders.

## Code standard

ink! is an EDSL based on Rust; therefore, we use clippy and rustfmt to make sure code is in compliance with Rust idioms.
```
rustup component add rustfmt --toolchain nightly
cargo +nightly fmt
cargo clippy
```

## Contract Build and Deploy Instructions

Before building your smart contract on Aleph Zero, you will first need to install some development tools. The comprehensive guide can be found at:
https://docs.alephzero.org/aleph-zero/build/installing-required-tools

Go to the contract folder you want to build under **contracts** and run
```
cargo +nightly contract build
```
or if you want to build the release version
```
cargo +nightly contract build --release
```
After the contract is built successfully, You will see under target/ink 3 files: **contract_name.wasm contract_name.contract and metadata.json**. Follow this instruction to deploy the contract:
https://docs.alephzero.org/aleph-zero/build/deploying-your-contract-to-aleph-zero-testnet

## Contract Deployment Steps

1. Deploy Profile Contract
2. Deploy NFT Standard Contract
3. Deploy Collection Contract
4. Deploy LaunchPad Standard Contract using any launchpad_contract_address. The purpose is to get the Hash Code of the contract.
5. LaunchPad Manager with Code Hash of LaunchPad Standard
6. Add PMP Project -> create Collection -> Update base URI -> Update admin address
NFT Smartnet: **ipfs://QmXtnr9aEJVywiLs1keZdyiKbQwignZT3FhwKYivF15oZp/**
NFT Mainnet: **To be updated**
7. Deploy Staking Contract With Address of PMP Project
8. Deploy Marketplace Contract with staking and collection manager contract addresses

## Deploy customized NFT contract

You can customize the **psp34_standard** contract to create your own version of NFT Contract. Please check this guide for more information
https://medium.com/@artzero_io/how-to-create-nft-contract-on-azero-to-work-with-artzero-a14c8f17f90a

## Code Hashes and Contract Addresses for TestNet

- Profile Hash Code and Contract
**0x0fae9732950d2e127019b1a433095d40f1b07dee55c1c5c69fd3a38b649c6d8a**
**5DxuvUB5p4F4fB1GN3oFsaoUFuU2y1agcQ2PBY3NfiAFkXjR**
- ArtZero NFT Staking Hash Code and Contract
**0x38ce87a8a2bc330b42a3bac4cfb22647c2c265a0408cae442f7a8f6e4a6ebc01**
**5D9wPyetDqaUCsfLURuYwtT2JgP1Pqs8o1tFkw5VMqHbn9P7**
- Marketplace Hash Code and Contract
**0x3fa6ae3c0e8f9fb0381c4ea05eff8b0fd68812be93ba46988195e7b211905f76**
**5Cbufyf2Qnh9dbchqMb25eL5cySWinvQqCwpkKYZRD43Pw35**
- LaunchPad Manager Hash Code and Contract
**0x060da68e80d2a5b90a928f02f8cd48f00eaad7fc9628554937a90a220a6bf847**
**5FJwP3rWoHMnPqYaSPQa8AA4ssLgj9ciW3CauUVbeSchZaDe**
- Collection Manager Hash Code and Contract
**0x3cc56938bd78552e9b720786ace3cf63bd59fffad6154d6708ace9cd9d48572b**
**5FFqxQBrtnVMjSyXXqyoHzmUHHthTqX3rY9Kr7nx9bzE9cD5**
- LaunchPad Standard Hash Code
**0x87de62029c759dae7f49e29f37b9e6c2ac8dfb7744fb663c5ba52d805e691e36**
- NFT Standard Hash Code
**0x2f64f82f62a48b477c86b73a8d70ad0f632682acd9fc49bbac87edf91e9dbde9**

## Code Hashes and Contract Addresses for MainNet

**To be updated**

## Test

The general quick tests for contracts can be found in **tests* folder

## Works are in Progress

- Marketplace for PSP37 (ERC1155 equivalence)
- Multichain support
- NFT Staking (can be found in InkWhale project repo) https://github.com/InkWhale-net

## Support

If you need futher support from us, please contact us using:
- Telegram: https://t.me/artzero_io
- Discord: https://discord.gg/wzkZ2JTvN4
- Twitter: https://twitter.com/ArtZero_io
- Email: support@artzero.io
or read our articles at https://medium.com/@artzero_io
