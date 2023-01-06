/* This file is auto-generated */

import BN from 'bn.js';
import { Hash, AccountId, CollectionType, Id } from '../types-arguments/artzero_collection_manager';

export interface ArgumentsTypes {
	"12": Hash;
	"14": (string | number | BN);
	"5": (number | string | BN);
	"31": string;
	"38": Array<string>;
	"18": boolean;
	"0": AccountId;
	"17": CollectionType;
	"13": (number | string | BN);
	"45": Id;
}

export interface ArgumentsTuples {
	"initialize": readonly [ Hash,  (string | number | BN),  (string | number | BN),  (number | string | BN) ];
	"auto_new_collection": readonly [ string,  string,  Array<string>,  Array<string>,  boolean,  (number | string | BN) ];
	"add_new_collection": readonly [ AccountId,  Array<string>,  Array<string>,  boolean,  (number | string | BN) ];
	"update_collection_owner": readonly [ AccountId,  AccountId ];
	"update_nft_contract_address": readonly [ AccountId,  AccountId ];
	"set_multiple_attributes": readonly [ AccountId,  Array<string>,  Array<string> ];
	"get_attributes": readonly [ AccountId,  Array<string> ];
	"get_attribute": readonly [ AccountId,  string ];
	"has_attribute": readonly [ AccountId,  string ];
	"get_collection_attribute_index": readonly [ AccountId,  string ];
	"get_collection_attribute_count": readonly [ AccountId ];
	"update_contract_type": readonly [ AccountId,  CollectionType ];
	"update_is_collect_royalty_fee": readonly [ AccountId,  boolean ];
	"update_royalty_fee": readonly [ AccountId,  (number | string | BN) ];
	"update_show_on_chain_metadata": readonly [ AccountId,  boolean ];
	"update_is_active": readonly [ AccountId,  boolean ];
	"update_simple_mode_adding_fee": readonly [ (string | number | BN) ];
	"update_standard_nft_hash": readonly [ Hash ];
	"update_advance_mode_adding_fee": readonly [ (string | number | BN) ];
	"update_max_royalty_fee_rate": readonly [ (number | string | BN) ];
	"get_collection_by_address": readonly [ AccountId ];
	"get_collections_by_owner": readonly [ AccountId ];
	"get_standard_nft_hash": readonly [ ];
	"get_contract_by_id": readonly [ (number | string | BN) ];
	"get_collection_count": readonly [ ];
	"get_active_collection_count": readonly [ ];
	"get_simple_mode_adding_fee": readonly [ ];
	"get_advance_mode_adding_fee": readonly [ ];
	"get_max_royalty_fee_rate": readonly [ ];
	"AccessControl::get_role_admin": readonly [ (number | string | BN) ];
	"AccessControl::grant_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::renounce_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::has_role": readonly [ (number | string | BN),  AccountId ];
	"AccessControl::revoke_role": readonly [ (number | string | BN),  AccountId ];
	"Ownable::owner": readonly [ ];
	"Ownable::transfer_ownership": readonly [ AccountId ];
	"Ownable::renounce_ownership": readonly [ ];
	"ArtZeroCollectionTrait::get_collection_owner": readonly [ AccountId ];
	"ArtZeroCollectionTrait::is_active": readonly [ AccountId ];
	"ArtZeroCollectionTrait::get_royalty_fee": readonly [ AccountId ];
	"ArtZeroCollectionTrait::get_contract_type": readonly [ AccountId ];
	"AdminTrait::withdraw_fee": readonly [ (string | number | BN),  AccountId ];
	"AdminTrait::tranfer_nft": readonly [ AccountId,  Id,  AccountId ];
	"AdminTrait::tranfer_psp22": readonly [ AccountId,  (string | number | BN),  AccountId ];
}