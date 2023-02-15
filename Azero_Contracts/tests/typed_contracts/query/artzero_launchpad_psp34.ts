/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_launchpad_psp34';
import type * as ReturnTypes from '../types-returns/artzero_launchpad_psp34';
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
	* @param { (number | string | BN) } maxPhasesPerProject,
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } projectAddingFee,
	* @param { (number | string | BN) } projectMintFeeRate,
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"initialize" (
		maxPhasesPerProject: (number | string | BN),
		standardNftHash: ArgumentTypes.Hash,
		projectAddingFee: (string | number | BN),
		projectMintFeeRate: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "initialize", [maxPhasesPerProject, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount, adminAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
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
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
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
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "addNewProject", [totalSupply, startTime, endTime, projectInfo, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

	/**
	* editProject
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { (number | string | BN) } startTime,
	* @param { (number | string | BN) } endTime,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"editProject" (
		contractAddress: ArgumentTypes.AccountId,
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "editProject", [contractAddress, startTime, endTime], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

	/**
	* updateProjectAddingFee
	*
	* @param { (string | number | BN) } projectAddingFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateProjectAddingFee" (
		projectAddingFee: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateProjectAddingFee", [projectAddingFee], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

	/**
	* updatePublicMaxMintingAmount
	*
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"updatePublicMaxMintingAmount" (
		publicMaxMintingAmount: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updatePublicMaxMintingAmount", [publicMaxMintingAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_launchpad_psp34')); });
	}

	/**
	* updateProjectMintFeeRate
	*
	* @param { (number | string | BN) } projectMintFeeRate,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"updateProjectMintFeeRate" (
		projectMintFeeRate: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateProjectMintFeeRate", [projectMintFeeRate], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateStandardNftHash", [standardNftHash], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

	/**
	* updateIsActiveProject
	*
	* @param { boolean } isActive,
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateIsActiveProject" (
		isActive: boolean,
		contractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateIsActiveProject", [isActive, contractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getProjectAddingFee" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectAddingFee", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_launchpad_psp34')); });
	}

	/**
	* getActiveProjectCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActiveProjectCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActiveProjectCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getProjectCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
	}

	/**
	* getStandardNftHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"getStandardNftHash" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStandardNftHash", [], __options , (result) => { return handleReturnType(result, getTypeDescription(26, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectById
	*
	* @param { (number | string | BN) } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getProjectById" (
		id: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectById", [id], __options , (result) => { return handleReturnType(result, getTypeDescription(27, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"getProjectsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectsByOwner", [ownerAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(29, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectByNftAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.Project | null, ReturnTypes.LangError> }
	*/
	"getProjectByNftAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Project | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectByNftAddress", [nftContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(30, 'artzero_launchpad_psp34')); });
	}

	/**
	* getMaxPhasesPerProject
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getMaxPhasesPerProject" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getMaxPhasesPerProject", [], __options , (result) => { return handleReturnType(result, getTypeDescription(33, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::grantRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::renounceRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::revokeRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options , (result) => { return handleReturnType(result, getTypeDescription(35, 'artzero_launchpad_psp34')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options , (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(36, 'artzero_launchpad_psp34')); });
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectMintFeeRate
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getProjectMintFeeRate" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getProjectMintFeeRate", [], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_launchpad_psp34')); });
	}

	/**
	* getPublicMaxMintingAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPublicMaxMintingAmount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getPublicMaxMintingAmount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::withdrawFee", [value, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "upgradableTrait::setCode", [codeHash], __options , (result) => { return handleReturnType(result, getTypeDescription(12, 'artzero_launchpad_psp34')); });
	}

}