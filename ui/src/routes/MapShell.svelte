<script lang="ts">
	import { browser } from '$app/environment';
	import { LoaderCircle, RotateCcw } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import { Card } from '$lib/components/ui/card';
	import { t } from '$lib/i18n/translation';
	import LevelSelect from '$lib/LevelSelect.svelte';
	import type { Location } from '$lib/Location';
	import Map from '$lib/map/Map.svelte';
	import Drawer from '$lib/map/Drawer.svelte';
	import type { IsochronesOptions, IsochronesPos } from '$lib/map/IsochronesShared';
	import type { PrePostDirectMode } from '$lib/Modes';
	import type { PedestrianProfile } from '@motis-project/motis-client';
	import maplibregl from 'maplibre-gl';
	import type { Snippet } from 'svelte';
	import MapHeaderControls from './MapHeaderControls.svelte';
	import MapLayers from './MapLayers.svelte';
	import type { ActiveTab } from './pageState';

	type ConnectionsMapClickTarget = 'from' | 'to';

	let {
		initLoading,
		initError,
		dataLoaded,
		onRetry,
		hasDebug,
		map = $bindable<maplibregl.Map | undefined>(undefined),
		bounds = $bindable<maplibregl.LngLatBoundsLike | undefined>(undefined),
		zoom = $bindable<number>(15),
		center = $bindable<[number, number]>([0, 0]),
		level = $bindable<number>(0),
		theme,
		style,
		showMap = $bindable<boolean>(true),
		activeTab,
		isSmallScreen,
		dataAttributionLink,
		mapClickTarget = $bindable<ConnectionsMapClickTarget | undefined>(undefined),
		from = $bindable<Location>(),
		to = $bindable<Location>(),
		one = $bindable<Location>(),
		stop = $bindable<Location | undefined>(),
		stopMarker = $bindable<maplibregl.Marker | undefined>(),
		isochronesData,
		isochronesOptions = $bindable<IsochronesOptions>(),
		arriveBy,
		preTransitModes,
		postTransitModes,
		pedestrianProfile,
		maxPreTransitTime,
		maxPostTransitTime,
		isochronesCircleResolution,
		children
	}: {
		initLoading: boolean;
		initError: string | undefined;
		dataLoaded: boolean;
		onRetry: () => void;
		hasDebug: boolean;
		map: maplibregl.Map | undefined;
		bounds: maplibregl.LngLatBoundsLike | undefined;
		zoom: number;
		center: [number, number];
		level: number;
		theme: 'light' | 'dark';
		style: maplibregl.StyleSpecification | undefined;
		showMap: boolean;
		activeTab: ActiveTab;
		isSmallScreen: boolean;
		dataAttributionLink: string | undefined;
		mapClickTarget: ConnectionsMapClickTarget | undefined;
		from: Location;
		to: Location;
		one: Location;
		stop: Location | undefined;
		stopMarker: maplibregl.Marker | undefined;
		isochronesData: IsochronesPos[];
		isochronesOptions: IsochronesOptions;
		arriveBy: boolean;
		preTransitModes: PrePostDirectMode[];
		postTransitModes: PrePostDirectMode[];
		pedestrianProfile: PedestrianProfile;
		maxPreTransitTime: number;
		maxPostTransitTime: number;
		isochronesCircleResolution: number | undefined;
		children?: Snippet;
	} = $props();
</script>

{#if initLoading}
	<div class="h-dvh flex items-center justify-center bg-secondary text-foreground">
		<div class="rounded-lg border bg-background px-6 py-5 flex items-center gap-3 shadow-sm">
			<LoaderCircle class="h-5 w-5 animate-spin" />
			<span class="text-sm font-medium">{$t.loadingData}</span>
		</div>
	</div>
{:else if initError}
	<div class="h-dvh flex items-center justify-center bg-secondary px-4">
		<Card class="w-full max-w-xl p-5 flex flex-col gap-3">
			<h2 class="text-lg font-semibold">{$t.startupFailed}</h2>
			<p class="text-sm text-muted-foreground break-words">{initError}</p>
			<div>
				<Button class="gap-2" onclick={onRetry}>
					<RotateCcw class="h-4 w-4" />
					{$t.retry}
				</Button>
			</div>
		</Card>
	</div>
{:else if dataLoaded}
	<Map
		bind:map
		bind:bounds
		bind:zoom
		bind:center
		class={`h-dvh overflow-clip ${theme}`}
		style={showMap ? style : undefined}
		attribution={false}
	>
		<MapHeaderControls {hasDebug} {bounds} {level} {zoom} />

		<LevelSelect {bounds} {zoom} bind:level />

		{#if browser}
			{#if isSmallScreen}
				<Drawer class="relative z-10 h-full mt-5 flex flex-col" bind:showMap>
					{#if children}
						{@render children()}
					{/if}
				</Drawer>
			{:else}
				<div class="maplibregl-ctrl-top-left flex flex-col max-h-[97vh]">
					{#if children}
						{@render children()}
					{/if}
				</div>
			{/if}
		{/if}

		<MapLayers
			{map}
			{bounds}
			{zoom}
			{level}
			{theme}
			{hasDebug}
			{showMap}
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
			{maxPreTransitTime}
			{maxPostTransitTime}
			{isochronesCircleResolution}
		/>
	</Map>
{/if}
