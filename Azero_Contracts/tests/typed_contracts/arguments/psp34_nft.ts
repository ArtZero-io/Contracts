/* This file is auto-generated */

import BN from 'bn.js';
import { AccountId, Id } from '../types-arguments/psp34_nft';

export interface ArgumentsTypes {
	"38": Array<[string, string]>;
	"8": AccountId;
	"14": Id | null;
	"30": boolean;
	"1": Id;
	"7": Array<(number | string | BN)>;
	"6": (string | number | BN);
	"45": Array<string>;
	"4": (number | string | BN);
	"33": string;
	"5": (number | string | BN);
}

export interface ArgumentsTuples {
	"mint": readonly [ ];
	"mint_with_attributes": readonly [ Array<[string, string]> ];
	"Ownable::transfer_ownership": readonly [ AccountId ];
	"Ownable::owner": readonly [ ];
	"Ownable::renounce_ownership": readonly [ ];
	"PSP34::balance_of": readonly [ AccountId ];
	"PSP34::approve": readonly [ AccountId,  Id | null,  boolean ];
	"PSP34::transfer": readonly [ AccountId,  Id,  Array<(number | string | BN)> ];
	"PSP34::owner_of": readonly [ Id ];
	"PSP34::collection_id": readonly [ ];
	"PSP34::allowance": readonly [ AccountId,  AccountId,  Id | null ];
	"PSP34::total_supply": readonly [ ];
	"PSP34Metadata::get_attribute": readonly [ Id,  Array<(number | string | BN)> ];
	"PSP34Enumerable::owners_token_by_index": readonly [ AccountId,  (string | number | BN) ];
	"PSP34Enumerable::token_by_index": readonly [ (string | number | BN) ];
	"Psp34Traits::set_multiple_attributes": readonly [ Id,  Array<[string, string]> ];
	"Psp34Traits::get_attribute_count": readonly [ ];
	"Psp34Traits::lock": readonly [ Id ];
	"Psp34Traits::get_attributes": readonly [ Id,  Array<string> ];
	"Psp34Traits::get_attribute_name": readonly [ (number | string | BN) ];
	"Psp34Traits::is_locked_nft": readonly [ Id ];
	"Psp34Traits::set_base_uri": readonly [ string ];
	"Psp34Traits::get_owner": readonly [ ];
	"Psp34Traits::get_last_token_id": readonly [ ];
	"Psp34Traits::get_locked_token_count": readonly [ ];
	"Psp34Traits::burn": readonly [ Id ];
	"Psp34Traits::token_uri": readonly [ (number | string | BN) ];
	"AdminTrait::withdraw_fee": readonly [ (string | number | BN),  AccountId ];
	"AdminTrait::tranfer_nft": readonly [ AccountId,  Id,  AccountId ];
	"AdminTrait::tranfer_psp22": readonly [ AccountId,  (string | number | BN),  AccountId ];
}