/* This file is auto-generated */

import BN from 'bn.js';
import { AccountId, Id } from '../types-arguments/launchpad_psp34_nft_standard';

export interface ArgumentsTypes {
	"55": string;
	"43": boolean;
	"6": (string | number | BN);
	"5": (number | string | BN);
	"8": AccountId;
	"2": (number | string | BN);
	"7": Array<(number | string | BN)>;
	"56": Array<string>;
	"57": Array<boolean>;
	"58": Array<(string | number | BN)>;
	"59": Array<(number | string | BN)>;
	"14": Id | null;
	"1": Id;
	"4": (number | string | BN);
	"74": Array<[string, string]>;
}

export interface ArgumentsTuples {
	"add_new_phase": readonly [ string,  boolean,  (string | number | BN),  (number | string | BN),  (number | string | BN),  (number | string | BN),  (number | string | BN) ];
	"update_whitelist": readonly [ AccountId,  (number | string | BN),  (number | string | BN),  (string | number | BN) ];
	"add_whitelist": readonly [ AccountId,  (number | string | BN),  (number | string | BN),  (string | number | BN) ];
	"mint": readonly [ (number | string | BN) ];
	"public_mint": readonly [ (number | string | BN),  (number | string | BN) ];
	"whitelist_mint": readonly [ (number | string | BN),  (number | string | BN) ];
	"deactive_phase": readonly [ (number | string | BN) ];
	"update_schedule_phase": readonly [ (number | string | BN),  string,  boolean,  (string | number | BN),  (number | string | BN),  (number | string | BN),  (number | string | BN),  (number | string | BN) ];
	"update_schedule_phases": readonly [ Array<(number | string | BN)>,  Array<string>,  Array<boolean>,  Array<(string | number | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)>,  Array<(number | string | BN)> ];
	"edit_project_information": readonly [ string ];
	"get_owner_claimed_amount": readonly [ ];
	"get_owner_available_amount": readonly [ ];
	"get_limit_phase_count": readonly [ ];
	"get_public_minted_count": readonly [ ];
	"get_project_info": readonly [ ];
	"get_phase_schedule_by_id": readonly [ (number | string | BN) ];
	"get_whitelist_by_account_id": readonly [ AccountId,  (number | string | BN) ];
	"get_phase_account_link": readonly [ (number | string | BN),  (number | string | BN) ];
	"get_current_phase": readonly [ ];
	"is_in_schedule_phase": readonly [ (number | string | BN) ];
	"get_whitelist_count": readonly [ ];
	"get_last_phase_id": readonly [ ];
	"get_active_phase_count": readonly [ ];
	"get_last_token_id": readonly [ ];
	"get_phase_account_public_claimed_amount": readonly [ AccountId,  (number | string | BN) ];
	"get_phase_account_last_index": readonly [ (number | string | BN) ];
	"get_total_supply": readonly [ ];
	"get_available_token_amount": readonly [ ];
	"Ownable::transfer_ownership": readonly [ AccountId ];
	"Ownable::renounce_ownership": readonly [ ];
	"Ownable::owner": readonly [ ];
	"PSP34::total_supply": readonly [ ];
	"PSP34::approve": readonly [ AccountId,  Id | null,  boolean ];
	"PSP34::transfer": readonly [ AccountId,  Id,  Array<(number | string | BN)> ];
	"PSP34::collection_id": readonly [ ];
	"PSP34::balance_of": readonly [ AccountId ];
	"PSP34::owner_of": readonly [ Id ];
	"PSP34::allowance": readonly [ AccountId,  AccountId,  Id | null ];
	"PSP34Metadata::get_attribute": readonly [ Id,  Array<(number | string | BN)> ];
	"PSP34Enumerable::owners_token_by_index": readonly [ AccountId,  (string | number | BN) ];
	"PSP34Enumerable::token_by_index": readonly [ (string | number | BN) ];
	"Psp34Traits::set_base_uri": readonly [ string ];
	"Psp34Traits::get_locked_token_count": readonly [ ];
	"Psp34Traits::get_attribute_name": readonly [ (number | string | BN) ];
	"Psp34Traits::get_owner": readonly [ ];
	"Psp34Traits::get_attributes": readonly [ Id,  Array<string> ];
	"Psp34Traits::set_multiple_attributes": readonly [ Id,  Array<[string, string]> ];
	"Psp34Traits::token_uri": readonly [ (number | string | BN) ];
	"Psp34Traits::is_locked_nft": readonly [ Id ];
	"Psp34Traits::get_attribute_count": readonly [ ];
	"Psp34Traits::lock": readonly [ Id ];
	"Psp34Traits::get_last_token_id": readonly [ ];
	"AccessControl::grant_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::has_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::get_role_admin": readonly [ (number | string | BN) ];
	"AccessControl::revoke_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::renounce_role": readonly [ (number | string | BN),  AccountId ];
	"AdminTrait::tranfer_nft": readonly [ AccountId,  Id,  AccountId ];
	"AdminTrait::withdraw_fee": readonly [ (string | number | BN),  AccountId ];
	"AdminTrait::tranfer_psp22": readonly [ AccountId,  (string | number | BN),  AccountId ];
	"PSP34Burnable::burn": readonly [ AccountId,  Id ];
}