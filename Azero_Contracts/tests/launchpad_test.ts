import {ApiPromise} from "@polkadot/api";
import ContractPSP34_LP from "./typed_contracts/contracts/launchpad_psp34_nft_standard";
import ConstructorsPSP34_LP from "./typed_contracts/constructors/launchpad_psp34_nft_standard";
import ContractLaunchPadManager from "./typed_contracts/contracts/artzero_launchpad_psp34";
import ConstructorsLaunchPadManager from "./typed_contracts/constructors/artzero_launchpad_psp34";
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

// Take the Hash from /artifacts/launchpad_psp34_nft_standard.json
// Note that: psp34 code must be deployed on the network
const PSP34_LP_HASH = "0x0d71c157e11bee476676196894f7452e5c7eec75eea061df6518217f23365167";
// Once deployed using createContracts(), LP Manager Contract can be used throughout the tests
const LaunchPad_Manager_Address = "5Cy2FaMenpyYpYyA1GrddFBFwJLT3jhhew7SCs46sX4MGWku";

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
  // const contract = new ContractCollectionManager(Collection_Manager_Contract, defaultSigner, api);
  // //Check the total supply and the balance of defaultSigner
  // const {value: nftHash} = await contract.query.get_standard_nft_hash();
  // console.log(`NFT Hash is ${nftHash}`);
  // const {value: contractOwner} = await contract.query["Ownable::owner"]();
  // console.log(`Contract Owner is ${contractOwner}`);

}

// Create new LP Manager and LP PSP34 contracts
async function createLPContracts(){
  const constructors_LP = new ConstructorsLaunchPadManager(api, defaultSigner);
  console.log('Creating LaunchPad contract...');
  /**
   * @arg: args: [
   * 0: maxPhasesPerProject - 2,
   * 1: adminAddress - 0,
   * 2: standardNftHash - 12,
   * 3: projectAddingFee - 26,
   * 4: projectMintFeeRate - 5,
   * 5: publicMaxMintingAmount - 13,
   * ]
   */
   // standardNftHash is from launchpad_psp34_nft_standard.json, need to be deployed to the network
   const {address: LP_Manager_Contract} = await constructors_LP.new(
     10,
     alice.address,
     PSP34_LP_HASH,
     10**12,
     500,
     10
   );
   console.log('LaunchPad Manager Contract deployed at:', LP_Manager_Contract);

}

async function createLP_PSP34Contracts(){

   const constructors_PSP34_LP = new ConstructorsPSP34_LP(api, defaultSigner);
   console.log('Creating LaunchPad PSP34 contract...');
  /**
   * @arg: args: [
   * 0: launchpadContractAddress - 8,
   * 1: limitPhaseCount - 2,
   * 2: contractOwner - 8,
   * 3: totalSupply - 5,
   * 4: projectInfo - 55,
   * 5: codePhases - 56,
   * 6: isPublicPhases - 57,
   * 7: publicMintingFeePhases - 58,
   * 8: publicMintingAmountPhases - 59,
   * 9: publicMaxMintingAmountPhases - 59,
   * 10: startTimePhases - 59,
   * 11: endTimePhases - 59,
   * ]
   */
   //Create to upload Hash code to network but dont use the address. The PSP34 contract should be managed by Launchpad Manager
   const codePhases: Array<string> = ["phase 1", "phase 2"];
   const isPublicPhases: Array<boolean> = [true, false];
   const publicMintingFeePhases: Array<number> = [10**12, 10**12];
   const publicMintingAmountPhases: Array<number> = [100, 1000];
   const publicMaxMintingAmountPhases: Array<number> = [10, 10];
   const startTimePhases: Array<number> = [10, 30];
   const endTimePhases: Array<number> = [20, 40];
   const {address: TOKEN_ADDRESS} = await constructors_PSP34_LP.new(
     LaunchPad_Manager_Address,
     10,
     defaultSigner.address,
     100000,
     "Test Project",
     codePhases,
     isPublicPhases,
     publicMintingFeePhases,
     publicMintingAmountPhases,
     publicMaxMintingAmountPhases,
     startTimePhases,
     endTimePhases
   );
   console.log('LaunchPad PSP34 Contract deployed at:', TOKEN_ADDRESS);
}

/*
  testCreateProject
  Test case:
  Pre-requisite: none
  Expect:
*/
async function testCreateProject(){
  let contract = new ContractLaunchPadManager(LaunchPad_Manager_Address, defaultSigner, api);
  /** add_new_project
   * @arg: args: [
   * 0: totalSupply,
   * 1: startTime,
   * 2: endTime,
   * 3: projectInfo,
   * 4: codePhases,
   * 5: isPublicPhases,
   * 6: publicMintingFeePhases,
   * 7: publicMintingAmountPhases,
   * 8: publicMaxMintingAmountPhases,
   * 9: startTimePhases,
   * 10: endTimePhases,
   * ]
   */

   let {value: addingFee} = await contract.query.get_project_adding_fee();
   console.log(`Project Adding Fee is ${addingFee}`);

  try {
    console.log('Creating new Project (should ok)');
    const codePhases: Array<string> = ["phase 1", "phase 2"];
    const isPublicPhases: Array<boolean> = [true, false];
    const publicMintingFeePhases: Array<number> = [10**12, 10**12];
    const publicMintingAmountPhases: Array<number> = [100, 1000];
    const publicMaxMintingAmountPhases: Array<number> = [10, 10];
    const startTimePhases: Array<number> = [20, 40];
    const endTimePhases: Array<number> = [30, 50];

    const trans = await contract.tx.add_new_project(
      10000,
      10,
      90,
      "Test Project",
      codePhases,
      isPublicPhases,
      publicMintingFeePhases,
      publicMintingAmountPhases,
      publicMaxMintingAmountPhases,
      startTimePhases,
      endTimePhases
      ,{
      gasLimit : 100000000000,
      value : addingFee
    });
    console.log('transaction id',trans.txHash);
  }
  catch (e:any){
    //console.log(e.error.message);
    console.log("Failed to excecute contract call");
  }

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
  // await createLP_PSP34Contracts();
  // await createLPContracts();

  // await generalInfo();
  // await testCreateProject();
});

api.on("error", (err) => {
  console.log('error', err );
});
