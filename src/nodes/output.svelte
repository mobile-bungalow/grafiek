<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
	import { downloadImage, type CommonNodeData } from '$lib/common';
	import { onMount } from 'svelte';
	type $Props = NodeProps;

	export let data: CommonNodeData;
	let preview_canvas: HTMLCanvasElement;
	const { label, ty, engine, id } = data;

	onMount(() => {
		engine.register_surface(id, preview_canvas);
		engine.update_preview(id);
	});
  const onExportClick = async () => {
    let im = await engine.export_image_output(label);
    if (im) {
      downloadImage(im);
    } else {
      console.error("null ouput");
    }
  }
</script>

<div class="node">
	<Handle type="target" position={Position.Left} />
	<div class="col">
		{label} - {ty}
		<canvas bind:this={preview_canvas}></canvas>
    <button on:click={onExportClick}>
      Export
    </button>
	</div>
</div>

<style>
	.col {
		display: flex;
		flex-direction: column;
	}
</style>
