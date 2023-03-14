import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsCollectionManager from './typed_contracts/constructors/artzero_collection_manager';
import ContractCollectionManager from './typed_contracts/contracts/artzero_collection_manager';
import * as CollectionManagerArgs from './typed_contracts/types-arguments/artzero_collection_manager';

import psp34NftStandard from './artifacts/psp34_nft.json';
import ConstructorsPsp34Nft from './typed_contracts/constructors/psp34_nft';


describe('Collection manager test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let adminAddress: string;
    let standardNftHash: string;
    let simpleModeAddingFee: string;
    let advanceModeAddingFee: string;
    let maxRoyaltyFeeRate: number;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        adminAddress = defaultSigner.address;
        standardNftHash = psp34NftStandard.source.hash;
        simpleModeAddingFee = "3000000000000";
        advanceModeAddingFee = "5000000000000";
        maxRoyaltyFeeRate = 500;
       
        // "refTime: 1556009780"
        let gasLimit = setGasLimit(api, 3_200_000_000, 0);
        
        const contractFactory = new ConstructorsCollectionManager(api, defaultSigner);
        
        contractAddress = (
            await contractFactory.new(
                adminAddress,
                standardNftHash,
                simpleModeAddingFee,
                advanceModeAddingFee,
                maxRoyaltyFeeRate,
                {gasLimit}
            )
        ).address;

        console.log("contractAddress =", contractAddress);

        contract = new ContractCollectionManager(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    it('Creating collection in simple mode works', async () => {   
        // Alice creates collection in simple mode
        let nftName = "Simple mode collection";
        let nftSymbol = "SMC";
        let attributes = ["Hair", "Color", "Type"];
        let attributeVals = ["Straight", "Yellow", "Round"];
        let isCollectRoyaltyFee = true;
        let royaltyFee = 400;

        await contract.withSigner(alice).tx.autoNewCollection(
            nftName,
            nftSymbol,
            attributes,
            attributeVals,
            isCollectRoyaltyFee,
            royaltyFee,
            {value: simpleModeAddingFee} 
        );

        let collectionCount = (await query.getCollectionCount()).value.ok;

        expect(collectionCount).to.equal(1);
    });

    it('Creating collection in advanced mode works', async () => {   
        // Bob creates collection in advance mode

        // Step 1: Bob creates a nft
        let gasLimit = setGasLimit(api, 1_600_000_000, 0);
        
        let contractFactory = new ConstructorsPsp34Nft(api, bob);
        let nftName = "Advanced mode collection";
        let nftSymbol = "AMC";

        let nftContractAddress = (
            await contractFactory.new(
                bob.address,
                nftName,
                nftSymbol,
                {gasLimit}
            )
        ).address;

        // console.log("Bob's nftContractAddress =", nftContractAddress);

        // Step 2: Bob puts nft contract address to addNewCollection func
        let attributes = ["Processor", "RAM", "Thread", "Weight"];
        let attributeVals = ["Intel P4", "32 GB", "8", "2"];
        let isCollectRoyaltyFee = true;
        let royaltyFee = 400;

        await contract.withSigner(bob).tx.addNewCollection(
            nftContractAddress,
            attributes,
            attributeVals,
            isCollectRoyaltyFee,
            royaltyFee,
            {value: advanceModeAddingFee} 
        );

        let collectionCount = (await query.getCollectionCount()).value.ok;
        // console.log("collectionCount =", collectionCount);

        expect(collectionCount).to.equal(2);
    });

    it('Can update collection', async () => {   
        // Step 1: Set multiple attributes 
        
        // Get Alice's nft contract address
        let nftContractAddress = (await query.getContractById(1)).value.ok;
        let attributes = ["Color", "Skin"];
        let values = ["Red", "Dark"];
        
        // console.log("nftContractAddress =", nftContractAddress);

        await contract.withSigner(alice).tx.setMultipleAttributes(
            nftContractAddress,
            attributes,
            values,
        );

        let gAttributes = ["Hair", "Color", "Type", "Skin"];
        let expectedValues = ["Straight", "Red", "Round", "Dark"];
        let rValues = (await query.getAttributes(
            nftContractAddress,
            gAttributes
        )).value.ok;

        // console.log("rValues =", rValues);
        
        expect(rValues.toString()).to.equal(expectedValues.toString());

        // Step 2: update is acvite
        let newIsActive = true;  
        await contract.tx.updateIsActive(
            nftContractAddress,
            newIsActive
        );  

        let rIsActive = (await query.isActive(
            nftContractAddress
        )).value.ok;

        expect(rIsActive).to.equal(newIsActive);

        // Step 3: update royalty fee
        let newRoyaltyFee = 500;  
        await contract.tx.updateRoyaltyFee(
            nftContractAddress,
            newRoyaltyFee
        );  

        let rRoyaltyFee = (await query.getRoyaltyFee(
            nftContractAddress
        )).value.ok;

        expect(rRoyaltyFee).to.equal(newRoyaltyFee);

        // Step 4: update contract type
        let newContractType = CollectionManagerArgs.CollectionType.reserved1;  
        await contract.tx.updateContractType(
            nftContractAddress,
            newContractType
        );  

        let rContractType = (await query.getContractType(
            nftContractAddress
        )).value.ok;

        expect(rContractType).to.equal(newContractType);

        // Step 5: update simple mode adding fee
        let newSimpleModeAddingFee = "4000000000000";;  
        await contract.tx.updateSimpleModeAddingFee(
            newSimpleModeAddingFee
        );  

        let rSimpleModeAddingFee = (await query.getSimpleModeAddingFee(
            nftContractAddress
        )).value.ok!.rawNumber.toString();

        expect(rSimpleModeAddingFee).to.equal(newSimpleModeAddingFee);

        // Step 6: update advance mode adding fee
        let newAdvanceModeAddingFee = "8000000000000";;  
        await contract.tx.updateAdvanceModeAddingFee(
            newAdvanceModeAddingFee
        );  

        let rAdvanceModeAddingFee = (await query.getAdvanceModeAddingFee(
            nftContractAddress
        )).value.ok!.rawNumber.toString();

        expect(rAdvanceModeAddingFee).to.equal(newAdvanceModeAddingFee);

        // Step 7: update royalty fee
        let newMaxRoyaltyFeeRate = 600;  
        await contract.tx.updateMaxRoyaltyFeeRate(
            newMaxRoyaltyFeeRate
        );  

        let rMaxRoyaltyFeeRate = (await query.getMaxRoyaltyFeeRate()).value.ok;

        expect(rMaxRoyaltyFeeRate).to.equal(newMaxRoyaltyFeeRate);

        // Step 8: update standard nft hash
        const newHash = "0x0837d0b95b94e620dc19103852bd83c4bd86eb34e9a5b00fd27210d969965007";
       
        // Only admin can edit
        await contract.tx.updateStandardNftHash(
            newHash
        );    

        let rNewHash = (await query.getStandardNftHash()).value.ok.toString();
        
        expect(rNewHash).to.equal(newHash);
    });
    
    after(async () => {
        // console.log("End");      
    });
});