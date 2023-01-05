// In this example we will deploy & interact with psp22 token to mint some tokens to the owner and get total supply.
import Contract from "./typed_contracts/contracts/psp34_nft";
import {ApiPromise, WsProvider} from "@polkadot/api";
import Constructors from "./typed_contracts/constructors/psp34_nft";
import { getSigners, showAZBalance } from './helpers'
import jsonrpc from "@polkadot/types/interfaces/jsonrpc";
import '@polkadot/api-augment';
import BN from 'bn.js'
import { KeyringPair } from '@polkadot/keyring/types'
import { Keyring } from '@polkadot/keyring'

let signers: KeyringPair[];
let defaultSigner: KeyringPair;
let alice: KeyringPair;
let bob: KeyringPair;

const TOKEN_ADDRESS = "5GA9BmAxWV8KtjENR4mW4wHCNLTFgsoNKMZW1wYkGm3oUs52";

function setup() {
  signers = getSigners();
  defaultSigner = signers[2];
  alice = signers[0];
  bob = signers[1];
}
// Create new PSP34 Contract, this should be performed once then replace the new contract address to TOKEN_ADDRESS
async function createNFTContract(){
  const constructors = new Constructors(api, defaultSigner);
  console.log('Creating PSP34 NFT contract...');
  /** Create new contract for PSP34_NFT Standard
   * @arg: args: [
   * 0: contractOwner - 8,
   * 1: name - 33,
   * 2: symbol - 33,
   * ]
   */
   const {address: TOKEN_ADDRESS} = await constructors.new(defaultSigner.address,'TEST','ABC');
   console.log('Contract deployed at:', TOKEN_ADDRESS);
}

