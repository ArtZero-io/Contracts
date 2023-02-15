/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_staking_nft';
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
	 * @param { ArgumentTypes.AccountId } artzeroNftContract,
	 * @param { (number | string | BN) } limitUnstakeTime,
	 * @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"initialize" (
		artzeroNftContract: ArgumentTypes.AccountId,
		limitUnstakeTime: (number | string | BN),
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "initialize", [artzeroNftContract, limitUnstakeTime, adminAddress], __options);
	}

	/**
	 * setArtzeroNftContract
	 *
	 * @param { ArgumentTypes.AccountId } artzeroNftContract,
	*/
	"setArtzeroNftContract" (
		artzeroNftContract: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setArtzeroNftContract", [artzeroNftContract], __options);
	}

	/**
	 * setLimitUnstakeTime
	 *
	 * @param { (number | string | BN) } limitUnstakeTime,
	*/
	"setLimitUnstakeTime" (
		limitUnstakeTime: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setLimitUnstakeTime", [limitUnstakeTime], __options);
	}

	/**
	 * updateAdminAddress
	 *
	 * @param { ArgumentTypes.AccountId } adminAddress,
	*/
	"updateAdminAddress" (
		adminAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateAdminAddress", [adminAddress], __options);
	}

	/**
	 * updateIsLocked
	 *
	 * @param { boolean } isLocked,
	*/
	"updateIsLocked" (
		isLocked: boolean,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "updateIsLocked", [isLocked], __options);
	}

	/**
	 * startRewardDistribution
	 *
	*/
	"startRewardDistribution" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "startRewardDistribution", [], __options);
	}

	/**
	 * stopRewardDistribution
	 *
	*/
	"stopRewardDistribution" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "stopRewardDistribution", [], __options);
	}

	/**
	 * addReward
	 *
	*/
	"addReward" (
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "addReward", [], __options);
	}

	/**
	 * setClaimedStatus
	 *
	 * @param { ArgumentTypes.AccountId } staker,
	*/
	"setClaimedStatus" (
		staker: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setClaimedStatus", [staker], __options);
	}

	/**
	 * claimReward
	 *
	*/
	"claimReward" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "claimReward", [], __options);
	}

	/**
	 * getRewardPool
	 *
	*/
	"getRewardPool" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getRewardPool", [], __options);
	}

	/**
	 * getClaimableReward
	 *
	*/
	"getClaimableReward" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getClaimableReward", [], __options);
	}

	/**
	 * isClaimed
	 *
	 * @param { ArgumentTypes.AccountId } account,
	*/
	"isClaimed" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "isClaimed", [account], __options);
	}

	/**
	 * getRewardStarted
	 *
	*/
	"getRewardStarted" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getRewardStarted", [], __options);
	}

	/**
	 * getArtzeroNftContract
	 *
	*/
	"getArtzeroNftContract" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getArtzeroNftContract", [], __options);
	}

	/**
	 * getIsLocked
	 *
	*/
	"getIsLocked" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getIsLocked", [], __options);
	}

	/**
	 * getAdminAddress
	 *
	*/
	"getAdminAddress" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAdminAddress", [], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getRequestUnstakeTime", [account, tokenId], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakedId", [account, index], __options);
	}

	/**
	 * getStakedAccountsIndexByAccount
	 *
	 * @param { ArgumentTypes.AccountId } account,
	*/
	"getStakedAccountsIndexByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakedAccountsIndexByAccount", [account], __options);
	}

	/**
	 * getStakedAccountsAccountByIndex
	 *
	 * @param { (number | string | BN) } index,
	*/
	"getStakedAccountsAccountByIndex" (
		index: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakedAccountsAccountByIndex", [index], __options);
	}

	/**
	 * getStakedAccountsLastIndex
	 *
	*/
	"getStakedAccountsLastIndex" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakedAccountsLastIndex", [], __options);
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
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPendingUnstakedId", [account, index], __options);
	}

	/**
	 * getTotalStaked
	 *
	*/
	"getTotalStaked" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getTotalStaked", [], __options);
	}

	/**
	 * getLimitUnstakeTime
	 *
	*/
	"getLimitUnstakeTime" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getLimitUnstakeTime", [], __options);
	}

	/**
	 * stake
	 *
	 * @param { Array<(number | string | BN)> } tokenIds,
	*/
	"stake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "stake", [tokenIds], __options);
	}

	/**
	 * requestUnstake
	 *
	 * @param { Array<(number | string | BN)> } tokenIds,
	*/
	"requestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "requestUnstake", [tokenIds], __options);
	}

	/**
	 * cancelRequestUnstake
	 *
	 * @param { Array<(number | string | BN)> } tokenIds,
	*/
	"cancelRequestUnstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "cancelRequestUnstake", [tokenIds], __options);
	}

	/**
	 * unstake
	 *
	 * @param { Array<(number | string | BN)> } tokenIds,
	*/
	"unstake" (
		tokenIds: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "unstake", [tokenIds], __options);
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
	 * renounceOwnership
	 *
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * getTotalPendingUnstakedByAccount
	 *
	 * @param { ArgumentTypes.AccountId } account,
	*/
	"getTotalPendingUnstakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroStakingTrait::getTotalPendingUnstakedByAccount", [account], __options);
	}

	/**
	 * getTotalStakedByAccount
	 *
	 * @param { ArgumentTypes.AccountId } account,
	*/
	"getTotalStakedByAccount" (
		account: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "artZeroStakingTrait::getTotalStakedByAccount", [account], __options);
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