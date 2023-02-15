import {CodePromise} from "@polkadot/api-contract";
import type {KeyringPair} from "@polkadot/keyring/types";
import Files from "fs";
import type {ApiPromise} from "@polkadot/api";
import {_genValidGasLimitAndValue, _signAndSend, SignAndSendSuccessResponse} from "@727-ventures/typechain-types";
import type {ConstructorOptions} from "@727-ventures/typechain-types";
import type {WeightV2} from "@polkadot/types/interfaces";
import type * as ArgumentTypes from '../types-arguments/artzero_collection_manager';
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
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	*/
   	async "new" (
		adminAddress: ArgumentTypes.AccountId,
		standardNftHash: ArgumentTypes.Hash,
		simpleModeAddingFee: (string | number | BN),
		advanceModeAddingFee: (string | number | BN),
		maxRoyaltyFeeRate: (number | string | BN),
		__options ? : ConstructorOptions,
   	) {
   		const __contract = JSON.parse(Files.readFileSync("./artifacts/artzero_collection_manager.contract").toString());
		const code = new CodePromise(this.nativeAPI, __contract, __contract.source.wasm);
		const gasLimit = (await _genValidGasLimitAndValue(this.nativeAPI, __options)).gasLimit as WeightV2;

		const storageDepositLimit = __options?.storageDepositLimit;
			const tx = code.tx["new"]!({ gasLimit, storageDepositLimit, value: __options?.value }, adminAddress, standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate);
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