import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_genValidGasLimitAndValue, _signAndSend, SignAndSendSuccessResponse} from "@727-ventures/typechain-types";
import type {ConstructorOptions} from "@727-ventures/typechain-types";
import type {WeightV2} from "@polkadot/types/interfaces";
import type * as ArgumentTypes from '../types-arguments/artzero_launchpad_psp34';
import type BN from 'bn.js';

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
	* new
	*
	* @param { (number | string | BN) } maxPhasesPerProject,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } projectAddingFee,
	* @param { (number | string | BN) } projectMintFeeRate,
	* @param { (number | string | BN) } publicMaxMintingAmount,
	*/
   	async "new" (
		maxPhasesPerProject: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		standardNftHash: ArgumentTypes.Hash,
		projectAddingFee: (string | number | BN),
		projectMintFeeRate: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		__options ? : ConstructorOptions,
   	) {
   		const __contract = JSON.parse(Files.readFileSync("./artifacts/artzero_launchpad_psp34.contract").toString());
		const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);
		const gasLimit = (await _genValidGasLimitAndValue(this.nativeAPI, __options)).gasLimit as WeightV2;

		const storageDepositLimit = __options?.storageDepositLimit;
			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, maxPhasesPerProject, adminAddress, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount);
			let response;

			try {
				response = await _signAndSend(this.nativeAPI.registry, tx, this.signer, (event: any) => event);
			}
			catch (error) {
				console.log(error);
			}

		return {
			result: response as SignAndSendSuccessResponse,
			// @ts-ignore
			address: (response as SignAndSendSuccessResponse)!.result!.contract.address.toString(),
		};
	}
}