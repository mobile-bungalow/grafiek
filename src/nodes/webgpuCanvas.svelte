<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { writable } from 'svelte/store';

	export let device: GPUDevice;

	export const get_canvas = () => {
		return canvas;
	}

	let canvas: HTMLCanvasElement;
	let context: GPUCanvasContext;
	let observer: IntersectionObserver;
	const swapChainFormat: GPUTextureFormat = 'rgba8unorm';

	const visible = writable(false);

	onMount(() => {
		if (!canvas) return;

		context = canvas.getContext('webgpu') as GPUCanvasContext;
		context.configure({
			device,
			format: swapChainFormat,
			usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC | GPUTextureUsage.COPY_DST
		});

		observer = new IntersectionObserver((entries) => {
			entries.forEach((entry) => {
				visible.set(entry.isIntersecting);
			});
		});

		observer.observe(canvas);

		clear();
	});

	onDestroy(() => {
		observer.disconnect();
	});

	const getCurrentTexture = (): GPUTexture | null => {
		if (!context) return null;
		return context.getCurrentTexture();
	};

	const copy = (in_tex: GPUTexture) => {
		const texture = getCurrentTexture();
		if (!texture) return;

		const textureWidth = texture.width;
		const textureHeight = texture.height;

		let enc = device.createCommandEncoder();

		enc.copyTextureToTexture(
			{ texture: in_tex },
			{ texture },
			{ width: textureWidth, height: textureHeight, depthOrArrayLayers: 1 }
		);

		device.queue.submit([enc.finish()]);
	};

	const clear = () => {
		const texture = getCurrentTexture();
		if (!texture) return;

		const textureWidth = texture.width;
		const textureHeight = texture.height;

		const transparentBuffer = new Uint8Array(textureWidth * textureHeight * 4).fill(0);

		device.queue.writeTexture(
			{ texture },
			transparentBuffer,
			{ bytesPerRow: textureWidth * 4, rowsPerImage: textureHeight },
			{ width: textureWidth, height: textureHeight, depthOrArrayLayers: 1 }
		);

		device.queue.submit([]);
	};
</script>

<div class="canvas-wrapper">
	<canvas bind:this={canvas} width="256" height="256"></canvas>
</div>

<style>
	canvas {
		width: 100%;
		height: 100%;
		position: relative;
		z-index: 2;
	}
</style>
