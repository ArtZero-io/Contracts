import type * as EventTypes from '../event-types/artzero_marketplace_psp34';
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

	public subscribeOnNewListEventEvent(callback : (event : EventTypes.NewListEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('NewListEvent', 'artzero_marketplace_psp34')) as EventTypes.NewListEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'NewListEvent');
	}

	public subscribeOnUnListEventEvent(callback : (event : EventTypes.UnListEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('UnListEvent', 'artzero_marketplace_psp34')) as EventTypes.UnListEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'UnListEvent');
	}

	public subscribeOnPurchaseEventEvent(callback : (event : EventTypes.PurchaseEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('PurchaseEvent', 'artzero_marketplace_psp34')) as EventTypes.PurchaseEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'PurchaseEvent');
	}

	public subscribeOnBidWinEventEvent(callback : (event : EventTypes.BidWinEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('BidWinEvent', 'artzero_marketplace_psp34')) as EventTypes.BidWinEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'BidWinEvent');
	}

	public subscribeOnBidEventEvent(callback : (event : EventTypes.BidEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('BidEvent', 'artzero_marketplace_psp34')) as EventTypes.BidEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'BidEvent');
	}

	public subscribeOnRemoveBidEventEvent(callback : (event : EventTypes.RemoveBidEvent) => void) {
		const callbackWrapper = (args: any[], event: any) => {
			const _event: Record < string, any > = {};

			for (let i = 0; i < args.length; i++) {
				_event[event.args[i]!.name] = args[i]!.toJSON();
			}

			callback(handleEventReturn(_event, getEventTypeDescription('RemoveBidEvent', 'artzero_marketplace_psp34')) as EventTypes.RemoveBidEvent);
		};

		return this.__subscribeOnEvent(callbackWrapper, (eventName : string) => eventName == 'RemoveBidEvent');
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