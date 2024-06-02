<script lang="ts">
	import default_graph from '../resources/demo.json';
	import {  layout_graph } from '$lib/layout';
	import { SvelteFlow, Background, type Edge, type Node, Position } from '@xyflow/svelte';
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

		const loadedNodes: Node[] = wrapper.list_nodes().map((node) => ({
			id: `${node.id}`,
			data: { label: node.label },
			position: { x: 0, y: 0 },
			targetPosition: Position.Left,
			sourcePosition: Position.Right,
		}));

		const loadedEdges: Edge[] = wrapper.list_edges().map((edge) => ({
			id: `${edge.source_node_id}-${edge.sync_node_id}`,
			source: `${edge.source_node_id}`,
			target: `${edge.sync_node_id}`
		}));

		// TODO: only run this on the condition that there is no layout info
		layout_graph(loadedNodes, loadedEdges);

		nodes.set(loadedNodes);
		edges.set(loadedEdges);
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
