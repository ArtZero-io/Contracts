/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ArgumentsTypes } from '../arguments/artzero_launchpad_psp34';
import type OkishReturns from '../return-values/artzero_launchpad_psp34';
import type { GasLimit, GasLimitAndRequiredValue } from '../_sdk/types';
import type { QueryReturnType } from '../_sdk/query';
import { queryJSON } from '../_sdk/query';
import { txSignAndSend } from '../_sdk/tx';


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
	 * @arg: args: [
	 * 0: maxPhasesPerProject,
	 * 1: standardNftHash,
	 * 2: projectAddingFee,
	 * 3: projectMintFeeRate,
	 * 4: publicMaxMintingAmount,
	 * ]
	 */
	"initialize" (
		maxPhasesPerProject: ArgumentsTypes[2],
		standardNftHash: ArgumentsTypes[12],
		projectAddingFee: ArgumentsTypes[26],
		projectMintFeeRate: ArgumentsTypes[5],
		publicMaxMintingAmount: ArgumentsTypes[13],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", [maxPhasesPerProject, standardNftHash, projectAddingFee, projectMintFeeRate, publicMaxMintingAmount], __options);
	}

	/**
	 * @arg: args: [
	 * 0: totalSupply,
	 * 1: startTime,
	 * 2: endTime,
	 * 3: projectInfo,
	 * 4: codePhases,
	 * 5: isPublicPhases,
	 * 6: publicMintingFeePhases,
	 * 7: publicMintingAmountPhases,
	 * 8: publicMaxMintingAmountPhases,
	 * 9: startTimePhases,
	 * 10: endTimePhases,
	 * ]
	 */
	"add_new_project" (
		totalSupply: ArgumentsTypes[13],
		startTime: ArgumentsTypes[13],
		endTime: ArgumentsTypes[13],
		projectInfo: ArgumentsTypes[29],
		codePhases: ArgumentsTypes[32],
		isPublicPhases: ArgumentsTypes[33],
		publicMintingFeePhases: ArgumentsTypes[34],
		publicMintingAmountPhases: ArgumentsTypes[35],
		publicMaxMintingAmountPhases: ArgumentsTypes[35],
		startTimePhases: ArgumentsTypes[35],
		endTimePhases: ArgumentsTypes[35],
		__options: GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "addNewProject", [totalSupply, startTime, endTime, projectInfo, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: startTime,
	 * 2: endTime,
	 * ]
	 */
	"edit_project" (
		contractAddress: ArgumentsTypes[0],
		startTime: ArgumentsTypes[13],
		endTime: ArgumentsTypes[13],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "editProject", [contractAddress, startTime, endTime], __options);
	}

	/**
	 * @arg: args: [
	 * 0: projectAddingFee,
	 * ]
	 */
	"update_project_adding_fee" (
		projectAddingFee: ArgumentsTypes[26],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectAddingFee", [projectAddingFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: publicMaxMintingAmount,
	 * ]
	 */
	"update_public_max_minting_amount" (
		publicMaxMintingAmount: ArgumentsTypes[13],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updatePublicMaxMintingAmount", [publicMaxMintingAmount], __options);
	}

	/**
	 * @arg: args: [
	 * 0: projectMintFeeRate,
	 * ]
	 */
	"update_project_mint_fee_rate" (
		projectMintFeeRate: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateProjectMintFeeRate", [projectMintFeeRate], __options);
	}

	/**
	 * @arg: args: [
	 * 0: standardNftHash,
	 * ]
	 */
	"update_standard_nft_hash" (
		standardNftHash: ArgumentsTypes[12],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateStandardNftHash", [standardNftHash], __options);
	}

	/**
	 * @arg: args: [
	 * 0: isActive,
	 * 1: contractAddress,
	 * ]
	 */
	"update_is_active_project" (
		isActive: ArgumentsTypes[16],
		contractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "updateIsActiveProject", [isActive, contractAddress], __options);
	}

	/** */
	"get_project_adding_fee" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["26"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getProjectAddingFee", [], __options);
	}

	/** */
	"get_active_project_count" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["13"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getActiveProjectCount", [], __options);
	}

	/** */
	"get_project_count" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["13"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getProjectCount", [], __options);
	}

	/** */
	"get_standard_nft_hash" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["12"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getStandardNftHash", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * ]
	 */
	"get_project_by_id" (
		id: ArgumentsTypes[13],
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["37"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getProjectById", [id], __options);
	}

	/**
	 * @arg: args: [
	 * 0: ownerAddress,
	 * ]
	 */
	"get_projects_by_owner" (
		ownerAddress: ArgumentsTypes[0],
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["23"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getProjectsByOwner", [ownerAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"get_project_by_nft_address" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["38"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getProjectByNftAddress", [nftContractAddress], __options);
	}

	/** */
	"get_max_phases_per_project" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["2"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "getMaxPhasesPerProject", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: address,
	 * ]
	 */
	"AccessControl::has_role" (
		role: ArgumentsTypes[5],
		address: ArgumentsTypes[0],
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["16"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "accessControl::hasRole", [role, address], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * ]
	 */
	"AccessControl::get_role_admin" (
		role: ArgumentsTypes[5],
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["5"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "accessControl::getRoleAdmin", [role], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::grant_role" (
		role: ArgumentsTypes[5],
		account: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::grantRole", [role, account], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::revoke_role" (
		role: ArgumentsTypes[5],
		account: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::revokeRole", [role, account], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::renounce_role" (
		role: ArgumentsTypes[5],
		account: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "accessControl::renounceRole", [role, account], __options);
	}

	/** */
	"Ownable::owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["0"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: newOwner,
	 * ]
	 */
	"Ownable::transfer_ownership" (
		newOwner: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::transferOwnership", [newOwner], __options);
	}

	/** */
	"Ownable::renounce_ownership" (
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", [], __options);
	}

	/** */
	"ArtZeroLaunchPadTrait::get_project_mint_fee_rate" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["5"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getProjectMintFeeRate", [], __options);
	}

	/** */
	"ArtZeroLaunchPadTrait::get_public_max_minting_amount" (
		__options: GasLimit,
	): Promise< QueryReturnType< OkishReturns["13"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "artZeroLaunchPadTrait::getPublicMaxMintingAmount", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: value,
	 * 1: receiver,
	 * ]
	 */
	"AdminTrait::withdraw_fee" (
		value: ArgumentsTypes[26],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::withdrawFee", [value, receiver], __options);
	}

	/**
	 * @arg: args: [
	 * 0: psp22ContractAddress,
	 * 1: amount,
	 * 2: receiver,
	 * ]
	 */
	"AdminTrait::tranfer_psp22" (
		psp22ContractAddress: ArgumentsTypes[0],
		amount: ArgumentsTypes[26],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * 1: tokenId,
	 * 2: receiver,
	 * ]
	 */
	"AdminTrait::tranfer_nft" (
		nftContractAddress: ArgumentsTypes[0],
		tokenId: ArgumentsTypes[40],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options);
	}

}