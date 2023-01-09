import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_signAndSend, SignAndSendSuccessResponse} from "../_sdk/tx";
import type {ConstructorOptions} from "../_sdk/types";
import type { ArgumentsTypes } from '../arguments/launchpad_psp34_nft_standard';

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
    	async "new" (
    		launchpadContractAddress: ArgumentsTypes[8],
    		limitPhaseCount: ArgumentsTypes[2],
    		contractOwner: ArgumentsTypes[8],
    		totalSupply: ArgumentsTypes[5],
    		projectInfo: ArgumentsTypes[55],
    		codePhases: ArgumentsTypes[56],
    		isPublicPhases: ArgumentsTypes[57],
    		publicMintingFeePhases: ArgumentsTypes[58],
    		publicMintingAmountPhases: ArgumentsTypes[59],
    		publicMaxMintingAmountPhases: ArgumentsTypes[59],
    		startTimePhases: ArgumentsTypes[59],
    		endTimePhases: ArgumentsTypes[59],
    		__options ? : ConstructorOptions,
    	) {
    		const __contract = JSON.parse(Files.readFileSync("./artifacts/launchpad_psp34_nft_standard.contract").toString());

			const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);

			const gasLimit = 100000 * 1000000 || __options?.gasLimit;
			const storageDepositLimit = __options?.storageDepositLimit;

			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, launchpadContractAddress, limitPhaseCount, contractOwner, totalSupply, projectInfo, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases);

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