import type BN from 'bn.js';
import type {ReturnNumber} from '@727-ventures/typechain-types';

export type AccountId = string | number[]

export enum LangError {
	couldNotReadInput = 'CouldNotReadInput'
}

export interface Error {
	custom ? : string,
	onlyOwner ? : null,
	onlyAdmin ? : null,
	invalidCaller ? : null,
	invalidFee ? : null,
	tokenOwnerNotMatch ? : null,
	notApproved ? : null,
	cannotTransfer ? : null,
	cannotMint ? : null,
	notPublicMint ? : null,
	notEnoughBalance ? : null,
	maxSupply ? : null,
	alreadyInit ? : null,
	notOwner ? : null,
	notTokenOwner ? : null,
	projectNotExist ? : null,
	projectOwnerAndAdmin ? : null,
	invalidStartTimeAndEndTime ? : null,
	invalidPhaseCount ? : null,
	collectionOwnerAndAdmin ? : null,
	collectionNotActive ? : null,
	invalidInput ? : null,
	invalidType ? : null,
	claimedAll ? : null,
	tokenLimitReached ? : null,
	updatePhase ? : null,
	phaseNotExist ? : null,
	phaseExpired ? : null,
	whitelistNotExist ? : null,
	withdrawFeeError ? : null,
	withdrawNftError ? : null,
	withdrawPsp22Error ? : null,
	notListed ? : null,
	bidAlreadyExist ? : null,
	bidNotExist ? : null,
	notInMarket ? : null,
	notForSale ? : null,
	notInSaleList ? : null,
	invalidBidLength ? : null,
	invalidCollectionOwner ? : null,
	invalidTime ? : null,
	rewardStarted ? : null,
	rewardNotStarted ? : null,
	claimMustBeFalse ? : null,
	ownableError ? : OwnableError,
	accessControlError ? : AccessControlError
}

export class ErrorBuilder {
	static Custom(value: string): Error {
		return {
			custom: value,
		};
	}
	static OnlyOwner(): Error {
		return {
			onlyOwner: null,
		};
	}
	static OnlyAdmin(): Error {
		return {
			onlyAdmin: null,
		};
	}
	static InvalidCaller(): Error {
		return {
			invalidCaller: null,
		};
	}
	static InvalidFee(): Error {
		return {
			invalidFee: null,
		};
	}
	static TokenOwnerNotMatch(): Error {
		return {
			tokenOwnerNotMatch: null,
		};
	}
	static NotApproved(): Error {
		return {
			notApproved: null,
		};
	}
	static CannotTransfer(): Error {
		return {
			cannotTransfer: null,
		};
	}
	static CannotMint(): Error {
		return {
			cannotMint: null,
		};
	}
	static NotPublicMint(): Error {
		return {
			notPublicMint: null,
		};
	}
	static NotEnoughBalance(): Error {
		return {
			notEnoughBalance: null,
		};
	}
	static MaxSupply(): Error {
		return {
			maxSupply: null,
		};
	}
	static AlreadyInit(): Error {
		return {
			alreadyInit: null,
		};
	}
	static NotOwner(): Error {
		return {
			notOwner: null,
		};
	}
	static NotTokenOwner(): Error {
		return {
			notTokenOwner: null,
		};
	}
	static ProjectNotExist(): Error {
		return {
			projectNotExist: null,
		};
	}
	static ProjectOwnerAndAdmin(): Error {
		return {
			projectOwnerAndAdmin: null,
		};
	}
	static InvalidStartTimeAndEndTime(): Error {
		return {
			invalidStartTimeAndEndTime: null,
		};
	}
	static InvalidPhaseCount(): Error {
		return {
			invalidPhaseCount: null,
		};
	}
	static CollectionOwnerAndAdmin(): Error {
		return {
			collectionOwnerAndAdmin: null,
		};
	}
	static CollectionNotActive(): Error {
		return {
			collectionNotActive: null,
		};
	}
	static InvalidInput(): Error {
		return {
			invalidInput: null,
		};
	}
	static InvalidType(): Error {
		return {
			invalidType: null,
		};
	}
	static ClaimedAll(): Error {
		return {
			claimedAll: null,
		};
	}
	static TokenLimitReached(): Error {
		return {
			tokenLimitReached: null,
		};
	}
	static UpdatePhase(): Error {
		return {
			updatePhase: null,
		};
	}
	static PhaseNotExist(): Error {
		return {
			phaseNotExist: null,
		};
	}
	static PhaseExpired(): Error {
		return {
			phaseExpired: null,
		};
	}
	static WhitelistNotExist(): Error {
		return {
			whitelistNotExist: null,
		};
	}
	static WithdrawFeeError(): Error {
		return {
			withdrawFeeError: null,
		};
	}
	static WithdrawNFTError(): Error {
		return {
			withdrawNftError: null,
		};
	}
	static WithdrawPSP22Error(): Error {
		return {
			withdrawPsp22Error: null,
		};
	}
	static NotListed(): Error {
		return {
			notListed: null,
		};
	}
	static BidAlreadyExist(): Error {
		return {
			bidAlreadyExist: null,
		};
	}
	static BidNotExist(): Error {
		return {
			bidNotExist: null,
		};
	}
	static NotInMarket(): Error {
		return {
			notInMarket: null,
		};
	}
	static NotForSale(): Error {
		return {
			notForSale: null,
		};
	}
	static NotInSaleList(): Error {
		return {
			notInSaleList: null,
		};
	}
	static InvalidBidLength(): Error {
		return {
			invalidBidLength: null,
		};
	}
	static InvalidCollectionOwner(): Error {
		return {
			invalidCollectionOwner: null,
		};
	}
	static InvalidTime(): Error {
		return {
			invalidTime: null,
		};
	}
	static RewardStarted(): Error {
		return {
			rewardStarted: null,
		};
	}
	static RewardNotStarted(): Error {
		return {
			rewardNotStarted: null,
		};
	}
	static ClaimMustBeFalse(): Error {
		return {
			claimMustBeFalse: null,
		};
	}
	static OwnableError(value: OwnableError): Error {
		return {
			ownableError: value,
		};
	}
	static AccessControlError(value: AccessControlError): Error {
		return {
			accessControlError: value,
		};
	}
}

