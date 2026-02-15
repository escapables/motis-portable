<script lang="ts">
	import { page } from '$app/state';
	import { pushState } from '$app/navigation';
	import { X } from '@lucide/svelte';
	import { Card } from '$lib/components/ui/card';
	import SearchMask from '$lib/SearchMask.svelte';
	import ItineraryList from '$lib/ItineraryList.svelte';
	import ConnectionDetail from '$lib/ConnectionDetail.svelte';
	import { Button } from '$lib/components/ui/button';
	import ItineraryGeoJson from '$lib/map/itineraries/ItineraryGeoJSON.svelte';
	import * as Tabs from '$lib/components/ui/tabs';
	import DeparturesMask from '$lib/DeparturesMask.svelte';
	import IsochronesInfo from '$lib/IsochronesInfo.svelte';
	import IsochronesMask from '$lib/IsochronesMask.svelte';
	import Control from '$lib/map/Control.svelte';
	import StopTimes from '$lib/StopTimes.svelte';
	import StopGeoJSON from '$lib/map/stops/StopsGeoJSON.svelte';
	import { preprocessItinerary } from '$lib/preprocessItinerary';
	import { t } from '$lib/i18n/translation';
	import type { IsochronesOptions } from '$lib/map/IsochronesShared';
	import type { Location } from '$lib/Location';
	import type { PrePostDirectMode } from '$lib/Modes';
	import type {
		ElevationCosts,
		Mode,
		PedestrianProfile,
		PlanData,
		PlanResponse,
		ServerConfig
	} from '@motis-project/motis-client';
	import { resolveTabTransition, type ActiveTab } from './pageState';
	import maplibregl from 'maplibre-gl';

	type ConnectionsMapClickTarget = 'from' | 'to';

	let {
		activeTab = $bindable<ActiveTab>('connections'),
		center,
		serverConfig,
		from = $bindable(),
		to = $bindable(),
		one = $bindable(),
		mapClickTarget = $bindable<ConnectionsMapClickTarget | undefined>(undefined),
		time = $bindable(),
		arriveBy = $bindable(),
		useRoutedTransfers = $bindable(),
		maxTransfers = $bindable(),
		pedestrianProfile = $bindable(),
		requireCarTransport = $bindable(),
		requireBikeTransport = $bindable(),
		transitModes = $bindable(),
		preTransitModes = $bindable(),
		postTransitModes = $bindable(),
		directModes = $bindable(),
		elevationCosts = $bindable(),
		maxTravelTime = $bindable(),
		maxPreTransitTime = $bindable(),
		maxPostTransitTime = $bindable(),
		maxDirectTime = $bindable(),
		ignorePreTransitRentalReturnConstraints = $bindable(),
		ignorePostTransitRentalReturnConstraints = $bindable(),
		ignoreDirectRentalReturnConstraints = $bindable(),
		preTransitProviderGroups = $bindable(),
		postTransitProviderGroups = $bindable(),
		directProviderGroups = $bindable(),
		isochronesOptions = $bindable(),
		baseResponse,
		routingResponses,
		baseQuery,
		showMap,
		isSmallScreen,
		level,
		theme,
		stop = $bindable(),
		stopMarker = $bindable(),
		stopNameFromResponse = $bindable()
	}: {
		activeTab: ActiveTab;
		center: maplibregl.LngLatLike;
		serverConfig: ServerConfig | undefined;
		from: Location;
		to: Location;
		one: Location;
		mapClickTarget: ConnectionsMapClickTarget | undefined;
		time: Date;
		arriveBy: boolean;
		useRoutedTransfers: boolean;
		maxTransfers: number;
		pedestrianProfile: PedestrianProfile;
		requireCarTransport: boolean;
		requireBikeTransport: boolean;
		transitModes: Mode[];
		preTransitModes: PrePostDirectMode[];
		postTransitModes: PrePostDirectMode[];
		directModes: PrePostDirectMode[];
		elevationCosts: ElevationCosts;
		maxTravelTime: number;
		maxPreTransitTime: number;
		maxPostTransitTime: number;
		maxDirectTime: number;
		ignorePreTransitRentalReturnConstraints: boolean;
		ignorePostTransitRentalReturnConstraints: boolean;
		ignoreDirectRentalReturnConstraints: boolean;
		preTransitProviderGroups: string[];
		postTransitProviderGroups: string[];
		directProviderGroups: string[];
		isochronesOptions: IsochronesOptions;
		baseResponse: Promise<PlanResponse> | undefined;
		routingResponses: Array<Promise<PlanResponse>>;
		baseQuery: PlanData | undefined;
		showMap: boolean;
		isSmallScreen: boolean;
		level: number;
		theme: 'light' | 'dark';
		stop: Location | undefined;
		stopMarker: maplibregl.Marker | undefined;
		stopNameFromResponse: string;
	} = $props();
</script>

