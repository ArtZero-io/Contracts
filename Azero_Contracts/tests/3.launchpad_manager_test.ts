import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsLaunchpadPsp34 from './typed_contracts/constructors/artzero_launchpad_psp34';
import ContractLaunchpadPsp34 from './typed_contracts/contracts/artzero_launchpad_psp34';

import launchpadPsp34NftStandard from './artifacts/launchpad_psp34_nft_standard.json';

describe('Launchpad manager test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let maxPhasesPerProject: number;
    let standardNftHash: string;
    let projectAddingFee: string;
    let projectMintFeeRate: number;
    let publicMaxMintingAmount: number;
    let adminAddress: string;

    let totalSupply: number;
    let startTime: number;
    let endTime: number;
    let projectInfo: string;
    let codePhases: Array<string>;
    let isPublicPhases: Array<boolean>;
    let publicMintingFeePhases: Array<string>;
    let publicMintingAmountPhases: Array<number>;
    let publicMaxMintingAmountPhases: Array<number>;
    let startTimePhases: Array<number>;
    let endTimePhases: Array<number>;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        maxPhasesPerProject = 4;
        adminAddress = defaultSigner.address;
        standardNftHash = launchpadPsp34NftStandard.source.hash;
        projectAddingFee = "5000000000000"; // 5 TZERO
        projectMintFeeRate = 200;  // 2%
        publicMaxMintingAmount = 2; 

        // "refTime: 1174764604"
        // "proofSize: 23552"
        let gasLimit = setGasLimit(api, 3_200_000_000, 48_000);
        
        const contractFactory = new ConstructorsLaunchpadPsp34(api, defaultSigner);
        
        contractAddress = (
            await contractFactory.new(
                maxPhasesPerProject,
                adminAddress,
                standardNftHash,
                projectAddingFee,
                projectMintFeeRate,
                publicMaxMintingAmount,
                {gasLimit}
            )
        ).address;

        // console.log("contractAddress =", contractAddress);

        contract = new ContractLaunchpadPsp34(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    it('Can add project', async () => {   
        // Alice adds a new project

        totalSupply = 100;
        startTime = new Date().getTime();
        endTime = new Date().getTime() + 8 * 86400000;
        projectInfo = "Testing project";
        codePhases = ["Phase 1", "Phase 2"];
        isPublicPhases = [true, false];
        publicMintingFeePhases = ["1000000000000", "2000000000000"];
        publicMintingAmountPhases = [50, 30];
        publicMaxMintingAmountPhases = [2, 1];
        startTimePhases = [startTime, startTime + 4 * 86400000];
        endTimePhases = [startTime + 3 * 86400000, startTime + 5 * 86400000];
        
        await contract.withSigner(alice).tx.addNewProject(
            totalSupply,
            startTime,
            endTime,
            projectInfo,
            codePhases,
            isPublicPhases,
            publicMintingFeePhases,
            publicMintingAmountPhases,
            publicMaxMintingAmountPhases,
            startTimePhases,
            endTimePhases,
            {value: projectAddingFee}
        ); 

        let projectCount = (await query.getProjectCount()).value.ok;
        expect(projectCount).to.equal(1);
    });

    // it('Can update project adding fee', async () => {
    // }

    it('Can update project', async () => {
        // Step 1: Edit start, end time
        // console.log("Step 1: Edit start, end time");
        
        let contractAddress = (await query.getProjectById(1)).value.ok;
        
        let newStartTime = new Date().getTime();
        let newEndTime = newStartTime + 86400000;

        // console.log("contractAddress = ", contractAddress, "newStartTime = ", newStartTime, "newEndTime = ", newEndTime);
        
        // Only admin can edit
        await contract.tx.editProject(
            contractAddress,
            newStartTime,
            newEndTime
        );    

        let project = (await query.getProjectByNftAddress(contractAddress)).value.ok;
        // console.log("project = ", project);
        
        expect(project.startTime).to.equal(newStartTime);
        expect(project.endTime).to.equal(newEndTime);
        
        // Step 2: Update project adding fee
        // console.log("Step 2: Update project adding fee");
        
        let newProjectAddingFee = "4000000000000"; // 4 TZERO 
        // Only admin can edit
        await contract.tx.updateProjectAddingFee(
            newProjectAddingFee
        );    

        let rNewProjectAddingFee = (await query.getProjectAddingFee()).value.ok!.rawNumber.toString();
        // console.log("rNewProjectAddingFee = ", rNewProjectAddingFee);
        
        expect(rNewProjectAddingFee).to.equal(newProjectAddingFee);

        // Step 3: Update public max minting amount
        // console.log("Step 3: Update public max minting amount");   
        
        let newPublicMaxMintingAmount = 3; // 4 TZERO 
        // Only admin can edit
        await contract.tx.updatePublicMaxMintingAmount(
            newPublicMaxMintingAmount
        );    

        let rNewPublicMaxMintingAmount = (await query.getPublicMaxMintingAmount()).value.ok;
        // console.log("rNewPublicMaxMintingAmount = ", rNewPublicMaxMintingAmount);
        
        expect(rNewPublicMaxMintingAmount).to.equal(newPublicMaxMintingAmount);

        // Step 4: Update project mint fee rate
        // console.log("Step 4: Update update project mint fee rate");   
        
        let newProjectMintFeeRate = 100; // 1%
        // Only admin can edit
        await contract.tx.updateProjectMintFeeRate(
            newProjectMintFeeRate
        );    

        let rNewProjectMintFeeRate = (await query.getProjectMintFeeRate()).value.ok;
        // console.log("rNewProjectMintFeeRate = ", rNewProjectMintFeeRate);
        
        expect(rNewProjectMintFeeRate).to.equal(newProjectMintFeeRate);
    
        // Step 5: Update is active project
        // console.log("Step 5: Update is active project");

        let newIsActive = true;
       
        // Only admin can edit
        await contract.tx.updateIsActiveProject(
            newIsActive,
            contractAddress
        );    

        project = (await query.getProjectByNftAddress(contractAddress)).value.ok;
        // console.log("project = ", project);
        
        expect(project.isActive).to.equal(newIsActive);
        
        // Step 6: Update standard nft hash
        // console.log("Step 6: Update standard nft hash");
        const newHash = "0x0837d0b95b94e620dc19103852bd83c4bd86eb34e9a5b00fd27210d969965007";
       
        // Only admin can edit
        await contract.tx.updateStandardNftHash(
            newHash
        );    

        let rNewHash = (await query.getStandardNftHash()).value.ok.toString();
        // console.log("rNewHash = ", rNewHash);
        
        expect(rNewHash).to.equal(newHash);
    }); 

    after(async () => {
        // console.log("End");      
    });
});