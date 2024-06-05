<script lang="ts">
	import { Handle, Position } from '@xyflow/svelte';
	import test_card from '../resources/test_card.jpg';
	import { onMount } from 'svelte';
	import { handleImageUpload, loadImageSrcThen, type CommonNodeData } from '$lib/common';

	export let data: CommonNodeData;
	export let file_input: HTMLInputElement;

	const { id, label, ty, engine } = data;

	let preview_canvas: HTMLCanvasElement;

	const onImageLoad = (im: { data: Uint8ClampedArray; width: number; height: number }) => {
    engine.set_input_image(Uint8Array.from(im.data), im.width, im.height, id);
    engine.update_preview(id);
	};

	const handleChange = (e: Event) => {
		handleImageUpload(e, onImageLoad);
	};

	onMount(() => {
		engine.register_surface(id, preview_canvas);
		loadImageSrcThen(test_card, onImageLoad);
	});

</script>

<div class="flow-node node">
	<div>
		{label} - {ty}
	</div>
	<div>
    <canvas bind:this={preview_canvas}></canvas>
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