<Control>
	<Tabs.Root
		bind:value={
			() => activeTab,
			(v) => {
				const transition = resolveTabTransition(activeTab, v, from, one);
				from = transition.from;
				one = transition.one;
				activeTab = v;
				pushState('', { activeTab: v });
			}
		}
		class="max-w-full w-full md:w-[520px] overflow-y-auto"
	>
		<Tabs.List class="grid grid-cols-3">
			<Tabs.Trigger value="connections">{$t.connections}</Tabs.Trigger>
			<Tabs.Trigger value="departures">{$t.departures}</Tabs.Trigger>
			<Tabs.Trigger value="isochrones">{$t.isochrones.title}</Tabs.Trigger>
		</Tabs.List>
		<Tabs.Content value="connections">
			<Card class="overflow-y-auto overflow-x-hidden bg-background rounded-lg">
				<SearchMask
					geocodingBiasPlace={center}
					{serverConfig}
					bind:from
					bind:to
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
					bind:maxPreTransitTime
					bind:maxPostTransitTime
					bind:maxDirectTime
					bind:ignorePreTransitRentalReturnConstraints
					bind:ignorePostTransitRentalReturnConstraints
					bind:ignoreDirectRentalReturnConstraints
					bind:preTransitProviderGroups
					bind:postTransitProviderGroups
					bind:directProviderGroups
				/>
			</Card>
		</Tabs.Content>
		<Tabs.Content value="departures">
			<Card class="overflow-y-auto overflow-x-hidden bg-background rounded-lg">
				<DeparturesMask bind:from bind:time active={activeTab == 'departures'} />
			</Card>
		</Tabs.Content>
		<Tabs.Content value="isochrones">
			<Card class="overflow-y-auto overflow-x-hidden bg-background rounded-lg">
				<IsochronesMask
					bind:one
					{serverConfig}
					bind:maxTravelTime
					geocodingBiasPlace={center}
					bind:time
					bind:useRoutedTransfers
					bind:pedestrianProfile
					bind:requireCarTransport
					bind:requireBikeTransport
					bind:transitModes
					bind:maxTransfers
					bind:preTransitModes
					bind:postTransitModes
					bind:maxPreTransitTime
					bind:maxPostTransitTime
					bind:arriveBy
					bind:elevationCosts
					bind:ignorePreTransitRentalReturnConstraints
					bind:ignorePostTransitRentalReturnConstraints
					bind:options={isochronesOptions}
					bind:preTransitProviderGroups
					bind:postTransitProviderGroups
					bind:directProviderGroups
				/>
			</Card>
		</Tabs.Content>
	</Tabs.Root>
</Control>

{#if activeTab == 'connections' && routingResponses.length !== 0 && !page.state.selectedItinerary}
	<Control class="min-h-0 md:flex md:flex-col md:mb-2">
		<Card
			class="scrollable w-full md:w-[520px] h-full md:h-[70vh] {isSmallScreen
				? 'border-0 shadow-none'
				: ''} overflow-x-hidden bg-background rounded-lg mb-2"
		>
			<ItineraryList
				{baseResponse}
				{routingResponses}
				{baseQuery}
				selectItinerary={(selectedItinerary) => {
					pushState('', {
						selectedItinerary: selectedItinerary,
						scrollY: undefined,
						activeTab: 'connections'
					});
				}}
				updateStartDest={preprocessItinerary(from, to)}
			/>
		</Card>
	</Control>
	{#if showMap && !page.state.selectedItinerary}
		{#each routingResponses as r, rI (rI)}
			{#await r then r}
				{#each r.itineraries as it, i (i)}
					<ItineraryGeoJson
						itinerary={it}
						id="{rI}-{i}"
						selected={false}
						selectItinerary={() => {
							pushState('', {
								selectedItinerary: it,
								activeTab: 'connections'
							});
						}}
						{level}
						{theme}
					/>
				{/each}
			{/await}
		{/each}
	{/if}
{/if}

{#if activeTab == 'connections' && page.state.selectedItinerary}
	<Control class="min-h-0 md:mb-2 md:flex">
		<Card class="w-full md:w-[520px] bg-background rounded-lg flex flex-col mb-2">
			<div class="w-full flex justify-between items-center shadow-md pl-1 mb-1">
				<h2 class="ml-2 text-base font-semibold">{$t.journeyDetails}</h2>
				<Button
					variant="ghost"
					onclick={() => {
						history.back();
					}}
				>
					<X />
				</Button>
			</div>
			<div
				class={'p-2 md:p-4 overflow-y-auto overflow-x-hidden min-h-0 ' +
					(showMap ? 'md:max-h-[60vh]' : '')}
			>
				<ConnectionDetail itinerary={page.state.selectedItinerary} />
			</div>
		</Card>
	</Control>
	{#if showMap}
		<ItineraryGeoJson itinerary={page.state.selectedItinerary} selected={true} {level} {theme} />
		<StopGeoJSON itinerary={page.state.selectedItinerary} {theme} />
	{/if}
{/if}

{#if activeTab == 'departures' && page.state.selectedStop}
	<Control class="min-h-0 md:mb-2">
		<Card
			class="w-full md:w-[520px] md:max-h-[60vh] h-full bg-background rounded-lg flex flex-col mb-2"
		>
			<div class="w-full flex justify-between items-center shadow-md pl-1 mb-1">
				<h2 class="ml-2 text-base font-semibold">
					{#if page.state.stopArriveBy}
						{$t.arrivals}
					{:else}
						{$t.departures}
					{/if}
					:
					{stopNameFromResponse}
				</h2>
				<Button
					variant="ghost"
					onclick={() => {
						history.back();
					}}
				>
					<X />
				</Button>
			</div>
			<div class="p-2 md:p-4 overflow-y-auto overflow-x-hidden min-h-0 md:max-h-[60vh]">
				<StopTimes
					stopId={page.state.selectedStop.stopId}
					stopName={page.state.selectedStop.name}
					time={page.state.selectedStop.time}
					bind:stop
					bind:stopMarker
					bind:stopNameFromResponse
					arriveBy={page.state.stopArriveBy}
				/>
			</div>
		</Card>
	</Control>
{/if}

{#if activeTab == 'isochrones' && one.match}
	<Control class="min-h-0 md:mb-2 {isochronesOptions.status == 'DONE' ? 'hide' : ''}">
		<Card class="w-full md:w-[520px] overflow-y-auto overflow-x-hidden bg-background rounded-lg">
			<IsochronesInfo options={isochronesOptions} />
		</Card>
	</Control>
{/if}