// Get general information from the contract and user balances
async function generalInfo() {
  console.log('Checking Accounts and Balance')
  for (var i=0;i<signers.length; i++){
    const { data: { free, reserved, miscFrozen } } = await api.query.system.account(signers[i].address);
    const balance =
      new BN(free).div(new BN(10 ** 6)).toNumber() / 10 ** 6 -
      new BN(miscFrozen).div(new BN(10 ** 6)).toNumber() / 10 ** 6;
    console.log(`Account ${i} has address: ${signers[i].address} and balance: ${balance} TZERO`);
  }
  const contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  //Check the total supply and the balance of defaultSigner
  const {value: totalSupply} = await contract.query['PSP34::total_supply']();
  console.log('NFT Total Supply:',totalSupply);
  const {value: contractOwner} = await contract.query['Psp34Traits::get_owner']();
  console.log('NFT Contract Owner:',contractOwner);
}
/*
  testAdminTransferPSP22
  Test case: transfer PSP22 from Contract to Alice Account
  Pre-requisite: transfer tokens from PSP22 contract to NFT Contract
  Expect: transfer from contract using bob fails and using defaultSigner ok
*/
async function testAdminTransferPSP22(){
  //This test use PSP22 contract 5DthbdX2xEE7NsvCnHHxtw7fcr9KXJ6cnzS5dBaqgNc34hiX for testing
  const psp22_contract = "5DthbdX2xEE7NsvCnHHxtw7fcr9KXJ6cnzS5dBaqgNc34hiX";
  let contract = new Contract(TOKEN_ADDRESS, bob, api);
  try {
    console.log('Transfer Tokens to Alice account using Bob (should fail)');
    const withdraw = await contract.tx["AdminTrait::tranfer_psp22" ](psp22_contract,100*10**9,alice.address,{
      gasLimit : 100000000000
    });
    console.log('withdraw',withdraw.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  console.log('Transfer Token to Alice account using defaultSigner');
  contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  const withdraw = await contract.tx["AdminTrait::tranfer_psp22" ](psp22_contract,100*10**9,alice.address,{
    gasLimit : 100000000000
  });
  console.log('withdraw',withdraw.txHash);
}
/*
  testAdminTransfer
  Test case: Mint new NFT, transfer NFT to contract, transfer NFT from contract to alice
  Pre-requisite: none
  Expect: mint OK, transfer NFT to contract OK, transfer from contract using bob fails and using defaultSigner ok
*/
async function testAdminTransfer() {
  let contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  console.log('Minting a new NFT ...');
  const mintTx = await contract.tx.mint({
    gasLimit : 100000000000
  });
  console.log('mintTx',mintTx.txHash);
  let {value: totalSupplyAfterMint} = await contract.query['PSP34::total_supply']();

  {
    let {value: ownerAddress} = await contract.query['PSP34::owner_of']({u64:totalSupplyAfterMint});
    console.log(`Owner of NFT ${totalSupplyAfterMint} is ${ownerAddress}`);
  }

  console.log(`Transfer NFT ${totalSupplyAfterMint} to Contract ...`);

  const transferTx = await contract.tx["PSP34::transfer"](TOKEN_ADDRESS,{u64:totalSupplyAfterMint},[],{
    gasLimit : 100000000000
  });

  console.log('transferTx',transferTx.txHash);

  {
    let {value: ownerAddress} = await contract.query['PSP34::owner_of']({u64:totalSupplyAfterMint});
    console.log(`Owner of NFT ${totalSupplyAfterMint} is ${ownerAddress}`);
  }

  try {
    console.log('Transfer NFT to Alice account using Bob (should fail)');
    contract = new Contract(TOKEN_ADDRESS, bob, api);
    const withdraw = await contract.tx["AdminTrait::tranfer_nft" ](TOKEN_ADDRESS,{u64:totalSupplyAfterMint},alice.address,{
      gasLimit : 100000000000
    });
    console.log('withdraw',withdraw.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  console.log('Transfer NFT to Alice account using defaultSigner');
  contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  const withdraw = await contract.tx["AdminTrait::tranfer_nft" ](TOKEN_ADDRESS,{u64:totalSupplyAfterMint},alice.address,{
    gasLimit : 100000000000
  });
  console.log('withdraw',withdraw.txHash);

  {
    let {value: ownerAddress} = await contract.query['PSP34::owner_of']({u64:totalSupplyAfterMint});
    console.log(`Owner of NFT ${totalSupplyAfterMint} is ${ownerAddress}`);
  }

}
/*
  testLockNFT
  Test case: Mint new NFT, set Attributes to the NFT, show Attributes and then Lock the NFT
  Pre-requisite: none
  Expect: mint OK, Attributes displays correctly, alice fails to lock, defaultSigner can lock, then cannot set attributes
*/
async function testLockNFT(){

  let contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  console.log('Minting a new NFT ...');
  const mintTx = await contract.tx.mint({
    gasLimit : 100000000000
  });
  console.log('mintTx',mintTx.txHash);

  let {value: totalSupplyAfterMint} = await contract.query['PSP34::total_supply']();
  console.log("Set Attributes for new NFT ",totalSupplyAfterMint);

  const attributes: Array<[string, string]> = [["ghi","333"],["klm","555"]];
  const setAttrTx = await contract.tx["Psp34Traits::set_multiple_attributes"]({u64:totalSupplyAfterMint},attributes,{
    gasLimit : 100000000000
  });
  // Get Attribute count
  const {value: attrCount} = await contract.query['Psp34Traits::get_attribute_count']();
  console.log("attrCount",attrCount);

  let attNames: Array<string> = [];
  for (var i=1;i<=attrCount;i++){
    const {value: name} = await contract.query['Psp34Traits::get_attribute_name'](i);
    attNames.push(name);
    console.log(`Attribute ${i} Name is ${name}`);
  }

  const {value: attrValues} = await contract.query['Psp34Traits::get_attributes']({u64:totalSupplyAfterMint},attNames);
  console.log(`NFT ID ${totalSupplyAfterMint} has attrValues ${attrValues} for ${attNames}`);

  console.log(`Locking NFT ${totalSupplyAfterMint} using Alice account (should fail)...`);

  contract = new Contract(TOKEN_ADDRESS, alice, api);
  //should fail
  try {
    const lock = await contract.tx["Psp34Traits::lock" ]({u64:totalSupplyAfterMint},{
    	gasLimit : 100000000000
    });
    console.log('lock',lock.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  console.log(`Locking NFT ${totalSupplyAfterMint} using NFT Owner account...`);
  contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  const lock = await contract.tx["Psp34Traits::lock" ]({u64:totalSupplyAfterMint},{
    gasLimit : 100000000000
  });
  console.log('lock',lock.txHash);

  const {value: isLocked} = await contract.query['Psp34Traits::is_locked_nft']({u64:totalSupplyAfterMint});
  console.log(`NFT ID ${totalSupplyAfterMint} lock Status ${isLocked}`);

  console.log(`Setting Attributes to NFT ${totalSupplyAfterMint} (should fail)...`);
  try {
    const attributes: Array<[string, string]> = [["ghi","32233"],["klm","55225"]];
    const setAttrTx = await contract.tx["Psp34Traits::set_multiple_attributes"]({u64:totalSupplyAfterMint},attributes,{
      gasLimit : 100000000000
    });
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  const {value: lockNFTCount} = await contract.query['Psp34Traits::get_locked_token_count']();
  console.log(`Total locked NFT: ${lockNFTCount}`);
}
/*
  testWithdrawFee
  Test case: Test admin WithdrawFee using Contract Owner
  Pre-requisite: Use any account to transfer 10 AZERO to contract
  Expect: Contract send back the amount to receiver
*/
async function testWithdrawFee() {

  console.log(`Contract balance: ${await showAZBalance(api,TOKEN_ADDRESS)} TZERO`);
  console.log(`Alice balance: ${await showAZBalance(api,alice.address)} TZERO`);

  console.log('Transfer 10 AZERO to Alice ...');
  try {
    const contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
    const withdraw = await contract.tx["AdminTrait::withdraw_fee" ](10*10**12,alice.address,{
    	gasLimit : 100000000000
    });
    console.log('withdraw',withdraw.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  console.log(`Contract balance: ${await showAZBalance(api,TOKEN_ADDRESS)} TZERO`);
  console.log(`Alice balance: ${await showAZBalance(api,alice.address)} TZERO`);
}
/*
  testWithdrawFeeNoOwner
  Test case: Test admin WithdrawFee not using Contract Owner
  Pre-requisite: Use any account to transfer 10 AZERO to contract
  Expect: contract fails
*/
async function testWithdrawFeeNoOwner() {

  console.log(`Contract balance: ${await showAZBalance(api,TOKEN_ADDRESS)} TZERO`);
  console.log(`Alice balance: ${await showAZBalance(api,alice.address)} TZERO`);

  console.log('Transfer 10 AZERO to Alice ...');
  try {
    const contract = new Contract(TOKEN_ADDRESS, bob, api);
    const withdraw = await contract.tx["AdminTrait::withdraw_fee" ](10*10**12,alice.address,{
    	gasLimit : 100000000000
    });
    console.log('withdraw',withdraw.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  console.log(`Contract balance: ${await showAZBalance(api,TOKEN_ADDRESS)} TZERO`);
  console.log(`Alice balance: ${await showAZBalance(api,alice.address)} TZERO`);
}
/*
  testMintNotOwner
  Test case: Test Mint functions with Caller != Contract Owner
  Expect: Contract Call fails
*/
async function testMintNotOwner() {

  const contract = new Contract(TOKEN_ADDRESS, alice, api);

  console.log('Caller Address:',alice.address);
  // //Test Minting Function
  console.log('Minting a new NFT using Alice ...');
  try {
    const mintTx = await contract.tx.mint({
    	gasLimit : 100000000000
    });
    console.log('mintTx',mintTx);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  console.log('Minting a new NFT with attributes using Alice ...');
  try {
    const attributes: Array<[string, string]> = [["abc","123"],["def","456"]];
    const mintAttrTx = await contract.tx.mint_with_attributes(attributes,{
      gasLimit : 100000000000
    });
    console.log('mintAttrTx',mintAttrTx);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

}

/*
  testMint
  Test case: Test Mint functions with Caller == Contract Owner
  Expect: Contract Call excecute successfully with total supply increases and caller balance increase
*/
async function testMint() {
  const contract = new Contract(TOKEN_ADDRESS, defaultSigner, api);
  //Check the total supply and the balance of defaultSigner
  const {value: defaultBalance} = await contract.query['PSP34::balance_of'](defaultSigner.address);
  console.log('defaultSigner Balance:',defaultBalance);

  // //Test Minting Function
  console.log('Minting a new NFT ...');
  const mintTx = await contract.tx.mint({
  	gasLimit : 100000000000
  });
  console.log('mintTx',mintTx.txHash);

  let {value: totalSupplyAfterMint} = await contract.query['PSP34::total_supply']();
  console.log('NFT Total Supply After Mint:',totalSupplyAfterMint);
  let {value: defaultBalanceAfterMint} = await contract.query['PSP34::balance_of'](defaultSigner.address);
  console.log('defaultSigner Balance After Mint:',defaultBalanceAfterMint);

  //Test Minting Function with Attributes
  console.log('Minting a new NFT with attributes ...');
  const attributes: Array<[string, string]> = [["abc","123"],["def","456"]];
  const mintAttrTx = await contract.tx.mint_with_attributes(attributes,{
    gasLimit : 100000000000
  });
  console.log('mintAttrTx',mintAttrTx.txHash);

  let {value: totalSupplyAfterMintA} = await contract.query['PSP34::total_supply']();
  console.log('NFT Total Supply After Mint with Attribute:',totalSupplyAfterMintA);
  let {value: defaultBalanceAfterMintA} = await contract.query['PSP34::balance_of'](defaultSigner.address);
  console.log('defaultSigner Balance After Mint with Attribute:',defaultBalanceAfterMintA);

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
  setup();
  // await createNFTContract();
  await generalInfo();
  //await testMint();
  //await testMintNotOwner();
  // await testWithdrawFee();
  // await testWithdrawFeeNoOwner();
  //await testLockNFT();
  // await testAdminTransfer();
  // await testAdminTransferPSP22();
});

api.on("error", (err) => {
  console.log('error', err );
});
