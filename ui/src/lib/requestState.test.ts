import { describe, expect, it, vi } from 'vitest';
import {
	buildConnectionsQueryState,
	buildIsochronesQueryState,
	createDebouncedRunner
} from './requestState';

describe('buildConnectionsQueryState', () => {
	it('omits names when labels match place strings', () => {
		const query = {
			fromPlace: 'from-stop',
			toPlace: 'to-stop',
			time: '2026-02-14T10:00:00.000Z'
		};
		expect(buildConnectionsQueryState(query, 'from-stop', 'to-stop')).toEqual(query);
	});

	it('adds names when labels differ from place strings', () => {
		expect(
			buildConnectionsQueryState(
				{
					fromPlace: 'from-stop',
					toPlace: 'to-stop',
					time: '2026-02-14T10:00:00.000Z'
				},
				'From Label',
				'To Label'
			)
		).toEqual({
			fromPlace: 'from-stop',
			toPlace: 'to-stop',
			time: '2026-02-14T10:00:00.000Z',
			fromName: 'From Label',
			toName: 'To Label'
		});
	});
});

describe('buildIsochronesQueryState', () => {
	it('converts maxTravelTime to seconds and carries display options', () => {
		expect(
			buildIsochronesQueryState(
				{
					one: 'stop-a',
					maxTravelTime: 5,
					time: '2026-02-14T10:00:00.000Z',
					transitModes: ['TRANSIT'],
					maxTransfers: 1,
					arriveBy: false
				},
				{
					oneName: 'Stop A',
					isochronesColor: '#ff0',
					isochronesOpacity: 66,
					isochronesDisplayLevel: 'OVERLAY_CIRCLES',
					isochronesCircleResolution: 6
				}
			)
		).toEqual({
			one: 'stop-a',
			maxTravelTime: 300,
			time: '2026-02-14T10:00:00.000Z',
			transitModes: ['TRANSIT'],
			maxTransfers: 1,
			arriveBy: false,
			oneName: 'Stop A',
			isochronesColor: '#ff0',
			isochronesOpacity: 66,
			isochronesDisplayLevel: 'OVERLAY_CIRCLES',
			isochronesCircleResolution: 6
		});
	});

	it('omits circle resolution when <= 2 and one name when unchanged', () => {
		expect(
			buildIsochronesQueryState(
				{
					one: 'Same',
					maxTravelTime: 3,
					time: '2026-02-14T10:00:00.000Z',
					transitModes: ['TRANSIT'],
					maxTransfers: 1,
					arriveBy: false
				},
				{
					oneName: 'Same',
					isochronesColor: '#ff0',
					isochronesOpacity: 66,
					isochronesDisplayLevel: 'OVERLAY_CIRCLES',
					isochronesCircleResolution: 2
				}
			)
		).toEqual({
			one: 'Same',
			maxTravelTime: 180,
			time: '2026-02-14T10:00:00.000Z',
			transitModes: ['TRANSIT'],
			maxTransfers: 1,
			arriveBy: false,
			isochronesColor: '#ff0',
			isochronesOpacity: 66,
			isochronesDisplayLevel: 'OVERLAY_CIRCLES'
		});
	});
});

describe('createDebouncedRunner', () => {
	it('keeps only the latest scheduled callback and supports clear', () => {
		vi.useFakeTimers();
		const run = vi.fn();
		const debounced = createDebouncedRunner();

		debounced.schedule(100, run);
		debounced.schedule(100, run);
		vi.advanceTimersByTime(99);
		expect(run).not.toHaveBeenCalled();
		vi.advanceTimersByTime(1);
		expect(run).toHaveBeenCalledTimes(1);

		debounced.schedule(100, run);
		debounced.clear();
		vi.advanceTimersByTime(100);
		expect(run).toHaveBeenCalledTimes(1);

		vi.useRealTimers();
	});
});
