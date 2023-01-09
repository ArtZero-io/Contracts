/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ArgumentsTypes } from '../arguments/launchpad_psp34_nft_standard';
import type { GasLimit, GasLimitAndRequiredValue } from '../_sdk/types';
import { buildSubmittableExtrinsic } from '../_sdk/tx';


export default class Methods {
	private __nativeContract : ContractPromise;

	constructor(
		nativeContract : ContractPromise,
	) {
		this.__nativeContract = nativeContract;
	}
	/**
	 * @arg: args: [
	 * 0: phaseCode,
	 * 1: isPublic,
	 * 2: publicMintingFee,
	 * 3: publicMintingAmount,
	 * 4: publicMaxMintingAmount,
	 * 5: startTime,
	 * 6: endTime,
	 * ]
	 */
	"add_new_phase" (
		phaseCode: ArgumentsTypes[55],
		isPublic: ArgumentsTypes[43],
		publicMintingFee: ArgumentsTypes[6],
		publicMintingAmount: ArgumentsTypes[5],
		publicMaxMintingAmount: ArgumentsTypes[5],
		startTime: ArgumentsTypes[5],
		endTime: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "addNewPhase", [phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
	}

	/**
	 * @arg: args: [
	 * 0: account,
	 * 1: phaseId,
	 * 2: whitelistAmount,
	 * 3: whitelistPrice,
	 * ]
	 */
	"update_whitelist" (
		account: ArgumentsTypes[8],
		phaseId: ArgumentsTypes[2],
		whitelistAmount: ArgumentsTypes[5],
		whitelistPrice: ArgumentsTypes[6],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	 * @arg: args: [
	 * 0: account,
	 * 1: phaseId,
	 * 2: whitelistAmount,
	 * 3: whitelistPrice,
	 * ]
	 */
	"add_whitelist" (
		account: ArgumentsTypes[8],
		phaseId: ArgumentsTypes[2],
		whitelistAmount: ArgumentsTypes[5],
		whitelistPrice: ArgumentsTypes[6],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "addWhitelist", [account, phaseId, whitelistAmount, whitelistPrice], __options);
	}

	/**
	 * @arg: args: [
	 * 0: mintAmount,
	 * ]
	 */
	"mint" (
		mintAmount: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "mint", [mintAmount], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * 1: mintAmount,
	 * ]
	 */
	"public_mint" (
		phaseId: ArgumentsTypes[2],
		mintAmount: ArgumentsTypes[5],
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "publicMint", [phaseId, mintAmount], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * 1: mintAmount,
	 * ]
	 */
	"whitelist_mint" (
		phaseId: ArgumentsTypes[2],
		mintAmount: ArgumentsTypes[5],
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "whitelistMint", [phaseId, mintAmount], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * ]
	 */
	"deactive_phase" (
		phaseId: ArgumentsTypes[2],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "deactivePhase", [phaseId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * 1: phaseCode,
	 * 2: isPublic,
	 * 3: publicMintingFee,
	 * 4: publicMintingAmount,
	 * 5: publicMaxMintingAmount,
	 * 6: startTime,
	 * 7: endTime,
	 * ]
	 */
	"update_schedule_phase" (
		phaseId: ArgumentsTypes[2],
		phaseCode: ArgumentsTypes[55],
		isPublic: ArgumentsTypes[43],
		publicMintingFee: ArgumentsTypes[6],
		publicMintingAmount: ArgumentsTypes[5],
		publicMaxMintingAmount: ArgumentsTypes[5],
		startTime: ArgumentsTypes[5],
		endTime: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateSchedulePhase", [phaseId, phaseCode, isPublic, publicMintingFee, publicMintingAmount, publicMaxMintingAmount, startTime, endTime], __options);
	}

	/**
	 * @arg: args: [
	 * 0: idPhases,
	 * 1: codePhases,
	 * 2: isPublicPhases,
	 * 3: publicMintingFeePhases,
	 * 4: publicMintingAmountPhases,
	 * 5: publicMaxMintingAmountPhases,
	 * 6: startTimePhases,
	 * 7: endTimePhases,
	 * ]
	 */
	"update_schedule_phases" (
		idPhases: ArgumentsTypes[7],
		codePhases: ArgumentsTypes[56],
		isPublicPhases: ArgumentsTypes[57],
		publicMintingFeePhases: ArgumentsTypes[58],
		publicMintingAmountPhases: ArgumentsTypes[59],
		publicMaxMintingAmountPhases: ArgumentsTypes[59],
		startTimePhases: ArgumentsTypes[59],
		endTimePhases: ArgumentsTypes[59],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateSchedulePhases", [idPhases, codePhases, isPublicPhases, publicMintingFeePhases, publicMintingAmountPhases, publicMaxMintingAmountPhases, startTimePhases, endTimePhases], __options);
	}

	/**
	 * @arg: args: [
	 * 0: projectInfo,
	 * ]
	 */
	"edit_project_information" (
		projectInfo: ArgumentsTypes[55],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "editProjectInformation", [projectInfo], __options);
	}

	/** */
	"get_owner_claimed_amount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getOwnerClaimedAmount", [], __options);
	}

	/** */
	"get_owner_available_amount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getOwnerAvailableAmount", [], __options);
	}

	/** */
	"get_limit_phase_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getLimitPhaseCount", [], __options);
	}

	/** */
	"get_public_minted_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getPublicMintedCount", [], __options);
	}

	/** */
	"get_project_info" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getProjectInfo", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * ]
	 */
	"get_phase_schedule_by_id" (
		phaseId: ArgumentsTypes[2],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getPhaseScheduleById", [phaseId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: account,
	 * 1: phaseId,
	 * ]
	 */
	"get_whitelist_by_account_id" (
		account: ArgumentsTypes[8],
		phaseId: ArgumentsTypes[2],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getWhitelistByAccountId", [account, phaseId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * 1: accountIndex,
	 * ]
	 */
	"get_phase_account_link" (
		phaseId: ArgumentsTypes[2],
		accountIndex: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getPhaseAccountLink", [phaseId, accountIndex], __options);
	}

	/** */
	"get_current_phase" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCurrentPhase", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: time,
	 * ]
	 */
	"is_in_schedule_phase" (
		time: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "isInSchedulePhase", [time], __options);
	}

	/** */
	"get_whitelist_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getWhitelistCount", [], __options);
	}

	/** */
	"get_last_phase_id" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getLastPhaseId", [], __options);
	}

	/** */
	"get_active_phase_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getActivePhaseCount", [], __options);
	}

	/** */
	"get_last_token_id" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getLastTokenId", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: accountId,
	 * 1: phaseId,
	 * ]
	 */
	"get_phase_account_public_claimed_amount" (
		accountId: ArgumentsTypes[8],
		phaseId: ArgumentsTypes[2],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getPhaseAccountPublicClaimedAmount", [accountId, phaseId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: phaseId,
	 * ]
	 */
	"get_phase_account_last_index" (
		phaseId: ArgumentsTypes[2],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getPhaseAccountLastIndex", [phaseId], __options);
	}

	/** */
	"get_total_supply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getTotalSupply", [], __options);
	}

	/** */
	"get_available_token_amount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getAvailableTokenAmount", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: newOwner,
	 * ]
	 */
	"Ownable::transfer_ownership" (
		newOwner: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::transferOwnership", [newOwner], __options);
	}

	/** */
	"Ownable::renounce_ownership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/** */
	"Ownable::owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::owner", [], __options);
	}

	/** */
	"PSP34::total_supply" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::totalSupply", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: operator,
	 * 1: id,
	 * 2: approved,
	 * ]
	 */
	"PSP34::approve" (
		operator: ArgumentsTypes[8],
		id: ArgumentsTypes[14],
		approved: ArgumentsTypes[43],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::approve", [operator, id, approved], __options);
	}

	/**
	 * @arg: args: [
	 * 0: to,
	 * 1: id,
	 * 2: data,
	 * ]
	 */
	"PSP34::transfer" (
		to: ArgumentsTypes[8],
		id: ArgumentsTypes[1],
		data: ArgumentsTypes[7],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::transfer", [to, id, data], __options);
	}

	/** */
	"PSP34::collection_id" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::collectionId", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: owner,
	 * ]
	 */
	"PSP34::balance_of" (
		owner: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::balanceOf", [owner], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * ]
	 */
	"PSP34::owner_of" (
		id: ArgumentsTypes[1],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::ownerOf", [id], __options);
	}

	/**
	 * @arg: args: [
	 * 0: owner,
	 * 1: operator,
	 * 2: id,
	 * ]
	 */
	"PSP34::allowance" (
		owner: ArgumentsTypes[8],
		operator: ArgumentsTypes[8],
		id: ArgumentsTypes[14],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * 1: key,
	 * ]
	 */
	"PSP34Metadata::get_attribute" (
		id: ArgumentsTypes[1],
		key: ArgumentsTypes[7],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Metadata::getAttribute", [id, key], __options);
	}

	/**
	 * @arg: args: [
	 * 0: owner,
	 * 1: index,
	 * ]
	 */
	"PSP34Enumerable::owners_token_by_index" (
		owner: ArgumentsTypes[8],
		index: ArgumentsTypes[6],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options);
	}

	/**
	 * @arg: args: [
	 * 0: index,
	 * ]
	 */
	"PSP34Enumerable::token_by_index" (
		index: ArgumentsTypes[6],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Enumerable::tokenByIndex", [index], __options);
	}

	/**
	 * @arg: args: [
	 * 0: uri,
	 * ]
	 */
	"Psp34Traits::set_base_uri" (
		uri: ArgumentsTypes[55],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::setBaseUri", [uri], __options);
	}

	/** */
	"Psp34Traits::get_locked_token_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getLockedTokenCount", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: index,
	 * ]
	 */
	"Psp34Traits::get_attribute_name" (
		index: ArgumentsTypes[4],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getAttributeName", [index], __options);
	}

	/** */
	"Psp34Traits::get_owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getOwner", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * 1: attributes,
	 * ]
	 */
	"Psp34Traits::get_attributes" (
		tokenId: ArgumentsTypes[1],
		attributes: ArgumentsTypes[56],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getAttributes", [tokenId, attributes], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * 1: metadata,
	 * ]
	 */
	"Psp34Traits::set_multiple_attributes" (
		tokenId: ArgumentsTypes[1],
		metadata: ArgumentsTypes[74],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::setMultipleAttributes", [tokenId, metadata], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::token_uri" (
		tokenId: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::tokenUri", [tokenId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::is_locked_nft" (
		tokenId: ArgumentsTypes[1],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::isLockedNft", [tokenId], __options);
	}

	/** */
	"Psp34Traits::get_attribute_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getAttributeCount", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::lock" (
		tokenId: ArgumentsTypes[1],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::lock", [tokenId], __options);
	}

	/** */
	"Psp34Traits::get_last_token_id" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Traits::getLastTokenId", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::grant_role" (
		role: ArgumentsTypes[4],
		account: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::grantRole", [role, account], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: address,
	 * ]
	 */
	"AccessControl::has_role" (
		role: ArgumentsTypes[4],
		address: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::hasRole", [role, address], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * ]
	 */
	"AccessControl::get_role_admin" (
		role: ArgumentsTypes[4],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::getRoleAdmin", [role], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::revoke_role" (
		role: ArgumentsTypes[4],
		account: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::revokeRole", [role, account], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * 1: account,
	 * ]
	 */
	"AccessControl::renounce_role" (
		role: ArgumentsTypes[4],
		account: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::renounceRole", [role, account], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * 1: tokenId,
	 * 2: receiver,
	 * ]
	 */
	"AdminTrait::tranfer_nft" (
		nftContractAddress: ArgumentsTypes[8],
		tokenId: ArgumentsTypes[1],
		receiver: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options);
	}

	/**
	 * @arg: args: [
	 * 0: value,
	 * 1: receiver,
	 * ]
	 */
	"AdminTrait::withdraw_fee" (
		value: ArgumentsTypes[6],
		receiver: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::withdrawFee", [value, receiver], __options);
	}

	/**
	 * @arg: args: [
	 * 0: psp22ContractAddress,
	 * 1: amount,
	 * 2: receiver,
	 * ]
	 */
	"AdminTrait::tranfer_psp22" (
		psp22ContractAddress: ArgumentsTypes[8],
		amount: ArgumentsTypes[6],
		receiver: ArgumentsTypes[8],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options);
	}

	/**
	 * @arg: args: [
	 * 0: account,
	 * 1: id,
	 * ]
	 */
	"PSP34Burnable::burn" (
		account: ArgumentsTypes[8],
		id: ArgumentsTypes[1],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "psp34Burnable::burn", [account, id], __options);
	}

}