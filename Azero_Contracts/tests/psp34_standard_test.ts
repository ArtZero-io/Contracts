// In this example we will deploy & interact with psp22 token to mint some tokens to the owner and get total supply.
import Contract from "./typed_contracts/contracts/psp34_nft";
import {ApiPromise, WsProvider} from "@polkadot/api";
import Constructors from "./typed_contracts/constructors/psp34_nft";
import {getSigners} from './helpers'
import jsonrpc from "@polkadot/types/interfaces/jsonrpc";
import '@polkadot/api-augment';
import BN from 'bn.js'

async function main() {

  const signers = getSigners();
  const defaultSigner = signers[2];
  const alice = signers[0];
  const bob = signers[1];

  console.log('Checking Accounts and Balance')
  for (var i=0;i<signers.length; i++){
    const { data: { free, reserved, miscFrozen } } = await api.query.system.account(signers[i].address);
    const balance =
      new BN(free).div(new BN(10 ** 6)).toNumber() / 10 ** 6 -
      new BN(miscFrozen).div(new BN(10 ** 6)).toNumber() / 10 ** 6;
    console.log(`Account ${i} has address: ${signers[i].address} and balance: ${balance} TZERO`);
  }

	const constructors = new Constructors(api, defaultSigner);
  /** Create new contract for PSP34_NFT Standard
   * @arg: args: [
   * 0: contractOwner - 8,
   * 1: name - 33,
   * 2: symbol - 33,
   * ]
   */
  console.log('Creating PSP34 NFT contract...');

	// const {address: TOKEN_ADDRESS} = await constructors.new(defaultSigner.address,'TEST','ABC');
	// console.log('Contract deployed at:', TOKEN_ADDRESS);

  /* can use 5FuTnDDTux2wVLwrHSwRBHcd1zkZ2br4Hm6UJEaWZNYcYM6S if dont want to create new contract */
  const TOKEN_ADDRESS = "5FuTnDDTux2wVLwrHSwRBHcd1zkZ2br4Hm6UJEaWZNYcYM6S";
  console.log('Contract deployed at:', TOKEN_ADDRESS);

  const contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);

  //Check the total supply and the balance of defaultSigner
  const {value: totalSupply} = await contract.query['PSP34::total_supply']();
  console.log('NFT Total Supply:',totalSupply);
  const {value: defaultBalance} = await contract.query['PSP34::balance_of'](defaultSigner.address);
  console.log('defaultSigner Balance:',defaultBalance);

  const {value: contractOwner} = await contract.query['Psp34Traits::get_owner']();
  console.log('NFT Contract Owner:',contractOwner);

  //Test Minting Function
  console.log('Minting a new NFT ...');
  const mintTx = await contract.tx.mint({
  	gasLimit : 100000000000
  });
  console.log('mintTx',mintTx.txHash);

  const {value: totalSupplyAfterMint} = await contract.query['PSP34::total_supply']();
  console.log('NFT Total Supply After Mint:',totalSupplyAfterMint);
  const {value: defaultBalanceAfterMint} = await contract.query['PSP34::balance_of'](defaultSigner.address);
  console.log('defaultSigner Balance After Mint:',defaultBalanceAfterMint);
  //
  // console.log('Total Supply:',await contract.query['PSP34::total_supply']());
  // console.log('defaultSigner Balance:',await contract.query['PSP34::balance_of'](defaultSigner.address));

	// const totalSupply = await contract.query['PSP34::total_supply']();
	// const balance = await contract.query['PSP34::balance_of'](aliceKeyringPair.address);
  //
	// console.log(`%c Total supply before minting: ${totalSupply.value}`, 'color: green');
	// console.log(`%c Balance of Alice before minting: ${balance.value}`, 'color: green');
  //
	// const mintTx = await contract.tx['PSP22Mintable::mint'](aliceKeyringPair.address, '10000');
  //
	// const totalSupplyAfterMint = await contract.query['PSP22::total_supply']();
	// const balanceAfterMint = await contract.query['PSP22::balance_of'](aliceKeyringPair.address);
  //
	// console.log(`%c Total supply after minting: ${totalSupplyAfterMint.value}`, 'color: green');
	// console.log(`%c Balance of Alice after minting: ${balanceAfterMint.value}`, 'color: green');

	//await api.disconnect();
}

  const provider = new WsProvider("wss://ws.test.azero.dev");
  const api = new ApiPromise({
    provider,
    rpc: jsonrpc
  });
  api.on("connected", () => {
    api.isReady.then((api) => {
      //console.log("Testnet AZERO Connected");
    });
  });

  api.on("ready", async () => {
    console.log("Testnet AZERO Ready. Running Test Now...");
    await main();
  });

  api.on("error", (err) => {
    console.log('error', err );
  });
