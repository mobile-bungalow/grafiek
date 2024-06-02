<script lang="ts">
	import { SvelteFlow, Background, type Edge, type Node } from '@xyflow/svelte';
	import { writable } from 'svelte/store';
	import '@xyflow/svelte/dist/style.css';
	import { GREY } from '$lib/common';
	import init from '../../grafiek_wasm/pkg';
	// we need onMount to run init
	import { onMount } from 'svelte';

	onMount(async () => {
		await init(); // init initializes memory addresses needed by WASM and that will be used by JS/TS
	});

	const initialNodes: Node[] = [];
	const initialEdges: Edge[] = [];
	const nodes = writable<Node[]>(initialNodes);
	const edges = writable(initialEdges);
</script>

<svelte:head></svelte:head>

<section style="height:100vh;">
	<SvelteFlow {nodes} {edges} fitView proOptions={{ hideAttribution: true }}>
		<Background bgColor={GREY} />
	</SvelteFlow>
</section>

<style>
</style>
