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
3. Deploy Collection Contract via Proxy
4. Deploy LaunchPad Standard Contract
5. LaunchPad Manager via Proxy with Code Hash of LaunchPad Standard
Add PMP Project -> create Collection -> Update base URI -> Update admin address
NFT Smartnet: ipfs://QmXtnr9aEJVywiLs1keZdyiKbQwignZT3FhwKYivF15oZp/
NFT Mainnet: To be updated
6. Deploy Staking Contract via Proxy
With Address of PMP Project
7. Deploy Marketplace Contract via Proxy

## Deploy customized NFT contract

You can customize the **psp34_standard** contract to create your own version of NFT Contract. Please check this guide for more information
https://medium.com/@artzero_io/how-to-create-nft-contract-on-azero-to-work-with-artzero-a14c8f17f90a

## Code Hashes and Contract Addresses for TestNet

- Proxy Contract Hash Code
**To be updated**
- Profile Hash Code and Contract
**To be updated**
- ArtZero NFT Staking Hash Code and Contract
**To be updated**
- Marketplace Hash Code and Contract
**To be updated**
- LaunchPad Manager Hash Code and Contract
**To be updated**
- Collection Manager Hash Code and Contract
**To be updated**
- LaunchPad Standard Hash Code
**To be updated**
- NFT Standard Hash Code
**To be updated**

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
