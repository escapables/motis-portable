import type { Mode, Place } from '@motis-project/motis-client';

type MetroLine = '10' | '11' | '13' | '14' | '17' | '18' | '19';
type RailLine = '25' | '26' | '27' | '28' | '29';
type GothenburgTramLine = '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '10' | '11' | '13';

type StockholmMetroLegLike = {
	mode?: Mode;
	routeShortName?: string;
	routeLongName?: string;
	tripShortName?: string;
	displayName?: string;
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

type StockholmRailInfo = {
	line?: RailLine;
	serviceName: 'Saltsjöbanan' | 'Roslagsbanan';
	color: string;
	direction?: string;
};

type StockholmBusInfo = {
	line: string;
	direction?: string;
};

type GothenburgTramInfo = {
	line?: GothenburgTramLine;
	serviceName: 'Göteborgs spårvagn';
	color: string;
	direction?: string;
};

const STOCKHOLM_SL_AGENCY_IDS = new Set(['500000000000000114', '505000000000000001']);
const STOCKHOLM_FERRY_ROUTE_SHORT_NAMES = new Set(['80', '82', '83', '84', '85', '89']);
const VASTTRAFIK_AGENCY_NAME_MATCH = 'vasttrafik';
const GOTHENBURG_TRAM_COLOR = '#0069b4';

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

const RAIL_LINE_INFO: Record<
	RailLine,
	{ serviceName: StockholmRailInfo['serviceName']; color: string }
> = {
	'25': { serviceName: 'Saltsjöbanan', color: '#f44336' },
	'26': { serviceName: 'Saltsjöbanan', color: '#f44336' },
	'27': { serviceName: 'Roslagsbanan', color: '#f44336' },
	'28': { serviceName: 'Roslagsbanan', color: '#f44336' },
	'29': { serviceName: 'Roslagsbanan', color: '#f44336' }
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

const parseRailLine = (routeShortName?: string): RailLine | undefined => {
	const line = routeShortName?.trim();
	const normalized = line?.match(/^(\d{2})(?:[A-Za-z])?$/)?.[1] ?? line;
	if (
		normalized === '25' ||
		normalized === '26' ||
		normalized === '27' ||
		normalized === '28' ||
		normalized === '29'
	) {
		return normalized;
	}
	return undefined;
};

const parseGothenburgTramLine = (routeShortName?: string): GothenburgTramLine | undefined => {
	const line = routeShortName?.trim();
	if (
		line === '1' ||
		line === '2' ||
		line === '3' ||
		line === '4' ||
		line === '5' ||
		line === '6' ||
		line === '7' ||
		line === '8' ||
		line === '9' ||
		line === '10' ||
		line === '11' ||
		line === '13'
	) {
		return line;
	}
	return undefined;
};

const normalizeText = (text?: string): string | undefined => {
	const normalized = text?.trim();
	return normalized ? normalized : undefined;
};

const normalizeForMatch = (text?: string): string | undefined => {
	const normalized = normalizeText(text);
	if (!normalized) {
		return undefined;
	}
	return normalized
		.toLowerCase()
		.normalize('NFD')
		.replace(/[\u0300-\u036f]/g, '');
};

const isStockholmSlContext = (leg: StockholmMetroLegLike): boolean => {
	if (leg.agencyId && STOCKHOLM_SL_AGENCY_IDS.has(leg.agencyId)) {
		return true;
	}
	if (leg.routeId?.startsWith('9011114')) {
		return true;
	}
	const name = normalizeForMatch(leg.agencyName);
	return Boolean(name && (name.includes('storstockholm') || name === '114' || name === 'sl'));
};

const isVasttrafikContext = (leg: StockholmMetroLegLike): boolean => {
	if (leg.agencyId?.toLowerCase().includes(VASTTRAFIK_AGENCY_NAME_MATCH)) {
		return true;
	}
	if (leg.routeId?.toLowerCase().includes(VASTTRAFIK_AGENCY_NAME_MATCH)) {
		return true;
	}
	const agencyName = normalizeForMatch(leg.agencyName);
	return Boolean(agencyName && agencyName.includes(VASTTRAFIK_AGENCY_NAME_MATCH));
};

const inferRailServiceByText = (
	leg: Pick<StockholmMetroLegLike, 'routeLongName' | 'displayName' | 'headsign' | 'tripShortName'>
): StockholmRailInfo['serviceName'] | undefined => {
	const fields = [leg.routeLongName, leg.displayName, leg.headsign, leg.tripShortName];
	for (const field of fields) {
		const normalized = normalizeForMatch(field);
		if (!normalized) {
			continue;
		}
		if (normalized.includes('roslagsbanan')) {
			return 'Roslagsbanan';
		}
		if (normalized.includes('saltsjobanan')) {
			return 'Saltsjöbanan';
		}
	}
	return undefined;
};

const inferGothenburgTramByText = (
	leg: Pick<StockholmMetroLegLike, 'routeLongName' | 'displayName' | 'headsign' | 'tripShortName'>
): boolean => {
	const fields = [leg.routeLongName, leg.displayName, leg.headsign, leg.tripShortName];
	for (const field of fields) {
		const normalized = normalizeForMatch(field);
		if (!normalized) {
			continue;
		}
		if (normalized.includes('sparvagn') || normalized.includes('tram')) {
			return true;
		}
	}
	return false;
};

export const getStockholmMetroInfo = (
	leg: StockholmMetroLegLike
): StockholmMetroInfo | undefined => {
	if (!leg.mode || (leg.mode !== 'SUBWAY' && leg.mode !== 'FERRY')) {
		return undefined;
	}

	const line = parseMetroLine(leg.routeShortName);
	if (!line) {
		return undefined;
	}

	if (leg.mode === 'FERRY' && !isStockholmSlContext(leg)) {
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
	return info.direction ? `${info.line} mot ${info.direction}` : info.line;
};

export const getStockholmRailInfo = (leg: StockholmMetroLegLike): StockholmRailInfo | undefined => {
	if (!leg.mode || (leg.mode !== 'FERRY' && leg.mode !== 'RAIL' && leg.mode !== 'SUBURBAN')) {
		return undefined;
	}

	if (!isStockholmSlContext(leg)) {
		return undefined;
	}

	const line = parseRailLine(leg.routeShortName);
	const lineMeta = line ? RAIL_LINE_INFO[line] : undefined;
	const serviceName = lineMeta?.serviceName ?? inferRailServiceByText(leg);
	if (!serviceName) {
		return undefined;
	}

	const direction = normalizeText(leg.tripTo?.name) ?? normalizeText(leg.headsign);
	return {
		line,
		serviceName,
		color: lineMeta?.color ?? '#f44336',
		direction
	};
};

export const getStockholmRailDisplayName = (leg: StockholmMetroLegLike): string | undefined => {
	const info = getStockholmRailInfo(leg);
	if (!info) {
		return undefined;
	}
	const line = info.line ? ` ${info.line}` : '';
	return info.direction
		? `${info.serviceName}${line} mot ${info.direction}`
		: `${info.serviceName}${line}`;
};

export const getStockholmBusInfo = (leg: StockholmMetroLegLike): StockholmBusInfo | undefined => {
	if (leg.mode !== 'FERRY' || !isStockholmSlContext(leg)) {
		return undefined;
	}

	const line = normalizeText(leg.routeShortName);
	if (!line || STOCKHOLM_FERRY_ROUTE_SHORT_NAMES.has(line)) {
		return undefined;
	}

	// Metro and local rail are handled by dedicated normalizers.
	if (parseMetroLine(line) || parseRailLine(line)) {
		return undefined;
	}

	const direction = normalizeText(leg.tripTo?.name) ?? normalizeText(leg.headsign);
	return { line, direction };
};

export const getStockholmBusDisplayName = (leg: StockholmMetroLegLike): string | undefined => {
	const info = getStockholmBusInfo(leg);
	if (!info) {
		return undefined;
	}
	return info.direction ? `${info.line} mot ${info.direction}` : info.line;
};

export const getGothenburgTramInfo = (
	leg: StockholmMetroLegLike
): GothenburgTramInfo | undefined => {
	if (!leg.mode || (leg.mode !== 'FERRY' && leg.mode !== 'TRAM')) {
		return undefined;
	}

	if (leg.mode === 'FERRY' && !isVasttrafikContext(leg)) {
		return undefined;
	}

	const line = parseGothenburgTramLine(leg.routeShortName);
	if (!line && !inferGothenburgTramByText(leg)) {
		return undefined;
	}

	const direction = normalizeText(leg.tripTo?.name) ?? normalizeText(leg.headsign);
	return {
		line,
		serviceName: 'Göteborgs spårvagn',
		color: GOTHENBURG_TRAM_COLOR,
		direction
	};
};

export const getGothenburgTramDisplayName = (leg: StockholmMetroLegLike): string | undefined => {
	const info = getGothenburgTramInfo(leg);
	if (!info) {
		return undefined;
	}
	if (info.line) {
		return info.direction ? `Spårvagn ${info.line} mot ${info.direction}` : `Spårvagn ${info.line}`;
	}
	return info.direction ? `${info.serviceName} mot ${info.direction}` : info.serviceName;
};
