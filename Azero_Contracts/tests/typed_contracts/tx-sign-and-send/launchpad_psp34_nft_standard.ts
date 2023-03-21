/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/launchpad_psp34_nft_standard';
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
		__options ? : GasLimit,
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
	*/
	"updateWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options ? : GasLimit,
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
	*/
	"addWhitelist" (
		account: ArgumentTypes.AccountId,
		phaseId: (number | string | BN),
		whitelistAmount: (number | string | BN),
		whitelistPrice: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addWhitelist", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	* mint
	*
	* @param { (number | string | BN) } mintAmount,
	*/
	"mint" (
		mintAmount: (number | string | BN),
		__options ? : GasLimit,
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
	*/
	"publicMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
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
	*/
	"whitelistMint" (
		phaseId: (number | string | BN),
		mintAmount: (number | string | BN),
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "whitelistMint", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId, mintAmount], __options);
	}

	/**
	* deactivePhase
	*
	* @param { (number | string | BN) } phaseId,
	*/
	"deactivePhase" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
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
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateSchedulePhases", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [idPhases, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	* editProjectInformation
	*
	* @param { string } projectInfo,
	*/
	"editProjectInformation" (
		projectInfo: string,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "editProjectInformation", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [projectInfo], __options);
	}

	/**
	* getOwnerClaimedAmount
	*
	*/
	"getOwnerClaimedAmount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getOwnerClaimedAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getOwnerAvailableAmount
	*
	*/
	"getOwnerAvailableAmount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getOwnerAvailableAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getLimitPhaseCount
	*
	*/
	"getLimitPhaseCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getLimitPhaseCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getPublicMintedCount
	*
	*/
	"getPublicMintedCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPublicMintedCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getProjectInfo
	*
	*/
	"getProjectInfo" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getProjectInfo", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getPhaseScheduleById
	*
	* @param { (number | string | BN) } phaseId,
	*/
	"getPhaseScheduleById" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPhaseScheduleById", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getWhitelistByAccountId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, phaseId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPhaseAccountLink", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId, accountIndex], __options);
	}

	/**
	* getCurrentPhase
	*
	*/
	"getCurrentPhase" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCurrentPhase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* isInSchedulePhase
	*
	* @param { (number | string | BN) } time,
	*/
	"isInSchedulePhase" (
		time: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "isInSchedulePhase", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [time], __options);
	}

	/**
	* getWhitelistCount
	*
	*/
	"getWhitelistCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getWhitelistCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getLastPhaseId
	*
	*/
	"getLastPhaseId" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getLastPhaseId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getActivePhaseCount
	*
	*/
	"getActivePhaseCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getActivePhaseCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getLastTokenId
	*
	*/
	"getLastTokenId" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getLastTokenId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPhaseAccountPublicClaimedAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [accountId, phaseId], __options);
	}

	/**
	* getPhaseAccountLastIndex
	*
	* @param { (number | string | BN) } phaseId,
	*/
	"getPhaseAccountLastIndex" (
		phaseId: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPhaseAccountLastIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [phaseId], __options);
	}

	/**
	* getTotalSupply
	*
	*/
	"getTotalSupply" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getTotalSupply", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getAvailableTokenAmount
	*
	*/
	"getAvailableTokenAmount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAvailableTokenAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* owner
	*
	*/
	"owner" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::owner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* renounceOwnership
	*
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [newOwner], __options);
	}

	/**
	* balanceOf
	*
	* @param { ArgumentTypes.AccountId } owner,
	*/
	"balanceOf" (
		owner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::balanceOf", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [owner], __options);
	}

	/**
	* totalSupply
	*
	*/
	"totalSupply" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::totalSupply", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::allowance", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [owner, operator, id], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::approve", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [operator, id, approved], __options);
	}

	/**
	* collectionId
	*
	*/
	"collectionId" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::collectionId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::transfer", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [to, id, data], __options);
	}

	/**
	* ownerOf
	*
	* @param { ArgumentTypes.Id } id,
	*/
	"ownerOf" (
		id: ArgumentTypes.Id,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34::ownerOf", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [id], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Metadata::getAttribute", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [id, key], __options);
	}

	/**
	* tokenByIndex
	*
	* @param { (string | number | BN) } index,
	*/
	"tokenByIndex" (
		index: (string | number | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Enumerable::tokenByIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [index], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Enumerable::ownersTokenByIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [owner, index], __options);
	}

	/**
	* getAttributeCount
	*
	*/
	"getAttributeCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getAttributeCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* lock
	*
	* @param { ArgumentTypes.Id } tokenId,
	*/
	"lock" (
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::lock", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId], __options);
	}

	/**
	* getLockedTokenCount
	*
	*/
	"getLockedTokenCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getLockedTokenCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* psp34Traits::getLastTokenId
	*
	*/
	"psp34Traits::getLastTokenId" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getLastTokenId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* isLockedNft
	*
	* @param { ArgumentTypes.Id } tokenId,
	*/
	"isLockedNft" (
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::isLockedNft", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getAttributes", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId, attributes], __options);
	}

	/**
	* getOwner
	*
	*/
	"getOwner" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getOwner", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [], __options);
	}

	/**
	* getAttributeName
	*
	* @param { (number | string | BN) } index,
	*/
	"getAttributeName" (
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::getAttributeName", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [index], __options);
	}

	/**
	* tokenUri
	*
	* @param { (number | string | BN) } tokenId,
	*/
	"tokenUri" (
		tokenId: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Traits::tokenUri", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [tokenId], __options);
	}

	/**
	* setBaseUri
	*
	* @param { string } uri,
	*/
	"setBaseUri" (
		uri: string,
		__options ? : GasLimit,
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
	*/
	"setMultipleAttributes" (
		tokenId: ArgumentTypes.Id,
		metadata: Array<[string, string]>,
		__options ? : GasLimit,
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
	*/
	"revokeRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
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
	*/
	"grantRole" (
		role: (number | string | BN),
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::grantRole", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, address], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [value, receiver], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "psp34Burnable::burn", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "launchpad_psp34_nft_standard");
		}, [account, id], __options);
	}

}