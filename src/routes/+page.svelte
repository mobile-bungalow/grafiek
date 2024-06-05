<script lang="ts">
	import default_graph from '../resources/demo.json';
	import { SvelteFlow, Background, type Edge, type Node } from '@xyflow/svelte';
	import { writable } from 'svelte/store';
	import '@xyflow/svelte/dist/style.css';
	import { GREY } from '$lib/common';
	import init, { EngineWrapper } from '../../grafiek_wasm/pkg';
	// we need onMount to run init
	import { onMount } from 'svelte';
	import { App } from '$lib/app';
	import { nodeTypes } from '$lib/common';


	const nodes = writable([]);
	const edges = writable([]);
	var wrapper: EngineWrapper | undefined;
	var app : App | undefined;


	onMount(async () => {
		const adapter = await navigator.gpu.requestAdapter();
		if (!adapter) {
			console.error('Failed to get GPU adapter');
			return;
		}
		const device = await adapter.requestDevice();
		await init();
		try {
			wrapper = await EngineWrapper.init(JSON.stringify(default_graph));
		} catch (e) {
			//TODO: error display and recovery
			console.log(e);
			return;
		}

		app = new App(wrapper, nodes, edges, device);
	});
</script>

<svelte:head></svelte:head>
<section style="height:100vh;">
	<button on:click={() => {app?.wrapper.render()}}> RENDER </button>
	<SvelteFlow {nodes} {edges} {nodeTypes} fitView proOptions={{ hideAttribution: true }}>
		<Background bgColor={GREY} />
	</SvelteFlow>
</section>

<style>
</style>
