<script lang="ts">
	import { getStyle } from '$lib/map/style';
	import type { Location } from '$lib/Location';
	import {
		oneToAll,
		plan,
		type OneToAllData,
		type PlanResponse,
		type PlanData,
		type ServerConfig
	} from '@motis-project/motis-client';
	import maplibregl from 'maplibre-gl';
	import { browser } from '$app/environment';
	import { pushStateWithQueryString } from '$lib/utils';
	import { client } from '@motis-project/motis-client';
	import { onMount, tick, untrack } from 'svelte';
	import { getTranslations } from '$lib/i18n/translation';
	import { page } from '$app/state';
	import { preprocessItinerary } from '$lib/preprocessItinerary';
	import type { IsochronesOptions, IsochronesPos } from '$lib/map/IsochronesShared';
	import { defaultQuery } from '$lib/defaults';
	import {
		buildConnectionsQueryState,
		buildIsochronesQueryState,
		createDebouncedRunner
	} from '$lib/requestState';
	import { deriveInitialActiveTab, formatInitError, parseIntOr, type ActiveTab } from './pageState';
	import { flyToItineraries, flyToLocation } from './pageMap';
	import { requestInitialState, applyPageStateFromURL } from './pageBootstrap';
	import { registerDebugPlanClipboardImport } from './pageDebugImport';
	import { deriveInitialPlannerState } from './pagePlannerState';
	import {
		createBaseQuery,
		createIsochronesQuery,
		deriveMaxPostTransitTimeDefault,
		mapReachablePlaces
	} from './pageQueries';
	import { applyNativeTouchpadPinch, type NativeTouchpadPinchDetail } from './nativeTouchpadPinch';
	import MapShell from './MapShell.svelte';
	import ResultContent from './ResultContent.svelte';
	const urlParams = browser ? new URLSearchParams(window.location.search) : undefined;
	const hasDebug: boolean = Boolean(urlParams?.has('debug'));
	const hasDark: boolean = Boolean(urlParams?.has('dark'));
	const hasLight: boolean = Boolean(urlParams?.has('light'));
	const isSmallScreen = browser && window.innerWidth < 768;
	let activeTab = $derived<ActiveTab>(page.state.activeTab ?? deriveInitialActiveTab(urlParams));
	let dataAttributionLink: string | undefined = $state(undefined);
	let showMap = $state(!isSmallScreen);
	type ConnectionsMapClickTarget = 'from' | 'to';
	let mapClickTarget = $state<ConnectionsMapClickTarget | undefined>(undefined);
	let lastOneToAllQuery: OneToAllData | undefined = undefined;
	let lastPlanQuery: PlanData | undefined = undefined;
	let serverConfig: ServerConfig | undefined = $state();
	let dataLoaded: boolean = $state(false);
	let initLoading: boolean = $state(true);
	let initError: string | undefined = $state(undefined);
	let theme: 'light' | 'dark' =
		(hasDark ? 'dark' : hasLight ? 'light' : undefined) ??
		(browser && window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches
			? 'dark'
			: 'light');
	if (theme === 'dark') {
		document.documentElement.classList.add('dark');
	}
	let center = $state.raw<[number, number]>([2.258882912876089, 48.72559118651327]);
	let level = $state(0);
	let zoom = $state(15);
	let bounds = $state<maplibregl.LngLatBoundsLike>();
	let map = $state<maplibregl.Map>();
	let nativePinchLastScale = $state<number | undefined>(undefined);
	let style = $derived(
		browser
			? getStyle(
					theme,
					level,
					window.location.origin + window.location.pathname,
					client.getConfig().baseUrl
						? client.getConfig().baseUrl + '/'
						: window.location.origin + window.location.pathname
				)
			: undefined
	);
	const handleNativeTouchpadPinch = (event: Event) => {
		const detail = (event as CustomEvent<NativeTouchpadPinchDetail>).detail;
		nativePinchLastScale = applyNativeTouchpadPinch(
			detail,
			map,
			nativePinchLastScale,
			document.elementFromPoint.bind(document)
		);
	};
	const loadInitialState = async () => {
		initLoading = true;
		initError = undefined;
		const t = getTranslations();
		const state = await requestInitialState(
			(error) => formatInitError(error, t),
			t.noInitializationData
		);
		dataAttributionLink = state.dataAttributionLink;
		if (state.center) {
			center = state.center;
		}
		if (state.zoom != undefined) {
			zoom = state.zoom;
		}
		serverConfig = state.serverConfig;
		initError = state.initError;
		if (!state.initError) {
			await tick();
			applyPageStateFromURL(urlParams);
		}
		dataLoaded = true;
		initLoading = false;
	};
	onMount(() => loadInitialState());
	onMount(() => {
		window.addEventListener('__motis_touchpad_pinch', handleNativeTouchpadPinch);
		return () => {
			window.removeEventListener('__motis_touchpad_pinch', handleNativeTouchpadPinch);
		};
	});
	let stopMarker = $state<maplibregl.Marker>();
	const initialPlannerState = deriveInitialPlannerState(urlParams);
	let from = $state<Location>(initialPlannerState.from);
	let to = $state<Location>(initialPlannerState.to);
	let one = $state<Location>(initialPlannerState.one);
	let stop = $state<Location>();
	let time = $state<Date>(initialPlannerState.time);
	let timetableView = $state(initialPlannerState.timetableView);
	let searchWindow = $state(initialPlannerState.searchWindow);
	let numItineraries = $state(initialPlannerState.numItineraries);
	let maxItineraries = $state(initialPlannerState.maxItineraries);
	let arriveBy = $state<boolean>(initialPlannerState.arriveBy);
	let algorithm = $state<PlanData['query']['algorithm']>(initialPlannerState.algorithm);
	let useRoutedTransfers = $state(initialPlannerState.useRoutedTransfers);
	let pedestrianProfile = $state(initialPlannerState.pedestrianProfile);
	let requireBikeTransport = $state(initialPlannerState.requireBikeTransport);
	let requireCarTransport = $state(initialPlannerState.requireCarTransport);
	let transitModes = $state(initialPlannerState.transitModes);
	let preTransitModes = $state(initialPlannerState.preTransitModes);
	let postTransitModes = $state(initialPlannerState.postTransitModes);
	let directModes = $state(initialPlannerState.directModes);
	let preTransitProviderGroups = $state<string[]>(initialPlannerState.preTransitProviderGroups);
	let postTransitProviderGroups = $state<string[]>(initialPlannerState.postTransitProviderGroups);
	let directProviderGroups = $state<string[]>(initialPlannerState.directProviderGroups);
	let elevationCosts = $state(initialPlannerState.elevationCosts);
	let maxTransfers = $state<number>(initialPlannerState.maxTransfers);
	let maxTravelTime = $derived<number>(
		parseIntOr(
			urlParams?.get('maxTravelTime'),
			Math.min(
				defaultQuery.maxTravelTime,
				60 * (serverConfig?.maxOneToAllTravelTimeLimit ?? Infinity)
			)
		)
	);
	let maxPreTransitTime = $derived<number>(
		parseIntOr(
			urlParams?.get('maxPreTransitTime'),
			Math.min(defaultQuery.maxPreTransitTime, serverConfig?.maxPrePostTransitTimeLimit ?? Infinity)
		)
	);
	let maxPostTransitTime = $derived<number>(
		parseIntOr(
			urlParams?.get('maxPostTransitTime'),
			deriveMaxPostTransitTimeDefault(activeTab, serverConfig?.maxPrePostTransitTimeLimit)
		)
	);
	let maxDirectTime = $derived<number>(
		parseIntOr(
			urlParams?.get('maxDirectTime'),
			Math.min(defaultQuery.maxDirectTime, serverConfig?.maxDirectTimeLimit ?? Infinity)
		)
	);
	let ignorePreTransitRentalReturnConstraints = $state(
		initialPlannerState.ignorePreTransitRentalReturnConstraints
	);
	let ignorePostTransitRentalReturnConstraints = $state(
		initialPlannerState.ignorePostTransitRentalReturnConstraints
	);
	let ignoreDirectRentalReturnConstraints = $state(
		initialPlannerState.ignoreDirectRentalReturnConstraints
	);
	let slowDirect = $state(initialPlannerState.slowDirect);

	let isochronesData = $state<IsochronesPos[]>([]);
	let isochronesOptions = $state<IsochronesOptions>(initialPlannerState.isochronesOptions);
	const isochronesCircleResolution = initialPlannerState.isochronesCircleResolution;

	let baseQuery = $derived(
		createBaseQuery({
			from,
			to,
			time,
			arriveBy,
			timetableView,
			searchWindow,
			numItineraries,
			maxItineraries,
			slowDirect,
			pedestrianProfile,
			transitModes,
			preTransitModes,
			postTransitModes,
			directModes,
			preTransitProviderGroups,
			postTransitProviderGroups,
			directProviderGroups,
			requireBikeTransport,
			requireCarTransport,
			elevationCosts,
			useRoutedTransfers,
			maxTransfers,
			maxPreTransitTime,
			maxPostTransitTime,
			maxDirectTime,
			ignorePreTransitRentalReturnConstraints,
			ignorePostTransitRentalReturnConstraints,
			ignoreDirectRentalReturnConstraints,
			algorithm
		})
	);

	let isochronesQuery = $derived(
		createIsochronesQuery({
			one,
			time,
			maxTravelTime,
			transitModes,
			maxTransfers,
			arriveBy,
			useRoutedTransfers,
			pedestrianProfile,
			requireBikeTransport,
			requireCarTransport,
			preTransitModes,
			postTransitModes,
			maxPreTransitTime,
			maxPostTransitTime,
			elevationCosts,
			ignorePreTransitRentalReturnConstraints,
			ignorePostTransitRentalReturnConstraints
		})
	);

	const searchDebounce = createDebouncedRunner();
	let baseResponse = $state<Promise<PlanResponse>>();
	let routingResponses = $state<Array<Promise<PlanResponse>>>([]);
	let stopNameFromResponse = $state<string>('');
	$effect(() => {
		if (!baseQuery || activeTab != 'connections') {
			searchDebounce.clear();
			return;
		}
		if (baseQuery != lastPlanQuery) {
			lastPlanQuery = baseQuery;
			searchDebounce.clear();
			const query = baseQuery;
			const fromSnapshot = from;
			const toSnapshot = to;
			searchDebounce.schedule(400, () => {
				const q = query.query;
				if (hasDebug) {
					console.info('[MOTIS] plan query', {
						fromPlace: q.fromPlace,
						toPlace: q.toPlace,
						fromName: fromSnapshot.label,
						toName: toSnapshot.label
					});
				}
				const base = plan(query).then(preprocessItinerary(fromSnapshot, toSnapshot));
				baseResponse = base;
				routingResponses = [base];
				pushStateWithQueryString(
					buildConnectionsQueryState(q, fromSnapshot.label, toSnapshot.label),
					{ activeTab: 'connections' },
					true
				);
			});
		}
	});
	const isochronesDebounce = createDebouncedRunner();
	let isochronesRequestGeneration = 0;
	$effect(() => {
		if (!isochronesQuery || activeTab != 'isochrones') {
			isochronesRequestGeneration += 1;
			isochronesDebounce.clear();
			return;
		}
		if (lastOneToAllQuery != isochronesQuery) {
			lastOneToAllQuery = isochronesQuery;
			isochronesDebounce.clear();
			isochronesOptions.status = 'WORKING';
			isochronesOptions.errorMessage = undefined;
			isochronesOptions.errorCode = undefined;

			const requestQuery = isochronesQuery;
			const requestGeneration = ++isochronesRequestGeneration;
			const requestMaxTravelTime = maxTravelTime;
			isochronesDebounce.schedule(60, async () => {
				try {
					const { data, error, response } = await oneToAll(requestQuery);
					if (
						requestGeneration !== isochronesRequestGeneration ||
						activeTab != 'isochrones' ||
						lastOneToAllQuery != requestQuery
					) {
						return;
					}
					if (error) {
						isochronesOptions.status = 'FAILED';
						isochronesOptions.errorCode = response.status;
						isochronesOptions.errorMessage = error.error;
						return;
					}
					isochronesData = mapReachablePlaces(data!.all!, requestMaxTravelTime);
					isochronesOptions.status = isochronesData.length == 0 ? 'EMPTY' : 'WORKING';
				} catch (e) {
					if (
						requestGeneration !== isochronesRequestGeneration ||
						activeTab != 'isochrones' ||
						lastOneToAllQuery != requestQuery
					) {
						return;
					}
					isochronesOptions.status = 'FAILED';
					isochronesOptions.errorMessage = String(e);
					isochronesOptions.errorCode = 404;
				}
			});
		}
	});
	$effect(() => {
		if (isochronesQuery && activeTab == 'isochrones') {
			const q = isochronesQuery.query;
			const [isochronesColor, isochronesOpacity, isochronesDisplayLevel] = [
				isochronesOptions.color,
				isochronesOptions.opacity,
				isochronesOptions.displayLevel
			];
			untrack(() => {
				pushStateWithQueryString(
					buildIsochronesQueryState(q, {
						oneName: one.label,
						isochronesColor,
						isochronesOpacity,
						isochronesDisplayLevel,
						isochronesCircleResolution
					}),
					{ activeTab: 'isochrones' },
					true
				);
			});
		}
	});

	const setDebugPlanResponse = (response: Promise<PlanResponse>): void => {
		baseResponse = response;
		routingResponses = [response];
	};

	onMount(() => {
		if (!browser || !hasDebug) {
			return;
		}
		return registerDebugPlanClipboardImport(window, setDebugPlanResponse);
	});

	onMount(() => {
		return () => {
			searchDebounce.clear();
			isochronesDebounce.clear();
			isochronesRequestGeneration += 1;
		};
	});

	const flyToSelectedItinerary = () => {
		if (page.state.selectedItinerary && map) {
			flyToItineraries([page.state.selectedItinerary], map, isSmallScreen);
		}
	};

	$effect(() => {
		if (map) {
			if (page.state.selectedItinerary && activeTab == 'connections') {
				flyToSelectedItinerary();
			} else if (activeTab == 'departures' && stop && stop.match) {
				flyToLocation(map, stop);
			} else if (activeTab == 'isochrones' && one && one.match) {
				flyToLocation(map, one);
			}
		}
	});

	$effect(() => {
		if (!map || activeTab != 'connections' || !baseQuery) return;
		Promise.all(routingResponses).then((responses) => {
			if (map) {
				let it = responses.flatMap((response) => response.itineraries);
				if (it.length !== 0) {
					flyToItineraries(it, map, isSmallScreen);
				}
			}
		});
	});
