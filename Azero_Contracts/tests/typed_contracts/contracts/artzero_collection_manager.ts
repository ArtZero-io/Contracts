/* This file is auto-generated */

import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import { ContractPromise } from '@polkadot/api-contract';
import ABI from '../../artifacts/artzero_collection_manager.json';
import QueryMethods from '../query/artzero_collection_manager';
import BuildExtrinsicMethods from '../build-extrinsic/artzero_collection_manager';
import TxSignAndSendMethods from '../tx-sign-and-send/artzero_collection_manager';
import MixedMethods from '../mixed-methods/artzero_collection_manager';


export default class Contract {
	readonly query : QueryMethods;
	readonly buildExtrinsic : BuildExtrinsicMethods;
	readonly tx : TxSignAndSendMethods;
	readonly methods : MixedMethods;

	constructor(
		address : string,
		signer : KeyringPair,
		nativeAPI : ApiPromise,
	) {
		const nativeContract = new ContractPromise(nativeAPI, ABI, address);

		this.query = new QueryMethods(nativeContract, signer.address);
		this.buildExtrinsic = new BuildExtrinsicMethods(nativeContract);
		this.tx = new TxSignAndSendMethods(nativeAPI, nativeContract, signer);
		this.methods = new MixedMethods(nativeAPI, nativeContract, signer);
	}
}