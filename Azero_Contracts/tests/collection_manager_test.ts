import {ApiPromise} from "@polkadot/api";
import ContractPSP34 from "./typed_contracts/contracts/psp34_nft";
import ConstructorsPSP34 from "./typed_contracts/constructors/psp34_nft";
import ContractCollectionManager from "./typed_contracts/contracts/artzero_collection_manager";
import ConstructorsCollectionManager from "./typed_contracts/constructors/artzero_collection_manager";
import { getSigners, showAZBalance, provider } from './helpers'
import jsonrpc from "@polkadot/types/interfaces/jsonrpc";
import '@polkadot/api-augment';
import BN from 'bn.js'
import { KeyringPair } from '@polkadot/keyring/types'
import { Keyring } from '@polkadot/keyring'

let signers: KeyringPair[];
let defaultSigner: KeyringPair;
let alice: KeyringPair;
let bob: KeyringPair;

// Take the Hash from /artifacts/psp34_nft.json
// Note that: psp34 code must be deployed on the network
const PSP34_STANDARD_HASH = "0xe18b958e0d6f0e9210eca1d231756f7cd3fe33067e22bddeb0c07331ad43d344";
// Once deployed, Collection Manager Contract can be used throughout the tests
const Collection_Manager_Contract = "5Dk82usj4spc6kKhhfJguK4waLzjep3FnYFvhkyVvYaZaFqH";

