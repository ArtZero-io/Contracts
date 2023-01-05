/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ArgumentsTypes } from '../arguments/psp34_nft';
import type OkishReturns from '../return-values/psp34_nft';
import type { GasLimit, GasLimitAndRequiredValue } from '../_sdk/types';
import type { QueryReturnType } from '../_sdk/query';
import { queryJSON, queryOkJSON } from '../_sdk/query';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
	}

	/** */
	"mint" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "mint", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: metadata,
	 * ]
	 */
	"mint_with_attributes" (
		metadata: ArgumentsTypes[38],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "mintWithAttributes", [metadata], __options);
	}

	/** */
	"Ownable::owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["8"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options);
	}

	/** */
	"Ownable::renounce_ownership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["40"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: newOwner,
	 * ]
	 */
	"Ownable::transfer_ownership" (
		newOwner: ArgumentsTypes[8],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["40"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options);
	}

	/** */
	"PSP34::collection_id" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["1"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::collectionId", [], __options);
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
		approved: ArgumentsTypes[30],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["41"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34::approve", [operator, id, approved], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["41"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34::transfer", [to, id, data], __options);
	}

	/** */
	"PSP34::total_supply" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["6"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::totalSupply", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: owner,
	 * ]
	 */
	"PSP34::balance_of" (
		owner: ArgumentsTypes[8],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["4"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::balanceOf", [owner], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["30"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::allowance", [owner, operator, id], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * ]
	 */
	"PSP34::owner_of" (
		id: ArgumentsTypes[1],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["19"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34::ownerOf", [id], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["43"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Metadata::getAttribute", [id, key], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["44"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::ownersTokenByIndex", [owner, index], __options);
	}

	/**
	 * @arg: args: [
	 * 0: index,
	 * ]
	 */
	"PSP34Enumerable::token_by_index" (
		index: ArgumentsTypes[6],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["44"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Enumerable::tokenByIndex", [index], __options);
	}

	/**
	 * @arg: args: [
	 * 0: uri,
	 * ]
	 */
	"Psp34Traits::set_base_uri" (
		uri: ArgumentsTypes[33],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::setBaseUri", [uri], __options);
	}

	/** */
	"Psp34Traits::get_attribute_count" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["4"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeCount", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: index,
	 * ]
	 */
	"Psp34Traits::get_attribute_name" (
		index: ArgumentsTypes[4],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["33"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributeName", [index], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::token_uri" (
		tokenId: ArgumentsTypes[5],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["33"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::tokenUri", [tokenId], __options);
	}

	/** */
	"Psp34Traits::get_last_token_id" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["5"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getLastTokenId", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * 1: metadata,
	 * ]
	 */
	"Psp34Traits::set_multiple_attributes" (
		tokenId: ArgumentsTypes[1],
		metadata: ArgumentsTypes[38],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::setMultipleAttributes", [tokenId, metadata], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::is_locked_nft" (
		tokenId: ArgumentsTypes[1],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["30"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::isLockedNft", [tokenId], __options);
	}

	/**
	 * @arg: args: [
	 * 0: id,
	 * ]
	 */
	"Psp34Traits::burn" (
		id: ArgumentsTypes[1],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::burn", [id], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * 1: attributes,
	 * ]
	 */
	"Psp34Traits::get_attributes" (
		tokenId: ArgumentsTypes[1],
		attributes: ArgumentsTypes[45],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["45"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getAttributes", [tokenId, attributes], __options);
	}

	/** */
	"Psp34Traits::get_locked_token_count" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["5"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getLockedTokenCount", [], __options);
	}

	/** */
	"Psp34Traits::get_owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["8"] > >{
		return queryJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::getOwner", [], __options);
	}

	/**
	 * @arg: args: [
	 * 0: tokenId,
	 * ]
	 */
	"Psp34Traits::lock" (
		tokenId: ArgumentsTypes[1],
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "psp34Traits::lock", [tokenId], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "adminTrait::withdrawFee", [value, receiver], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options);
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< OkishReturns["34"] > >{
		return queryOkJSON( this.__nativeContract, this.__callerAddress, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options);
	}

}