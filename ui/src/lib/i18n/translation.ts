import { browser, dev } from '$app/environment';
import { derived, get, writable } from 'svelte/store';
import bg from './bg';
import en from './en';
import de from './de';
import fr from './fr';
import pl from './pl';
import cs from './cs';
import sv from './sv';

export type Translations = {
	language: string;
	loadingData: string;
	startupFailed: string;
	retry: string;
	noInitializationData: string;
	unknownInitializationError: string;
	fareFor: string;
	fareAs: string;
	ticket: string;
	ticketOptions: string;
	includedInTicket: string;
	journeyDetails: string;
	transfers: string;
	walk: string;
	bike: string;
	cargoBike: string;
	scooterStanding: string;
	scooterSeated: string;
	car: string;
	taxi: string;
	moped: string;
	unknownVehicleType: string;
	electricAssist: string;
	electric: string;
	combustion: string;
	combustionDiesel: string;
	hybrid: string;
	plugInHybrid: string;
	hydrogenFuelCell: string;
	from: string;
	to: string;
	viaStop: string;
	viaStops: string;
	addViaStop: string;
	removeViaStop: string;
	viaStayDuration: string;
	position: string;
	arrival: string;
	departure: string;
	duration: string;
	later: string;
	earlier: string;
	arrivals: string;
	departures: string;
	connections: string;
	switchToArrivals: string;
	switchToDepartures: string;
	arrivalOnTrack: string;
	track: string;
	platform: string;
	trackAbr: string;
	platformAbr: string;
	tripIntermediateStops: (n: number) => string;
	sharingProvider: string;
	sharingProviders: string;
	returnOnlyAtStations: string;
	roundtripStationReturnConstraint: string;
	rentalStation: string;
	rentalGeofencingZone: string;
	noItinerariesFound: string;
	advancedSearchOptions: string;
	selectTransitModes: string;
	defaultSelectedModes: string;
	defaultSelectedProviders: string;
	selectElevationCosts: string;
	wheelchair: string;
	useRoutedTransfers: string;
	bikeRental: string;
	requireBikeTransport: string;
	requireCarTransport: string;
	considerRentalReturnConstraints: string;
	default: string;
	timetableSources: string;
	tripCancelled: string;
	stopCancelled: string;
	inOutDisallowed: string;
	inDisallowed: string;
	outDisallowed: string;
	unscheduledTrip: string;
	alertsAvailable: string;
	dataExpiredSince: string;
	FLEX: string;
	WALK: string;
	BIKE: string;
	RENTAL: string;
	CAR: string;
	CAR_DROPOFF: string;
	CAR_PARKING: string;
	TRANSIT: string;
	TRAM: string;
	SUBWAY: string;
	FERRY: string;
	AIRPLANE: string;
	SUBURBAN: string;
	BUS: string;
	COACH: string;
	RAIL: string;
	HIGHSPEED_RAIL: string;
	LONG_DISTANCE: string;
	NIGHT_RAIL: string;
	REGIONAL_FAST_RAIL: string;
	REGIONAL_RAIL: string;
	OTHER: string;
	routingSegments: {
		maxTransfers: string;
		maxTravelTime: string;
		firstMile: string;
		lastMile: string;
		direct: string;
		maxPreTransitTime: string;
		maxPostTransitTime: string;
		maxDirectTime: string;
	};
	elevationCosts: {
		NONE: string;
		LOW: string;
		HIGH: string;
	};
	isochrones: {
		title: string;
		displayLevel: string;
		maxComputeLevel: string;
		canvasRects: string;
		canvasCircles: string;
		geojsonCircles: string;
		styling: string;
		noData: string;
		requestFailed: string;
	};
	alerts: {
		validFrom: string;
		until: string;
		information: string;
		more: string;
	};
	RENTAL_BICYCLE: string;
	RENTAL_CARGO_BICYCLE: string;
	RENTAL_CAR: string;
	RENTAL_MOPED: string;
	RENTAL_SCOOTER_STANDING: string;
	RENTAL_SCOOTER_SEATED: string;
	RENTAL_OTHER: string;
	incline: string;
	CABLE_CAR: string;
	FUNICULAR: string;
	AERIAL_LIFT: string;
	toll: string;
	accessRestriction: string;
	continuesAs: string;
	rent: string;
	copyToClipboard: string;
	rideThroughAllowed: string;
	rideThroughNotAllowed: string;
	rideEndAllowed: string;
	rideEndNotAllowed: string;
};

