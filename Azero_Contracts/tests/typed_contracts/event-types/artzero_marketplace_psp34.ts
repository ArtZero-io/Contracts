import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/artzero_marketplace_psp34';

export interface NewListEvent {
	trader: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
	price: ReturnNumber;
}

export interface UnListEvent {
	trader: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
}

export interface PurchaseEvent {
	buyer: ReturnTypes.AccountId | null;
	seller: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
	price: ReturnNumber;
	platformFee: ReturnNumber;
	royaltyFee: ReturnNumber;
}

export interface BidWinEvent {
	bidder: ReturnTypes.AccountId | null;
	seller: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
	price: ReturnNumber;
	platformFee: ReturnNumber;
	royaltyFee: ReturnNumber;
}

export interface BidEvent {
	bidder: ReturnTypes.AccountId | null;
	seller: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
	price: ReturnNumber;
	bidValue: ReturnNumber;
}

export interface RemoveBidEvent {
	bidder: ReturnTypes.AccountId | null;
	seller: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	tokenId: ReturnTypes.Id;
	bidValue: ReturnNumber;
}

