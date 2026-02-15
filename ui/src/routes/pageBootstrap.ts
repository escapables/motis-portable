import { initial, type ServerConfig } from '@motis-project/motis-client';
import { sanitizeExternalHttpUrl } from '$lib/utils';
import { onClickStop, onClickTrip } from '$lib/utils';

export type InitialStateResult = {
	dataAttributionLink: string | undefined;
	center: [number, number] | undefined;
	zoom: number | undefined;
	serverConfig: ServerConfig | undefined;
	initError: string | undefined;
};

export const requestInitialState = async (
	formatError: (error: unknown) => string,
	noInitializationData: string
): Promise<InitialStateResult> => {
	try {
		const d = await initial();
		if (d.error) {
			const status = d.response.status;
			const formatted = formatError(d.error);
			return {
				dataAttributionLink: undefined,
				center: undefined,
				zoom: undefined,
				serverConfig: undefined,
				initError: status ? `${formatted} (HTTP ${status})` : formatted
			};
		}

		const dataAttributionLink = d.response.headers.has('Link')
			? sanitizeExternalHttpUrl(
					d.response.headers.get('Link')!.replace(/^<(.*)>; rel="license"$/, '$1')
				)
			: undefined;
		const r = d.data;
		if (!r) {
			return {
				dataAttributionLink,
				center: undefined,
				zoom: undefined,
				serverConfig: undefined,
				initError: noInitializationData
			};
		}
		return {
			dataAttributionLink,
			center: [r.lon, r.lat],
			zoom: r.zoom,
			serverConfig: r.serverConfig,
			initError: undefined
		};
	} catch (error) {
		return {
			dataAttributionLink: undefined,
			center: undefined,
			zoom: undefined,
			serverConfig: undefined,
			initError: formatError(error)
		};
	}
};

export const applyPageStateFromURL = (urlParams: URLSearchParams | undefined): void => {
	if (!urlParams) {
		return;
	}

	const tripId = urlParams.get('tripId');
	if (tripId !== null) {
		onClickTrip(tripId, true);
	}

	const stopId = urlParams.get('stopId');
	if (stopId !== null) {
		const time = urlParams.has('time') ? new Date(urlParams.get('time')!) : new Date();
		onClickStop('', stopId, time, urlParams.get('stopArriveBy') == 'true', true);
	}
};
