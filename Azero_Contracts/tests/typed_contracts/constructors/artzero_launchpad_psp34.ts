import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_signAndSend, SignAndSendSuccessResponse} from "../_sdk/tx";
import type {ConstructorOptions} from "../_sdk/types";
import type { ArgumentsTypes } from '../arguments/artzero_launchpad_psp34';

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
    	 * 0: maxPhasesPerProject - 2,
    	 * 1: adminAddress - 0,
    	 * 2: standardNftHash - 12,
    	 * 3: projectAddingFee - 26,
    	 * 4: projectMintFeeRate - 5,
    	 * 5: publicMaxMintingAmount - 13,
    	 * ]
    	 */
    	async "new" (
    		maxPhasesPerProject: ArgumentsTypes[2],
    		adminAddress: ArgumentsTypes[0],
    		standardNftHash: ArgumentsTypes[12],
    		projectAddingFee: ArgumentsTypes[26],
    		projectMintFeeRate: ArgumentsTypes[5],
    		publicMaxMintingAmount: ArgumentsTypes[13],
    		__options ? : ConstructorOptions,
    	) {
    		const __contract = JSON.parse(Files.readFileSync("./artifacts/artzero_launchpad_psp34.contract").toString());

			const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);

			const gasLimit = 100000 * 1000000 || __options?.gasLimit;
			const storageDepositLimit = __options?.storageDepositLimit;

			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, maxPhasesPerProject, adminAddress, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount);

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