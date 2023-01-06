import type BN from 'bn.js';

export type AccountId = string | number[]

export type Hash = string | number[]

export type Collection = {
	collectionOwner: AccountId,
	nftContractAddress: AccountId,
	contractType: CollectionType,
	isCollectRoyaltyFee: boolean,
	royaltyFee: number,
	isActive: boolean,
	showOnChainMetadata: boolean
}

export enum CollectionType {
	unknown = 'Unknown',
	psp34Manual = 'Psp34Manual',
	psp34Auto = 'Psp34Auto',
	psp1155Manual = 'Psp1155Manual',
	psp1155Auto = 'Psp1155Auto',
	reserved1 = 'Reserved1',
	reserved2 = 'Reserved2'
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
	notEnoughBalance ? : null,
	alreadyInit ? : null,
	notOwner ? : null,
	notTokenOwner ? : null,
	projectNotExist ? : null,
	projectOwnerAndAdmin ? : null,
	invalidStartTimeAndEndTime ? : null,
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
	static NotEnoughBalance(): Error {
		return {
			notEnoughBalance: null,
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
	u128 ? : (string | number),
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
	static U128(value: (string | number)): Id {
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

