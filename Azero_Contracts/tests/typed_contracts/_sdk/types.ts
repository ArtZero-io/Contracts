// Copyright (c) 2012-2022 Supercolony
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the"Software"),
// to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be
// included in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

import type BN from 'bn.js';
// @ts-ignore
import type { ContractExecResultErr } from '@polkadot/types/interfaces/contracts/types';
import type {AnyJson} from "@polkadot/types-codec/types";

export type RequestArgumentType = number | string | boolean | bigint
	| (string | number)[]
	| BN | null | AnyJson | Object;

export interface GasLimit {
	/**
	 * Defaults to `-1`
	 */
	gasLimit ? : bigint | BN | string | number;
}

export interface GasLimitAndValue extends GasLimit {
	/**
	 * Only required for 'payable' methods
	 * Defaults to `0`
	 */
	value ? : bigint | BN | string | number;
};

export interface GasLimitAndRequiredValue extends GasLimit {
	/**
	 * Only required for 'payable' methods
	 * Defaults to `0`
	 */
	value : bigint | BN | string | number;
}

export interface ConstructorOptions extends GasLimitAndValue {
	storageDepositLimit ? : bigint | BN | string | number;
}

//

export interface ErrorWithTexts {
	texts ? : string[];
};

export type MethodDoesntExistError = ErrorWithTexts & {
	issue : 'METHOD_DOESNT_EXIST',
};


export type QueryCallError = MethodDoesntExistError | ErrorWithTexts & (
	{
		issue : 'FAIL_AT_CALL';
		caughtError : unknown;
	} | {
		issue : 'FAIL_AFTER_CALL::IS_ERROR';
		_resultIsOk : boolean;
		_asError ? : ContractExecResultErr;
	} | {
		issue : 'FAIL_AFTER_CALL::RESULT_NOT_OK';
		_asError ? : ContractExecResultErr;
	} | {
		issue : 'OUTPUT_IS_NULL',
	}
);

export type QueryOkCallError = QueryCallError | {
	issue : 'READ_ERR_IN_BODY',
	_err : any;
} | {
	issue : 'BODY_ISNT_OKERR',
	value : any;
};