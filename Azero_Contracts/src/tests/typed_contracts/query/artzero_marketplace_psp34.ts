/* This file is auto-generated */

import type { ContractPromise } from '@polkadot/api-contract';
import type { ApiPromise } from '@polkadot/api';
import type { GasLimit, GasLimitAndRequiredValue, Result } from '@727-ventures/typechain-types';
import type { QueryReturnType } from '@727-ventures/typechain-types';
import { queryJSON, queryOkJSON, handleReturnType } from '@727-ventures/typechain-types';
import type * as ArgumentTypes from '../types-arguments/artzero_marketplace_psp34';
import type * as ReturnTypes from '../types-returns/artzero_marketplace_psp34';
import type BN from 'bn.js';
//@ts-ignore
import {ReturnNumber} from '@727-ventures/typechain-types';
import {getTypeDescription} from './../shared/utils';


export default class Methods {
	private __nativeContract : ContractPromise;
	private __apiPromise: ApiPromise;
	private __callerAddress : string;

	constructor(
		nativeContract : ContractPromise,
		nativeApi : ApiPromise,
		callerAddress : string,
	) {
		this.__nativeContract = nativeContract;
		this.__callerAddress = callerAddress;
		this.__apiPromise = nativeApi;
	}

	/**
	* initialize
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	* @param { (number | string | BN) } platformFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"initialize" (
		collectionContractAddress: ArgumentTypes.AccountId,
		stakingContractAddress: ArgumentTypes.AccountId,
		platformFee: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "initialize", [collectionContractAddress, stakingContractAddress, platformFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* list
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (string | number | BN) } price,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"list" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		price: (string | number | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "list", [nftContractAddress, tokenId, price], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* unlist
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"unlist" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "unlist", [nftContractAddress, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* buy
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"buy" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "buy", [nftContractAddress, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* bid
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"bid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimitAndRequiredValue,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "bid", [nftContractAddress, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* removeBid
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"removeBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "removeBid", [nftContractAddress, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* acceptBid
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @param { (number | string | BN) } bidIndex,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"acceptBid" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		bidIndex: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "acceptBid", [nftContractAddress, tokenId, bidIndex], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* setCollectionContractAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setCollectionContractAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setCollectionContractAddress", [collectionContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* setPlatformFee
	*
	* @param { (number | string | BN) } platformFee,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setPlatformFee" (
		platformFee: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setPlatformFee", [platformFee], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* setStakingContractAddress
	*
	* @param { ArgumentTypes.AccountId } stakingContractAddress,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setStakingContractAddress" (
		stakingContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setStakingContractAddress", [stakingContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* setDiscount
	*
	* @param { Array<(number | string | BN)> } criteria,
	* @param { Array<(number | string | BN)> } rates,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setDiscount" (
		criteria: Array<(number | string | BN)>,
		rates: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "setDiscount", [criteria, rates], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* receiveHoldAmount
	*
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"receiveHoldAmount" (
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "receiveHoldAmount", [receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.ForSaleItem | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getNftSaleInfo", [nftContractAddress, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(22, 'artzero_marketplace_psp34')); });
	}

	/**
	* getPlatformFee
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getPlatformFee" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getPlatformFee", [], __options , (result) => { return handleReturnType(result, getTypeDescription(25, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingDiscountCriteria
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getStakingDiscountCriteria" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingDiscountCriteria", [], __options , (result) => { return handleReturnType(result, getTypeDescription(26, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingDiscountRate
	*
	* @returns { Result<Array<number>, ReturnTypes.LangError> }
	*/
	"getStakingDiscountRate" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<number>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingDiscountRate", [], __options , (result) => { return handleReturnType(result, getTypeDescription(27, 'artzero_marketplace_psp34')); });
	}

	/**
	* getListedTokenCountByCollectionAddress
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getListedTokenCountByCollectionAddress" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getListedTokenCountByCollectionAddress", [collectionContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_marketplace_psp34')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.Id | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getForSaleTokenId", [nftContractAddress, userAccount, index], __options , (result) => { return handleReturnType(result, getTypeDescription(29, 'artzero_marketplace_psp34')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getSaleTokensIdsCount", [nftContractAddress, userAccount], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "totalTokensForSale", [nftContractAddress, userAccount], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
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
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Array<ReturnTypes.BidInformation> | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getAllBids", [nftContractAddress, userAccount, tokenId], __options , (result) => { return handleReturnType(result, getTypeDescription(32, 'artzero_marketplace_psp34')); });
	}

	/**
	* getCollectionContractAddress
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getCollectionContractAddress" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCollectionContractAddress", [], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
	}

	/**
	* getStakingContractAddress
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"getStakingContractAddress" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getStakingContractAddress", [], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
	}

	/**
	* getTotalVolume
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getTotalVolume" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalVolume", [], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getVolumeByCollection
	*
	* @param { ArgumentTypes.AccountId } collectionContractAddress,
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getVolumeByCollection" (
		collectionContractAddress: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getVolumeByCollection", [collectionContractAddress], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getTotalProfit
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getTotalProfit" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getTotalProfit", [], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getCurrentProfit
	*
	* @returns { Result<ReturnNumber, ReturnTypes.LangError> }
	*/
	"getCurrentProfit" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getCurrentProfit", [], __options , (result) => { return handleReturnType(result, getTypeDescription(31, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldAmountOfBidder
	*
	* @param { ArgumentTypes.AccountId } bidder,
	* @returns { Result<ReturnNumber | null, ReturnTypes.LangError> }
	*/
	"getHoldAmountOfBidder" (
		bidder: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnNumber | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldAmountOfBidder", [bidder], __options , (result) => { return handleReturnType(result, getTypeDescription(35, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldBiddersByIndex
	*
	* @param { (number | string | BN) } index,
	* @returns { Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> }
	*/
	"getHoldBiddersByIndex" (
		index: (number | string | BN),
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId | null, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldBiddersByIndex", [index], __options , (result) => { return handleReturnType(result, getTypeDescription(37, 'artzero_marketplace_psp34')); });
	}

	/**
	* getHoldBidderCount
	*
	* @returns { Result<number, ReturnTypes.LangError> }
	*/
	"getHoldBidderCount" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<number, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "getHoldBidderCount", [], __options , (result) => { return handleReturnType(result, getTypeDescription(28, 'artzero_marketplace_psp34')); });
	}

	/**
	* withdrawProfit
	*
	* @param { (string | number | BN) } value,
	* @param { ArgumentTypes.AccountId } reciever,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"withdrawProfit" (
		value: (string | number | BN),
		reciever: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "withdrawProfit", [value, reciever], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* transferOwnership
	*
	* @param { ArgumentTypes.AccountId } newOwner,
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"transferOwnership" (
		newOwner: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::transferOwnership", [newOwner], __options , (result) => { return handleReturnType(result, getTypeDescription(39, 'artzero_marketplace_psp34')); });
	}

	/**
	* renounceOwnership
	*
	* @returns { Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> }
	*/
	"renounceOwnership" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.OwnableError>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::renounceOwnership", [], __options , (result) => { return handleReturnType(result, getTypeDescription(39, 'artzero_marketplace_psp34')); });
	}

	/**
	* owner
	*
	* @returns { Result<ReturnTypes.AccountId, ReturnTypes.LangError> }
	*/
	"owner" (
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<ReturnTypes.AccountId, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "ownable::owner", [], __options , (result) => { return handleReturnType(result, getTypeDescription(34, 'artzero_marketplace_psp34')); });
	}

	/**
	* tranferPsp22
	*
	* @param { ArgumentTypes.AccountId } psp22ContractAddress,
	* @param { (string | number | BN) } amount,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"tranferPsp22" (
		psp22ContractAddress: ArgumentTypes.AccountId,
		amount: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferPsp22", [psp22ContractAddress, amount, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* tranferNft
	*
	* @param { ArgumentTypes.AccountId } nftContractAddress,
	* @param { ArgumentTypes.Id } tokenId,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"tranferNft" (
		nftContractAddress: ArgumentTypes.AccountId,
		tokenId: ArgumentTypes.Id,
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::tranferNft", [nftContractAddress, tokenId, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* withdrawFee
	*
	* @param { (string | number | BN) } value,
	* @param { ArgumentTypes.AccountId } receiver,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"withdrawFee" (
		value: (string | number | BN),
		receiver: ArgumentTypes.AccountId,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "adminTrait::withdrawFee", [value, receiver], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

	/**
	* setCode
	*
	* @param { Array<(number | string | BN)> } codeHash,
	* @returns { Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> }
	*/
	"setCode" (
		codeHash: Array<(number | string | BN)>,
		__options ? : GasLimit,
	): Promise< QueryReturnType< Result<Result<null, ReturnTypes.Error>, ReturnTypes.LangError> > >{
		return queryOkJSON( this.__apiPromise, this.__nativeContract, this.__callerAddress, "upgradableTrait::setCode", [codeHash], __options , (result) => { return handleReturnType(result, getTypeDescription(15, 'artzero_marketplace_psp34')); });
	}

}