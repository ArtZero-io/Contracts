/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryOkJSON, queryJSON, handleReturnType } from '@727-ventures/typechain-types';
import { txSignAndSend } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_marketplace_psp34';
import type * as ReturnTypes from '../types-returns/artzero_marketplace_psp34';
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
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	* @param { (number | string | BN) } platformFee,
	* @returns { void }
	*/
	"initialize" (
		collectionContractAddress: ArgumentTypes.AccountId,
		stakingContractAddress: ArgumentTypes.AccountId,
		platformFee: (number | string | BN),
		__options: GasLimit,
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
	* @returns { void }
	*/
	"list" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		price: (string | number | BN),
		__options: GasLimit,
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
	* @returns { void }
	*/
	"unlist" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"buy" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimitAndRequiredValue,
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
	* @returns { void }
	*/
	"bid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimitAndRequiredValue,
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
	* @returns { void }
	*/
	"removeBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"acceptBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		bidIndex: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "acceptBid", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId, bidIndex], __options);
	}

	/**
	* setCollectionContractAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { void }
	*/
	"setCollectionContractAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setCollectionContractAddress", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [collectionContractAddress], __options);
	}

	/**
	* setPlatformFee
	*
	* @param { (number | string | BN) } platformFee,
	* @returns { void }
	*/
	"setPlatformFee" (
		platformFee: (number | string | BN),
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setPlatformFee", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [platformFee], __options);
	}

	/**
	* setStakingContractAddress
	*
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	* @returns { void }
	*/
	"setStakingContractAddress" (
		stakingContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { void }
	*/
	"setDiscount" (
		criteria: Array<(number | string | BN)>,
		rates: Array<(number | string | BN)>,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "setDiscount", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [criteria, rates], __options);
	}

	/**
	* receiveHoldAmount
	*
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { void }
	*/
	"receiveHoldAmount" (
		receiver: ArgumentTypes.AccountId,
		__options: GasLimit,
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
	* @returns { Result<ReturnTypes.ForSaleItem | null, ReturnTypes.LangError> }
	*/
	"getNftSaleInfo" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.ForSaleItem | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getNftSaleInfo", [nftContractAddress, tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_marketplace_psp34')); });
	}

	/**
	* getPlatformFee
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPlatformFee" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPlatformFee", [], __options, (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingDiscountCriteria
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getStakingDiscountCriteria" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingDiscountCriteria", [], __options, (result) => { return handleReturnType(result, getTypeDescription(26, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingDiscountRate
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getStakingDiscountRate" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingDiscountRate", [], __options, (result) => { return handleReturnType(result, getTypeDescription(27, 'artzero_marketplace_psp34')); });
	}

	/**
	* getListedTokenCountByCollectionAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getListedTokenCountByCollectionAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getListedTokenCountByCollectionAddress", [collectionContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_marketplace_psp34')); });
	}

	/**
	* getForSaleTokenId
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.AccountId } userAccount,
	* @param { (string | number | BN) } index,
	* @returns { Result<ReturnTypes.Id | null, ReturnTypes.LangError> }
	*/
	"getForSaleTokenId" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		index: (string | number | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getForSaleTokenId", [nftContractAddress, userAccount, index], __options, (result) => { return handleReturnType(result, getTypeDescription(29, 'artzero_marketplace_psp34')); });
	}

	/**
	* getSaleTokensIdsCount
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.AccountId } userAccount,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getSaleTokensIdsCount" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getSaleTokensIdsCount", [nftContractAddress, userAccount], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* totalTokensForSale
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.AccountId } userAccount,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"totalTokensForSale" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "totalTokensForSale", [nftContractAddress, userAccount], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getAllBids
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.AccountId } userAccount,
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Array<ReturnTypes.BidInformation> | null, ReturnTypes.LangError> }
	*/
	"getAllBids" (
		nftContractAddress: ArgumentTypes.AccountId,
		userAccount: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.BidInformation> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAllBids", [nftContractAddress, userAccount, tokenId], __options, (result) => { return handleReturnType(result, getTypeDescription(32, 'artzero_marketplace_psp34')); });
	}

	/**
	* getCollectionContractAddress
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getCollectionContractAddress" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionContractAddress", [], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingContractAddress
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getStakingContractAddress" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingContractAddress", [], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
	}

	/**
	* getTotalVolume
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getTotalVolume" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalVolume", [], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getVolumeByCollection
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getVolumeByCollection" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getVolumeByCollection", [collectionContractAddress], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getTotalProfit
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getTotalProfit" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalProfit", [], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getCurrentProfit
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getCurrentProfit" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCurrentProfit", [], __options, (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldAmountOfBidder
	*
	* @param { ArgumentTypes.AccountId } bidder,
	* @returns { Result<ReturnNumber | null, ReturnTypes.LangError> }
	*/
	"getHoldAmountOfBidder" (
		bidder: ArgumentTypes.AccountId,
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldAmountOfBidder", [bidder], __options, (result) => { return handleReturnType(result, getTypeDescription(35, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldBiddersByIndex
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getHoldBiddersByIndex" (
		index: (number | string | BN),
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldBiddersByIndex", [index], __options, (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldBidderCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getHoldBidderCount" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldBidderCount", [], __options, (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_marketplace_psp34')); });
	}

	/**
	* withdrawProfit
	*
	* @param { (string | number | BN) } value,
	* @param { ArgumentTypes.AccountId } reciever,
	* @returns { void }
	*/
	"withdrawProfit" (
		value: (string | number | BN),
		reciever: ArgumentTypes.AccountId,
		__options: GasLimit,
	){
		return txSignAndSend( this.__apiPromise, this.__nativeContract, this.__keyringPair, "withdrawProfit", (events: EventRecord) => {
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [value, reciever], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [newOwner], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [], __options);
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options: GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options, (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [nftContractAddress, tokenId, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [value, receiver], __options);
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
			return decodeEvents(events, this.__nativeContract, "artzero_marketplace_psp34");
		}, [codeHash], __options);
	}

}