import { provider, expect, getSigners, checkAccountsBalance, decodeToBytes, toString, setGasLimit} from './helpers';
import { ApiPromise } from '@polkadot/api';

import ConstructorsProfileManager from './typed_contracts/constructors/profile_manager';
import ContractProfileManager from './typed_contracts/contracts/profile_manager';

describe('Profile test', () => {
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

    async function setup() {
        api = await ApiPromise.create({ provider });

        signers = getSigners();
        defaultSigner = signers[2];
        alice = signers[0];
        bob = signers[1];

        await checkAccountsBalance(signers, api);

        // "refTime: 374963835"
        let gasLimit = setGasLimit(api, 800_000_000, 0);
        
        const contractFactory = new ConstructorsProfileManager(api, defaultSigner);
        
        contractAddress = (
            await contractFactory.new(
                {gasLimit}
            )
        ).address;

        // console.log("contractAddress =", contractAddress);

        contract = new ContractProfileManager(contractAddress, defaultSigner, api);    
        query = contract.query;
        tx = contract.tx;
    };

    before(async () => {
        // console.log("Start");
        await setup();
    });
    
    it('Can get set attributes', async () => {   
        // console.log("step 0");

        attrs = ["userName", "twitter", "facebook", "telegram"];
        values = ["Charlie", "https://twitter.com/charlie_artzero", "https://facebook.com/charlie.artzero", "https://t.me/charlie.artzero"];

        await contract.withSigner(alice).tx.setMultipleAttributes(
            attrs,
            values
        ); 

        // console.log("step 1 values = ", values);

        let rValues: Array<string> = (await query.getAttributes(
            alice.address,
            attrs
        )).value.ok; 

        // console.log("step 2 rValues =", rValues);
        
        expect(values.toString()).to.equal(rValues.toString());
    });

    after(async () => {
        // console.log("End");      
    });
});