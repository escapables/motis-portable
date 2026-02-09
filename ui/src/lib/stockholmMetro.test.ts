import { describe, expect, it } from 'vitest';
import {
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
});
