import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit, delay} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsLaunchpadPsp34NftStandard from './typed_contracts/constructors/launchpad_psp34_nft_standard';
import ContractLaunchpadPsp34NftStandard from './typed_contracts/contracts/launchpad_psp34_nft_standard';
import launchpadPsp34NftStandard from './artifacts/launchpad_psp34_nft_standard.json';

import ConstructorsLaunchpadPsp34 from './typed_contracts/constructors/artzero_launchpad_psp34';
import ContractLaunchpadPsp34 from './typed_contracts/contracts/artzero_launchpad_psp34';

import {BN} from '@polkadot/util';

describe('Psp34 launchpad standard test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let lmContractAddress;
    let lmContract: any;
    let lmQuery: any;
    let lmTx: any;

    let contractOwner;
    let totalSupply;
    let startTime: number;
    let period: number;
    let projectInfo;
    let codePhases;
    let isPublicPhases;
    let publicMintingFeePhases: string[];
    let publicMintingAmountPhases;
    let publicMaxMintingAmountPhases; 
    let startTimePhases: number[];
    let endTimePhases; 
    let preparationDuration: number;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        // Step 1: Create launchpad manager contract
        let lmMaxPhasesPerProject = 4;
        let lmAdminAddress = defaultSigner.address;
        let lmStandardNftHash = launchpadPsp34NftStandard.source.hash;
        let lmProjectAddingFee = "5000000000000"; // 5 TZERO
        let lmProjectMintFeeRate = 200;  // 2%
        let lmPublicMaxMintingAmount = 2; 

        // "refTime: 1546286098"
        let lmGasLimit = setGasLimit(api, 3_200_000_000, 0);
        
        const lmContractFactory = new ConstructorsLaunchpadPsp34(api, defaultSigner);
        
        lmContractAddress = (
            await lmContractFactory.new(
                lmMaxPhasesPerProject,
                lmAdminAddress,
                lmStandardNftHash,
                lmProjectAddingFee,
                lmProjectMintFeeRate,
                lmPublicMaxMintingAmount,
                {gasLimit: lmGasLimit}
            )
        ).address;

        console.log("lmContractAddress =", lmContractAddress);

        lmContract = new ContractLaunchpadPsp34(lmContractAddress, defaultSigner, api);    
        lmQuery = lmContract.query;
        lmTx = lmContract.tx;

        // Step 2: Create launchpad psp34 nft standard contract  
        contractOwner = defaultSigner.address;
        totalSupply = 200;

        preparationDuration = 8000; // 8s for adding/updating whitelist 
        startTime = new Date().getTime() + preparationDuration; // currentTime + preparationDuration
        period = 3000;
        projectInfo = "Testing project";
        codePhases = ["Phase 1", "Phase 2", "Phase 3"];
        isPublicPhases = [true, true, false];
        publicMintingFeePhases = ["2000000000000", "4000000000000", "6000000000000"];
        publicMintingAmountPhases = [50, 30, 10];
        publicMaxMintingAmountPhases = [2, 1, 1];
        startTimePhases = [startTime, startTime + 2 * period, startTime + 4 * period];
        endTimePhases = [startTime + 2 * period - 500, startTime + 3 * period, startTime + 6 * period];
        
        // "refTime: 1797943620"
        let gasLimit = setGasLimit(api, 3_600_000_000, 0);
        
        const contractFactory = new ConstructorsLaunchpadPsp34NftStandard(api, defaultSigner);
        
        contractAddress = (
            await contractFactory.new(
                lmContractAddress,
                lmMaxPhasesPerProject,
                contractOwner,
                totalSupply,
                projectInfo,
                codePhases,
                isPublicPhases,
                publicMintingFeePhases,
                publicMintingAmountPhases,
                publicMaxMintingAmountPhases,
                startTimePhases,
                endTimePhases,
                {gasLimit}
            )
        ).address;

        console.log("contractAddress =", contractAddress);

        contract = new ContractLaunchpadPsp34NftStandard(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    it('Can add whitelist', async () => {   
        // Add Alice and Bob to whitelist in phase 1
        await tx.addWhitelist(
            alice.address,
            1,
            10,
            "1000000000000"
        );

        await tx.addWhitelist(
            bob.address,
            1,
            5,
            "1000000000000"
        );

        // Add Bob to whitelist in phase 3
        await tx.addWhitelist(
            bob.address,
            3,
            8,
            "3000000000000"
        );

        let whitelistCount = (await query.getWhitelistCount()).value.ok;
        expect(whitelistCount).to.equal(3); 
    });

    it('Can update whitelist', async () => {   
        let whitelistAmount = 1;
        let mintingFee = "1500000000000"; 
        await tx.updateWhitelist(
            bob.address,
            1,
            whitelistAmount,
            mintingFee
        );

        let whitelist = (await query.getWhitelistByAccountId(bob.address, 1)).value.ok;
        // console.log("whitelist = ", whitelist);

        expect(whitelist.whitelistAmount).to.equal(whitelistAmount);
        expect(whitelist.mintingFee.toString()).to.equal(mintingFee);

        let currentTime = new Date().getTime();
        if (startTimePhases[0] > currentTime) {
            // console.log("waiting time for phase 1 = ", startTimePhases[0] - currentTime + 300);
            await delay(startTimePhases[0] - currentTime + 300);
        }
    });
    
    it('Not allow to public mint amount > public max minting amount', async () => {
        let phaseId;
        let mintAmount;
        let claimedAmount;

        // Alice mints 2 NFT in phase 1 (currentTime >= startTime)
        phaseId = 1;
        mintAmount = 3;

        try {
            await contract.withSigner(alice).tx.publicMint(
                phaseId,
                mintAmount,
                {value: new BN(publicMintingFeePhases[phaseId - 1]).mul(new BN(mintAmount))}
            ); 
        } catch (error) {            
        }
 
        claimedAmount = (await query.getPhaseAccountPublicClaimedAmount(
            alice.address,
            phaseId
        )).value.ok; 

        expect(claimedAmount).to.equal(null);
    });
    
    it('Can public mint', async () => {
        let phaseId;
        let mintAmount;
        let claimedAmount;

        // Alice public mints 2 NFT in phase 1 (currentTime >= startTime)
        phaseId = 1;
        mintAmount = 2;

        await contract.withSigner(alice).tx.publicMint(
            phaseId,
            mintAmount,
            {value: new BN(publicMintingFeePhases[phaseId - 1]).mul(new BN(mintAmount))}
        );

        claimedAmount = (await query.getPhaseAccountPublicClaimedAmount(
            alice.address,
            phaseId
        )).value.ok;
        expect(claimedAmount).to.equal(mintAmount);

        // Wait for phase 2
        let currentTime = new Date().getTime();
        if (startTimePhases[1] > currentTime) {
            // console.log("waiting time for phase 2 = ", startTimePhases[1] - currentTime + 300);
            await delay(startTimePhases[1] - currentTime + 300);
        }

        // Bob public mints 1 NFT in phase 2
        // console.log("Bob mints");
        phaseId = 2;
        mintAmount = 1;

        await contract.withSigner(bob).tx.publicMint(
            phaseId,
            mintAmount,
            {value: new BN(publicMintingFeePhases[phaseId - 1]).mul(new BN(mintAmount))}
        );

        // console.log("Get Bob claimedAmount");
        claimedAmount = (await query.getPhaseAccountPublicClaimedAmount(
            bob.address,
            phaseId
        )).value.ok;

        // console.log("claimedAmount = ", claimedAmount);
        expect(claimedAmount).to.equal(mintAmount);
    }); 
    
    it('Not allow public mint in a private phase', async () => {
        // Wait for phase 3
        let currentTime = new Date().getTime();
        if (startTimePhases[2] > currentTime) {
            // console.log("waiting time for phase 3 = ", startTimePhases[2] - currentTime + 300);
            await delay(startTimePhases[2] - currentTime + 300);
        }

        let phaseId;
        let mintAmount;
        let claimedAmount;

        // Alice mints 1 NFT in phase 3 (private phase -> error)
        phaseId = 3;
        mintAmount = 1;

        try {
            await contract.withSigner(alice).tx.publicMint(
                phaseId,
                mintAmount,
                {value: new BN(publicMintingFeePhases[phaseId - 1]).mul(new BN(mintAmount))}
            ); 
        } catch (error) {            
        }
 
        claimedAmount = (await query.getWhitelistByAccountId(
            alice.address,
            phaseId
        )).value.ok; 

        expect(claimedAmount).to.equal(null);
    });
    
    it('Can whitelist mint', async () => {
        let phaseId;
        let mintAmount;

        // Bob mints 1 NFT in phase 3 (private phase)
        phaseId = 3;
        mintAmount = 3;

        let whitelist = (await query.getWhitelistByAccountId(bob.address, phaseId)).value.ok;
        
        await contract.withSigner(bob).tx.whitelistMint(
            phaseId,
            mintAmount,
            {value: new BN(whitelist.mintingFee).mul(new BN(mintAmount))}
        );  
 
        whitelist = (await query.getWhitelistByAccountId(bob.address, phaseId)).value.ok;
        // console.log("whitelist = ", whitelist);

        expect(whitelist.claimedAmount).to.equal(mintAmount);
    });

    it('Owner can mint', async () => {
        let mintAmount;

        // Bob mints 1 NFT in phase 3 (private phase)
        mintAmount = 10;

        await tx.mint(
            mintAmount
        );  
        
        let ownerClaimedAmount = (await query.getOwnerClaimedAmount()).value.ok;
        expect(ownerClaimedAmount).to.equal(mintAmount);
    });

    after(async () => {
        // console.log("End");      
    });
});