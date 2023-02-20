import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsMarketplace from './typed_contracts/constructors/artzero_marketplace_psp34';
import ContractMarketplace from './typed_contracts/contracts/artzero_marketplace_psp34';

import ConstructorsStakingNft from './typed_contracts/constructors/artzero_staking_nft';
import ContractStakingNft from './typed_contracts/contracts/artzero_staking_nft';

import ConstructorsPsp34Nft from './typed_contracts/constructors/psp34_nft';
import ContractPsp34Nft from './typed_contracts/contracts/psp34_nft';
import psp34NftStandard from './artifacts/psp34_nft.json';
import * as PSP34Args from './typed_contracts/types-arguments/psp34_nft';

import ConstructorsCollectionManager from './typed_contracts/constructors/artzero_collection_manager';
import ContractCollectionManager from './typed_contracts/contracts/artzero_collection_manager';
import * as CollectionManagerArgs from './typed_contracts/types-arguments/artzero_collection_manager';

describe('Profile test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let nftContractAddress: any;
    let standardNftHash: string;
    let simpleModeAddingFee: string;
    let advanceModeAddingFee: string;
    let maxRoyaltyFeeRate: number;
    
    let stakingContractAddress: any;
    let stakingContract: any;

    let collectionManagerContractAddress: any;
    let collectionManagerContract: any;

    let platformFee: number;
    let contractAddress: any;
    let contract: any;
    let query: any;
    let nftTx: any;
    let tx: any;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        /**
         * Step 1: Alice creates a Psp34 NFT contract
         */
        // "refTime: 769080524"
        let nftContractGasLimit = setGasLimit(api, 1_600_000_000, 0);
        const nftContractFactory = new ConstructorsPsp34Nft(api, alice);
        
        let nftContractOwner = alice.address;
        let nftName = "Test PMP Staking";
        let nftSymbol = "TPS";

        stakingContractAddress = (
            await nftContractFactory.new(
                nftContractOwner,
                nftName,
                nftSymbol,
                {gasLimit: nftContractGasLimit}
            )
        ).address;

        /** 
         * Step 2: Alice mints 2 NFTs and sends NFT tokenId #2 to Bob
         */ 
        // Mint 2 NFTs
        await nftTx.mint(); // Mint #1
        await nftTx.mint(); // Mint #2
        const nftTokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2
        let nftData = new Array<number>();

        // Transfer #2 to Bob
        await nftTx.transfer(
            bob.address,
            nftTokenId,
            nftData
        );

        /** 
         * Step 3: Create staking contract
         */ 
        let stakingContractGasLimit = setGasLimit(api, 3_200_000_000, 0); 
        const stakingContractFactory = new ConstructorsStakingNft(api, defaultSigner);
        
        let adminAddress = defaultSigner.address;
        let limitUnstakeTime = 1;
        
        stakingContractAddress = (
            await stakingContractFactory.new(
                adminAddress,
                nftContractAddress,
                limitUnstakeTime,
                {gasLimit: stakingContractGasLimit}
            )
        ).address;
        stakingContract = new ContractStakingNft(stakingContractAddress, defaultSigner, api);

        /**
         * Step 4: Create  collection manager contract
         */
        standardNftHash = psp34NftStandard.source.hash;
        simpleModeAddingFee = "3000000000000";
        advanceModeAddingFee = "5000000000000";
        maxRoyaltyFeeRate = 500;
        let collectionManagerContractGasLimit = setGasLimit(api, 3_200_000_000, 0);
        
        const collectioManagerContractFactory = new ConstructorsCollectionManager(api, defaultSigner);
        collectionManagerContractAddress = (
            await collectioManagerContractFactory.new(
                adminAddress,
                standardNftHash,
                simpleModeAddingFee,
                advanceModeAddingFee,
                maxRoyaltyFeeRate,
                {gasLimit: collectionManagerContractGasLimit}
            )
        ).address;
        collectionManagerContract = new ContractCollectionManager(collectionManagerContractAddress, defaultSigner, api); 

        /**
         * Step 5: Create marketplace contract
         */
        let marketplaceGasLimit = setGasLimit(api, 3_200_000_000, 0); 
        const marketplaceContractFactory = new ConstructorsMarketplace(api, defaultSigner);
        
        adminAddress = defaultSigner.address;
        platformFee = 500;

        contractAddress = (
            await marketplaceContractFactory.new(
                collectionManagerContract,
                stakingContractAddress,
                platformFee,
                {gasLimit: marketplaceGasLimit}
            )
        ).address;
        contract = new ContractMarketplace(contractAddress, defaultSigner, api);
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });

    it('Listing the NFT onto the marketplac', async () => {   
        // Alice creates collection in simple mode
        
    });
});