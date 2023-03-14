/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_launchpad_psp34';
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
		__options ? : GasLimit,
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
	*/
	"editProject" (
		contractAddress: ArgumentTypes.AccountId,
		startTime: (number | string | BN),
		endTime: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "editProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [contractAddress, startTime, endTime], __options);
	}

	/**
	* updateProjectAddingFee
	*
	* @param { (string | number | BN) } projectAddingFee,
	*/
	"updateProjectAddingFee" (
		projectAddingFee: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [projectAddingFee], __options);
	}

	/**
	* updatePublicMaxMintingAmount
	*
	* @param { (number | string | BN) } publicMaxMintingAmount,
	*/
	"updatePublicMaxMintingAmount" (
		publicMaxMintingAmount: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updatePublicMaxMintingAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [publicMaxMintingAmount], __options);
	}

	/**
	* updateProjectMintFeeRate
	*
	* @param { (number | string | BN) } projectMintFeeRate,
	*/
	"updateProjectMintFeeRate" (
		projectMintFeeRate: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectMintFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [projectMintFeeRate], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [standardNftHash], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsActiveProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [isActive, contractAddress], __options);
	}

	/**
	* getProjectAddingFee
	*
	*/
	"getProjectAddingFee" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectAddingFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getActiveProjectCount
	*
	*/
	"getActiveProjectCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getActiveProjectCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getProjectCount
	*
	*/
	"getProjectCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getStandardNftHash
	*
	*/
	"getStandardNftHash" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStandardNftHash", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getProjectById
	*
	* @param { (number | string | BN) } id,
	*/
	"getProjectById" (
		id: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectById", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [id], __options);
	}

	/**
	* getProjectsByOwner
	*
	* @param { ArgumentTypes.AccountId } ownerAddress,
	*/
	"getProjectsByOwner" (
		ownerAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectsByOwner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [ownerAddress], __options);
	}

	/**
	* getProjectByNftAddress
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	*/
	"getProjectByNftAddress" (
		nftContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectByNftAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [nftContractAddress], __options);
	}

	/**
	* getMaxPhasesPerProject
	*
	*/
	"getMaxPhasesPerProject" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getMaxPhasesPerProject", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [role, address], __options);
	}

	/**
	* renounceOwnership
	*
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [newOwner], __options);
	}

	/**
	* owner
	*
	*/
	"owner" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::owner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getProjectMintFeeRate
	*
	*/
	"getProjectMintFeeRate" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroLaunchPadTrait::getProjectMintFeeRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
	}

	/**
	* getPublicMaxMintingAmount
	*
	*/
	"getPublicMaxMintingAmount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroLaunchPadTrait::getPublicMaxMintingAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_launchpad_psp34");
		}, [codeHash], __options);
	}

}