import type {ReturnNumber} from "@727-ventures/typechain-types";
import type * as ReturnTypes from '../types-returns/artzero_launchpad_psp34';

export interface AddNewProjectEvent {
	projectId: number;
	nftContractAddress: ReturnTypes.AccountId | null;
}

