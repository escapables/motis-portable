import type { Mode, Place } from '@motis-project/motis-client';

type MetroLine = '10' | '11' | '13' | '14' | '17' | '18' | '19';

type StockholmMetroLegLike = {
	mode?: Mode;
	routeShortName?: string;
	agencyId?: string;
	agencyName?: string;
	routeId?: string;
	headsign?: string;
	tripTo?: Place;
};

type StockholmMetroInfo = {
	line: MetroLine;
	lineName: 'Blå linje' | 'Röda linjen' | 'Gröna linjen';
	color: string;
	direction?: string;
};

const STOCKHOLM_METRO_AGENCY_IDS = new Set(['500000000000000114', '505000000000000001']);

const METRO_LINE_INFO: Record<
	MetroLine,
	{ lineName: StockholmMetroInfo['lineName']; color: string }
> = {
	'10': { lineName: 'Blå linje', color: '#005AA3' },
	'11': { lineName: 'Blå linje', color: '#005AA3' },
	'13': { lineName: 'Röda linjen', color: '#E31E24' },
	'14': { lineName: 'Röda linjen', color: '#E31E24' },
	'17': { lineName: 'Gröna linjen', color: '#009540' },
	'18': { lineName: 'Gröna linjen', color: '#009540' },
	'19': { lineName: 'Gröna linjen', color: '#009540' }
};

const parseMetroLine = (routeShortName?: string): MetroLine | undefined => {
	const line = routeShortName?.trim();
	if (
		line === '10' ||
		line === '11' ||
		line === '13' ||
		line === '14' ||
		line === '17' ||
		line === '18' ||
		line === '19'
	) {
		return line;
	}
	return undefined;
};

const normalizeText = (text?: string): string | undefined => {
	const normalized = text?.trim();
	return normalized ? normalized : undefined;
};

const isStockholmMetroAgency = (leg: StockholmMetroLegLike): boolean => {
	if (leg.agencyId && STOCKHOLM_METRO_AGENCY_IDS.has(leg.agencyId)) {
		return true;
	}
	if (leg.routeId?.startsWith('9011114')) {
		return true;
	}
	const name = leg.agencyName?.toLowerCase();
	return Boolean(name && (name.includes('storstockholm') || name === '114'));
};

export const getStockholmMetroInfo = (leg: StockholmMetroLegLike): StockholmMetroInfo | undefined => {
	if (!leg.mode || (leg.mode !== 'SUBWAY' && leg.mode !== 'FERRY')) {
		return undefined;
	}

	const line = parseMetroLine(leg.routeShortName);
	if (!line) {
		return undefined;
	}

	if (leg.mode === 'FERRY' && !isStockholmMetroAgency(leg)) {
		return undefined;
	}

	const direction = normalizeText(leg.tripTo?.name) ?? normalizeText(leg.headsign);
	const meta = METRO_LINE_INFO[line];
	return {
		line,
		lineName: meta.lineName,
		color: meta.color,
		direction
	};
};

export const getStockholmMetroDisplayName = (leg: StockholmMetroLegLike): string | undefined => {
	const info = getStockholmMetroInfo(leg);
	if (!info) {
		return undefined;
	}
	return info.direction
		? `${info.lineName} ${info.line} mot ${info.direction}`
		: `${info.lineName} ${info.line}`;
};
