/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { GasLimit, GasLimitAndRequiredValue } from '@727-ventures/typechain-types';
import { buildSubmittableExtrinsic } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_marketplace_psp34';
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
	 * @param { ArgumentTypes.AccountId } collectionContractAddress,
	 * @param { ArgumentTypes.AccountId } stakingContractAddress,
	 * @param { (number | string | BN) } platformFee,
	*/
	"initialize" (
		collectionContractAddress: ArgumentTypes.AccountId,
		stakingContractAddress: ArgumentTypes.AccountId,
		platformFee: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "initialize", [collectionContractAddress, stakingContractAddress, platformFee], __options);
	}

	/**
	 * list
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (string | number | BN) } price,
	*/
	"list" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		price: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "list", [nftContractAddress, tokenId, price], __options);
	}

	/**
	 * unlist
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"unlist" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "unlist", [nftContractAddress, tokenId], __options);
	}

	/**
	 * buy
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"buy" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "buy", [nftContractAddress, tokenId], __options);
	}

	/**
	 * bid
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"bid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimitAndRequiredValue,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "bid", [nftContractAddress, tokenId], __options);
	}

	/**
	 * removeBid
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"removeBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "removeBid", [nftContractAddress, tokenId], __options);
	}

	/**
	 * acceptBid
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	 * @param { (number | string | BN) } bidIndex,
	*/
	"acceptBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		bidIndex: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "acceptBid", [nftContractAddress, tokenId, bidIndex], __options);
	}

	/**
	 * setCollectionContractAddress
	 *
	 * @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"setCollectionContractAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setCollectionContractAddress", [collectionContractAddress], __options);
	}

	/**
	 * setPlatformFee
	 *
	 * @param { (number | string | BN) } platformFee,
	*/
	"setPlatformFee" (
		platformFee: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setPlatformFee", [platformFee], __options);
	}

	/**
	 * setStakingContractAddress
	 *
	 * @param { ArgumentTypes.AccountId } stakingContractAddress,
	*/
	"setStakingContractAddress" (
		stakingContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setStakingContractAddress", [stakingContractAddress], __options);
	}

	/**
	 * setDiscount
	 *
	 * @param { Array<(number | string | BN)> } criteria,
	 * @param { Array<(number | string | BN)> } rates,
	*/
	"setDiscount" (
		criteria: Array<(number | string | BN)>,
		rates: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "setDiscount", [criteria, rates], __options);
	}

	/**
	 * receiveHoldAmount
	 *
	 * @param { ArgumentTypes.AccountId } receiver,
	*/
	"receiveHoldAmount" (
		receiver: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "receiveHoldAmount", [receiver], __options);
	}

	/**
	 * getNftSaleInfo
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"getNftSaleInfo" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getNftSaleInfo", [nftContractAddress, tokenId], __options);
	}

	/**
	 * getPlatformFee
	 *
	*/
	"getPlatformFee" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getPlatformFee", [], __options);
	}

	/**
	 * getStakingDiscountCriteria
	 *
	*/
	"getStakingDiscountCriteria" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakingDiscountCriteria", [], __options);
	}

	/**
	 * getStakingDiscountRate
	 *
	*/
	"getStakingDiscountRate" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakingDiscountRate", [], __options);
	}

	/**
	 * getListedTokenCountByCollectionAddress
	 *
	 * @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"getListedTokenCountByCollectionAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getListedTokenCountByCollectionAddress", [collectionContractAddress], __options);
	}

	/**
	 * getForSaleTokenId
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.AccountId } userAccount,
	 * @param { (string | number | BN) } index,
	*/
	"getForSaleTokenId" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getForSaleTokenId", [nftContractAddress, userAccount, index], __options);
	}

	/**
	 * getSaleTokensIdsCount
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.AccountId } userAccount,
	*/
	"getSaleTokensIdsCount" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getSaleTokensIdsCount", [nftContractAddress, userAccount], __options);
	}

	/**
	 * totalTokensForSale
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.AccountId } userAccount,
	*/
	"totalTokensForSale" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "totalTokensForSale", [nftContractAddress, userAccount], __options);
	}

	/**
	 * getAllBids
	 *
	 * @param { ArgumentTypes.AccountId } nftContractAddress,
	 * @param { ArgumentTypes.AccountId } userAccount,
	 * @param { ArgumentTypes.Id } tokenId,
	*/
	"getAllBids" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getAllBids", [nftContractAddress, userAccount, tokenId], __options);
	}

	/**
	 * getCollectionContractAddress
	 *
	*/
	"getCollectionContractAddress" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCollectionContractAddress", [], __options);
	}

	/**
	 * getStakingContractAddress
	 *
	*/
	"getStakingContractAddress" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getStakingContractAddress", [], __options);
	}

	/**
	 * getTotalVolume
	 *
	*/
	"getTotalVolume" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getTotalVolume", [], __options);
	}

	/**
	 * getVolumeByCollection
	 *
	 * @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"getVolumeByCollection" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getVolumeByCollection", [collectionContractAddress], __options);
	}

	/**
	 * getTotalProfit
	 *
	*/
	"getTotalProfit" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getTotalProfit", [], __options);
	}

	/**
	 * getCurrentProfit
	 *
	*/
	"getCurrentProfit" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getCurrentProfit", [], __options);
	}

	/**
	 * getHoldAmountOfBidder
	 *
	 * @param { ArgumentTypes.AccountId } bidder,
	*/
	"getHoldAmountOfBidder" (
		bidder: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getHoldAmountOfBidder", [bidder], __options);
	}

	/**
	 * getHoldBiddersByIndex
	 *
	 * @param { (number | string | BN) } index,
	*/
	"getHoldBiddersByIndex" (
		index: (number | string | BN),
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getHoldBiddersByIndex", [index], __options);
	}

	/**
	 * getHoldBidderCount
	 *
	*/
	"getHoldBidderCount" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "getHoldBidderCount", [], __options);
	}

	/**
	 * withdrawProfit
	 *
	 * @param { (string | number | BN) } value,
	 * @param { ArgumentTypes.AccountId } reciever,
	*/
	"withdrawProfit" (
		value: (string | number | BN),
		reciever: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "withdrawProfit", [value, reciever], __options);
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
	 * renounceOwnership
	 *
	*/
	"renounceOwnership" (
		__options: GasLimit,
	){
		return buildSubmittableExtrinsic( this.__apiPromise, this.__nativeContract, "ownable::renounceOwnership", [], __options);
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