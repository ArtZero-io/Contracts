/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_collection_manager';
import type BN from 'bn.js';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise: ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
	}

	/**
	* initialize
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	* @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"initialize" (
		standardNftHash: ArgumentTypes.Hash,
		simpleModeAddingFee: (string | number | BN),
		advanceModeAddingFee: (string | number | BN),
		maxRoyaltyFeeRate: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate, adminAddress], __options);
	}

	/**
	* autoNewCollection
	*
	* @param { string } nftName,
	* @param { string } nftSymbol,
	* @param { Array<string> } attributes,
	* @param { Array<string> } attributeVals,
	* @param { boolean } isCollectRoyaltyFee,
	* @param { (number | string | BN) } royaltyFee,
	*/
	"autoNewCollection" (
		nftName: string,
		nftSymbol: string,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "autoNewCollection", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftName, nftSymbol, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
	}

	/**
	* addNewCollection
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { Array<string> } attributes,
	* @param { Array<string> } attributeVals,
	* @param { boolean } isCollectRoyaltyFee,
	* @param { (number | string | BN) } royaltyFee,
	*/
	"addNewCollection" (
		nftContractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addNewCollection", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
	}

	/**
	* updateCollectionOwner
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.AccountId } newOwner,
	*/
	"updateCollectionOwner" (
		contractAddress: ArgumentTypes.AccountId,
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateCollectionOwner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, newOwner], __options);
	}

	/**
	* updateNftContractAddress
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"updateNftContractAddress" (
		contractAddress: ArgumentTypes.AccountId,
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateNftContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, nftContractAddress], __options);
	}

	/**
	* setMultipleAttributes
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { Array<string> } attributes,
	* @param { Array<string> } values,
	*/
	"setMultipleAttributes" (
		contractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		values: Array<string>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setMultipleAttributes", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, attributes, values], __options);
	}

	/**
	* getAttributes
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { Array<string> } attributes,
	*/
	"getAttributes" (
		contractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAttributes", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, attributes], __options);
	}

	/**
	* getAttribute
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	*/
	"getAttribute" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAttribute", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, attributeKey], __options);
	}

	/**
	* hasAttribute
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	*/
	"hasAttribute" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "hasAttribute", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, attributeKey], __options);
	}

	/**
	* getCollectionAttributeIndex
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	*/
	"getCollectionAttributeIndex" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionAttributeIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, attributeKey], __options);
	}

	/**
	* getCollectionAttributeCount
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	*/
	"getCollectionAttributeCount" (
		contractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionAttributeCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress], __options);
	}

	/**
	* updateContractType
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.CollectionType } contractType,
	*/
	"updateContractType" (
		contractAddress: ArgumentTypes.AccountId,
		contractType: ArgumentTypes.CollectionType,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateContractType", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, contractType], __options);
	}

	/**
	* updateIsCollectRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } isCollectRoyaltyFee,
	*/
	"updateIsCollectRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		isCollectRoyaltyFee: boolean,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsCollectRoyaltyFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, isCollectRoyaltyFee], __options);
	}

	/**
	* updateRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { (number | string | BN) } newFee,
	*/
	"updateRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		newFee: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateRoyaltyFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, newFee], __options);
	}

	/**
	* updateShowOnChainMetadata
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } showOnChainMetadata,
	*/
	"updateShowOnChainMetadata" (
		contractAddress: ArgumentTypes.AccountId,
		showOnChainMetadata: boolean,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateShowOnChainMetadata", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, showOnChainMetadata], __options);
	}

	/**
	* updateIsActive
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } isActive,
	*/
	"updateIsActive" (
		contractAddress: ArgumentTypes.AccountId,
		isActive: boolean,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsActive", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, isActive], __options);
	}

	/**
	* updateSimpleModeAddingFee
	*
	* @param { (string | number | BN) } simpleModeAddingFee,
	*/
	"updateSimpleModeAddingFee" (
		simpleModeAddingFee: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateSimpleModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [simpleModeAddingFee], __options);
	}

	/**
	* updateStandardNftHash
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	*/
	"updateStandardNftHash" (
		standardNftHash: ArgumentTypes.Hash,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateStandardNftHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [standardNftHash], __options);
	}

	/**
	* updateAdvanceModeAddingFee
	*
	* @param { (string | number | BN) } advanceModeAddingFee,
	*/
	"updateAdvanceModeAddingFee" (
		advanceModeAddingFee: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateAdvanceModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [advanceModeAddingFee], __options);
	}

	/**
	* updateMaxRoyaltyFeeRate
	*
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	*/
	"updateMaxRoyaltyFeeRate" (
		maxRoyaltyFeeRate: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateMaxRoyaltyFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [maxRoyaltyFeeRate], __options);
	}

	/**
	* getCollectionByAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getCollectionByAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionByAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress], __options);
	}

	/**
	* getCollectionsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	*/
	"getCollectionsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionsByOwner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [ownerAddress], __options);
	}

	/**
	* getStandardNftHash
	*
	*/
	"getStandardNftHash" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStandardNftHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getContractById
	*
	* @param { (number | string | BN) } id,
	*/
	"getContractById" (
		id: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getContractById", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [id], __options);
	}

	/**
	* getCollectionCount
	*
	*/
	"getCollectionCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getActiveCollectionCount
	*
	*/
	"getActiveCollectionCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getActiveCollectionCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getSimpleModeAddingFee
	*
	*/
	"getSimpleModeAddingFee" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getSimpleModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getAdvanceModeAddingFee
	*
	*/
	"getAdvanceModeAddingFee" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAdvanceModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getMaxRoyaltyFeeRate
	*
	*/
	"getMaxRoyaltyFeeRate" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getMaxRoyaltyFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* getRoleAdmin
	*
	* @param { (number | string | BN) } role,
	*/
	"getRoleAdmin" (
		role: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::getRoleAdmin", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role], __options);
	}

	/**
	* grantRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	*/
	"grantRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::grantRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role, account], __options);
	}

	/**
	* renounceRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	*/
	"renounceRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::renounceRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role, account], __options);
	}

	/**
	* hasRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } address,
	*/
	"hasRole" (
		role: (number | string | BN),
		address: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::hasRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role, address], __options);
	}

	/**
	* revokeRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	*/
	"revokeRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::revokeRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role, account], __options);
	}

	/**
	* owner
	*
	*/
	"owner" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::owner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* renounceOwnership
	*
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::transferOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [newOwner], __options);
	}

	/**
	* getContractType
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getContractType" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroCollectionTrait::getContractType", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress], __options);
	}

	/**
	* isActive
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"isActive" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroCollectionTrait::isActive", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress], __options);
	}

	/**
	* getCollectionOwner
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getCollectionOwner" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroCollectionTrait::getCollectionOwner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress], __options);
	}

	/**
	* getRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getRoyaltyFee" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroCollectionTrait::getRoyaltyFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress], __options);
	}

	/**
	* tranferPsp22
	*
	* @param { ArgumentTypes.AccountId } psp22ContractAddress,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId } receiver,
	*/
	"tranferPsp22" (
		psp22ContractAddress: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::tranferPsp22", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [psp22ContractAddress, amount, receiver], __options);
	}

	/**
	* tranferNft
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @param { ArgumentTypes.AccountId } receiver,
	*/
	"tranferNft" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::tranferNft", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [nftContractAddress, tokenId, receiver], __options);
	}

	/**
	* withdrawFee
	*
	* @param { (string | number | BN) } value,
	* @param { ArgumentTypes.AccountId } receiver,
	*/
	"withdrawFee" (
		value: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::withdrawFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [value, receiver], __options);
	}

	/**
	* setCode
	*
	* @param { Array<(number | string | BN)> } codeHash,
	*/
	"setCode" (
		codeHash: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "upgradableTrait::setCode", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [codeHash], __options);
	}

}