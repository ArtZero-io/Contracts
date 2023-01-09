/* This file is auto-generated */

import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import { ContractPromise } from '@polkadot/api-contract';
import ABI from '../../artifacts/launchpad_psp34_nft_standard.json';
import QueryMethods from '../query/launchpad_psp34_nft_standard';
import BuildExtrinsicMethods from '../build-extrinsic/launchpad_psp34_nft_standard';
import TxSignAndSendMethods from '../tx-sign-and-send/launchpad_psp34_nft_standard';
import MixedMethods from '../mixed-methods/launchpad_psp34_nft_standard';


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