</script>

{#snippet resultPanel()}
	<ResultContent
		bind:activeTab
		{center}
		{serverConfig}
		bind:from
		bind:to
		bind:one
		bind:mapClickTarget
		bind:time
		bind:arriveBy
		bind:useRoutedTransfers
		bind:maxTransfers
		bind:pedestrianProfile
		bind:requireCarTransport
		bind:requireBikeTransport
		bind:transitModes
		bind:preTransitModes
		bind:postTransitModes
		bind:directModes
		bind:elevationCosts
		bind:maxTravelTime
		bind:maxPreTransitTime
		bind:maxPostTransitTime
		bind:maxDirectTime
		bind:ignorePreTransitRentalReturnConstraints
		bind:ignorePostTransitRentalReturnConstraints
		bind:ignoreDirectRentalReturnConstraints
		bind:preTransitProviderGroups
		bind:postTransitProviderGroups
		bind:directProviderGroups
		bind:isochronesOptions
		{baseResponse}
		{routingResponses}
		{baseQuery}
		{showMap}
		{isSmallScreen}
		{level}
		{theme}
		bind:stop
		bind:stopMarker
		bind:stopNameFromResponse
	/>
{/snippet}

<MapShell
	{initLoading}
	{initError}
	{dataLoaded}
	onRetry={loadInitialState}
	{hasDebug}
	bind:map
	bind:bounds
	bind:zoom
	bind:center
	bind:level
	{theme}
	{style}
	bind:showMap
	{activeTab}
	{isSmallScreen}
	{dataAttributionLink}
	bind:mapClickTarget
	bind:from
	bind:to
	bind:one
	bind:stop
	bind:stopMarker
	{isochronesData}
	bind:isochronesOptions
	{arriveBy}
	{preTransitModes}
	{postTransitModes}
	{pedestrianProfile}
	maxPreTransitTime={maxPreTransitTime ?? defaultQuery.maxPreTransitTime}
	maxPostTransitTime={maxPostTransitTime ?? defaultQuery.maxPostTransitTime}
	isochronesCircleResolution={isochronesCircleResolution ?? defaultQuery.circleResolution}
>
	{@render resultPanel()}
</MapShell>
