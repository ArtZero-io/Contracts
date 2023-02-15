/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_launchpad_psp34';
import type * as ReturnTypes from '../types-returns/artzero_launchpad_psp34';
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
	* @param { (number | string | BN) } maxPhasesPerProject,
	* @param { ArgumentTypes.Hash } standardNftHash,
	* @param { (string | number | BN) } projectAddingFee,
	* @param { (number | string | BN) } projectMintFeeRate,
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { void }
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
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [maxPhasesPerProject, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount, adminAddress], __options);
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
	* @returns { void }
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
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addNewProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [totalSupply, startTime, endTime, projectInfo, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	* editProject
	*
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @param { (number | string | BN) } startTime,
	* @param { (number | string | BN) } endTime,
	* @returns { void }
	*/
	"editProject" (
		contractAddress: ArgumentTypes.AccountId,
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "editProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [contractAddress, startTime, endTime], __options);
	}

	/**
	* updateProjectAddingFee
	*
	* @param { (string | number | BN) } projectAddingFee,
	* @returns { void }
	*/
	"updateProjectAddingFee" (
		projectAddingFee: (string | number | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [projectAddingFee], __options);
	}

	/**
	* updatePublicMaxMintingAmount
	*
	* @param { (number | string | BN) } publicMaxMintingAmount,
	* @returns { void }
	*/
	"updatePublicMaxMintingAmount" (
		publicMaxMintingAmount: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updatePublicMaxMintingAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [publicMaxMintingAmount], __options);
	}

	/**
	* updateProjectMintFeeRate
	*
	* @param { (number | string | BN) } projectMintFeeRate,
	* @returns { void }
	*/
	"updateProjectMintFeeRate" (
		projectMintFeeRate: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectMintFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [projectMintFeeRate], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [standardNftHash], __options);
	}

	/**
	* updateIsActiveProject
	*
	* @param { boolean } isActive,
	* @param { ArgumentTypes.AccountId } contractAddress,
	* @returns { void }
	*/
	"updateIsActiveProject" (
		isActive: boolean,
		contractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsActiveProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [isActive, contractAddress], __options);
	}

	/**
	* getProjectAddingFee
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getProjectAddingFee" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectAddingFee", [], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_launchpad_psp34')); });
	}

	/**
	* getActiveProjectCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getActiveProjectCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getActiveProjectCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getProjectCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
	}

	/**
	* getStandardNftHash
	*
	* @returns { Result<ReturnTypes.Hash, ReturnTypes.LangError> }
	*/
	"getStandardNftHash" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Hash, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStandardNftHash", [], __options, (result) => { return handleReturnType(result, getTypeDescription(26, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectById
	*
	* @param { (number | string | BN) } id,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getProjectById" (
		id: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectById", [id], __options, (result) => { return handleReturnType(result, getTypeDescription(27, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	* @returns { Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> }
	*/
	"getProjectsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.AccountId>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectsByOwner", [ownerAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(29, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectByNftAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @returns { Result<ReturnTypes.Project | null, ReturnTypes.LangError> }
	*/
	"getProjectByNftAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Project | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getProjectByNftAddress", [nftContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(30, 'artzero_launchpad_psp34')); });
	}

	/**
	* getMaxPhasesPerProject
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getMaxPhasesPerProject" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getMaxPhasesPerProject", [], __options, (result) => { return handleReturnType(result, getTypeDescription(33, 'artzero_launchpad_psp34')); });
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_launchpad_psp34')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options, (result) => { return handleReturnType(result, getTypeDescription(35, 'artzero_launchpad_psp34')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [newOwner], __options);
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(38, 'artzero_launchpad_psp34')); });
	}

	/**
	* getProjectMintFeeRate
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getProjectMintFeeRate" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getProjectMintFeeRate", [], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_launchpad_psp34')); });
	}

	/**
	* getPublicMaxMintingAmount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPublicMaxMintingAmount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getPublicMaxMintingAmount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_launchpad_psp34')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [codeHash], __options);
	}

}