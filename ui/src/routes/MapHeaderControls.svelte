<script lang="ts">
	import Control from '$lib/map/Control.svelte';
	import Debug from '$lib/Debug.svelte';
	import { cn } from '$lib/utils';
	import { language, setLanguage, t } from '$lib/i18n/translation';
	import maplibregl from 'maplibre-gl';

	let {
		hasDebug,
		bounds,
		level,
		zoom
	}: {
		hasDebug: boolean;
		bounds: maplibregl.LngLatBoundsLike | undefined;
		level: number;
		zoom: number;
	} = $props();
</script>

{#if hasDebug}
	<Control position="top-right" class="text-right">
		<Debug {bounds} {level} {zoom} />
	</Control>
{/if}

<Control position="top-right" class="pt-2 text-right">
	<div
		class="inline-flex overflow-hidden rounded-md border bg-background shadow-sm"
		role="group"
		aria-label={$t.language}
	>
		<button
			class={cn(
				'px-2 py-1 text-xs font-semibold',
				$language === 'en'
					? 'bg-blue-600 text-white'
					: 'bg-background text-foreground hover:bg-accent'
			)}
			type="button"
			onclick={() => setLanguage('en')}
			aria-pressed={$language === 'en'}
		>
			EN
		</button>
		<button
			class={cn(
				'border-l px-2 py-1 text-xs font-semibold',
				$language === 'sv'
					? 'bg-blue-600 text-white'
					: 'bg-background text-foreground hover:bg-accent'
			)}
			type="button"
			onclick={() => setLanguage('sv')}
			aria-pressed={$language === 'sv'}
		>
			SV
		</button>
	</div>
</Control>
