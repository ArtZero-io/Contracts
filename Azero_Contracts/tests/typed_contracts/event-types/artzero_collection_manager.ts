import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/artzero_collection_manager';

export interface AddNewCollectionEvent {
	collectionOwner: ReturnTypes.AccountId | null;
	nftContractAddress: ReturnTypes.AccountId | null;
	contractType: ReturnTypes.CollectionType;
	isActive: boolean;
	showOnChainMetadata: boolean;
}

