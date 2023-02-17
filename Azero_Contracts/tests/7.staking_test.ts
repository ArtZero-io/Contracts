import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit, showAZBalance, delay} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsStakingNft from './typed_contracts/constructors/artzero_staking_nft';
import ContractStakingNft from './typed_contracts/contracts/artzero_staking_nft';

import ConstructorsPsp34Nft from './typed_contracts/constructors/psp34_nft';
import ContractPsp34Nft from './typed_contracts/contracts/psp34_nft';

import * as PSP34Args from './typed_contracts/types-arguments/psp34_nft';

describe('Staking test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let nftContractAddress: any;
    let nftContract: any;
    let nftQuery: any;
    let nftTx: any;

    let adminAddress: string;
    let limitUnstakeTime: number;

    let rewards: string;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        // Step 1: Alice creates a Psp34 NFT contract
        // "refTime: 769080524"
        let nftContractGasLimit = setGasLimit(api, 1_600_000_000, 0);
        const nftContractFactory = new ConstructorsPsp34Nft(api, alice);
        
        let nftContractOwner = alice.address;
        let nftName = "Test staking";
        let nftSymbol = "TSK";

        nftContractAddress = (
            await nftContractFactory.new(
                nftContractOwner,
                nftName,
                nftSymbol,
                {gasLimit: nftContractGasLimit}
            )
        ).address;

        // console.log("nftContractAddress =", nftContractAddress);

        nftContract = new ContractPsp34Nft(nftContractAddress, alice, api);    
        nftQuery = nftContract.query;
        nftTx = nftContract.tx;
                      
        // Step 2: Alice mints 2 NFTs and sends NFT tokenId #2 to Bob
        // Mint 2 NFTs
        await nftTx.mint(); // Mint #1
        await nftTx.mint(); // Mint #2

        // let nftCount = (await nftQuery.getLastTokenId()).value.ok;
        // console.log("nftCount =", nftCount);
        
        const nftTokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2
        let nftData = new Array<number>();

        // Transfer #2 to Bob
        await nftTx.transfer(
            bob.address,
            nftTokenId,
            nftData
        );

        // const nftTokenId1 = PSP34Args.IdBuilder.U64(1);
        // let onwerNft1 = (await nftQuery.ownerOf(nftTokenId1)).value.ok;
        // console.log("onwerNft1 = ", onwerNft1);

        // let onwerNft2 = (await nftQuery.ownerOf(nftTokenId)).value.ok;
        // console.log("onwerNft2 = ", onwerNft2);

        // Step 3: Create staking contract 
        // "refTime: 1607624171"
        let gasLimit = setGasLimit(api, 3_200_000_000, 0);
        
        const contractFactory = new ConstructorsStakingNft(api, defaultSigner);
        
        adminAddress = defaultSigner.address;
        limitUnstakeTime = 1; // 1 min
        
        contractAddress = (
            await contractFactory.new(
                adminAddress,
                nftContractAddress,
                limitUnstakeTime,
                {gasLimit}
            )
        ).address;

        // console.log("contractAddress =", contractAddress);

        contract = new ContractStakingNft(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    // it('When staking is locked, not allow staking NFT', async () => {   
    //     // Lock staking
    //     await tx.updateIsLocked(true);

    //     // Alice approves nft #1 
    //     let tokenId = PSP34Args.IdBuilder.U64(1); // Nft id 1
    //     await nftContract.withSigner(alice).tx.approve(
    //         contract.address,
    //         tokenId,
    //         true
    //     ); 

    //     // Alice stakes nft #1
    //     try {
    //         await contract.withSigner(alice).tx.stake([1]);
    //     } catch (error) {
    //         // console.log("Staking contract is locked, cannot stake");
    //     }

    //     let stakingCount = (await query.getTotalStakedByAccount(alice.address)).value.ok;
    //     // console.log("stakingCount =", stakingCount);
        
    //     expect(stakingCount).to.equal(0);

    //     // Unlock staking
    //     await tx.updateIsLocked(false);
    // });

    it('Can stake NFT', async () => {   
        // Step 1: Alice appoves and stakes nft #1
        // console.log("step 1");

        // Alice approves nft #1 
        let tokenId = PSP34Args.IdBuilder.U64(1); // Nft id 1
        await nftContract.withSigner(alice).tx.approve(
            contract.address,
            tokenId,
            true
        ); 
        
        // Alice stakes nft #1
        await contract.withSigner(alice).tx.stake([1]);
        
        // Step 2: Bob appoves and stakes nft #2
        // console.log("step 2");
        // Bob approves nft #2 
        tokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2
        await nftContract.withSigner(bob).tx.approve(
            contract.address,
            tokenId,
            true
        ); 
        
        // Bob stakes nft #2
        await contract.withSigner(bob).tx.stake([2]);

        // Step 3: Check number of nft Alice, Bob staked
        // console.log("step 3");        

        let aliceStakingCount = (await query.getTotalStakedByAccount(alice.address)).value.ok;
        // console.log("aliceStakingCount =", aliceStakingCount);        
        expect(aliceStakingCount).to.equal(1);

        let bobStakingCount = (await query.getTotalStakedByAccount(bob.address)).value.ok;
        // console.log("bobStakingCount =", bobStakingCount);        
        expect(bobStakingCount).to.equal(1);    
    });

    it('Can lock staking and add rewards', async () => {   
        // Lock staking
        await tx.updateIsLocked(true);
        
        // Add rewards
        rewards = "10000000000000"; // 10 TZERO

        await tx.addReward(
            {value: rewards}
        );
        
        let rRewards = (await query.getRewardPool()).value.ok!.rawNumber.toString();
        expect(rewards).to.equal(rRewards);
    });

    it('Can set all stakers claimable', async () => {   
        await tx.setClaimedStatus(alice.address);
        await tx.setClaimedStatus(bob.address);

        let aliceClaimStatus = (await query.isClaimed(alice.address)).value.ok;
        let bobClaimStatus = (await query.isClaimed(bob.address)).value.ok;

        expect(aliceClaimStatus).to.equal(false);
        expect(bobClaimStatus).to.equal(false);
    });

    it('Can enable rewards distribution', async () => {   
        await tx.startRewardDistribution();

        let claimableReward = (await query.getClaimableReward()).value.ok!.rawNumber.toString();
        expect(claimableReward).to.equal(rewards);
        
        let rewardStarted = (await query.getRewardStarted(alice.address)).value.ok;
        expect(rewardStarted).to.equal(true);
    });

    it('Can claim rewards before stopping rewards distribution', async () => {  
        let claimableRewardBefore = (await query.getClaimableReward()).value.ok!.rawNumber;
        // console.log("claimableRewardBefore = ", claimableRewardBefore);

        // Alice claims rewards
        await contract.withSigner(alice).tx.claimReward();

        let claimableRewardAfter = (await query.getClaimableReward()).value.ok!.rawNumber;
        // console.log("claimableRewardAfter = ", claimableRewardAfter);

        let gain = claimableRewardBefore.sub(claimableRewardAfter).toString();
        // console.log("gain = ", gain);

        expect(gain).to.equal("5000000000000");
    });

    it('Can stop rewards distribution and unlock staking contract', async () => {  
        // Stop rewards distribution
        
        let claimableReward = (await query.getClaimableReward()).value.ok!.rawNumber.toString();
        // console.log("claimableReward = ", claimableReward);

        await tx.stopRewardDistribution();

        let rRewards = (await query.getRewardPool()).value.ok!.rawNumber.toString();
        expect(rRewards).to.equal(claimableReward);

        let rclaimableReward = (await query.getClaimableReward()).value.ok!.rawNumber.toString();
        expect(rclaimableReward).to.equal("0");
        
        let rewardStarted = (await query.getRewardStarted(alice.address)).value.ok;
        expect(rewardStarted).to.equal(false);

        // Unlock staking
        await tx.updateIsLocked(false);

        let isLocked = (await query.getIsLocked()).value.ok;
        expect(isLocked).to.equal(false);        
    });

    it('Not alow claiming rewards after stopping rewards distribution', async () => {  
        let rewardPoolBefore = (await query.getRewardPool()).value.ok!.rawNumber.toString();
        // console.log("rewardPoolBefore = ", rewardPoolBefore);

        // Bob claims rewards
        try {
            await contract.withSigner(bob).tx.claimReward();
        } catch (error) {
        }

        let rewardPoolAfter = (await query.getRewardPool()).value.ok!.rawNumber.toString();
        // console.log("rewardPoolAfter = ", rewardPoolAfter);

        expect(rewardPoolAfter).to.equal(rewardPoolBefore);
    });

    it('Can request to unstake', async () => {  
        await contract.withSigner(alice).tx.requestUnstake([1]);
        
        let totalPendingUnstaked = (await query.getTotalPendingUnstakedByAccount(alice.address)).value.ok;
        
        expect(totalPendingUnstaked).to.equal(1);
    });

    it('Can cancel unstaking request', async () => {  
        await contract.withSigner(alice).tx.cancelRequestUnstake([1]);
        
        let totalPendingUnstaked = (await query.getTotalPendingUnstakedByAccount(alice.address)).value.ok;
        
        expect(totalPendingUnstaked).to.equal(0);
    });

    it('Not allow unstake when elapsed time < unstaking time', async () => {
        // Alice requests to unstake
        await contract.withSigner(alice).tx.requestUnstake([1]);

        // Set waiting time 3s (min elapsed time is 1min)
        let waitingTime = 2000;
        await delay(waitingTime);

        // Alice unstakes before unstaking time
        try {
            await contract.withSigner(alice).tx.unstake([1]);    
        } catch (error) {
            
        }
        
        let totalPendingUnstaked = (await query.getTotalPendingUnstakedByAccount(alice.address)).value.ok;
        expect(totalPendingUnstaked).to.equal(1);
    });    

    it('Can unstake when elapsed time >= unstaking time', async () => { 
        // Don't need to request to unstake, it is requested above. Just set the new waiting time.
        // Set waiting time 
        let waitingTime = limitUnstakeTime * 60000 + 1000;
        await delay(waitingTime);
                
        // Alice unstakes before unstaking time
        try {
            await contract.withSigner(alice).tx.unstake([1]);    
        } catch (error) {
            console.log("step 3: error =", error);
        }
        
        let totalPendingUnstaked = (await query.getTotalPendingUnstakedByAccount(alice.address)).value.ok;
        // console.log("step 4 totalPendingUnstaked =", totalPendingUnstaked);
        
        expect(totalPendingUnstaked).to.equal(0);
    });    

    after(async () => {
        // console.log("End");      
    });
});