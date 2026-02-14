<script lang="ts">
	import { page } from '$app/state';
	import AddressTypeahead from '$lib/AddressTypeahead.svelte';
	import { type Location } from '$lib/Location';
	import { t } from '$lib/i18n/translation';
	import { onClickStop } from '$lib/utils';

	let {
		from = $bindable(),
		time = $bindable(),
		active = false
	}: {
		from: Location;
		time: Date;
		active?: boolean;
	} = $props();

	let fromItems = $state<Array<Location>>([]);

	const selectStop = (location: Location) => {
		if (!location.match || location.match.type !== 'STOP' || !location.match.id) {
			return;
		}
		const stopName = location.label || location.match.name || location.match.id;
		onClickStop(stopName, location.match.id, time);
	};

	$effect(() => {
		if (!active || !from?.match || from.match.type !== 'STOP' || !from.match.id) {
			return;
		}
		const stopKey = `${from.match.id}|${time.toISOString()}`;
		const selectedStop = page.state.selectedStop;
		const selectedStopKey = selectedStop
			? `${selectedStop.stopId}|${selectedStop.time.toISOString()}`
			: undefined;
		if (selectedStopKey === stopKey) {
			return;
		}
		selectStop(from);
	});
</script>

<div id="searchmask-container" class="flex flex-col space-y-4 p-4 relative">
	<AddressTypeahead
		name="from"
		placeholder={$t.from}
		bind:selected={from}
		bind:items={fromItems}
		type="STOP"
		onChange={(location) => {
			selectStop(location);
		}}
	/>
</div>
