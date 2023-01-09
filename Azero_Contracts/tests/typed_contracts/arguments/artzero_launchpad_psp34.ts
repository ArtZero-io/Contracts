/* This file is auto-generated */

import BN from 'bn.js';
import { Hash, AccountId, Id } from '../types-arguments/artzero_launchpad_psp34';

export interface ArgumentsTypes {
	"2": (number | string | BN);
	"12": Hash;
	"26": (string | number | BN);
	"5": (number | string | BN);
	"13": (number | string | BN);
	"29": string;
	"32": Array<string>;
	"33": Array<boolean>;
	"34": Array<(string | number | BN)>;
	"35": Array<(number | string | BN)>;
	"0": AccountId;
	"16": boolean;
	"40": Id;
}

export interface ArgumentsTuples {
	"initialize": readonly [ (number | string | BN),  Hash,  (string | number | BN),  (number | string | BN),  (number | string | BN) ];
	"add_new_project": readonly [ (number | string | BN),  (number | string | BN),  (number | string | BN),  string,  Array<string>,  Array<boolean>,  Array<(string | number | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)> ];
	"edit_project": readonly [ AccountId,  (number | string | BN),  (number | string | BN) ];
	"update_project_adding_fee": readonly [ (string | number | BN) ];
	"update_public_max_minting_amount": readonly [ (number | string | BN) ];
	"update_project_mint_fee_rate": readonly [ (number | string | BN) ];
	"update_standard_nft_hash": readonly [ Hash ];
	"update_is_active_project": readonly [ boolean,  AccountId ];
	"get_project_adding_fee": readonly [ ];
	"get_active_project_count": readonly [ ];
	"get_project_count": readonly [ ];
	"get_standard_nft_hash": readonly [ ];
	"get_project_by_id": readonly [ (number | string | BN) ];
	"get_projects_by_owner": readonly [ AccountId ];
	"get_project_by_nft_address": readonly [ AccountId ];
	"get_max_phases_per_project": readonly [ ];
	"AccessControl::has_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::get_role_admin": readonly [ (number | string | BN) ];
	"AccessControl::grant_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::revoke_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::renounce_role": readonly [ (number | string | BN),  AccountId ];
	"Ownable::owner": readonly [ ];
	"Ownable::transfer_ownership": readonly [ AccountId ];
	"Ownable::renounce_ownership": readonly [ ];
	"ArtZeroLaunchPadTrait::get_project_mint_fee_rate": readonly [ ];
	"ArtZeroLaunchPadTrait::get_public_max_minting_amount": readonly [ ];
	"AdminTrait::withdraw_fee": readonly [ (string | number | BN),  AccountId ];
	"AdminTrait::tranfer_psp22": readonly [ AccountId,  (string | number | BN),  AccountId ];
	"AdminTrait::tranfer_nft": readonly [ AccountId,  Id,  AccountId ];
}