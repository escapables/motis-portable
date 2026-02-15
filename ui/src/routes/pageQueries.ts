import type { Location } from '$lib/Location';
import {
	getFormFactors,
	possibleTransitModes,
	prePostModesToModes,
	type PrePostDirectMode
} from '$lib/Modes';
import { defaultQuery, omitDefaults } from '$lib/defaults';
import type { IsochronesPos } from '$lib/map/IsochronesShared';
import {
	type ElevationCosts,
	type Mode,
	type OneToAllData,
	type PedestrianProfile,
	type PlanData,
	type ReachablePlace
} from '@motis-project/motis-client';
import {
	DEFAULT_ISOCHRONES_MAX_POST_TRANSIT_TIME,
	providerGroupsForQuery,
	toPlaceString,
	type ActiveTab
} from './pageState';

export type BaseQueryParams = {
	from: Location;
	to: Location;
	time: Date;
	arriveBy: boolean;
	timetableView: boolean;
	searchWindow: number;
	numItineraries: number;
	maxItineraries: number | undefined;
	slowDirect: boolean;
	pedestrianProfile: PedestrianProfile;
	transitModes: Mode[];
	preTransitModes: PrePostDirectMode[];
	postTransitModes: PrePostDirectMode[];
	directModes: PrePostDirectMode[];
	preTransitProviderGroups: string[];
	postTransitProviderGroups: string[];
	directProviderGroups: string[];
	requireBikeTransport: boolean;
	requireCarTransport: boolean;
	elevationCosts: ElevationCosts;
	useRoutedTransfers: boolean;
	maxTransfers: number;
	maxPreTransitTime: number;
	maxPostTransitTime: number;
	maxDirectTime: number;
	ignorePreTransitRentalReturnConstraints: boolean;
	ignorePostTransitRentalReturnConstraints: boolean;
	ignoreDirectRentalReturnConstraints: boolean;
	algorithm: PlanData['query']['algorithm'];
};

export const createBaseQuery = (params: BaseQueryParams): PlanData | undefined => {
	if (!params.from.match || !params.to.match) {
		return undefined;
	}
	return {
		query: omitDefaults({
			time: params.time.toISOString(),
			fromPlace: toPlaceString(params.from),
			toPlace: toPlaceString(params.to),
			arriveBy: params.arriveBy,
			timetableView: params.timetableView,
			searchWindow: params.searchWindow,
			numItineraries: params.numItineraries,
			maxItineraries: params.maxItineraries,
			withFares: true,
			slowDirect: params.slowDirect,
			fastestDirectFactor: 1.5,
			pedestrianProfile: params.pedestrianProfile,
			joinInterlinedLegs: false,
			transitModes:
				params.transitModes.length == possibleTransitModes.length
					? defaultQuery.transitModes
					: params.transitModes,
			preTransitModes: prePostModesToModes(params.preTransitModes),
			postTransitModes: prePostModesToModes(params.postTransitModes),
			directModes: prePostModesToModes(params.directModes),
			preTransitRentalFormFactors: getFormFactors(params.preTransitModes),
			postTransitRentalFormFactors: getFormFactors(params.postTransitModes),
			directRentalFormFactors: getFormFactors(params.directModes),
			preTransitRentalProviderGroups: providerGroupsForQuery(
				params.preTransitModes,
				params.preTransitProviderGroups
			),
			postTransitRentalProviderGroups: providerGroupsForQuery(
				params.postTransitModes,
				params.postTransitProviderGroups
			),
			directRentalProviderGroups: providerGroupsForQuery(
				params.directModes,
				params.directProviderGroups
			),
			requireBikeTransport: params.requireBikeTransport,
			requireCarTransport: params.requireCarTransport,
			elevationCosts: params.elevationCosts,
			useRoutedTransfers: params.useRoutedTransfers,
			maxTransfers: params.maxTransfers,
			maxMatchingDistance: params.pedestrianProfile == 'WHEELCHAIR' ? 8 : 250,
			maxPreTransitTime: params.maxPreTransitTime,
			maxPostTransitTime: params.maxPostTransitTime,
			maxDirectTime: params.maxDirectTime,
			ignorePreTransitRentalReturnConstraints: params.ignorePreTransitRentalReturnConstraints,
			ignorePostTransitRentalReturnConstraints: params.ignorePostTransitRentalReturnConstraints,
			ignoreDirectRentalReturnConstraints: params.ignoreDirectRentalReturnConstraints,
			algorithm: params.algorithm
		} as PlanData['query'])
	};
};

export type IsochronesQueryParams = {
	one: Location;
	time: Date;
	maxTravelTime: number;
	transitModes: Mode[];
	maxTransfers: number;
	arriveBy: boolean;
	useRoutedTransfers: boolean;
	pedestrianProfile: PedestrianProfile;
	requireBikeTransport: boolean;
	requireCarTransport: boolean;
	preTransitModes: PrePostDirectMode[];
	postTransitModes: PrePostDirectMode[];
	maxPreTransitTime: number;
	maxPostTransitTime: number;
	elevationCosts: ElevationCosts;
	ignorePreTransitRentalReturnConstraints: boolean;
	ignorePostTransitRentalReturnConstraints: boolean;
};

export const createIsochronesQuery = (params: IsochronesQueryParams): OneToAllData | undefined => {
	if (!params.one?.match) {
		return undefined;
	}
	return {
		query: {
			one: toPlaceString(params.one),
			maxTravelTime: Math.ceil(params.maxTravelTime / 60),
			time: params.time.toISOString(),
			transitModes: params.transitModes,
			maxTransfers: params.maxTransfers,
			arriveBy: params.arriveBy,
			useRoutedTransfers: params.useRoutedTransfers,
			pedestrianProfile: params.pedestrianProfile,
			requireBikeTransport: params.requireBikeTransport,
			requireCarTransport: params.requireCarTransport,
			preTransitModes: prePostModesToModes(params.preTransitModes),
			postTransitModes: prePostModesToModes(params.postTransitModes),
			maxPreTransitTime: params.maxPreTransitTime,
			maxPostTransitTime: params.maxPostTransitTime,
			elevationCosts: params.elevationCosts,
			maxMatchingDistance: params.pedestrianProfile == 'WHEELCHAIR' ? 8 : 250,
			ignorePreTransitRentalReturnConstraints: params.ignorePreTransitRentalReturnConstraints,
			ignorePostTransitRentalReturnConstraints: params.ignorePostTransitRentalReturnConstraints
		} as OneToAllData['query']
	} as OneToAllData;
};

export const deriveMaxPostTransitTimeDefault = (
	activeTab: ActiveTab,
	maxPrePostTransitTimeLimit: number | undefined
): number => {
	return Math.min(
		activeTab == 'isochrones'
			? DEFAULT_ISOCHRONES_MAX_POST_TRANSIT_TIME
			: defaultQuery.maxPostTransitTime,
		maxPrePostTransitTimeLimit ?? Infinity
	);
};

export const mapReachablePlaces = (
	reachablePlaces: ReachablePlace[],
	maxTravelTime: number
): IsochronesPos[] => {
	return reachablePlaces.map(
		(p) =>
			({
				lat: p.place?.lat,
				lng: p.place?.lon,
				seconds: maxTravelTime - 60 * (p.duration ?? 0),
				name: p.place?.name
			}) as IsochronesPos
	);
};