function setup() {
  signers = getSigners();
  defaultSigner = signers[2];
  alice = signers[0];
  bob = signers[1];
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
  const contract = new ContractCollectionManager(Collection_Manager_Contract, defaultSigner, api);
  //Check the total supply and the balance of defaultSigner
  const {value: nftHash} = await contract.query.get_standard_nft_hash();
  console.log(`NFT Hash is ${nftHash}`);
  const {value: contractOwner} = await contract.query["Ownable::owner"]();
  console.log(`Contract Owner is ${contractOwner}`);

}
/*
  testInitialize
  Test case: Check if Initialize function can be called
  Pre-requisite: none
  Expect: Should Failed all the time as contract is created
*/
async function testInitialize() {
  let contract = new ContractCollectionManager(Collection_Manager_Contract, defaultSigner, api);
  try {
    console.log('Running Initialize using defaultSigner (should fail)');
    const trans = await contract.tx.initialize(
      PSP34_STANDARD_HASH,
  		10**12,
  		10**12,
  		500,
      {
      gasLimit : 100000000000
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  contract = new ContractCollectionManager(Collection_Manager_Contract, bob, api);
  try {
    console.log('Running Initialize using Bob (should fail)');
    const trans = await contract.tx.initialize(
      PSP34_STANDARD_HASH,
  		10**12,
  		10**12,
  		500,
      {
      gasLimit : 100000000000
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
}

/*
  testAutoNewCollection
  Test case: Test Auto Collection Creation
  Pre-requisite: none
  Expect: wrong Royaty Fee, adding fee and attributes should fails, correct inputs will ok
*/
async function testAutoNewCollection(){
  let contract = new ContractCollectionManager(Collection_Manager_Contract, bob, api);

  const {value: maxRoyalty} = await contract.query.get_max_royalty_fee_rate();
  console.log(`maxRoyalty is ${maxRoyalty}`);

  const {value: simpleModeFee} = await contract.query.get_simple_mode_adding_fee();
  console.log(`simpleModeFee is ${simpleModeFee}`);

  try {
    console.log('Creating new Collection with royalty free > than maxRoyalty (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.auto_new_collection(
      "abc",
  		"abc",
  		attributes,
  		values,
  		true,
  		1000,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty but attributes != values length (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123"];
    const trans = await contract.tx.auto_new_collection(
      "abc",
  		"abc",
  		attributes,
  		values,
  		true,
  		500,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty but attributes == values length, wrong fee (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.auto_new_collection(
      "abc",
  		"abc",
  		attributes,
  		values,
  		true,
  		500,
      {
      gasLimit : 100000000000,
      value: 5*10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty but attributes == values length, correct fee (should ok)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.auto_new_collection(
      "abc",
  		"abc",
  		attributes,
  		values,
  		true,
  		500,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
}

/*
  testManualNewCollection
  Test case: Test Manual Collection Creation
  Pre-requisite: make sure the contract address is different each time running the test
  Expect: wrong Royaty Fee, adding fee and attributes should fails, correct inputs will ok
*/
async function testManualNewCollection(){
  let contract = new ContractCollectionManager(Collection_Manager_Contract, bob, api);

  const {value: maxRoyalty} = await contract.query.get_max_royalty_fee_rate();
  console.log(`maxRoyalty is ${maxRoyalty}`);

  const {value: advancedModeFee} = await contract.query.get_advance_mode_adding_fee();
  console.log(`advancedModeFee is ${advancedModeFee}`);

  //make sure this is different each time running the test
  let contract_psp34_address = "5ErU3VCPM9JWBLyoZbzUEfwdBuTvqhh9aphM92A3MibgD6P2";

  const nft_contract = new ContractPSP34(contract_psp34_address, defaultSigner, api);

  const {value: contractOwner} = await nft_contract.query["Ownable::owner"]();
  console.log(`NFT Contract is ${contract_psp34_address}`);
  console.log(`NFT Contract Owner is ${contractOwner}`);

  const {value: isCollectionExist} = await contract.query.get_collection_by_address(contract_psp34_address);
  console.log(`NFT Contract Status: ${isCollectionExist ? isCollectionExist : "Not Exist"}`);

  try {
    console.log('Creating new Collection with royalty free > than maxRoyalty (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		1000,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty, attributes lenght != values length (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		400,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }
  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty, attributes lenght == values length, wrong fee (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		400,
      {
      gasLimit : 100000000000,
      value: 10**11,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty, attributes lenght == values length, correct fee, wrong owner (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		400,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  contract = new ContractCollectionManager(Collection_Manager_Contract, defaultSigner, api);
  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty, attributes lenght == values length, correct fee, same owner as caller (should OK)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		400,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  try {
    console.log('Creating new Collection with royalty free <= than maxRoyalty, attributes lenght == values length, correct fee, same owner as caller but existing contract address (should fail)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.add_new_collection(
      contract_psp34_address,
  		attributes,
  		values,
  		true,
  		400,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

}

/*
  testCollection
  Test case:
  Pre-requisite: create some sample collection in testManualNewCollection and testAutoNewCollection
  Expect:
*/
async function testCollection(){
  let contract = new ContractCollectionManager(Collection_Manager_Contract, bob, api);

  //"ADMINER"
  //Check Admin Role
  {
    const {value: hasRole} = await contract.query["AccessControl::has_role"](3739740293,defaultSigner.address);
    console.log(`defaultSigner ${defaultSigner.address} admin role is ${hasRole}`);
  }
  {
    const {value: hasRole} = await contract.query["AccessControl::has_role"](3739740293,bob.address);
    console.log(`bob ${bob.address} admin role is ${hasRole}`);
  }
  {
    const {value: hasRole} = await contract.query["AccessControl::has_role"](3739740293,alice.address);
    console.log(`alice ${alice.address} admin role is ${hasRole}`);
  }

  contract = new ContractCollectionManager(Collection_Manager_Contract, alice, api);
  try {
    console.log('Creating new Collection using alice (should ok)');
    const attributes: Array<string> = ["abc","def"];
    const values: Array<string> = ["123","456"];
    const trans = await contract.tx.auto_new_collection(
      "abc",
  		"abc",
  		attributes,
  		values,
  		true,
  		500,
      {
      gasLimit : 100000000000,
      value: 10**12,
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

  const {value: collectionCount} = await contract.query.get_collection_count();
  console.log(`collectionCount is ${collectionCount}`);


  const {value: collectionAddress} = await contract.query.get_contract_by_id(collectionCount);
  console.log(`Collection address is ${collectionAddress}`);
  if (collectionAddress){
    const {value: collectionOwner} = await contract.query["ArtZeroCollectionTrait::get_collection_owner"](collectionAddress.toString());
    console.log(`collectionOwner is ${collectionOwner}`);

    const {value: collectionStatus} = await contract.query["ArtZeroCollectionTrait::is_active"](collectionAddress.toString());
    console.log(`collection Status is ${collectionStatus ? "active" : "inactive"}`);

    try {
      console.log('Trying to activate contract using alice (not admin) (should fail)');
      const trans = await contract.tx.update_is_active(
        collectionAddress.toString(),
        true,
        {
        gasLimit : 100000000000,
      });
      console.log('transaction id',trans.txHash);
    }
    catch (e:any){
      //console.log(e.error.message);
      console.log("Failed to excecute contract call");
    }
    contract = new ContractCollectionManager(Collection_Manager_Contract, defaultSigner, api);
    try {
      console.log('Trying to activate contract using defaultSigner (admin) (should ok)');
      const trans = await contract.tx.update_is_active(
        collectionAddress.toString(),
        true,
        {
        gasLimit : 100000000000,
      });
      console.log('transaction id',trans.txHash);
    }
    catch (e:any){
      //console.log(e.error.message);
      console.log("Failed to excecute contract call");
    }
    {
      const {value: collectionStatus} = await contract.query["ArtZeroCollectionTrait::is_active"](collectionAddress.toString());
      console.log(`collection Status is ${collectionStatus ? "active" : "inactive"}`);
    }
    
  }
}
// Create new Collection Manager Contract
async function createContract(){
  const constructors = new ConstructorsCollectionManager(api, defaultSigner);
  console.log('Creating Collection Manager contract...');
  /** Create new contract for Collection manager using 5% max royalty fee
  /**
   * @arg: args: [
   * 0: adminAddress - 0,
   * 1: standardNftHash - 12,
   * 2: simpleModeAddingFee - 14,
   * 3: advanceModeAddingFee - 14,
   * 4: maxRoyaltyFeeRate - 5,
   * ]
   */
   const {address: TOKEN_ADDRESS} = await constructors.new(defaultSigner.address,PSP34_STANDARD_HASH, (10**12), (10**12), 500);
   console.log('Contract deployed at:', TOKEN_ADDRESS);
}

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
  // await createContract();
  await generalInfo();
  // await testInitialize();
  //await testAutoNewCollection();
  // await testManualNewCollection();
  await testCollection();
});

api.on("error", (err) => {
  console.log('error', err );
});
