import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_signAndSend, SignAndSendSuccessResponse} from "../_sdk/tx";
import type {ConstructorOptions} from "../_sdk/types";
import type { ArgumentsTypes } from '../arguments/psp34_nft';

export default class Constructors {
	readonly nativeAPI: ApiPromise;
	readonly signer: KeyringPair;

	constructor(
		nativeAPI: ApiPromise,
		signer: KeyringPair,
	) {
		this.nativeAPI = nativeAPI;
		this.signer = signer;
	}

    	/**
    	 * @arg: args: [
    	 * 0: contractOwner - 8,
    	 * 1: name - 33,
    	 * 2: symbol - 33,
    	 * ]
    	 */
    	async "new" (
    		contractOwner: ArgumentsTypes[8],
    		name: ArgumentsTypes[33],
    		symbol: ArgumentsTypes[33],
    		__options ? : ConstructorOptions,
    	) {
    		const __contract = JSON.parse(Files.readFileSync("./artifacts/psp34_nft.contract").toString());

			const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);

			const gasLimit = 100000 * 1000000 || __options?.gasLimit;
			const storageDepositLimit = __options?.storageDepositLimit;

			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, contractOwner, name, symbol);

			let response;
			try {
				response = await _signAndSend(this.nativeAPI.registry, tx, this.signer);
			}
			catch (error) {
				console.log(error);
			}

			return {
				result: response as SignAndSendSuccessResponse,
				// @ts-ignore
				address: (response as SignAndSendSuccessResponse)!.result!.contract.address.toString(),
			}
    	}

}