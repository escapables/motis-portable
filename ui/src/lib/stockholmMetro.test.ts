import { describe, expect, it } from 'vitest';
import {
	getStockholmBusInfo,
	getGothenburgTramDisplayName,
	getGothenburgTramInfo,
	getStockholmMetroDisplayName,
	getStockholmMetroInfo,
	getStockholmRailDisplayName,
	getStockholmRailInfo
} from '$lib/stockholmMetro';

describe('city transit normalization helpers', () => {
	it('detects Stockholm metro lines even when misclassified as ferry', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '14',
			agencyName: 'Storstockholms Lokaltrafik'
		} as const;

		const info = getStockholmMetroInfo(leg);
		expect(info?.line).toBe('14');
		expect(getStockholmMetroDisplayName({ ...leg, headsign: 'Mörby centrum' })).toBe(
			'14 mot Mörby centrum'
		);
	});

	it('detects Stockholm local rail lines and display naming', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '27',
			agencyId: '500000000000000114',
			headsign: 'Österskär'
		} as const;

		const info = getStockholmRailInfo(leg);
		expect(info?.serviceName).toBe('Roslagsbanan');
		expect(getStockholmRailDisplayName(leg)).toContain('Roslagsbanan 27');
	});

	it('detects Stockholm local rail lines with suffix variants (e.g. 28S)', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '28S',
			agencyId: '500000000000000114',
			headsign: 'Österskär'
		} as const;

		const info = getStockholmRailInfo(leg);
		expect(info?.serviceName).toBe('Roslagsbanan');
		expect(getStockholmRailDisplayName(leg)).toContain('Roslagsbanan 28');
	});

	it('detects Stockholm bus lines misclassified as ferry', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '4',
			agencyId: '500000000000000114',
			headsign: 'Radiohuset'
		} as const;

		const info = getStockholmBusInfo(leg);
		expect(info?.line).toBe('4');
	});

	it('does not remap known Stockholm ferry lines to bus', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '82',
			agencyId: '500000000000000114',
			headsign: 'Djurgården'
		} as const;

		expect(getStockholmBusInfo(leg)).toBeUndefined();
	});

	it('detects Gothenburg tram lines in Västtrafik context', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '6',
			agencyName: 'Västtrafik',
			headsign: 'Kortedala'
		} as const;

		const info = getGothenburgTramInfo(leg);
		expect(info?.line).toBe('6');
		expect(getGothenburgTramDisplayName(leg)).toBe('Spårvagn 6 mot Kortedala');
	});

	it('does not remap unrelated ferry legs to tram', () => {
		const leg = {
			mode: 'FERRY',
			routeShortName: '6',
			agencyName: 'Waxholmsbolaget',
			headsign: 'Vaxholm'
		} as const;

		expect(getGothenburgTramInfo(leg)).toBeUndefined();
	});

	it('detects Gothenburg tram legs misclassified as bus when tram text is present', () => {
		const leg = {
			mode: 'BUS',
			routeShortName: '6',
			agencyName: 'Västtrafik',
			routeLongName: 'Spårvagn',
			headsign: 'Kortedala'
		} as const;

		const info = getGothenburgTramInfo(leg);
		expect(info?.line).toBe('6');
		expect(getGothenburgTramDisplayName(leg)).toBe('Spårvagn 6 mot Kortedala');
	});

	it('does not remap Gothenburg bus legs without tram text', () => {
		const leg = {
			mode: 'BUS',
			routeShortName: '16',
			agencyName: 'Västtrafik',
			routeLongName: 'Buss',
			headsign: 'Eketrägatan'
		} as const;

		expect(getGothenburgTramInfo(leg)).toBeUndefined();
	});
});
