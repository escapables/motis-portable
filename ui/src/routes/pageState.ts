import { lngLatToStr } from '$lib/lngLatToStr';
import type { Location } from '$lib/Location';
import type { PrePostDirectMode } from '$lib/Modes';

export type ActiveTab = 'connections' | 'departures' | 'isochrones';
export type ColorMode = 'rt' | 'route' | 'mode' | 'none';

type InitErrorTranslations = {
	unknownInitializationError: string;
};

export const DEFAULT_ISOCHRONES_MAX_POST_TRANSIT_TIME = 5 * 60;

const OSM_OBJECT_ID_REGEX = /^(node|way|relation)\/\[\d+\]$/;

export const deriveInitialActiveTab = (urlParams: URLSearchParams | undefined): ActiveTab => {
	if (urlParams?.has('one')) {
		return 'isochrones';
	}
	if (urlParams?.has('stopId')) {
		return 'departures';
	}
	return 'connections';
};

export const parseIntOr = <T>(s: string | null | undefined, d: T): T | number => {
	if (!s) {
		return d;
	}
	const value = parseInt(s);
	return Number.isNaN(value) ? d : value;
};

export const formatInitError = (error: unknown, translations: InitErrorTranslations): string => {
	if (error == null) {
		return translations.unknownInitializationError;
	}
	if (typeof error === 'string') {
		return error;
	}
	if (error instanceof Error) {
		return error.message;
	}
	if (typeof error === 'object') {
		const value = error as Record<string, unknown>;
		const stage = typeof value.stage === 'string' ? `[${value.stage}] ` : '';
		const message =
			(typeof value.error === 'string' && value.error) ||
			(typeof value.message === 'string' && value.message) ||
			'';
		if (message) {
			return `${stage}${message}`;
		}
		try {
			return `${stage}${JSON.stringify(value)}`;
		} catch {
			return `${stage}${String(error)}`;
		}
	}
	return String(error);
};

export const toPlaceString = (location: Location): string => {
	if (!location.match) {
		return '';
	}
	const id = location.match.id?.trim() ?? '';
	const isResolvableStopId =
		location.match.type === 'STOP' && id.length > 0 && !OSM_OBJECT_ID_REGEX.test(id);
	if (isResolvableStopId) {
		return id;
	}
	if (location.match.level != undefined) {
		return `${lngLatToStr(location.match)},${location.match.level}`;
	}
	return `${lngLatToStr(location.match)}`;
};

export const providerGroupsForQuery = (modes: PrePostDirectMode[], groups: string[]): string[] => {
	if (!modes.some((mode) => mode.startsWith('RENTAL_'))) {
		return [];
	}
	return Array.from(new Set(groups));
};

export const resolveTabTransition = (
	previousTab: ActiveTab,
	nextTab: ActiveTab,
	from: Location,
	one: Location
): { from: Location; one: Location } => {
	if (nextTab === 'isochrones' && from?.match) {
		return { from, one: from };
	}
	if (previousTab === 'isochrones' && nextTab !== 'isochrones' && one?.match) {
		return { from: one, one };
	}
	return { from, one };
};

export const nextColorMode = (colorMode: ColorMode): ColorMode => {
	switch (colorMode) {
		case 'rt':
			return 'route';
		case 'route':
			return 'mode';
		case 'mode':
			return 'none';
		case 'none':
			return 'rt';
	}
};
