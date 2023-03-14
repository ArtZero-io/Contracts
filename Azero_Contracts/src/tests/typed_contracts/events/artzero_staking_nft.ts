import type * as EventTypes from '../event-types/artzero_staking_nft';
import type {ContractPromise} from "@polkadot/api-contract";
import type {ApiPromise} from "@polkadot/api";
import {getEventTypeDescription} from "../shared/utils";
import {handleEventReturn} from "@727-ventures/typechain-types";

export default class EventsClass {
	private __nativeContract : ContractPromise;
	private __api : ApiPromise;

	constructor(
		nativeContract : ContractPromise,
		api : ApiPromise,
	) {
		this.__nativeContract = nativeContract;
		this.__api = api;
	}

	public subscribeOnNewStakeEventEvent(callback : (event : EventTypes.NewStakeEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('NewStakeEvent', 'artzero_staking_nft')) as EventTypes.NewStakeEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'NewStakeEvent');
	}

	public subscribeOnUnstakeEventEvent(callback : (event : EventTypes.UnstakeEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('UnstakeEvent', 'artzero_staking_nft')) as EventTypes.UnstakeEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'UnstakeEvent');
	}

	public subscribeOnRequestUnstakeEventEvent(callback : (event : EventTypes.RequestUnstakeEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('RequestUnstakeEvent', 'artzero_staking_nft')) as EventTypes.RequestUnstakeEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'RequestUnstakeEvent');
	}

	public subscribeOnCancelRequestUnstakeEventEvent(callback : (event : EventTypes.CancelRequestUnstakeEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('CancelRequestUnstakeEvent', 'artzero_staking_nft')) as EventTypes.CancelRequestUnstakeEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'CancelRequestUnstakeEvent');
	}

	public subscribeOnClaimRewardEvent(callback : (event : EventTypes.ClaimReward) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('ClaimReward', 'artzero_staking_nft')) as EventTypes.ClaimReward);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'ClaimReward');
	}

	public subscribeOnAddRewardEvent(callback : (event : EventTypes.AddReward) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('AddReward', 'artzero_staking_nft')) as EventTypes.AddReward);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'AddReward');
	}


	private __subscribeOnEvent(
		callback : (args: any[], event: any) => void,
		filter : (eventName: string) => boolean = () => true
	) {
		// @ts-ignore
		return this.__api.query.system.events((events) => {
			events.forEach((record: any) => {
				const { event } = record;

				if (event.method == 'ContractEmitted') {
					const [address, data] = record.event.data;

					if (address.toString() === this.__nativeContract.address.toString()) {
						const {args, event} = this.__nativeContract.abi.decodeEvent(data);

						if (filter(event.identifier.toString()))
							callback(args, event);
					}
				}
			});
		});
	}

}