<script lang="ts">
	import { Handle, Position } from '@xyflow/svelte';
	import test_card from '../resources/test_card.jpg';
	import { onMount } from 'svelte';
	import { handleImageUpload, loadImageSrcThen, type CommonNodeData } from '$lib/common';
	import WebgpuCanvas from './webgpuCanvas.svelte';

	export let data: CommonNodeData;
	export let file_input: HTMLInputElement;

	const { id, label, ty, engine, device } = data;

	let preview_canvas: WebgpuCanvas;

	const onImageLoad = (_: { data: Uint8ClampedArray; width: number; height: number }) => {
		// set input on engine
		// then tell it to write to update the preview on this node
	};

	const handleChange = (e: Event) => {
		handleImageUpload(e, onImageLoad);
	};

	onMount(() => {
		engine.register_surface(id, preview_canvas.get_canvas());
		loadImageSrcThen(test_card, onImageLoad);
	});

</script>

<div class="flow-node node">
	<div>
		{label} - {ty}
	</div>
	<div>
    <WebgpuCanvas bind:this={preview_canvas} {device}></WebgpuCanvas>
	</div>
	<input bind:this={file_input} type="file" accept="image/*" on:change={handleChange} />
	<Handle type="source" position={Position.Right} />
</div>

<style>
	.flow-node {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

</style>
