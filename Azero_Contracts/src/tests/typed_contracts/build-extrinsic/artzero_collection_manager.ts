/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_collection_manager';
import type BN from 'bn.js';
import type { ApiPromise } from '@polkadot/api';



export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		apiPromise: ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__apiPromise = apiPromise;
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "initialize", [standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate, adminAddress], __options);
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
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "autoNewCollection", [nftName, nftSymbol, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
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
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "addNewCollection", [nftContractAddress, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateCollectionOwner", [contractAddress, newOwner], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateNftContractAddress", [contractAddress, nftContractAddress], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setMultipleAttributes", [contractAddress, attributes, values], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAttributes", [contractAddress, attributes], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAttribute", [contractAddress, attributeKey], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "hasAttribute", [contractAddress, attributeKey], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionAttributeIndex", [contractAddress, attributeKey], __options);
	}

	/**
	 * getCollectionAttributeCount
	 *
	 * @param { ArgumentTypes.AccountId } contractAddress,
	*/
	"getCollectionAttributeCount" (
		contractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionAttributeCount", [contractAddress], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateContractType", [contractAddress, contractType], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateIsCollectRoyaltyFee", [contractAddress, isCollectRoyaltyFee], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateRoyaltyFee", [contractAddress, newFee], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateShowOnChainMetadata", [contractAddress, showOnChainMetadata], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateIsActive", [contractAddress, isActive], __options);
	}

	/**
	 * updateSimpleModeAddingFee
	 *
	 * @param { (string | number | BN) } simpleModeAddingFee,
	*/
	"updateSimpleModeAddingFee" (
		simpleModeAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateSimpleModeAddingFee", [simpleModeAddingFee], __options);
	}

	/**
	 * updateStandardNftHash
	 *
	 * @param { ArgumentTypes.Hash } standardNftHash,
	*/
	"updateStandardNftHash" (
		standardNftHash: ArgumentTypes.Hash,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateStandardNftHash", [standardNftHash], __options);
	}

	/**
	 * updateAdvanceModeAddingFee
	 *
	 * @param { (string | number | BN) } advanceModeAddingFee,
	*/
	"updateAdvanceModeAddingFee" (
		advanceModeAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateAdvanceModeAddingFee", [advanceModeAddingFee], __options);
	}

	/**
	 * updateMaxRoyaltyFeeRate
	 *
	 * @param { (number | string | BN) } maxRoyaltyFeeRate,
	*/
	"updateMaxRoyaltyFeeRate" (
		maxRoyaltyFeeRate: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateMaxRoyaltyFeeRate", [maxRoyaltyFeeRate], __options);
	}

	/**
	 * getCollectionByAddress
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getCollectionByAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionByAddress", [nftContractAddress], __options);
	}

	/**
	 * getCollectionsByOwner
	 *
	 * @param { ArgumentTypes.AccountId } ownerAddress,
	*/
	"getCollectionsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionsByOwner", [ownerAddress], __options);
	}

	/**
	 * getStandardNftHash
	 *
	*/
	"getStandardNftHash" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStandardNftHash", [], __options);
	}

	/**
	 * getContractById
	 *
	 * @param { (number | string | BN) } id,
	*/
	"getContractById" (
		id: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getContractById", [id], __options);
	}

	/**
	 * getCollectionCount
	 *
	*/
	"getCollectionCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionCount", [], __options);
	}

	/**
	 * getActiveCollectionCount
	 *
	*/
	"getActiveCollectionCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getActiveCollectionCount", [], __options);
	}

	/**
	 * getSimpleModeAddingFee
	 *
	*/
	"getSimpleModeAddingFee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getSimpleModeAddingFee", [], __options);
	}

	/**
	 * getAdvanceModeAddingFee
	 *
	*/
	"getAdvanceModeAddingFee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAdvanceModeAddingFee", [], __options);
	}

	/**
	 * getMaxRoyaltyFeeRate
	 *
	*/
	"getMaxRoyaltyFeeRate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getMaxRoyaltyFeeRate", [], __options);
	}

	/**
	 * getRoleAdmin
	 *
	 * @param { (number | string | BN) } role,
	*/
	"getRoleAdmin" (
		role: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "accessControl::getRoleAdmin", [role], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "accessControl::grantRole", [role, account], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "accessControl::renounceRole", [role, account], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "accessControl::hasRole", [role, address], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "accessControl::revokeRole", [role, account], __options);
	}

	/**
	 * owner
	 *
	*/
	"owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::owner", [], __options);
	}

	/**
	 * renounceOwnership
	 *
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * transferOwnership
	 *
	 * @param { ArgumentTypes.AccountId } newOwner,
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::transferOwnership", [newOwner], __options);
	}

	/**
	 * getContractType
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getContractType" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroCollectionTrait::getContractType", [nftContractAddress], __options);
	}

	/**
	 * isActive
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"isActive" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroCollectionTrait::isActive", [nftContractAddress], __options);
	}

	/**
	 * getCollectionOwner
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getCollectionOwner" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroCollectionTrait::getCollectionOwner", [nftContractAddress], __options);
	}

	/**
	 * getRoyaltyFee
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getRoyaltyFee" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroCollectionTrait::getRoyaltyFee", [nftContractAddress], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "adminTrait::withdrawFee", [value, receiver], __options);
	}

	/**
	 * setCode
	 *
	 * @param { Array<(number | string | BN)> } codeHash,
	*/
	"setCode" (
		codeHash: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "upgradableTrait::setCode", [codeHash], __options);
	}

}