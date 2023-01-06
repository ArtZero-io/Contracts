import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_signAndSend, SignAndSendSuccessResponse} from "../_sdk/tx";
import type {ConstructorOptions} from "../_sdk/types";
import type { ArgumentsTypes } from '../arguments/artzero_collection_manager';

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
    	 * 0: adminAddress - 0,
    	 * 1: standardNftHash - 12,
    	 * 2: simpleModeAddingFee - 14,
    	 * 3: advanceModeAddingFee - 14,
    	 * 4: maxRoyaltyFeeRate - 5,
    	 * ]
    	 */
    	async "new" (
    		adminAddress: ArgumentsTypes[0],
    		standardNftHash: ArgumentsTypes[12],
    		simpleModeAddingFee: ArgumentsTypes[14],
    		advanceModeAddingFee: ArgumentsTypes[14],
    		maxRoyaltyFeeRate: ArgumentsTypes[5],
    		__options ? : ConstructorOptions,
    	) {
    		const __contract = JSON.parse(Files.readFileSync("./artifacts/artzero_collection_manager.contract").toString());

			const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);

			const gasLimit = 100000 * 1000000 || __options?.gasLimit;
			const storageDepositLimit = __options?.storageDepositLimit;

			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, adminAddress, standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate);

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