export const DEFAULT_LANGUAGE = 'en' as const;
const STORAGE_KEY = 'motis.language';

const translations = {
	bg,
	pl,
	en,
	de,
	fr,
	cs,
	sv
} satisfies Record<string, Translations>;

export type SupportedLanguage = keyof typeof translations;

const normalizeLanguage = (value: string | null | undefined): SupportedLanguage | undefined => {
	const normalized = value?.trim().toLowerCase().slice(0, 2);
	if (!normalized) {
		return undefined;
	}
	return normalized in translations ? (normalized as SupportedLanguage) : undefined;
};

const detectInitialLanguage = (): SupportedLanguage => {
	if (!browser) {
		return DEFAULT_LANGUAGE;
	}

	const urlLanguage = normalizeLanguage(new URLSearchParams(window.location.search).get('language'));
	if (urlLanguage) {
		return urlLanguage;
	}

	const storedLanguage = normalizeLanguage(window.localStorage.getItem(STORAGE_KEY));
	if (storedLanguage) {
		return storedLanguage;
	}

	for (const locale of navigator.languages) {
		const supported = normalizeLanguage(locale);
		if (supported) {
			return supported;
		}
	}

	return DEFAULT_LANGUAGE;
};

const collectMissingKeys = (
	reference: Record<string, unknown>,
	candidate: Record<string, unknown>,
	prefix = ''
): string[] => {
	const missing: string[] = [];
	for (const key of Object.keys(reference)) {
		const path = prefix ? `${prefix}.${key}` : key;
		if (!(key in candidate)) {
			missing.push(path);
			continue;
		}
		const refValue = reference[key];
		const candidateValue = candidate[key];
		if (
			refValue &&
			typeof refValue === 'object' &&
			!Array.isArray(refValue) &&
			candidateValue &&
			typeof candidateValue === 'object' &&
			!Array.isArray(candidateValue)
		) {
			missing.push(
				...collectMissingKeys(
					refValue as Record<string, unknown>,
					candidateValue as Record<string, unknown>,
					path
				)
			);
		}
	}
	return missing;
};

if (dev) {
	for (const [languageKey, translation] of Object.entries(translations)) {
		if (languageKey === DEFAULT_LANGUAGE) {
			continue;
		}
		const missingKeys = collectMissingKeys(
			en as Record<string, unknown>,
			translation as Record<string, unknown>
		);
		if (missingKeys.length > 0) {
			throw new Error(
				`[i18n] Missing translation keys for "${languageKey}": ${missingKeys.join(', ')}`
			);
		}
	}
}

const languageStore = writable<SupportedLanguage>(detectInitialLanguage());

if (browser) {
	languageStore.subscribe((nextLanguage) => {
		window.localStorage.setItem(STORAGE_KEY, nextLanguage);

		const url = new URL(window.location.href);
		const current = normalizeLanguage(url.searchParams.get('language'));
		if (current !== nextLanguage) {
			url.searchParams.set('language', nextLanguage);
			window.history.replaceState(window.history.state, '', `${url.pathname}${url.search}${url.hash}`);
		}
	});
}

export const supportedLanguages = Object.keys(translations) as SupportedLanguage[];

export const language = {
	subscribe: languageStore.subscribe
};

export const t = derived(languageStore, (languageKey) => translations[languageKey]);

export const getLanguage = (): SupportedLanguage => get(languageStore);

export const getTranslations = (): Translations => get(t);

export const setLanguage = (nextLanguage: string): SupportedLanguage => {
	const normalized = normalizeLanguage(nextLanguage) ?? DEFAULT_LANGUAGE;
	languageStore.set(normalized);
	return normalized;
};
