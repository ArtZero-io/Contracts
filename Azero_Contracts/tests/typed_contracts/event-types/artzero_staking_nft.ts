import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/artzero_staking_nft';

export interface NewStakeEvent {
	staker: ReturnTypes.AccountId | null;
	tokenId: number;
}

export interface UnstakeEvent {
	staker: ReturnTypes.AccountId | null;
	tokenId: number;
}

export interface RequestUnstakeEvent {
	staker: ReturnTypes.AccountId | null;
	tokenId: number;
}

export interface CancelRequestUnstakeEvent {
	staker: ReturnTypes.AccountId | null;
	tokenId: number;
}

export interface ClaimReward {
	staker: ReturnTypes.AccountId | null;
	rewardAmount: ReturnNumber;
	stakedAmount: number;
}

export interface AddReward {
	rewardAmount: ReturnNumber;
	totalStakedAmount: number;
}

