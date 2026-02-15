import type { Itinerary } from '@motis-project/motis-client';
import type { Location } from '$lib/Location';
import maplibregl from 'maplibre-gl';

export const flyToItineraries = (
	itineraries: Itinerary[],
	map: maplibregl.Map,
	isSmallScreen: boolean
) => {
	const start = maplibregl.LngLat.convert(itineraries[0].legs[0].from);
	const box = new maplibregl.LngLatBounds(start, start);
	itineraries.forEach((itinerary) => {
		itinerary.legs.forEach((leg) => {
			box.extend(leg.from);
			box.extend(leg.to);
			leg.intermediateStops?.forEach((stop) => {
				box.extend(stop);
			});
		});
	});
	map.flyTo({
		...map.cameraForBounds(box, {
			padding: {
				top: 96,
				right: 96,
				bottom: isSmallScreen ? window.innerHeight * 0.3 : 96,
				left: isSmallScreen ? 96 : 640
			}
		})
	});
};

export const flyToLocation = (map: maplibregl.Map | undefined, location: Location) => {
	map?.flyTo({ center: location.match, zoom: 11 });
};
