import {BN, BN_ONE } from '@polkadot/util';
import { KeyringPair } from '@polkadot/keyring/types';
import { Keyring } from '@polkadot/keyring';
import {ApiPromise, WsProvider} from "@polkadot/api";
import { PROVIDER_URL } from "./constants";
import type { WeightV2 } from '@polkadot/types/interfaces';

export const provider = new WsProvider(PROVIDER_URL);
export { expect } from 'chai';

export const bnArg = (value: number | string | number[] | Uint8Array | Buffer | BN, len = 32) => {
  return new BN(value, undefined, 'le').toArray('le', len);
}

export const oneDay = () => (24 * 60 * 60 * 1000);

let mnemonicCharlie = "gloom pumpkin sport soup slot leg legal obvious two pyramid process crunch";

export const getSigners = (): KeyringPair[] => {
  const keyring = new Keyring({type: 'sr25519'});

  const UserAlice: KeyringPair = keyring.addFromUri('//Alice_Artzero');
  const UserBob: KeyringPair = keyring.addFromUri('//Bob_Artzero');
  const UserCharlie: KeyringPair = keyring.addFromMnemonic(mnemonicCharlie);

  return [
    UserAlice, UserBob, UserCharlie
  ]
}

export function bytesToString(bytes: string): string {
  const outputNumber = bytes.substring(2).split('').map(x => parseInt(x as unknown as string, 16));

  const length = outputNumber.length;
  let result = '';
  for (let i = 0; i < length; i += 2) {
    result += String.fromCharCode(outputNumber[i] * 16 + outputNumber[i + 1]);
  }

  return result;
}

export async function showAZBalance(api: any, address: string ) {
    const { data: { free, reserved, miscFrozen } } = await api.query.system.account(address);
    const balance =
      new BN(free).div(new BN(10 ** 6)).toNumber() / 10 ** 6 -
      new BN(miscFrozen).div(new BN(10 ** 6)).toNumber() / 10 ** 6;
  
    return balance;
}

export async function checkAccountsBalance(signers: KeyringPair[], api: any) {
    // console.log('Checking Accounts and Balance');
   
    for (var i = 0; i < signers.length; i++){
      const balance = await showAZBalance(api, signers[i].address);
      // console.log(`Account ${i} has address: ${signers[i].address} and balance: ${balance} TZERO`);
    }
}

export const toBytes = (s: string) => Array.from(Buffer.from(s, 'utf8'));

const hexToDecimal = (hex: string) => parseInt(hex, 16);

export const decodeToBytes = (encodedString: string): number[] => {
    let len = encodedString.length / 2 - 1;
    let bytes: number[] = new Array();   

    for (let i = 0; i < len; i++) 
        bytes[i] = hexToDecimal(encodedString.substring(i * 2 + 2, i * 2 + 4));

    return bytes;
}

export const toString = (bytes: number[]) => String.fromCharCode(...bytes);

// const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
// 1_000_000_000_000
// 5_000_000_000_000

// For read-only queries we don't need the exact gas limit
// as the account will not be charged for making the call.
export function setGasLimit(api: ApiPromise, refTimeNum: any, proofSizeNum: any): WeightV2 {
  return api.registry.createType('WeightV2', {
    refTime: new BN(refTimeNum),
    proofSize: new BN(proofSizeNum)
  }) as WeightV2;
}

export function delay(milisec: number) {
  return new Promise((res) => setTimeout(res, milisec));
}