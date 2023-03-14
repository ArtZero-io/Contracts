/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_launchpad_psp34';
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
	 * @param { (number | string | BN) } maxPhasesPerProject,
	 * @param { ArgumentTypes.Hash } standardNftHash,
	 * @param { (string | number | BN) } projectAddingFee,
	 * @param { (number | string | BN) } projectMintFeeRate,
	 * @param { (number | string | BN) } publicMaxMintingAmount,
	 * @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"initialize" (
		maxPhasesPerProject: (number | string | BN),
		standardNftHash: ArgumentTypes.Hash,
		projectAddingFee: (string | number | BN),
		projectMintFeeRate: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "initialize", [maxPhasesPerProject, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount, adminAddress], __options);
	}

	/**
	 * addNewProject
	 *
	 * @param { (number | string | BN) } totalSupply,
	 * @param { (number | string | BN) } startTime,
	 * @param { (number | string | BN) } endTime,
	 * @param { string } projectInfo,
	 * @param { Array<string> } codePhases,
	 * @param { Array<boolean> } isPublicPhases,
	 * @param { Array<(string | number | BN)> } publicMintingFeePhases,
	 * @param { Array<(number | string | BN)> } publicMintingAmountPhases,
	 * @param { Array<(number | string | BN)> } publicMaxMintingAmountPhases,
	 * @param { Array<(number | string | BN)> } startTimePhases,
	 * @param { Array<(number | string | BN)> } endTimePhases,
	*/
	"addNewProject" (
		totalSupply: (number | string | BN),
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		projectInfo: string,
		codePhases: Array<string>,
		isPublicPhases: Array<boolean>,
		publicMintingFeePhases: Array<(string | number | BN)>,
		publicMintingAmountPhases: Array<(number | string | BN)>,
		publicMaxMintingAmountPhases: Array<(number | string | BN)>,
		startTimePhases: Array<(number | string | BN)>,
		endTimePhases: Array<(number | string | BN)>,
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "addNewProject", [totalSupply, startTime, endTime, projectInfo, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	 * editProject
	 *
	 * @param { ArgumentTypes.AccountId } contractAddress,
	 * @param { (number | string | BN) } startTime,
	 * @param { (number | string | BN) } endTime,
	*/
	"editProject" (
		contractAddress: ArgumentTypes.AccountId,
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "editProject", [contractAddress, startTime, endTime], __options);
	}

	/**
	 * updateProjectAddingFee
	 *
	 * @param { (string | number | BN) } projectAddingFee,
	*/
	"updateProjectAddingFee" (
		projectAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateProjectAddingFee", [projectAddingFee], __options);
	}

	/**
	 * updatePublicMaxMintingAmount
	 *
	 * @param { (number | string | BN) } publicMaxMintingAmount,
	*/
	"updatePublicMaxMintingAmount" (
		publicMaxMintingAmount: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updatePublicMaxMintingAmount", [publicMaxMintingAmount], __options);
	}

	/**
	 * updateProjectMintFeeRate
	 *
	 * @param { (number | string | BN) } projectMintFeeRate,
	*/
	"updateProjectMintFeeRate" (
		projectMintFeeRate: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateProjectMintFeeRate", [projectMintFeeRate], __options);
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
	 * updateIsActiveProject
	 *
	 * @param { boolean } isActive,
	 * @param { ArgumentTypes.AccountId } contractAddress,
	*/
	"updateIsActiveProject" (
		isActive: boolean,
		contractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateIsActiveProject", [isActive, contractAddress], __options);
	}

	/**
	 * getProjectAddingFee
	 *
	*/
	"getProjectAddingFee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectAddingFee", [], __options);
	}

	/**
	 * getActiveProjectCount
	 *
	*/
	"getActiveProjectCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getActiveProjectCount", [], __options);
	}

	/**
	 * getProjectCount
	 *
	*/
	"getProjectCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectCount", [], __options);
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
	 * getProjectById
	 *
	 * @param { (number | string | BN) } id,
	*/
	"getProjectById" (
		id: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectById", [id], __options);
	}

	/**
	 * getProjectsByOwner
	 *
	 * @param { ArgumentTypes.AccountId } ownerAddress,
	*/
	"getProjectsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectsByOwner", [ownerAddress], __options);
	}

	/**
	 * getProjectByNftAddress
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getProjectByNftAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getProjectByNftAddress", [nftContractAddress], __options);
	}

	/**
	 * getMaxPhasesPerProject
	 *
	*/
	"getMaxPhasesPerProject" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getMaxPhasesPerProject", [], __options);
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
	 * owner
	 *
	*/
	"owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::owner", [], __options);
	}

	/**
	 * getProjectMintFeeRate
	 *
	*/
	"getProjectMintFeeRate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroLaunchPadTrait::getProjectMintFeeRate", [], __options);
	}

	/**
	 * getPublicMaxMintingAmount
	 *
	*/
	"getPublicMaxMintingAmount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroLaunchPadTrait::getPublicMaxMintingAmount", [], __options);
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