export enum OwnableError {
	callerIsNotOwner = 'CallerIsNotOwner',
	newOwnerIsZero = 'NewOwnerIsZero'
}

export enum AccessControlError {
	invalidCaller = 'InvalidCaller',
	missingRole = 'MissingRole',
	roleRedundant = 'RoleRedundant'
}

export interface Id {
	u8 ? : number,
	u16 ? : number,
	u32 ? : number,
	u64 ? : number,
	u128 ? : ReturnNumber,
	bytes ? : Array<number>
}

export class IdBuilder {
	static U8(value: number): Id {
		return {
			u8: value,
		};
	}
	static U16(value: number): Id {
		return {
			u16: value,
		};
	}
	static U32(value: number): Id {
		return {
			u32: value,
		};
	}
	static U64(value: number): Id {
		return {
			u64: value,
		};
	}
	static U128(value: ReturnNumber): Id {
		return {
			u128: value,
		};
	}
	static Bytes(value: Array<number>): Id {
		return {
			bytes: value,
		};
	}
}

export interface PSP34Error {
	custom ? : Array<number>,
	selfApprove ? : null,
	notApproved ? : null,
	tokenExists ? : null,
	tokenNotExists ? : null,
	safeTransferCheckFailed ? : Array<number>
}

export class PSP34ErrorBuilder {
	static Custom(value: Array<number>): PSP34Error {
		return {
			custom: value,
		};
	}
	static SelfApprove(): PSP34Error {
		return {
			selfApprove: null,
		};
	}
	static NotApproved(): PSP34Error {
		return {
			notApproved: null,
		};
	}
	static TokenExists(): PSP34Error {
		return {
			tokenExists: null,
		};
	}
	static TokenNotExists(): PSP34Error {
		return {
			tokenNotExists: null,
		};
	}
	static SafeTransferCheckFailed(value: Array<number>): PSP34Error {
		return {
			safeTransferCheckFailed: value,
		};
	}
}

