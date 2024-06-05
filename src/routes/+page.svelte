<script lang="ts">
	import default_graph from '../resources/demo.json';
	import { SvelteFlow, Background, type Edge, type Node } from '@xyflow/svelte';
	import { writable } from 'svelte/store';
	import '@xyflow/svelte/dist/style.css';
	import { GREY } from '$lib/common';
	import init, { EngineWrapper } from '../../grafiek_wasm/pkg';
	import { onMount } from 'svelte';
	import { App } from '$lib/app';
	import { nodeTypes } from '$lib/common';

	const nodes = writable([]);
	const edges = writable([]);
	var wrapper: EngineWrapper | undefined;
	var app: App | undefined;

	onMount(async () => {
		await init();
		try {
			wrapper = await EngineWrapper.init(JSON.stringify(default_graph));
		} catch (e) {
			//TODO: error display and recovery
			console.log(e);
			return;
		}

		app = new App(wrapper, nodes, edges);
		const render = () => {
			app?.wrapper.render();
		};
	});
</script>

<svelte:head></svelte:head>
<section style="height:100vh;">
	<button
		on:click={() => {
			app?.wrapper.render();
		}}
	>
		RENDER
	</button>
	<SvelteFlow oninit={() => app?.wrapper.render() } {nodes} {edges} {nodeTypes} fitView proOptions={{ hideAttribution: true }}>
		<Background bgColor={GREY} />
	</SvelteFlow>
</section>

<style>
</style>
