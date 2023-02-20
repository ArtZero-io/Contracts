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

import {BN} from '@polkadot/util';

describe('Marketplace test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let platformFee: number;

    let nftPmpContractAddress: any;
    let nftPmpContract: any;
    let nftPmpQuery: any;
    let nftPmpTx: any;

    let aliceNftContractAddress: any;
    let aliceNftContract: any;
    let aliceNftQuery: any;
    let aliceNftTx: any;

    let bobNftContractAddress: any;
    let bobNftContract: any;
    let bobNftQuery: any;
    let bobNftTx: any;
    
    let stakingContractAddress: any;
    let stakingContract: any;
    let stakingQuery: any;
    let stakingTx: any;

    let stakingAdminAddress: string;
    let stakingLimitUnstakeTime: number;

    let cmContractAddress: any;
    let cmContract: any;
    let cmQuery: any;
    let cmTx: any;

    let cmAdminAddress: string;
    let cmStandardNftHash: string;
    let cmSimpleModeAddingFee: string;
    let cmAdvanceModeAddingFee: string;
    let cmMaxRoyaltyFeeRate: number;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        /**
         * Step 1: Alice creates a Psp34 PMP NFT contract
         */
        // "refTime: 769080524"
        let nftPmpContractGasLimit = setGasLimit(api, 1_600_000_000, 0);
        const nftPmpContractFactory = new ConstructorsPsp34Nft(api, alice);
        
        let nftPmpContractOwner = alice.address;
        let nftPmpName = "Test PMP Staking";
        let nftPmpSymbol = "TPS";

        nftPmpContractAddress = (
            await nftPmpContractFactory.new(
                nftPmpContractOwner,
                nftPmpName,
                nftPmpSymbol,
                {gasLimit: nftPmpContractGasLimit}
            )
        ).address;

        // console.log("nftPmpContractAddress =", nftPmpContractAddress);

        // PMP NFT contract
        nftPmpContract = new ContractPsp34Nft(nftPmpContractAddress, alice, api);    
        nftPmpQuery = nftPmpContract.query;
        nftPmpTx = nftPmpContract.tx;

        /** 
         * Step 2: Alice mints 2 PMP NFTs and sends NFT tokenId #2 to Bob
         */ 
        // Mint 2 PMP NFTs
        await nftPmpTx.mint(); // Mint #1
        await nftPmpTx.mint(); // Mint #2
        const nftPmpTokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2
        let nftPmpData = new Array<number>();

        // Transfer #2 to Bob
        await nftPmpTx.transfer(
            bob.address,
            nftPmpTokenId,
            nftPmpData
        );

        /** 
         * Step 3: Create staking contract
         */ 
        let stakingContractGasLimit = setGasLimit(api, 3_200_000_000, 0); 
        const stakingContractFactory = new ConstructorsStakingNft(api, defaultSigner);
        
        let stakingAdminAddress = defaultSigner.address;
        let stakingLimitUnstakeTime = 1; // 1 min
        
        stakingContractAddress = (
            await stakingContractFactory.new(
                stakingAdminAddress,
                nftPmpContractAddress,
                stakingLimitUnstakeTime,
                {gasLimit: stakingContractGasLimit}
            )
        ).address;

        // console.log("stakingContractAddress =", stakingContractAddress);
        
        stakingContract = new ContractStakingNft(stakingContractAddress, defaultSigner, api);
        stakingQuery = stakingContract.query;
        stakingTx = stakingContract.tx;

        /**
         * Step 4: Create  collection manager contract
         */
        cmAdminAddress = defaultSigner.address;
        cmStandardNftHash = psp34NftStandard.source.hash;
        cmSimpleModeAddingFee = "3000000000000";
        cmAdvanceModeAddingFee = "5000000000000";
        cmMaxRoyaltyFeeRate = 500;

        // "refTime: 1556009780"
        let cmContractGasLimit = setGasLimit(api, 3_200_000_000, 0);
        
        const cmContractFactory = new ConstructorsCollectionManager(api, defaultSigner);
        cmContractAddress = (
            await cmContractFactory.new(
                cmAdminAddress,
                cmStandardNftHash,
                cmSimpleModeAddingFee,
                cmAdvanceModeAddingFee,
                cmMaxRoyaltyFeeRate,
                {gasLimit: cmContractGasLimit}
            )
        ).address;

        // console.log("cmContractAddress =", cmContractAddress);       
        
        cmContract = new ContractCollectionManager(cmContractAddress, defaultSigner, api); 
        cmQuery = cmContract.query;
        cmTx = cmContract.tx;

        /**
         * Step 5: Alice and Bob create simple/advance collection and mint  
         */

        // Alice creates collection in simple mode
        let aliceNftName = "Simple mode collection";
        let aliceNftSymbol = "SMC";
        let aliceAttributes = ["Hair", "Color", "Type"];
        let aliceAttributeVals = ["Straight", "Yellow", "Round"];
        let aliceIsCollectRoyaltyFee = true;
        let aliceRoyaltyFee = 400;

        await cmContract.withSigner(alice).tx.autoNewCollection(
            aliceNftName,
            aliceNftSymbol,
            aliceAttributes,
            aliceAttributeVals,
            aliceIsCollectRoyaltyFee,
            aliceRoyaltyFee,
            {value: cmSimpleModeAddingFee} 
        );

        aliceNftContractAddress = (await cmQuery.getContractById(1)).value.ok;
        // console.log("aliceNftContractAddress =", aliceNftContractAddress);    

        aliceNftContract = new ContractPsp34Nft(aliceNftContractAddress, alice, api);    
        aliceNftQuery = aliceNftContract.query;
        aliceNftTx = aliceNftContract.tx;     
        
        // Active Alice's collection
        await cmTx.updateIsActive(
            aliceNftContractAddress,
            true
        );

        // Alice mints NFT tokenid #1
        await aliceNftTx.mint(); // Mint #1

        // Bob creates collection in advance mode
        // Bob creates a nft
        let bobNftgasLimit = setGasLimit(api, 1_600_000_000, 0);
        
        let bobNftContractFactory = new ConstructorsPsp34Nft(api, bob);
        let bobNftName = "Advanced mode collection";
        let bobNftSymbol = "AMC";

        bobNftContractAddress = (
            await bobNftContractFactory.new(
                bob.address,
                bobNftName,
                bobNftSymbol,
                {gasLimit: bobNftgasLimit}
            )
        ).address;

        // console.log("bobNftContractAddress =", bobNftContractAddress);

        bobNftContract = new ContractPsp34Nft(bobNftContractAddress, bob, api);    
        bobNftQuery = bobNftContract.query;
        bobNftTx = bobNftContract.tx;      
        
        // Bob puts nft contract address to addNewCollection func
        let bobAttributes = ["Processor", "RAM", "Thread", "Weight"];
        let bobAttributeVals = ["Intel P4", "32 GB", "8", "2"];
        let bobIsCollectRoyaltyFee = true;
        let bobRoyaltyFee = 400;

        await cmContract.withSigner(bob).tx.addNewCollection(
            bobNftContractAddress,
            bobAttributes,
            bobAttributeVals,
            bobIsCollectRoyaltyFee,
            bobRoyaltyFee,
            {value: cmAdvanceModeAddingFee} 
        );
        // console.log("Bob addNewCollection");
    
        // Active Bob's collection
        await cmTx.updateIsActive(
            bobNftContractAddress,
            true
        );
        // console.log("cm updateIsActive Bob collection");
        // Bob mints NFT tokenid #1
        await bobNftTx.mint(); // Mint #1
        // console.log("Bob mint NFT");

        /**
         * Step 6: Create marketplace contract
         */
        // "refTime: 961948207"
        let gasLimit = setGasLimit(api, 2_000_000_000, 0); 
        const contractFactory = new ConstructorsMarketplace(api, defaultSigner);
      
        platformFee = 500;

        contractAddress = (
            await contractFactory.new(
                cmContractAddress,
                stakingContractAddress,
                platformFee,
                {gasLimit}
            )
        ).address;

        // console.log("contractAddress =", contractAddress);
               
        contract = new ContractMarketplace(contractAddress, defaultSigner, api);
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });

    it('Can list NFT in marketplace', async () => {   
        let nftTokenId;
        let tokenId;
        let price;
        
        // Alice approves for marketplace contract to transfer NFT token #1
        nftTokenId = 1;
        tokenId = PSP34Args.IdBuilder.U64(nftTokenId); // Nft id 1
        
        await aliceNftTx.approve(
            contractAddress,
            tokenId,
            true
        );
        
        // Alice lists token #1        
        price = 10000000000000; // 10 TZERO;        
        await contract.withSigner(alice).tx.list(
            aliceNftContractAddress, 
            tokenId, 
            price
        );
        
        let count = (await query.getSaleTokensIdsCount(
            aliceNftContractAddress,
            alice.address
        )).value.ok!.rawNumber.toString();
        
        expect(count).to.equal("1");
        
        // let rTokenId = await query.getForSaleTokenId(
        //     aliceNftContractAddress,
        //     alice.address,
        //     new BN(1)
        // );
        
        // console.log("rTokenId = ", rTokenId);
    });
});