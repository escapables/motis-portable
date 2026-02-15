import type { Location } from '$lib/Location';
import { getPrePostDirectModes, type PrePostDirectMode } from '$lib/Modes';
import { defaultQuery } from '$lib/defaults';
import type { DisplayLevel, IsochronesOptions } from '$lib/map/IsochronesShared';
import { getUrlArray } from '$lib/utils';
import {
	type ElevationCosts,
	type Mode,
	type PedestrianProfile,
	type PlanData,
	type RentalFormFactor
} from '@motis-project/motis-client';
import { parseLocation } from '$lib/Location';
import { parseIntOr } from './pageState';

export type InitialPlannerState = {
	from: Location;
	to: Location;
	one: Location;
	time: Date;
	timetableView: boolean;
	searchWindow: number;
	numItineraries: number;
	maxItineraries: number | undefined;
	arriveBy: boolean;
	algorithm: PlanData['query']['algorithm'];
	useRoutedTransfers: boolean;
	pedestrianProfile: PedestrianProfile;
	requireBikeTransport: boolean;
	requireCarTransport: boolean;
	transitModes: Mode[];
	preTransitModes: PrePostDirectMode[];
	postTransitModes: PrePostDirectMode[];
	directModes: PrePostDirectMode[];
	preTransitProviderGroups: string[];
	postTransitProviderGroups: string[];
	directProviderGroups: string[];
	elevationCosts: ElevationCosts;
	maxTransfers: number;
	ignorePreTransitRentalReturnConstraints: boolean;
	ignorePostTransitRentalReturnConstraints: boolean;
	ignoreDirectRentalReturnConstraints: boolean;
	slowDirect: boolean;
	isochronesOptions: IsochronesOptions;
	isochronesCircleResolution: number | undefined;
};

const parseModes = (
	name: string,
	defaultModes: Mode[],
	formFactorName: string
): PrePostDirectMode[] => {
	return getPrePostDirectModes(
		getUrlArray(name, defaultModes) as Mode[],
		getUrlArray(formFactorName) as RentalFormFactor[]
	);
};

export const deriveInitialPlannerState = (
	urlParams: URLSearchParams | undefined
): InitialPlannerState => {
	const isochronesCircleResolutionParam = urlParams?.get('isochronesCircleResolution');
	const isochronesCircleResolution = isochronesCircleResolutionParam
		? parseIntOr(isochronesCircleResolutionParam, defaultQuery.circleResolution)
		: defaultQuery.circleResolution;

	return {
		from: parseLocation(urlParams?.get('fromPlace'), urlParams?.get('fromName')),
		to: parseLocation(urlParams?.get('toPlace'), urlParams?.get('toName')),
		one: parseLocation(urlParams?.get('one'), urlParams?.get('oneName')),
		time: new Date(urlParams?.get('time') || Date.now()),
		timetableView: urlParams?.get('timetableView') != 'false',
		searchWindow: urlParams?.get('searchWindow')
			? parseInt(urlParams.get('searchWindow')!)
			: defaultQuery.searchWindow,
		numItineraries: urlParams?.get('numItineraries')
			? parseIntOr(urlParams.get('numItineraries'), defaultQuery.numItineraries)
			: defaultQuery.numItineraries,
		maxItineraries: urlParams?.get('maxItineraries')
			? parseIntOr(urlParams.get('maxItineraries'), undefined)
			: undefined,
		arriveBy: urlParams?.get('arriveBy') == 'true',
		algorithm: (urlParams?.get('algorithm') ??
			defaultQuery.algorithm) as PlanData['query']['algorithm'],
		useRoutedTransfers:
			urlParams?.get('useRoutedTransfers') == 'true' || defaultQuery.useRoutedTransfers,
		pedestrianProfile: (urlParams?.has('pedestrianProfile')
			? urlParams.get('pedestrianProfile')
			: defaultQuery.pedestrianProfile) as PedestrianProfile,
		requireBikeTransport: urlParams?.get('requireBikeTransport') == 'true',
		requireCarTransport: urlParams?.get('requireCarTransport') == 'true',
		transitModes: getUrlArray('transitModes', defaultQuery.transitModes) as Mode[],
		preTransitModes: parseModes(
			'preTransitModes',
			defaultQuery.preTransitModes as Mode[],
			'preTransitRentalFormFactors'
		),
		postTransitModes: parseModes(
			'postTransitModes',
			defaultQuery.postTransitModes as Mode[],
			'postTransitRentalFormFactors'
		),
		directModes: parseModes(
			'directModes',
			defaultQuery.directModes as Mode[],
			'directRentalFormFactors'
		),
		preTransitProviderGroups: getUrlArray('preTransitRentalProviderGroups'),
		postTransitProviderGroups: getUrlArray('postTransitRentalProviderGroups'),
		directProviderGroups: getUrlArray('directRentalProviderGroups'),
		elevationCosts: (urlParams?.get('elevationCosts') ?? 'NONE') as ElevationCosts,
		maxTransfers: parseIntOr(urlParams?.get('maxTransfers'), defaultQuery.maxTransfers),
		ignorePreTransitRentalReturnConstraints:
			urlParams?.get('ignorePreTransitRentalReturnConstraints') == 'true',
		ignorePostTransitRentalReturnConstraints:
			urlParams?.get('ignorePostTransitRentalReturnConstraints') == 'true',
		ignoreDirectRentalReturnConstraints:
			urlParams?.get('ignoreDirectRentalReturnConstraints') == 'true',
		slowDirect: urlParams?.get('slowDirect') == 'true',
		isochronesOptions: {
			displayLevel:
				(urlParams?.get('isochronesDisplayLevel') as DisplayLevel) ??
				defaultQuery.isochronesDisplayLevel,
			color: urlParams?.get('isochronesColor') ?? defaultQuery.isochronesColor,
			opacity: parseIntOr(urlParams?.get('isochronesOpacity'), defaultQuery.isochronesOpacity),
			status: 'DONE',
			errorMessage: undefined,
			errorCode: undefined
		},
		isochronesCircleResolution
	};
};
