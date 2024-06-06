<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
	import { type CommonNodeData } from '$lib/common';
	import { onMount } from 'svelte';
	type $Props = NodeProps;

	export let data: CommonNodeData;
	let preview_canvas: HTMLCanvasElement;
	const { label, ty, engine, id } = data;

	onMount(() => {
		engine.register_surface(id, preview_canvas);
		engine.update_preview(id);
	});
</script>

<div class="node">
	<Handle type="target" position={Position.Left} />
	<div class="col">
		{label} - {ty}
		<canvas bind:this={preview_canvas}></canvas>
	</div>
	<Handle type="source" position={Position.Right} />
</div>

<style>
	.col {
		display: flex;
		flex-direction: column;
	}
</style>
