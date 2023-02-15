/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_staking_nft';
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
	* @param { ArgumentTypes.AccountId } artzeroNftContract,
	* @param { (number | string | BN) } limitUnstakeTime,
	* @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"initialize" (
		artzeroNftContract: ArgumentTypes.AccountId,
		limitUnstakeTime: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [artzeroNftContract, limitUnstakeTime, adminAddress], __options);
	}

	/**
	* setArtzeroNftContract
	*
	* @param { ArgumentTypes.AccountId } artzeroNftContract,
	*/
	"setArtzeroNftContract" (
		artzeroNftContract: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setArtzeroNftContract", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [artzeroNftContract], __options);
	}

	/**
	* setLimitUnstakeTime
	*
	* @param { (number | string | BN) } limitUnstakeTime,
	*/
	"setLimitUnstakeTime" (
		limitUnstakeTime: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setLimitUnstakeTime", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [limitUnstakeTime], __options);
	}

	/**
	* updateAdminAddress
	*
	* @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"updateAdminAddress" (
		adminAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateAdminAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [adminAddress], __options);
	}

	/**
	* updateIsLocked
	*
	* @param { boolean } isLocked,
	*/
	"updateIsLocked" (
		isLocked: boolean,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsLocked", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [isLocked], __options);
	}

	/**
	* startRewardDistribution
	*
	*/
	"startRewardDistribution" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "startRewardDistribution", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* stopRewardDistribution
	*
	*/
	"stopRewardDistribution" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "stopRewardDistribution", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* addReward
	*
	*/
	"addReward" (
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addReward", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* setClaimedStatus
	*
	* @param { ArgumentTypes.AccountId } staker,
	*/
	"setClaimedStatus" (
		staker: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setClaimedStatus", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [staker], __options);
	}

	/**
	* claimReward
	*
	*/
	"claimReward" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "claimReward", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getRewardPool
	*
	*/
	"getRewardPool" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getRewardPool", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getClaimableReward
	*
	*/
	"getClaimableReward" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getClaimableReward", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* isClaimed
	*
	* @param { ArgumentTypes.AccountId } account,
	*/
	"isClaimed" (
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "isClaimed", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account], __options);
	}

	/**
	* getRewardStarted
	*
	*/
	"getRewardStarted" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getRewardStarted", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getArtzeroNftContract
	*
	*/
	"getArtzeroNftContract" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getArtzeroNftContract", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getIsLocked
	*
	*/
	"getIsLocked" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getIsLocked", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getAdminAddress
	*
	*/
	"getAdminAddress" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAdminAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getRequestUnstakeTime
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } tokenId,
	*/
	"getRequestUnstakeTime" (
		account: ArgumentTypes.AccountId,
		tokenId: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getRequestUnstakeTime", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account, tokenId], __options);
	}

	/**
	* getStakedId
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } index,
	*/
	"getStakedId" (
		account: ArgumentTypes.AccountId,
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakedId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account, index], __options);
	}

	/**
	* getStakedAccountsIndexByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	*/
	"getStakedAccountsIndexByAccount" (
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakedAccountsIndexByAccount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account], __options);
	}

	/**
	* getStakedAccountsAccountByIndex
	*
	* @param { (number | string | BN) } index,
	*/
	"getStakedAccountsAccountByIndex" (
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakedAccountsAccountByIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [index], __options);
	}

	/**
	* getStakedAccountsLastIndex
	*
	*/
	"getStakedAccountsLastIndex" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakedAccountsLastIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getPendingUnstakedId
	*
	* @param { ArgumentTypes.AccountId } account,
	* @param { (number | string | BN) } index,
	*/
	"getPendingUnstakedId" (
		account: ArgumentTypes.AccountId,
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPendingUnstakedId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account, index], __options);
	}

	/**
	* getTotalStaked
	*
	*/
	"getTotalStaked" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getTotalStaked", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getLimitUnstakeTime
	*
	*/
	"getLimitUnstakeTime" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getLimitUnstakeTime", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* stake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	*/
	"stake" (
		tokenIds: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "stake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* requestUnstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	*/
	"requestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "requestUnstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* cancelRequestUnstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	*/
	"cancelRequestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "cancelRequestUnstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
	}

	/**
	* unstake
	*
	* @param { Array<(number | string | BN)> } tokenIds,
	*/
	"unstake" (
		tokenIds: Array<(number | string | BN)>,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "unstake", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [tokenIds], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [role], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [role, account], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [], __options);
	}

	/**
	* getTotalPendingUnstakedByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	*/
	"getTotalPendingUnstakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroStakingTrait::getTotalPendingUnstakedByAccount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account], __options);
	}

	/**
	* getTotalStakedByAccount
	*
	* @param { ArgumentTypes.AccountId } account,
	*/
	"getTotalStakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "artZeroStakingTrait::getTotalStakedByAccount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [account], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [psp22ContractAddress, amount, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_staking_nft");
		}, [codeHash], __options);
	}

}