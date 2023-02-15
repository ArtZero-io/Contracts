/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/launchpad_psp34_nft_standard';
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
	 * addNewPhase
	 *
	 * @param { string } phaseCode,
	 * @param { boolean } isPublic,
	 * @param { (string | number | BN) } publicMintingFee,
	 * @param { (number | string | BN) } publicMintingAmount,
	 * @param { (number | string | BN) } publicMaxMintingAmount,
	 * @param { (number | string | BN) } startTime,
	 * @param { (number | string | BN) } endTime,
	*/
	"addNewPhase" (
		phaseCode: string,
		isPublic: boolean,
		publicMintingFee: (string | number | BN),
		publicMintingAmount: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "addNewPhase", [phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
	}

	/**
	 * updateWhitelist
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { (number | string | BN) } phaseId,
	 * @param { (number | string | BN) } whitelistAmount,
	 * @param { (string | number | BN) } whitelistPrice,
	*/
	"updateWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	 * addWhitelist
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { (number | string | BN) } phaseId,
	 * @param { (number | string | BN) } whitelistAmount,
	 * @param { (string | number | BN) } whitelistPrice,
	*/
	"addWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "addWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	 * mint
	 *
	 * @param { (number | string | BN) } mintAmount,
	*/
	"mint" (
		mintAmount: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "mint", [mintAmount], __options);
	}

	/**
	 * publicMint
	 *
	 * @param { (number | string | BN) } phaseId,
	 * @param { (number | string | BN) } mintAmount,
	*/
	"publicMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "publicMint", [phaseId, mintAmount], __options);
	}

	/**
	 * whitelistMint
	 *
	 * @param { (number | string | BN) } phaseId,
	 * @param { (number | string | BN) } mintAmount,
	*/
	"whitelistMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "whitelistMint", [phaseId, mintAmount], __options);
	}

	/**
	 * deactivePhase
	 *
	 * @param { (number | string | BN) } phaseId,
	*/
	"deactivePhase" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "deactivePhase", [phaseId], __options);
	}

	/**
	 * updateSchedulePhase
	 *
	 * @param { (number | string | BN) } phaseId,
	 * @param { string } phaseCode,
	 * @param { boolean } isPublic,
	 * @param { (string | number | BN) } publicMintingFee,
	 * @param { (number | string | BN) } publicMintingAmount,
	 * @param { (number | string | BN) } publicMaxMintingAmount,
	 * @param { (number | string | BN) } startTime,
	 * @param { (number | string | BN) } endTime,
	*/
	"updateSchedulePhase" (
		phaseId: (number | string | BN),
		phaseCode: string,
		isPublic: boolean,
		publicMintingFee: (string | number | BN),
		publicMintingAmount: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateSchedulePhase", [phaseId, phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
	}

	/**
	 * updateSchedulePhases
	 *
	 * @param { Array<(number | string | BN)> } idPhases,
	 * @param { Array<string> } codePhases,
	 * @param { Array<boolean> } isPublicPhases,
	 * @param { Array<(string | number | BN)> } publicMintingFeePhases,
	 * @param { Array<(number | string | BN)> } publicMintingAmountPhases,
	 * @param { Array<(number | string | BN)> } publicMaxMintingAmountPhases,
	 * @param { Array<(number | string | BN)> } startTimePhases,
	 * @param { Array<(number | string | BN)> } endTimePhases,
	*/
	"updateSchedulePhases" (
		idPhases: Array<(number | string | BN)>,
		codePhases: Array<string>,
		isPublicPhases: Array<boolean>,
		publicMintingFeePhases: Array<(string | number | BN)>,
		publicMintingAmountPhases: Array<(number | string | BN)>,
		publicMaxMintingAmountPhases: Array<(number | string | BN)>,
		startTimePhases: Array<(number | string | BN)>,
		endTimePhases: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateSchedulePhases", [idPhases, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	 * editProjectInformation
	 *
	 * @param { string } projectInfo,
	*/
	"editProjectInformation" (
		projectInfo: string,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "editProjectInformation", [projectInfo], __options);
	}

	/**
	 * getOwnerClaimedAmount
	 *
	*/
	"getOwnerClaimedAmount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getOwnerClaimedAmount", [], __options);
	}

	/**
	 * getOwnerAvailableAmount
	 *
	*/
	"getOwnerAvailableAmount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getOwnerAvailableAmount", [], __options);
	}

	/**
	 * getLimitPhaseCount
	 *
	*/
	"getLimitPhaseCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getLimitPhaseCount", [], __options);
	}

	/**
	 * getPublicMintedCount
	 *
	*/
	"getPublicMintedCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPublicMintedCount", [], __options);
	}

	/**
	 * getProjectInfo
	 *
	*/
	"getProjectInfo" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectInfo", [], __options);
	}

	/**
	 * getPhaseScheduleById
	 *
	 * @param { (number | string | BN) } phaseId,
	*/
	"getPhaseScheduleById" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPhaseScheduleById", [phaseId], __options);
	}

	/**
	 * getWhitelistByAccountId
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { (number | string | BN) } phaseId,
	*/
	"getWhitelistByAccountId" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getWhitelistByAccountId", [account, phaseId], __options);
	}

	/**
	 * getPhaseAccountLink
	 *
	 * @param { (number | string | BN) } phaseId,
	 * @param { (number | string | BN) } accountIndex,
	*/
	"getPhaseAccountLink" (
		phaseId: (number | string | BN),
		accountIndex: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPhaseAccountLink", [phaseId, accountIndex], __options);
	}

	/**
	 * getCurrentPhase
	 *
	*/
	"getCurrentPhase" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCurrentPhase", [], __options);
	}

	/**
	 * isInSchedulePhase
	 *
	 * @param { (number | string | BN) } time,
	*/
	"isInSchedulePhase" (
		time: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "isInSchedulePhase", [time], __options);
	}

	/**
	 * getWhitelistCount
	 *
	*/
	"getWhitelistCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getWhitelistCount", [], __options);
	}

	/**
	 * getLastPhaseId
	 *
	*/
	"getLastPhaseId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getLastPhaseId", [], __options);
	}

	/**
	 * getActivePhaseCount
	 *
	*/
	"getActivePhaseCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getActivePhaseCount", [], __options);
	}

	/**
	 * getLastTokenId
	 *
	*/
	"getLastTokenId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getLastTokenId", [], __options);
	}

	/**
	 * getPhaseAccountPublicClaimedAmount
	 *
	 * @param { ArgumentTypes.AccountId } accountId,
	 * @param { (number | string | BN) } phaseId,
	*/
	"getPhaseAccountPublicClaimedAmount" (
		accountId: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPhaseAccountPublicClaimedAmount", [accountId, phaseId], __options);
	}

	/**
	 * getPhaseAccountLastIndex
	 *
	 * @param { (number | string | BN) } phaseId,
	*/
	"getPhaseAccountLastIndex" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPhaseAccountLastIndex", [phaseId], __options);
	}

	/**
	 * getTotalSupply
	 *
	*/
	"getTotalSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getTotalSupply", [], __options);
	}

	/**
	 * getAvailableTokenAmount
	 *
	*/
	"getAvailableTokenAmount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAvailableTokenAmount", [], __options);
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
	 * balanceOf
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::balanceOf", [owner], __options);
	}

	/**
	 * totalSupply
	 *
	*/
	"totalSupply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::totalSupply", [], __options);
	}

	/**
	 * allowance
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	*/
	"allowance" (
		owner: ArgumentTypes.AccountId,
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	 * approve
	 *
	 * @param { ArgumentTypes.AccountId } operator,
	 * @param { ArgumentTypes.Id | null } id,
	 * @param { boolean } approved,
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::approve", [operator, id, approved], __options);
	}

	/**
	 * collectionId
	 *
	*/
	"collectionId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::collectionId", [], __options);
	}

	/**
	 * transfer
	 *
	 * @param { ArgumentTypes.AccountId } to,
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } data,
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::transfer", [to, id, data], __options);
	}

	/**
	 * ownerOf
	 *
	 * @param { ArgumentTypes.Id } id,
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34::ownerOf", [id], __options);
	}

	/**
	 * getAttribute
	 *
	 * @param { ArgumentTypes.Id } id,
	 * @param { Array<(number | string | BN)> } key,
	*/
	"getAttribute" (
		id: ArgumentTypes.Id,
		key: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Metadata::getAttribute", [id, key], __options);
	}

	/**
	 * tokenByIndex
	 *
	 * @param { (string | number | BN) } index,
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Enumerable::tokenByIndex", [index], __options);
	}

	/**
	 * ownersTokenByIndex
	 *
	 * @param { ArgumentTypes.AccountId } owner,
	 * @param { (string | number | BN) } index,
	*/
	"ownersTokenByIndex" (
		owner: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options);
	}

	/**
	 * getAttributeCount
	 *
	*/
	"getAttributeCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getAttributeCount", [], __options);
	}

	/**
	 * lock
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"lock" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::lock", [tokenId], __options);
	}

	/**
	 * getLockedTokenCount
	 *
	*/
	"getLockedTokenCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getLockedTokenCount", [], __options);
	}

	/**
	 * psp34Traits::getLastTokenId
	 *
	*/
	"psp34Traits::getLastTokenId" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getLastTokenId", [], __options);
	}

	/**
	 * isLockedNft
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"isLockedNft" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::isLockedNft", [tokenId], __options);
	}

	/**
	 * getAttributes
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { Array<string> } attributes,
	*/
	"getAttributes" (
		tokenId: ArgumentTypes.Id,
		attributes: Array<string>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getAttributes", [tokenId, attributes], __options);
	}

	/**
	 * getOwner
	 *
	*/
	"getOwner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getOwner", [], __options);
	}

	/**
	 * getAttributeName
	 *
	 * @param { (number | string | BN) } index,
	*/
	"getAttributeName" (
		index: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::getAttributeName", [index], __options);
	}

	/**
	 * tokenUri
	 *
	 * @param { (number | string | BN) } tokenId,
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::tokenUri", [tokenId], __options);
	}

	/**
	 * setBaseUri
	 *
	 * @param { string } uri,
	*/
	"setBaseUri" (
		uri: string,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::setBaseUri", [uri], __options);
	}

	/**
	 * setMultipleAttributes
	 *
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { Array<[string, string]> } metadata,
	*/
	"setMultipleAttributes" (
		tokenId: ArgumentTypes.Id,
		metadata: Array<[string, string]>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Traits::setMultipleAttributes", [tokenId, metadata], __options);
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
	 * burn
	 *
	 * @param { ArgumentTypes.AccountId } account,
	 * @param { ArgumentTypes.Id } id,
	*/
	"burn" (
		account: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "psp34Burnable::burn", [account, id], __options);
	}

}