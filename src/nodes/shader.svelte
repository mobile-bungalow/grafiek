<script lang="ts">
	import { Handle, Position, type NodeProps } from '@xyflow/svelte';
	import WebgpuCanvas from './webgpuCanvas.svelte';
	import { type CommonNodeData } from '$lib/common';
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
	<Handle type="source" position={Position.Right} />
</div>
