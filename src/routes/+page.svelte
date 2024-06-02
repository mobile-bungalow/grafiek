<script lang="ts">
	import default_graph from '../resources/demo.json';
	import { SvelteFlow, Background, type Edge, type Node } from '@xyflow/svelte';
	import { writable } from 'svelte/store';
	import '@xyflow/svelte/dist/style.css';
	import { GREY } from '$lib/common';
	import init, { EngineWrapper } from '../../grafiek_wasm/pkg';
	// we need onMount to run init
	import { onMount } from 'svelte';

	const initialNodes: Node[] = [];
	const initialEdges: Edge[] = [];
	const nodes = writable<Node[]>(initialNodes);
	const edges = writable(initialEdges);
	var wrapper: EngineWrapper | undefined = undefined;

	onMount(async () => {
		await init();
		try {
			wrapper = await EngineWrapper.init(JSON.stringify(default_graph));
		} catch (e) {
			//TODO: error display and recovery
			console.log(e);
			return;
		}

		for (const node of wrapper.list_nodes()) {
			console.log(node.label, node.id)
		}

		for (const edge of wrapper.list_edges()) {
			console.log(edge.source_node_id, edge.sync_node_id, edge.source_arg_idx, edge.sync_arg_idx);
		}
		
	});
</script>

<svelte:head></svelte:head>

<section style="height:100vh;">
	<SvelteFlow {nodes} {edges} fitView proOptions={{ hideAttribution: true }}>
		<Background bgColor={GREY} />
	</SvelteFlow>
</section>

<style>
</style>
