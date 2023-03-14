/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/launchpad_psp34_nft_standard';
import type * as ReturnTypes from '../types-returns/launchpad_psp34_nft_standard';
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
	* addNewPhase
	*
	* @param { string } phaseCode,
	* @param { boolean } isPublic,
	* @param { (string | number | BN) } publicMintingFee,
	* @param { (number | string | BN) } publicMintingAmount,
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @param { (number | string | BN) } startTime,
	* @param { (number | string | BN) } endTime,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"addNewPhase" (
		phaseCode: string,
		isPublic: boolean,
		publicMintingFee: (string | number | BN),
		publicMintingAmount: (number | string | BN),
		publicMaxMintingAmount: (number | string | BN),
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "addNewPhase", [phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* updateWhitelist
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } whitelistAmount,
	* @param { (string | number | BN) } whitelistPrice,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"updateWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* addWhitelist
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } whitelistAmount,
	* @param { (string | number | BN) } whitelistPrice,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"addWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "addWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* mint
	*
	* @param { (number | string | BN) } mintAmount,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"mint" (
		mintAmount: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "mint", [mintAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* publicMint
	*
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } mintAmount,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"publicMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "publicMint", [phaseId, mintAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* whitelistMint
	*
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } mintAmount,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"whitelistMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "whitelistMint", [phaseId, mintAmount], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* deactivePhase
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"deactivePhase" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "deactivePhase", [phaseId], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
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
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateSchedulePhase", [phaseId, phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
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
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "updateSchedulePhases", [idPhases, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* editProjectInformation
	*
	* @param { string } projectInfo,
	* @returns { Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> }
	*/
	"editProjectInformation" (
		projectInfo: string,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.AccessControlError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "editProjectInformation", [projectInfo], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getOwnerClaimedAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getOwnerClaimedAmount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getOwnerClaimedAmount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getOwnerAvailableAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getOwnerAvailableAmount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getOwnerAvailableAmount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLimitPhaseCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLimitPhaseCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLimitPhaseCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPublicMintedCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPublicMintedCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPublicMintedCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getProjectInfo
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getProjectInfo" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectInfo", [], __options , (result) => { return handleReturnType(result, getTypeDescription(26, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseScheduleById
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<ReturnTypes.Phase | null, ReturnTypes.LangError> }
	*/
	"getPhaseScheduleById" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Phase | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseScheduleById", [phaseId], __options , (result) => { return handleReturnType(result, getTypeDescription(27, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getWhitelistByAccountId
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<ReturnTypes.Whitelist | null, ReturnTypes.LangError> }
	*/
	"getWhitelistByAccountId" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Whitelist | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getWhitelistByAccountId", [account, phaseId], __options , (result) => { return handleReturnType(result, getTypeDescription(30, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseAccountLink
	*
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } accountIndex,
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getPhaseAccountLink" (
		phaseId: (number | string | BN),
		accountIndex: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountLink", [phaseId, accountIndex], __options , (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getCurrentPhase
	*
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getCurrentPhase" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCurrentPhase", [], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* isInSchedulePhase
	*
	* @param { (number | string | BN) } time,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"isInSchedulePhase" (
		time: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "isInSchedulePhase", [time], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getWhitelistCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getWhitelistCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getWhitelistCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLastPhaseId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLastPhaseId" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLastPhaseId", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getActivePhaseCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActivePhaseCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActivePhaseCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLastTokenId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLastTokenId" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLastTokenId", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseAccountPublicClaimedAmount
	*
	* @param { ArgumentTypes.AccountId } accountId,
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getPhaseAccountPublicClaimedAmount" (
		accountId: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountPublicClaimedAmount", [accountId, phaseId], __options , (result) => { return handleReturnType(result, getTypeDescription(36, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseAccountLastIndex
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPhaseAccountLastIndex" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountLastIndex", [phaseId], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getTotalSupply
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getTotalSupply" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalSupply", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAvailableTokenAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getAvailableTokenAmount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAvailableTokenAmount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(38, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* balanceOf
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::balanceOf", [owner], __options , (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* totalSupply
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"totalSupply" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::totalSupply", [], __options , (result) => { return handleReturnType(result, getTypeDescription(41, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* allowance
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"allowance" (
		owner: ArgumentTypes.AccountId,
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::allowance", [owner, operator, id], __options , (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* approve
	*
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @param { boolean } approved,
	* @returns { Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::approve", [operator, id, approved], __options , (result) => { return handleReturnType(result, getTypeDescription(45, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* collectionId
	*
	* @returns { Result<ReturnTypes.Id, ReturnTypes.LangError> }
	*/
	"collectionId" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::collectionId", [], __options , (result) => { return handleReturnType(result, getTypeDescription(48, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* transfer
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } data,
	* @returns { Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::transfer", [to, id, data], __options , (result) => { return handleReturnType(result, getTypeDescription(45, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* ownerOf
	*
	* @param { ArgumentTypes.Id } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::ownerOf", [id], __options , (result) => { return handleReturnType(result, getTypeDescription(49, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttribute
	*
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } key,
	* @returns { Result<Array<number> | null, ReturnTypes.LangError> }
	*/
	"getAttribute" (
		id: ArgumentTypes.Id,
		key: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<number> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Metadata::getAttribute", [id, key], __options , (result) => { return handleReturnType(result, getTypeDescription(51, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* tokenByIndex
	*
	* @param { (string | number | BN) } index,
	* @returns { Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Enumerable::tokenByIndex", [index], __options , (result) => { return handleReturnType(result, getTypeDescription(53, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* ownersTokenByIndex
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @param { (string | number | BN) } index,
	* @returns { Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"ownersTokenByIndex" (
		owner: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options , (result) => { return handleReturnType(result, getTypeDescription(53, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttributeCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getAttributeCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* lock
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"lock" (
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::lock", [tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLockedTokenCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLockedTokenCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getLockedTokenCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* psp34Traits::getLastTokenId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"psp34Traits::getLastTokenId" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getLastTokenId", [], __options , (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* isLockedNft
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"isLockedNft" (
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::isLockedNft", [tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttributes
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { Array<string> } attributes,
	* @returns { Result<Array<string>, ReturnTypes.LangError> }
	*/
	"getAttributes" (
		tokenId: ArgumentTypes.Id,
		attributes: Array<string>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<string>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributes", [tokenId, attributes], __options , (result) => { return handleReturnType(result, getTypeDescription(55, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getOwner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getOwner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getOwner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttributeName
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<string, ReturnTypes.LangError> }
	*/
	"getAttributeName" (
		index: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeName", [index], __options , (result) => { return handleReturnType(result, getTypeDescription(56, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* tokenUri
	*
	* @param { (number | string | BN) } tokenId,
	* @returns { Result<string, ReturnTypes.LangError> }
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::tokenUri", [tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(56, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* setBaseUri
	*
	* @param { string } uri,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setBaseUri" (
		uri: string,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::setBaseUri", [uri], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* setMultipleAttributes
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { Array<[string, string]> } metadata,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setMultipleAttributes" (
		tokenId: ArgumentTypes.Id,
		metadata: Array<[string, string]>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::setMultipleAttributes", [tokenId, metadata], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::revokeRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::grantRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options , (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options , (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::renounceRole", [role, account], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::withdrawFee", [value, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(17, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* burn
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.Id } id,
	* @returns { Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"burn" (
		account: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Burnable::burn", [account, id], __options , (result) => { return handleReturnType(result, getTypeDescription(45, 'launchpad_psp34_nft_standard')); });
	}

}