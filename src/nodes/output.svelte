<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
  import { type CommonNodeData } from '$lib/common';
	import WebgpuCanvas from './webgpuCanvas.svelte';
	type $Props = NodeProps;

	export let data: CommonNodeData;
	const { label, device, ty } = data;
</script>

<div class="colorpicker node">
	<Handle type="target" position={Position.Left} />
	<div>
		{label} - {ty}
		<WebgpuCanvas {device}></WebgpuCanvas>
	</div>
</div>
