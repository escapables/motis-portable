import { describe, expect, it } from 'vitest';

import type { Location } from '$lib/Location';
import {
	deriveInitialActiveTab,
	formatInitError,
	nextColorMode,
	providerGroupsForQuery,
	resolveTabTransition,
	toPlaceString
} from './pageState';

const baseLocation = (overrides: Partial<Location> = {}): Location => ({
	label: '',
	...overrides
});

describe('pageState helpers', () => {
	it('derives initial active tab from query params', () => {
		expect(deriveInitialActiveTab(new URLSearchParams('one=1'))).toBe('isochrones');
		expect(deriveInitialActiveTab(new URLSearchParams('stopId=123'))).toBe('departures');
		expect(deriveInitialActiveTab(new URLSearchParams())).toBe('connections');
	});

	it('formats init errors with stage and message fields', () => {
		const message = formatInitError(
			{ stage: 'ipc', error: 'failed to start' },
			{ unknownInitializationError: 'unknown' }
		);
		expect(message).toBe('[ipc] failed to start');
	});

	it('converts stop locations to stop ids and place coordinates otherwise', () => {
		const stop = baseLocation({
			match: {
				type: 'STOP',
				id: 'de:123',
				name: 'X',
				lat: 1,
				lon: 2,
				score: 0,
				areas: [],
				tokens: []
			}
		});
		const coord = baseLocation({
			match: {
				type: 'PLACE',
				id: '',
				name: 'Y',
				lat: 48.1,
				lon: 11.2,
				level: 3,
				score: 0,
				areas: [],
				tokens: []
			}
		});
		expect(toPlaceString(stop)).toBe('de:123');
		expect(toPlaceString(coord)).toBe('48.1,11.2,3');
	});

	it('keeps provider groups only for rental mode mixes', () => {
		expect(providerGroupsForQuery(['WALK'], ['a', 'a', 'b'])).toEqual([]);
		expect(providerGroupsForQuery(['RENTAL_BICYCLE'], ['a', 'a', 'b'])).toEqual(['a', 'b']);
	});

	it('mirrors from/one values when switching into or out of isochrones', () => {
		const from = baseLocation({
			label: 'from',
			match: {
				type: 'PLACE',
				id: '',
				name: 'from',
				lat: 1,
				lon: 2,
				score: 0,
				areas: [],
				tokens: []
			}
		});
		const one = baseLocation({
			label: 'one',
			match: { type: 'PLACE', id: '', name: 'one', lat: 3, lon: 4, score: 0, areas: [], tokens: [] }
		});
		expect(resolveTabTransition('connections', 'isochrones', from, one).one).toBe(from);
		expect(resolveTabTransition('isochrones', 'connections', from, one).from).toBe(one);
	});

	it('cycles color mode in fixed order', () => {
		expect(nextColorMode('none')).toBe('rt');
		expect(nextColorMode('rt')).toBe('route');
		expect(nextColorMode('route')).toBe('mode');
		expect(nextColorMode('mode')).toBe('none');
	});
});
