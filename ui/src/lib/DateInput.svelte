<script lang="ts">
	import { SvelteDate } from 'svelte/reactivity';
	import { cn } from './utils';

	let {
		value = $bindable(),
		class: className
	}: {
		value: Date;
		class?: string;
	} = $props();

	let datePart = $state('');
	let timePart = $state('');

	const inputClasses =
		'flex h-9 rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50';

	function toLocalParts(current: Date): { date: string; time: string } {
		const local = new Date(current.getTime() - current.getTimezoneOffset() * 60000).toISOString();
		return { date: local.slice(0, 10), time: local.slice(11, 16) };
	}

	function commit(nextDate: string, nextTime: string): void {
		const dateTokens = nextDate.split('-').map(Number);
		const timeTokens = nextTime.split(':').map(Number);
		if (dateTokens.length !== 3 || timeTokens.length < 2) {
			return;
		}
		const [year, month, day] = dateTokens;
		const [hours, minutes] = timeTokens;
		if (
			Number.isNaN(year) ||
			Number.isNaN(month) ||
			Number.isNaN(day) ||
			Number.isNaN(hours) ||
			Number.isNaN(minutes)
		) {
			return;
		}
		value = new SvelteDate(year, month - 1, day, hours, minutes, 0, 0);
	}

	$effect(() => {
		value.setSeconds(0, 0);
		const parts = toLocalParts(value);
		datePart = parts.date;
		timePart = parts.time;
	});
</script>

<div class={cn('flex flex-row gap-2', className)}>
	<input
		type="date"
		class={cn(inputClasses, 'min-w-[11rem]')}
		bind:value={datePart}
		oninput={() => commit(datePart, timePart)}
	/>
	<input
		type="time"
		step="60"
		class={cn(inputClasses, 'min-w-[7rem]')}
		bind:value={timePart}
		oninput={() => commit(datePart, timePart)}
	/>
</div>
