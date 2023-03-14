/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_staking_nft';
import type * as ReturnTypes from '../types-returns/artzero_staking_nft';
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
	* @param { ArgumentTypes.AccountId } artzeroNftContract,
	* @param { (number | string | BN) } limitUnstakeTime,
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { void }
	*/
	"initialize" (
		artzeroNftContract: ArgumentTypes.AccountId,
		limitUnstakeTime: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [artzeroNftContract, limitUnstakeTime, adminAddress], __options);
	}

	/**
	* setArtzeroNftContract
	*
	* @param { ArgumentTypes.AccountId } artzeroNftContract,
	* @returns { void }
	*/
	"setArtzeroNftContract" (
		artzeroNftContract: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setArtzeroNftContract", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [artzeroNftContract], __options);
	}

	/**
	* setLimitUnstakeTime
	*
	* @param { (number | string | BN) } limitUnstakeTime,
	* @returns { void }
	*/
	"setLimitUnstakeTime" (
		limitUnstakeTime: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setLimitUnstakeTime", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [limitUnstakeTime], __options);
	}

	/**
	* updateAdminAddress
	*
	* @param { ArgumentTypes.AccountId } adminAddress,
	* @returns { void }
	*/
	"updateAdminAddress" (
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateAdminAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [adminAddress], __options);
	}

	/**
	* updateIsLocked
	*
	* @param { boolean } isLocked,
	* @returns { void }
	*/
	"updateIsLocked" (
		isLocked: boolean,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsLocked", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [isLocked], __options);
	}

	/**
	* startRewardDistribution
	*
	* @returns { void }
	*/
	"startRewardDistribution" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "startRewardDistribution", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* stopRewardDistribution
	*
	* @returns { void }
	*/
	"stopRewardDistribution" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "stopRewardDistribution", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* addReward
	*
	* @returns { void }
	*/
	"addReward" (
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addReward", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* setClaimedStatus
	*
	* @param { ArgumentTypes.AccountId } staker,
	* @returns { void }
	*/
	"setClaimedStatus" (
		staker: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setClaimedStatus", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [staker], __options);
	}

	/**
	* claimReward
	*
	* @returns { void }
	*/
	"claimReward" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "claimReward", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getRewardPool
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getRewardPool" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getRewardPool", [], __options, (result) => { return handleReturnType(result, getTypeDescription(16, 'artzero_staking_nft')); });
	}

	/**
	* getClaimableReward
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getClaimableReward" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getClaimableReward", [], __options, (result) => { return handleReturnType(result, getTypeDescription(16, 'artzero_staking_nft')); });
	}

	/**
	* isClaimed
	*
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"isClaimed" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "isClaimed", [account], __options, (result) => { return handleReturnType(result, getTypeDescription(17, 'artzero_staking_nft')); });
	}

	/**
	* getRewardStarted
	*
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"getRewardStarted" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getRewardStarted", [], __options, (result) => { return handleReturnType(result, getTypeDescription(17, 'artzero_staking_nft')); });
	}

	/**
	* getArtzeroNftContract
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getArtzeroNftContract" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getArtzeroNftContract", [], __options, (result) => { return handleReturnType(result, getTypeDescription(18, 'artzero_staking_nft')); });
	}

	/**
	* getIsLocked
	*
	* @returns { Result<boolean, ReturnTypes.LangError> }
	*/
	"getIsLocked" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<boolean, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getIsLocked", [], __options, (result) => { return handleReturnType(result, getTypeDescription(17, 'artzero_staking_nft')); });
	}

	/**
	* getAdminAddress
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getAdminAddress" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAdminAddress", [], __options, (result) => { return handleReturnType(result, getTypeDescription(18, 'artzero_staking_nft')); });
	}

	/**
	* getRequestUnstakeTime
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } tokenId,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getRequestUnstakeTime" (
		account: ArgumentTypes.AccountId,
		tokenId: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getRequestUnstakeTime", [account, tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
	}

	/**
	* getStakedId
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } index,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getStakedId" (
		account: ArgumentTypes.AccountId,
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakedId", [account, index], __options, (result) => { return handleReturnType(result, getTypeDescription(20, 'artzero_staking_nft')); });
	}

	/**
	* getStakedAccountsIndexByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<ReturnNumber | null, ReturnTypes.LangError> }
	*/
	"getStakedAccountsIndexByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakedAccountsIndexByAccount", [account], __options, (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_staking_nft')); });
	}

	/**
	* getStakedAccountsAccountByIndex
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getStakedAccountsAccountByIndex" (
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakedAccountsAccountByIndex", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(24, 'artzero_staking_nft')); });
	}

	/**
	* getStakedAccountsLastIndex
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getStakedAccountsLastIndex" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakedAccountsLastIndex", [], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
	}

	/**
	* getPendingUnstakedId
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } index,
	* @returns { Result<number | null, ReturnTypes.LangError> }
	*/
	"getPendingUnstakedId" (
		account: ArgumentTypes.AccountId,
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPendingUnstakedId", [account, index], __options, (result) => { return handleReturnType(result, getTypeDescription(20, 'artzero_staking_nft')); });
	}

	/**
	* getTotalStaked
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getTotalStaked" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalStaked", [], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
	}

	/**
	* getLimitUnstakeTime
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getLimitUnstakeTime" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getLimitUnstakeTime", [], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
	}

	/**
	* stake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	* @returns { void }
	*/
	"stake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "stake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* requestUnstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	* @returns { void }
	*/
	"requestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "requestUnstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* cancelRequestUnstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	* @returns { void }
	*/
	"cancelRequestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "cancelRequestUnstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* unstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	* @returns { void }
	*/
	"unstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "unstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options, (result) => { return handleReturnType(result, getTypeDescription(27, 'artzero_staking_nft')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options, (result) => { return handleReturnType(result, getTypeDescription(17, 'artzero_staking_nft')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(18, 'artzero_staking_nft')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getTotalPendingUnstakedByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getTotalPendingUnstakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroStakingTrait::getTotalPendingUnstakedByAccount", [account], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
	}

	/**
	* getTotalStakedByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getTotalStakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "artZeroStakingTrait::getTotalStakedByAccount", [account], __options, (result) => { return handleReturnType(result, getTypeDescription(19, 'artzero_staking_nft')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [codeHash], __options);
	}

}