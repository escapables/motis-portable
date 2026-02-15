<script lang="ts">
	import { Palette, Rss, Ban, TrainFront } from '@lucide/svelte';
	import { Button } from '$lib/components/ui/button';
	import Control from '$lib/map/Control.svelte';
	import Marker from '$lib/map/Marker.svelte';
	import Popup from '$lib/map/Popup.svelte';
	import Isochrones from '$lib/map/Isochrones.svelte';
	import Rentals from '$lib/map/rentals/Rentals.svelte';
	import RailViz from '$lib/RailViz.svelte';
	import { posToLocation, type Location } from '$lib/Location';
	import { t } from '$lib/i18n/translation';
	import { LEVEL_MIN_ZOOM } from '$lib/constants';
	import type { IsochronesOptions, IsochronesPos } from '$lib/map/IsochronesShared';
	import type { PrePostDirectMode } from '$lib/Modes';
	import type { PedestrianProfile } from '@motis-project/motis-client';
	import maplibregl from 'maplibre-gl';
	import { nextColorMode, type ActiveTab, type ColorMode } from './pageState';

	type ConnectionsMapClickTarget = 'from' | 'to';
	type CloseFn = () => void;

	let {
		map,
		bounds,
		zoom,
		level,
		theme,
		hasDebug,
		showMap,
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
		isochronesCircleResolution
	}: {
		map: maplibregl.Map | undefined;
		bounds: maplibregl.LngLatBoundsLike | undefined;
		zoom: number;
		level: number;
		theme: 'light' | 'dark';
		hasDebug: boolean;
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
	} = $props();

	let fromMarker = $state<maplibregl.Marker | undefined>();
	let toMarker = $state<maplibregl.Marker | undefined>();
	let oneMarker = $state<maplibregl.Marker | undefined>();
	let colorMode = $state<ColorMode>('none');

	$effect(() => {
		if (activeTab == 'isochrones') {
			colorMode = 'none';
		}
	});

	const setConnectionsLocationFromMap = (
		target: ConnectionsMapClickTarget,
		lngLat: maplibregl.LngLat
	) => {
		const location = posToLocation(lngLat, zoom > LEVEL_MIN_ZOOM ? level : undefined);
		if (target === 'from') {
			from = location;
			fromMarker?.setLngLat(location.match!);
			return;
		}
		to = location;
		toMarker?.setLngLat(location.match!);
	};

	const setIsochronesLocationFromMap = (lngLat: maplibregl.LngLat) => {
		one = posToLocation(lngLat, zoom > LEVEL_MIN_ZOOM ? level : undefined);
		oneMarker?.setLngLat(one.match!);
	};

	$effect(() => {
		if (activeTab != 'connections') {
			mapClickTarget = undefined;
		}
	});

	$effect(() => {
		if (!map || activeTab != 'connections' || !mapClickTarget) return;
		const mapInstance = map;
		const clickTarget = mapClickTarget;
		const handleMapClick = (e: maplibregl.MapMouseEvent) => {
			setConnectionsLocationFromMap(clickTarget, e.lngLat);
			mapClickTarget = undefined;
		};
		mapInstance.on('click', handleMapClick);
		return () => {
			mapInstance.off('click', handleMapClick);
		};
	});
</script>

{#snippet contextMenu(e: maplibregl.MapMouseEvent, close: CloseFn)}
	{#if activeTab == 'connections'}
		<Button
			variant="outline"
			onclick={() => {
				setConnectionsLocationFromMap('from', e.lngLat);
				close();
			}}
		>
			{$t.from}
		</Button>
		<Button
			variant="outline"
			onclick={() => {
				setConnectionsLocationFromMap('to', e.lngLat);
				close();
			}}
		>
			{$t.to}
		</Button>
	{:else if activeTab == 'isochrones'}
		<Button
			variant="outline"
			onclick={() => {
				setIsochronesLocationFromMap(e.lngLat);
				close();
			}}
		>
			{$t.position}
		</Button>
	{/if}
{/snippet}

<div class="maplibregl-ctrl-{isSmallScreen ? 'top-left' : 'bottom-right'}">
	<div class="maplibregl-ctrl maplibregl-ctrl-attrib">
		<div class="maplibregl-ctrl-attrib-inner">
			&copy; Stig
			{#if dataAttributionLink}
				|
				<a href={dataAttributionLink} target="_blank" rel="noopener noreferrer">
					{$t.timetableSources}
				</a>
			{/if}
		</div>
	</div>
</div>

{#if showMap}
	{#if activeTab != 'isochrones'}
		<Control position="top-right" class="pb-4 text-right">
			<Button
				size="icon"
				onclick={() => {
					colorMode = nextColorMode(colorMode);
				}}
			>
				{#if colorMode == 'rt'}
					<Rss class="h-[1.2rem] w-[1.2rem]" />
				{:else if colorMode == 'mode'}
					<TrainFront class="h-[1.2rem] w-[1.2rem]" />
				{:else if colorMode == 'none'}
					<Ban class="h-[1.2rem] w-[1.2rem]" />
				{:else}
					<Palette class="h-[1.2rem] w-[1.2rem]" />
				{/if}
			</Button>
		</Control>
		<Rentals {map} {bounds} {zoom} {theme} debug={hasDebug} />
	{/if}

	<RailViz {map} {bounds} {zoom} {colorMode} />
	<Isochrones
		{map}
		{bounds}
		{isochronesData}
		streetModes={arriveBy ? preTransitModes : postTransitModes}
		wheelchair={pedestrianProfile === 'WHEELCHAIR'}
		maxAllTime={arriveBy ? maxPreTransitTime : maxPostTransitTime}
		circleResolution={isochronesCircleResolution}
		active={activeTab == 'isochrones'}
		bind:options={isochronesOptions}
	/>

	<Popup trigger="contextmenu" children={contextMenu} />

	{#if from && activeTab == 'connections'}
		<Marker color="green" draggable={true} {level} bind:location={from} bind:marker={fromMarker} />
	{/if}

	{#if stop && activeTab == 'departures'}
		<Marker color="black" draggable={false} {level} bind:location={stop} bind:marker={stopMarker} />
	{/if}

	{#if to && activeTab == 'connections'}
		<Marker color="red" draggable={true} {level} bind:location={to} bind:marker={toMarker} />
	{/if}

	{#if one && activeTab == 'isochrones'}
		<Marker color="yellow" draggable={true} {level} bind:location={one} bind:marker={oneMarker} />
	{/if}
{/if}
