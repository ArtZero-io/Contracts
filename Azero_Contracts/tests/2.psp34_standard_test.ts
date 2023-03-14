import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsPsp34Nft from './typed_contracts/constructors/psp34_nft';
import ContractPsp34Nft from './typed_contracts/contracts/psp34_nft';

import * as PSP34Args from './typed_contracts/types-arguments/psp34_nft';

describe('Psp34 standard test', () => {
    let api: any;
    let signers: any;
    let defaultSigner: any;
    let alice: any;
    let bob: any;

    let contractAddress: any;
    let contract: any;
    let query: any;
    let tx: any;

    let attrs, values: Array<string>;

    let contractOwner: string;
    let name: string;
    let symbol: string;

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        // "refTime: 769080524"
        let gasLimit = setGasLimit(api, 1_600_000_000, 0);
        
        const contractFactory = new ConstructorsPsp34Nft(api, defaultSigner);
        
        contractOwner = defaultSigner.address;
        name = "Charlie";
        symbol = "CHL";

        contractAddress = (
            await contractFactory.new(
                contractOwner,
                name,
                symbol,
                {gasLimit}
            )
        ).address;

        console.log("contractAddress =", contractAddress);

        contract = new ContractPsp34Nft(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    it('Can mint without attrs', async () => {   
        await contract.tx.mint(); 

        let lastTokenId = (await query.getLastTokenId()).value.ok; 
        
        expect(lastTokenId).to.equal(1);
    });

    it('Can mint with attrs', async () => {   
        // console.log("step 0");

        let metadata: Array<[string, string]> = [
            ["background", "pale"],
            ["eyeColor", "blue"],
            ["weapon", "gun"],
        ];
        
        await contract.tx.mintWithAttributes(
            metadata
        ); 
        
        let lastTokenId = (await query.getLastTokenId()).value.ok;        
        // console.log("step 1 lastTokenId =", lastTokenId);

        expect(lastTokenId).to.equal(2);

        const tokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2
        
        attrs = ["background", "eyeColor", "weapon"];
        values = ["pale", "blue", "gun"];
        
        let rValues: Array<string> = (await query.getAttributes(
            tokenId, 
            attrs
        )).value.ok;  
        
        // console.log("step 2 rValues =", rValues);
        
        expect(values.toString()).to.equal(rValues.toString());
    });

    it('Can lock nft', async () => {   
        const tokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2

        await contract.tx.lock(tokenId); 

        let lockedTokenCount = (await query.getLockedTokenCount()).value.ok;
        expect(lockedTokenCount).to.equal(1);

        let lockedNft = (await query.isLockedNft(tokenId)).value.ok;
        expect(lockedNft).to.equal(true);
    });

     // let lastTokenId = (await query.getLastTokenId()).value.ok; 
        // expect(lastTokenId).to.equal(1); 
    it('Can burn nft', async () => {   
        const tokenId = PSP34Args.IdBuilder.U64(2); // Nft id 2

        await contract.tx.burn(
            contractOwner,
            tokenId
        );        

        let totalSupply = (await query.totalSupply()).value.ok!.rawNumber.toString();
        expect(totalSupply).to.equal("1");        

        // let lockedNft = (await query.isLockedNft(tokenId)).value.ok;
        // expect(lockedNft).to.equal(false);
    });

    after(async () => {
        // console.log("End");      
    });
});