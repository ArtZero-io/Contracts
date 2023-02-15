/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_collection_manager';
import type * as ReturnTypes from '../types-returns/artzero_collection_manager';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';
// @ts-ignore
import type {EventRecord} from "@polkadot/api/submittable";
import {decodeEvents} from "../shared/utils";


export default class Methods {
	private __nativeContract : ContractPromise;
	private __keyringPair : KeyringPair;
	private __callerAddress : string;
	private __apiPromise: ApiPromise;

	constructor(
		apiPromise : ApiPromise,
		nativeContract : ContractPromise,
		keyringPair : KeyringPair,
	) {
		this.__apiPromise = apiPromise;
		this.__nativeContract = nativeContract;
		this.__keyringPair = keyringPair;
		this.__callerAddress = keyringPair.address;
	}

	/**
	* initialize
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { void }
	*/
	"initialize" (
		standardNftHash: ArgumentTypes.Hash,
		simpleModeAddingFee: (string | number | BN),
		advanceModeAddingFee: (string | number | BN),
		maxRoyaltyFeeRate: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"autoNewCollection" (
		nftName: string,
		nftSymbol: string,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options: GasLimitAndRequiredValue,
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
	* @returns { void }
	*/
	"addNewCollection" (
		nftContractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options: GasLimitAndRequiredValue,
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
	* @returns { void }
	*/
	"updateCollectionOwner" (
		contractAddress: ArgumentTypes.AccountId,
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"updateNftContractAddress" (
		contractAddress: ArgumentTypes.AccountId,
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"setMultipleAttributes" (
		contractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		values: Array<string>,
		__options: GasLimit,
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
	* @returns { Result<Array<string>, ReturnTypes.LangError> }
	*/
	"getAttributes" (
		contractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<string>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAttributes", [contractAddress, attributes], __options, (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_collection_manager')); });
	}

	/**
	* getAttribute
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	* @returns { Result<string, ReturnTypes.LangError> }
	*/
	"getAttribute" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAttribute", [contractAddress, attributeKey], __options, (result) => { return handleReturnType(result, getTypeDescription(23, 'artzero_collection_manager')); });
	}

	/**
	* hasAttribute
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"hasAttribute" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "hasAttribute", [contractAddress, attributeKey], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionAttributeIndex
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { string } attributeKey,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getCollectionAttributeIndex" (
		contractAddress: ArgumentTypes.AccountId,
		attributeKey: string,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionAttributeIndex", [contractAddress, attributeKey], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionAttributeCount
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getCollectionAttributeCount" (
		contractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionAttributeCount", [contractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_collection_manager')); });
	}

	/**
	* updateContractType
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.CollectionType } contractType,
	* @returns { void }
	*/
	"updateContractType" (
		contractAddress: ArgumentTypes.AccountId,
		contractType: ArgumentTypes.CollectionType,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"updateIsCollectRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		isCollectRoyaltyFee: boolean,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"updateRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		newFee: (number | string | BN),
		__options: GasLimit,
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
	* @returns { void }
	*/
	"updateShowOnChainMetadata" (
		contractAddress: ArgumentTypes.AccountId,
		showOnChainMetadata: boolean,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"updateIsActive" (
		contractAddress: ArgumentTypes.AccountId,
		isActive: boolean,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsActive", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [contractAddress, isActive], __options);
	}

	/**
	* updateSimpleModeAddingFee
	*
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @returns { void }
	*/
	"updateSimpleModeAddingFee" (
		simpleModeAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateSimpleModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [simpleModeAddingFee], __options);
	}

	/**
	* updateStandardNftHash
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @returns { void }
	*/
	"updateStandardNftHash" (
		standardNftHash: ArgumentTypes.Hash,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateStandardNftHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [standardNftHash], __options);
	}

	/**
	* updateAdvanceModeAddingFee
	*
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @returns { void }
	*/
	"updateAdvanceModeAddingFee" (
		advanceModeAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateAdvanceModeAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [advanceModeAddingFee], __options);
	}

	/**
	* updateMaxRoyaltyFeeRate
	*
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	* @returns { void }
	*/
	"updateMaxRoyaltyFeeRate" (
		maxRoyaltyFeeRate: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateMaxRoyaltyFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [maxRoyaltyFeeRate], __options);
	}

	/**
	* getCollectionByAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.Collection | null, ReturnTypes.LangError> }
	*/
	"getCollectionByAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Collection | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionByAddress", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	* @returns { Result<Array<ReturnTypes.AccountId> | null, ReturnTypes.LangError> }
	*/
	"getCollectionsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionsByOwner", [ownerAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_collection_manager')); });
	}

	/**
	* getStandardNftHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"getStandardNftHash" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStandardNftHash", [], __options, (result) => { return handleReturnType(result, getTypeDescription(33, 'artzero_collection_manager')); });
	}

	/**
	* getContractById
	*
	* @param { (number | string | BN) } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getContractById" (
		id: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getContractById", [id], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getCollectionCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_collection_manager')); });
	}

	/**
	* getActiveCollectionCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActiveCollectionCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActiveCollectionCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_collection_manager')); });
	}

	/**
	* getSimpleModeAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getSimpleModeAddingFee" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getSimpleModeAddingFee", [], __options, (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_collection_manager')); });
	}

	/**
	* getAdvanceModeAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getAdvanceModeAddingFee" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAdvanceModeAddingFee", [], __options, (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_collection_manager')); });
	}

	/**
	* getMaxRoyaltyFeeRate
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getMaxRoyaltyFeeRate" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getMaxRoyaltyFeeRate", [], __options, (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* getRoleAdmin
	*
	* @param { (number | string | BN) } role,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getRoleAdmin" (
		role: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options, (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* grantRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	* @returns { void }
	*/
	"grantRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"renounceRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"hasRole" (
		role: (number | string | BN),
		address: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
	}

	/**
	* revokeRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	* @returns { void }
	*/
	"revokeRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::revokeRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [role, account], __options);
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(41, 'artzero_collection_manager')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { void }
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [], __options);
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { void }
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::transferOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [newOwner], __options);
	}

	/**
	* getContractType
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.CollectionType, ReturnTypes.LangError> }
	*/
	"getContractType" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.CollectionType, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getContractType", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(44, 'artzero_collection_manager')); });
	}

	/**
	* isActive
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"isActive" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::isActive", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionOwner
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getCollectionOwner" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getCollectionOwner", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_collection_manager')); });
	}

	/**
	* getRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getRoyaltyFee" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getRoyaltyFee", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* tranferPsp22
	*
	* @param { ArgumentTypes.AccountId } psp22ContractAddress,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { void }
	*/
	"tranferPsp22" (
		psp22ContractAddress: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"tranferNft" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		receiver: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"withdrawFee" (
		value: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::withdrawFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [value, receiver], __options);
	}

	/**
	* setCode
	*
	* @param { Array<(number | string | BN)> } codeHash,
	* @returns { void }
	*/
	"setCode" (
		codeHash: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "upgradableTrait::setCode", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_collection_manager");
		}, [codeHash], __options);
	}

}