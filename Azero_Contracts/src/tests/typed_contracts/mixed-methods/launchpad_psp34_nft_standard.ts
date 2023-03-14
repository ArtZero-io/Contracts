/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/launchpad_psp34_nft_standard';
import type * as ReturnTypes from '../types-returns/launchpad_psp34_nft_standard';
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
	* addNewPhase
	*
	* @param { string } phaseCode,
	* @param { boolean } isPublic,
	* @param { (string | number | BN) } publicMintingFee,
	* @param { (number | string | BN) } publicMintingAmount,
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @param { (number | string | BN) } startTime,
	* @param { (number | string | BN) } endTime,
	* @returns { void }
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
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addNewPhase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
	}

	/**
	* updateWhitelist
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } whitelistAmount,
	* @param { (string | number | BN) } whitelistPrice,
	* @returns { void }
	*/
	"updateWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateWhitelist", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	* addWhitelist
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } whitelistAmount,
	* @param { (string | number | BN) } whitelistPrice,
	* @returns { void }
	*/
	"addWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addWhitelist", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	* mint
	*
	* @param { (number | string | BN) } mintAmount,
	* @returns { void }
	*/
	"mint" (
		mintAmount: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "mint", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [mintAmount], __options);
	}

	/**
	* publicMint
	*
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } mintAmount,
	* @returns { void }
	*/
	"publicMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "publicMint", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId, mintAmount], __options);
	}

	/**
	* whitelistMint
	*
	* @param { (number | string | BN) } phaseId,
	* @param { (number | string | BN) } mintAmount,
	* @returns { void }
	*/
	"whitelistMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "whitelistMint", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId, mintAmount], __options);
	}

	/**
	* deactivePhase
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { void }
	*/
	"deactivePhase" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "deactivePhase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId], __options);
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
	* @returns { void }
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
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateSchedulePhase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId, phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
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
	* @returns { void }
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
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateSchedulePhases", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [idPhases, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	* editProjectInformation
	*
	* @param { string } projectInfo,
	* @returns { void }
	*/
	"editProjectInformation" (
		projectInfo: string,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "editProjectInformation", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [projectInfo], __options);
	}

	/**
	* getOwnerClaimedAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getOwnerClaimedAmount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getOwnerClaimedAmount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getOwnerAvailableAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getOwnerAvailableAmount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getOwnerAvailableAmount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLimitPhaseCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLimitPhaseCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLimitPhaseCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPublicMintedCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPublicMintedCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPublicMintedCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getProjectInfo
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getProjectInfo" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectInfo", [], __options, (result) => { return handleReturnType(result, getTypeDescription(26, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseScheduleById
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<ReturnTypes.Phase | null, ReturnTypes.LangError> }
	*/
	"getPhaseScheduleById" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Phase | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseScheduleById", [phaseId], __options, (result) => { return handleReturnType(result, getTypeDescription(27, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Whitelist | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getWhitelistByAccountId", [account, phaseId], __options, (result) => { return handleReturnType(result, getTypeDescription(30, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountLink", [phaseId, accountIndex], __options, (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getCurrentPhase
	*
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getCurrentPhase" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCurrentPhase", [], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* isInSchedulePhase
	*
	* @param { (number | string | BN) } time,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"isInSchedulePhase" (
		time: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "isInSchedulePhase", [time], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getWhitelistCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getWhitelistCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getWhitelistCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLastPhaseId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLastPhaseId" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLastPhaseId", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getActivePhaseCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActivePhaseCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActivePhaseCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getLastTokenId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLastTokenId" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLastTokenId", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountPublicClaimedAmount", [accountId, phaseId], __options, (result) => { return handleReturnType(result, getTypeDescription(36, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getPhaseAccountLastIndex
	*
	* @param { (number | string | BN) } phaseId,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPhaseAccountLastIndex" (
		phaseId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPhaseAccountLastIndex", [phaseId], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getTotalSupply
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getTotalSupply" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalSupply", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAvailableTokenAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getAvailableTokenAmount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAvailableTokenAmount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [newOwner], __options);
	}

	/**
	* balanceOf
	*
	* @param { ArgumentTypes.AccountId } owner,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::balanceOf", [owner], __options, (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* totalSupply
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"totalSupply" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::totalSupply", [], __options, (result) => { return handleReturnType(result, getTypeDescription(41, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::allowance", [owner, operator, id], __options, (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* approve
	*
	* @param { ArgumentTypes.AccountId } operator,
	* @param { ArgumentTypes.Id | null } id,
	* @param { boolean } approved,
	* @returns { void }
	*/
	"approve" (
		operator: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id | null,
		approved: boolean,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::approve", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [operator, id, approved], __options);
	}

	/**
	* collectionId
	*
	* @returns { Result<ReturnTypes.Id, ReturnTypes.LangError> }
	*/
	"collectionId" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::collectionId", [], __options, (result) => { return handleReturnType(result, getTypeDescription(48, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* transfer
	*
	* @param { ArgumentTypes.AccountId } to,
	* @param { ArgumentTypes.Id } id,
	* @param { Array<(number | string | BN)> } data,
	* @returns { void }
	*/
	"transfer" (
		to: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		data: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::transfer", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [to, id, data], __options);
	}

	/**
	* ownerOf
	*
	* @param { ArgumentTypes.Id } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34::ownerOf", [id], __options, (result) => { return handleReturnType(result, getTypeDescription(49, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<number> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Metadata::getAttribute", [id, key], __options, (result) => { return handleReturnType(result, getTypeDescription(51, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* tokenByIndex
	*
	* @param { (string | number | BN) } index,
	* @returns { Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> }
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Enumerable::tokenByIndex", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(53, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Result<ReturnTypes.Id, ReturnTypes.PSP34Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options, (result) => { return handleReturnType(result, getTypeDescription(53, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttributeCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getAttributeCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* lock
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { void }
	*/
	"lock" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::lock", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId], __options);
	}

	/**
	* getLockedTokenCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLockedTokenCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getLockedTokenCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* psp34Traits::getLastTokenId
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"psp34Traits::getLastTokenId" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getLastTokenId", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* isLockedNft
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"isLockedNft" (
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::isLockedNft", [tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
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
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<string>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributes", [tokenId, attributes], __options, (result) => { return handleReturnType(result, getTypeDescription(55, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getOwner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getOwner" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getOwner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(33, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* getAttributeName
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<string, ReturnTypes.LangError> }
	*/
	"getAttributeName" (
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeName", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(56, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* tokenUri
	*
	* @param { (number | string | BN) } tokenId,
	* @returns { Result<string, ReturnTypes.LangError> }
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<string, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "psp34Traits::tokenUri", [tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(56, 'launchpad_psp34_nft_standard')); });
	}

	/**
	* setBaseUri
	*
	* @param { string } uri,
	* @returns { void }
	*/
	"setBaseUri" (
		uri: string,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::setBaseUri", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [uri], __options);
	}

	/**
	* setMultipleAttributes
	*
	* @param { ArgumentTypes.Id } tokenId,
	* @param { Array<[string, string]> } metadata,
	* @returns { void }
	*/
	"setMultipleAttributes" (
		tokenId: ArgumentTypes.Id,
		metadata: Array<[string, string]>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::setMultipleAttributes", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId, metadata], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, account], __options);
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options, (result) => { return handleReturnType(result, getTypeDescription(40, 'launchpad_psp34_nft_standard')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options, (result) => { return handleReturnType(result, getTypeDescription(44, 'launchpad_psp34_nft_standard')); });
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [value, receiver], __options);
	}

	/**
	* burn
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { ArgumentTypes.Id } id,
	* @returns { void }
	*/
	"burn" (
		account: ArgumentTypes.AccountId,
		id: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Burnable::burn", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, id], __options);
	}

}