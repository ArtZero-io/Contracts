/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_collection_manager';
import type * as ReturnTypes from '../types-returns/artzero_collection_manager';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* initialize
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"initialize" (
		standardNftHash: ArgumentTypes.Hash,
		simpleModeAddingFee: (string | number | BN),
		advanceModeAddingFee: (string | number | BN),
		maxRoyaltyFeeRate: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "initialize", [standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate, adminAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
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
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"autoNewCollection" (
		nftName: string,
		nftSymbol: string,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "autoNewCollection", [nftName, nftSymbol, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* addNewCollection
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { Array<string> } attributes,
	* @param { Array<string> } attributeVals,
	* @param { boolean } isCollectRoyaltyFee,
	* @param { (number | string | BN) } royaltyFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"addNewCollection" (
		nftContractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		attributeVals: Array<string>,
		isCollectRoyaltyFee: boolean,
		royaltyFee: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "addNewCollection", [nftContractAddress, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateCollectionOwner
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateCollectionOwner" (
		contractAddress: ArgumentTypes.AccountId,
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateCollectionOwner", [contractAddress, newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateNftContractAddress
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateNftContractAddress" (
		contractAddress: ArgumentTypes.AccountId,
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateNftContractAddress", [contractAddress, nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* setMultipleAttributes
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { Array<string> } attributes,
	* @param { Array<string> } values,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setMultipleAttributes" (
		contractAddress: ArgumentTypes.AccountId,
		attributes: Array<string>,
		values: Array<string>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setMultipleAttributes", [contractAddress, attributes, values], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<string>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAttributes", [contractAddress, attributes], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_collection_manager')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAttribute", [contractAddress, attributeKey], __options , (result) => { return handleReturnType(result, getTypeDescription(23, 'artzero_collection_manager')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "hasAttribute", [contractAddress, attributeKey], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionAttributeIndex", [contractAddress, attributeKey], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionAttributeCount
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getCollectionAttributeCount" (
		contractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionAttributeCount", [contractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_collection_manager')); });
	}

	/**
	* updateContractType
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { ArgumentTypes.CollectionType } contractType,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateContractType" (
		contractAddress: ArgumentTypes.AccountId,
		contractType: ArgumentTypes.CollectionType,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateContractType", [contractAddress, contractType], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateIsCollectRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } isCollectRoyaltyFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateIsCollectRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		isCollectRoyaltyFee: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateIsCollectRoyaltyFee", [contractAddress, isCollectRoyaltyFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { (number | string | BN) } newFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateRoyaltyFee" (
		contractAddress: ArgumentTypes.AccountId,
		newFee: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateRoyaltyFee", [contractAddress, newFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateShowOnChainMetadata
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } showOnChainMetadata,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateShowOnChainMetadata" (
		contractAddress: ArgumentTypes.AccountId,
		showOnChainMetadata: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateShowOnChainMetadata", [contractAddress, showOnChainMetadata], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateIsActive
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { boolean } isActive,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateIsActive" (
		contractAddress: ArgumentTypes.AccountId,
		isActive: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateIsActive", [contractAddress, isActive], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateSimpleModeAddingFee
	*
	* @param { (string | number | BN) } simpleModeAddingFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateSimpleModeAddingFee" (
		simpleModeAddingFee: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateSimpleModeAddingFee", [simpleModeAddingFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateStandardNftHash
	*
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateStandardNftHash" (
		standardNftHash: ArgumentTypes.Hash,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateStandardNftHash", [standardNftHash], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateAdvanceModeAddingFee
	*
	* @param { (string | number | BN) } advanceModeAddingFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateAdvanceModeAddingFee" (
		advanceModeAddingFee: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateAdvanceModeAddingFee", [advanceModeAddingFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* updateMaxRoyaltyFeeRate
	*
	* @param { (number | string | BN) } maxRoyaltyFeeRate,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateMaxRoyaltyFeeRate" (
		maxRoyaltyFeeRate: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateMaxRoyaltyFeeRate", [maxRoyaltyFeeRate], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionByAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.Collection | null, ReturnTypes.LangError> }
	*/
	"getCollectionByAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Collection | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionByAddress", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	* @returns { Result<Array<ReturnTypes.AccountId> | null, ReturnTypes.LangError> }
	*/
	"getCollectionsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionsByOwner", [ownerAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_collection_manager')); });
	}

	/**
	* getStandardNftHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"getStandardNftHash" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStandardNftHash", [], __options , (result) => { return handleReturnType(result, getTypeDescription(33, 'artzero_collection_manager')); });
	}

	/**
	* getContractById
	*
	* @param { (number | string | BN) } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getContractById" (
		id: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getContractById", [id], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getCollectionCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_collection_manager')); });
	}

	/**
	* getActiveCollectionCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActiveCollectionCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActiveCollectionCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_collection_manager')); });
	}

	/**
	* getSimpleModeAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getSimpleModeAddingFee" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getSimpleModeAddingFee", [], __options , (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_collection_manager')); });
	}

	/**
	* getAdvanceModeAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getAdvanceModeAddingFee" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAdvanceModeAddingFee", [], __options , (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_collection_manager')); });
	}

	/**
	* getMaxRoyaltyFeeRate
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getMaxRoyaltyFeeRate" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getMaxRoyaltyFeeRate", [], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* getRoleAdmin
	*
	* @param { (number | string | BN) } role,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getRoleAdmin" (
		role: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* grantRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"grantRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::grantRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(39, 'artzero_collection_manager')); });
	}

	/**
	* renounceRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"renounceRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::renounceRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(39, 'artzero_collection_manager')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
	}

	/**
	* revokeRole
	*
	* @param { (number | string | BN) } role,
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"revokeRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::revokeRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(39, 'artzero_collection_manager')); });
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(41, 'artzero_collection_manager')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options , (result) => { return handleReturnType(result, getTypeDescription(42, 'artzero_collection_manager')); });
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(42, 'artzero_collection_manager')); });
	}

	/**
	* getContractType
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.CollectionType, ReturnTypes.LangError> }
	*/
	"getContractType" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.CollectionType, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getContractType", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(44, 'artzero_collection_manager')); });
	}

	/**
	* isActive
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"isActive" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::isActive", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_collection_manager')); });
	}

	/**
	* getCollectionOwner
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getCollectionOwner" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getCollectionOwner", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_collection_manager')); });
	}

	/**
	* getRoyaltyFee
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getRoyaltyFee" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroCollectionTrait::getRoyaltyFee", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_collection_manager')); });
	}

	/**
	* tranferPsp22
	*
	* @param { ArgumentTypes.AccountId } psp22ContractAddress,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"tranferPsp22" (
		psp22ContractAddress: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* tranferNft
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"tranferNft" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* withdrawFee
	*
	* @param { (string | number | BN) } value,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"withdrawFee" (
		value: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::withdrawFee", [value, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

	/**
	* setCode
	*
	* @param { Array<(number | string | BN)> } codeHash,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setCode" (
		codeHash: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "upgradableTrait::setCode", [codeHash], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_collection_manager')); });
	}

}