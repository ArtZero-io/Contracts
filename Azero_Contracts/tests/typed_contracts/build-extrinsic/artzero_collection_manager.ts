/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ArgumentsTypes } from '../arguments/artzero_collection_manager';
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
	 * 0: standardNftHash,
	 * 1: simpleModeAddingFee,
	 * 2: advanceModeAddingFee,
	 * 3: maxRoyaltyFeeRate,
	 * ]
	 */
	"initialize" (
		standardNftHash: ArgumentsTypes[12],
		simpleModeAddingFee: ArgumentsTypes[14],
		advanceModeAddingFee: ArgumentsTypes[14],
		maxRoyaltyFeeRate: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "initialize", [standardNftHash, simpleModeAddingFee, advanceModeAddingFee, maxRoyaltyFeeRate], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftName,
	 * 1: nftSymbol,
	 * 2: attributes,
	 * 3: attributeVals,
	 * 4: isCollectRoyaltyFee,
	 * 5: royaltyFee,
	 * ]
	 */
	"auto_new_collection" (
		nftName: ArgumentsTypes[31],
		nftSymbol: ArgumentsTypes[31],
		attributes: ArgumentsTypes[38],
		attributeVals: ArgumentsTypes[38],
		isCollectRoyaltyFee: ArgumentsTypes[18],
		royaltyFee: ArgumentsTypes[5],
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "autoNewCollection", [nftName, nftSymbol, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * 1: attributes,
	 * 2: attributeVals,
	 * 3: isCollectRoyaltyFee,
	 * 4: royaltyFee,
	 * ]
	 */
	"add_new_collection" (
		nftContractAddress: ArgumentsTypes[0],
		attributes: ArgumentsTypes[38],
		attributeVals: ArgumentsTypes[38],
		isCollectRoyaltyFee: ArgumentsTypes[18],
		royaltyFee: ArgumentsTypes[5],
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "addNewCollection", [nftContractAddress, attributes, attributeVals, isCollectRoyaltyFee, royaltyFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: newOwner,
	 * ]
	 */
	"update_collection_owner" (
		contractAddress: ArgumentsTypes[0],
		newOwner: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateCollectionOwner", [contractAddress, newOwner], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: nftContractAddress,
	 * ]
	 */
	"update_nft_contract_address" (
		contractAddress: ArgumentsTypes[0],
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateNftContractAddress", [contractAddress, nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: attributes,
	 * 2: values,
	 * ]
	 */
	"set_multiple_attributes" (
		contractAddress: ArgumentsTypes[0],
		attributes: ArgumentsTypes[38],
		values: ArgumentsTypes[38],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "setMultipleAttributes", [contractAddress, attributes, values], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: attributes,
	 * ]
	 */
	"get_attributes" (
		contractAddress: ArgumentsTypes[0],
		attributes: ArgumentsTypes[38],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getAttributes", [contractAddress, attributes], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: attributeKey,
	 * ]
	 */
	"get_attribute" (
		contractAddress: ArgumentsTypes[0],
		attributeKey: ArgumentsTypes[31],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getAttribute", [contractAddress, attributeKey], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: attributeKey,
	 * ]
	 */
	"has_attribute" (
		contractAddress: ArgumentsTypes[0],
		attributeKey: ArgumentsTypes[31],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "hasAttribute", [contractAddress, attributeKey], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: attributeKey,
	 * ]
	 */
	"get_collection_attribute_index" (
		contractAddress: ArgumentsTypes[0],
		attributeKey: ArgumentsTypes[31],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCollectionAttributeIndex", [contractAddress, attributeKey], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * ]
	 */
	"get_collection_attribute_count" (
		contractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCollectionAttributeCount", [contractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: contractType,
	 * ]
	 */
	"update_contract_type" (
		contractAddress: ArgumentsTypes[0],
		contractType: ArgumentsTypes[17],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateContractType", [contractAddress, contractType], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: isCollectRoyaltyFee,
	 * ]
	 */
	"update_is_collect_royalty_fee" (
		contractAddress: ArgumentsTypes[0],
		isCollectRoyaltyFee: ArgumentsTypes[18],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateIsCollectRoyaltyFee", [contractAddress, isCollectRoyaltyFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: newFee,
	 * ]
	 */
	"update_royalty_fee" (
		contractAddress: ArgumentsTypes[0],
		newFee: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateRoyaltyFee", [contractAddress, newFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: showOnChainMetadata,
	 * ]
	 */
	"update_show_on_chain_metadata" (
		contractAddress: ArgumentsTypes[0],
		showOnChainMetadata: ArgumentsTypes[18],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateShowOnChainMetadata", [contractAddress, showOnChainMetadata], __options);
	}

	/**
	 * @arg: args: [
	 * 0: contractAddress,
	 * 1: isActive,
	 * ]
	 */
	"update_is_active" (
		contractAddress: ArgumentsTypes[0],
		isActive: ArgumentsTypes[18],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateIsActive", [contractAddress, isActive], __options);
	}

	/**
	 * @arg: args: [
	 * 0: simpleModeAddingFee,
	 * ]
	 */
	"update_simple_mode_adding_fee" (
		simpleModeAddingFee: ArgumentsTypes[14],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateSimpleModeAddingFee", [simpleModeAddingFee], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "updateStandardNftHash", [standardNftHash], __options);
	}

	/**
	 * @arg: args: [
	 * 0: advanceModeAddingFee,
	 * ]
	 */
	"update_advance_mode_adding_fee" (
		advanceModeAddingFee: ArgumentsTypes[14],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateAdvanceModeAddingFee", [advanceModeAddingFee], __options);
	}

	/**
	 * @arg: args: [
	 * 0: maxRoyaltyFeeRate,
	 * ]
	 */
	"update_max_royalty_fee_rate" (
		maxRoyaltyFeeRate: ArgumentsTypes[5],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "updateMaxRoyaltyFeeRate", [maxRoyaltyFeeRate], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"get_collection_by_address" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCollectionByAddress", [nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: ownerAddress,
	 * ]
	 */
	"get_collections_by_owner" (
		ownerAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCollectionsByOwner", [ownerAddress], __options);
	}

	/** */
	"get_standard_nft_hash" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getStandardNftHash", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * ]
	 */
	"get_contract_by_id" (
		id: ArgumentsTypes[13],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getContractById", [id], __options);
	}

	/** */
	"get_collection_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getCollectionCount", [], __options);
	}

	/** */
	"get_active_collection_count" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getActiveCollectionCount", [], __options);
	}

	/** */
	"get_simple_mode_adding_fee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getSimpleModeAddingFee", [], __options);
	}

	/** */
	"get_advance_mode_adding_fee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getAdvanceModeAddingFee", [], __options);
	}

	/** */
	"get_max_royalty_fee_rate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "getMaxRoyaltyFeeRate", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: role,
	 * ]
	 */
	"AccessControl::get_role_admin" (
		role: ArgumentsTypes[5],
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
	"AccessControl::grant_role" (
		role: ArgumentsTypes[5],
		account: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::grantRole", [role, account], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::renounceRole", [role, account], __options);
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
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::hasRole", [role, address], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "accessControl::revokeRole", [role, account], __options);
	}

	/** */
	"Ownable::owner" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::owner", [], __options);
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
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::transferOwnership", [newOwner], __options);
	}

	/** */
	"Ownable::renounce_ownership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"ArtZeroCollectionTrait::get_collection_owner" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "artZeroCollectionTrait::getCollectionOwner", [nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"ArtZeroCollectionTrait::is_active" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "artZeroCollectionTrait::isActive", [nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"ArtZeroCollectionTrait::get_royalty_fee" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "artZeroCollectionTrait::getRoyaltyFee", [nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: nftContractAddress,
	 * ]
	 */
	"ArtZeroCollectionTrait::get_contract_type" (
		nftContractAddress: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "artZeroCollectionTrait::getContractType", [nftContractAddress], __options);
	}

	/**
	 * @arg: args: [
	 * 0: value,
	 * 1: receiver,
	 * ]
	 */
	"AdminTrait::withdraw_fee" (
		value: ArgumentsTypes[14],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::withdrawFee", [value, receiver], __options);
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
		tokenId: ArgumentsTypes[45],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options);
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
		amount: ArgumentsTypes[14],
		receiver: ArgumentsTypes[0],
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__nativeContract, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options);
	}

}