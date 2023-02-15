/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_marketplace_psp34';
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
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	* @param { (number | string | BN) } platformFee,
	*/
	"initialize" (
		collectionContractAddress: ArgumentTypes.AccountId,
		stakingContractAddress: ArgumentTypes.AccountId,
		platformFee: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "initialize", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [collectionContractAddress, stakingContractAddress, platformFee], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "list", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId, price], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "unlist", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId], __options);
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
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "buy", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId], __options);
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
		__options ? : GasLimitAndRequiredValue,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "bid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "removeBid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "acceptBid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId, bidIndex], __options);
	}

	/**
	* setCollectionContractAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"setCollectionContractAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setCollectionContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [collectionContractAddress], __options);
	}

	/**
	* setPlatformFee
	*
	* @param { (number | string | BN) } platformFee,
	*/
	"setPlatformFee" (
		platformFee: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setPlatformFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [platformFee], __options);
	}

	/**
	* setStakingContractAddress
	*
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	*/
	"setStakingContractAddress" (
		stakingContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setStakingContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [stakingContractAddress], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setDiscount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [criteria, rates], __options);
	}

	/**
	* receiveHoldAmount
	*
	* @param { ArgumentTypes.AccountId } receiver,
	*/
	"receiveHoldAmount" (
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "receiveHoldAmount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [receiver], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getNftSaleInfo", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId], __options);
	}

	/**
	* getPlatformFee
	*
	*/
	"getPlatformFee" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getPlatformFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getStakingDiscountCriteria
	*
	*/
	"getStakingDiscountCriteria" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakingDiscountCriteria", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getStakingDiscountRate
	*
	*/
	"getStakingDiscountRate" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakingDiscountRate", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getListedTokenCountByCollectionAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"getListedTokenCountByCollectionAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getListedTokenCountByCollectionAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [collectionContractAddress], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getForSaleTokenId", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, userAccount, index], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getSaleTokensIdsCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, userAccount], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "totalTokensForSale", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, userAccount], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getAllBids", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, userAccount, tokenId], __options);
	}

	/**
	* getCollectionContractAddress
	*
	*/
	"getCollectionContractAddress" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCollectionContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getStakingContractAddress
	*
	*/
	"getStakingContractAddress" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getStakingContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getTotalVolume
	*
	*/
	"getTotalVolume" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getTotalVolume", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getVolumeByCollection
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	*/
	"getVolumeByCollection" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getVolumeByCollection", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [collectionContractAddress], __options);
	}

	/**
	* getTotalProfit
	*
	*/
	"getTotalProfit" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getTotalProfit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getCurrentProfit
	*
	*/
	"getCurrentProfit" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getCurrentProfit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* getHoldAmountOfBidder
	*
	* @param { ArgumentTypes.AccountId } bidder,
	*/
	"getHoldAmountOfBidder" (
		bidder: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getHoldAmountOfBidder", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [bidder], __options);
	}

	/**
	* getHoldBiddersByIndex
	*
	* @param { (number | string | BN) } index,
	*/
	"getHoldBiddersByIndex" (
		index: (number | string | BN),
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getHoldBiddersByIndex", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [index], __options);
	}

	/**
	* getHoldBidderCount
	*
	*/
	"getHoldBidderCount" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "getHoldBidderCount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
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
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "withdrawProfit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [value, reciever], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [newOwner], __options);
	}

	/**
	* renounceOwnership
	*
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "ownable::renounceOwnership", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [codeHash], __options);
	}

}