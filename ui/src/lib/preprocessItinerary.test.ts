import { describe, expect, it } from 'vitest';
import { preprocessItinerary } from '$lib/preprocessItinerary';
import type { Error as ApiError, PlanResponse } from '@motis-project/motis-client';
import type { RequestResult } from '@hey-api/client-fetch';

const createLeg = (overrides: Record<string, unknown> = {}) => ({
	mode: 'WALK',
	displayName: 'walk',
	routeShortName: '',
	routeLongName: '',
	agencyId: '',
	agencyName: '',
	routeId: '',
	headsign: '',
	tripTo: undefined,
	interlineWithPreviousLeg: false,
	from: { name: 'A' },
	to: { name: 'B' },
	duration: 60,
	startTime: '2026-02-14T08:00:00Z',
	endTime: '2026-02-14T08:01:00Z',
	scheduledStartTime: '2026-02-14T08:00:00Z',
	scheduledEndTime: '2026-02-14T08:01:00Z',
	realTime: false,
	intermediateStops: [],
	legGeometry: {
		points: '',
		precision: 5,
		length: 0
	},
	...overrides
});

const createItinerary = (legOverrides: Record<string, unknown>) => ({
	legs: [createLeg(legOverrides)]
});

const asPlanRequestResult = (
	data: PlanResponse
): Awaited<RequestResult<PlanResponse, ApiError, false>> =>
	({
		error: undefined,
		data
	}) as Awaited<RequestResult<PlanResponse, ApiError, false>>;

describe('preprocessItinerary city transit normalization', () => {
	it('normalizes Stockholm local rail misclassified as SUBURBAN to RAIL', () => {
		const transform = preprocessItinerary(
			{ label: 'Start', match: undefined },
			{ label: 'End', match: undefined }
		);

		const result = transform(
			asPlanRequestResult({
				itineraries: [
					createItinerary({
						mode: 'SUBURBAN',
						routeShortName: '27',
						agencyId: '500000000000000114',
						headsign: 'Österskär'
					})
				],
				direct: []
			} as unknown as PlanResponse)
		);

		expect(result.itineraries[0].legs[0].mode).toBe('RAIL');
		expect(result.itineraries[0].legs[0].displayName).toContain('Roslagsbanan 27');
	});

	it('normalizes Gothenburg tram misclassified as BUS to TRAM', () => {
		const transform = preprocessItinerary(
			{ label: 'Start', match: undefined },
			{ label: 'End', match: undefined }
		);

		const result = transform(
			asPlanRequestResult({
				itineraries: [
					createItinerary({
						mode: 'BUS',
						routeShortName: '6',
						agencyName: 'Västtrafik',
						routeLongName: 'Spårvagn',
						headsign: 'Kortedala'
					})
				],
				direct: []
			} as unknown as PlanResponse)
		);

		expect(result.itineraries[0].legs[0].mode).toBe('TRAM');
		expect(result.itineraries[0].legs[0].displayName).toBe('Spårvagn 6 mot Kortedala');
	});
});
