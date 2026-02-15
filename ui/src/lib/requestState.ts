import type { OneToAllData, PlanData } from '@motis-project/motis-client';
import type { DisplayLevel } from '$lib/map/IsochronesShared';

type QueryState = Record<string, unknown>;

export const createDebouncedRunner = () => {
	let timeout: ReturnType<typeof setTimeout> | undefined;

	const clear = () => {
		if (timeout === undefined) {
			return;
		}
		clearTimeout(timeout);
		timeout = undefined;
	};

	const schedule = (delayMs: number, run: () => void | Promise<void>) => {
		clear();
		timeout = setTimeout(() => {
			timeout = undefined;
			void run();
		}, delayMs);
	};

	return { clear, schedule };
};

export const buildConnectionsQueryState = (
	query: PlanData['query'],
	fromName: string,
	toName: string
): QueryState => {
	return {
		...query,
		...(query.fromPlace === fromName ? {} : { fromName }),
		...(query.toPlace === toName ? {} : { toName })
	};
};

type IsochronesQueryStateOptions = {
	oneName: string;
	isochronesColor: string;
	isochronesOpacity: number;
	isochronesDisplayLevel: DisplayLevel;
	isochronesCircleResolution?: number;
};

export const buildIsochronesQueryState = (
	query: OneToAllData['query'],
	options: IsochronesQueryStateOptions
): QueryState => {
	return {
		...query,
		...(query.one === options.oneName ? {} : { oneName: options.oneName }),
		maxTravelTime: query.maxTravelTime * 60,
		isochronesColor: options.isochronesColor,
		isochronesOpacity: options.isochronesOpacity,
		isochronesDisplayLevel: options.isochronesDisplayLevel,
		...(options.isochronesCircleResolution && options.isochronesCircleResolution > 2
			? { isochronesCircleResolution: options.isochronesCircleResolution }
			: {})
	};
};
