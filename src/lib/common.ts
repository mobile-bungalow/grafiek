import { EngineWrapper, ImageInfo } from '../../grafiek_wasm/pkg/grafiek_wasm';
import Shader from '../nodes/shader.svelte';
import Output from '../nodes/output.svelte';
import Input from '../nodes/input.svelte';

export const GREY = '#333';
export const CHALK = '0xDDD';

export const nodeTypes = {
	GrayScale: Shader,
	Input: Input,
	Output: Output
};

export type Action =
	| {
			tag: 'addNode';
			id: number;
	  }
	| { tag: 'addEdge'; id: number };

export type CommonEdgeData = {
	id: number;
};

export type CommonNodeData = {
	label: string;
	ty: string;
	engine: EngineWrapper;
	id: number;
	device: GPUDevice;
};

type ImageDataCallback = (imageData: {
	data: Uint8ClampedArray;
	width: number;
	height: number;
}) => void;

export const handleImageUpload = (event: Event, setImageData: ImageDataCallback) => {
	const fileList = (event.target as HTMLInputElement).files;
	if (!fileList || fileList.length === 0) {
		console.error('No file selected');
		return;
	}

	const file = fileList[0];
	const reader = new FileReader();

	reader.onload = (event) => {
		if (event.target) {
			loadImageSrcThen(event.target.result as string, setImageData);
		}
	};

	reader.readAsDataURL(file);
};

export const loadImageSrcThen = (src: string, setImageData: ImageDataCallback) => {
	const imgElement = new Image();
	imgElement.onload = () => {
		const canvas = new OffscreenCanvas(imgElement.width, imgElement.height);
		const ctx = canvas.getContext('2d');

		if (!ctx) {
			console.error('Canvas context is null');
			return;
		}

		ctx.drawImage(imgElement, 0, 0);

		const imageData = ctx.getImageData(0, 0, imgElement.width, imgElement.height);
		const { data, width, height } = imageData;

		setImageData({
			width,
			height,
			data
		});
	};

	imgElement.src = src;
};

export const downloadImage = (im: ImageInfo) => {
	const width = im.width;
	const height = im.height;
	const rgba_pixels = im.data;

	const canvas = new OffscreenCanvas(width, height);
	const ctx = canvas.getContext('2d');
	if (!ctx) {
		console.error('Failed to get canvas 2D context');
		return;
	}

	const imageData = new ImageData(Uint8ClampedArray.from(rgba_pixels), width, height);
	ctx.putImageData(imageData, 0, 0);

	canvas
		.convertToBlob({ type: 'image/png' })
		.then((blob) => {
			const url = URL.createObjectURL(blob);
			const link = document.createElement('a');
			link.href = url;
			link.download = 'downloaded_image.png';
			document.body.appendChild(link);
			link.click();
			document.body.removeChild(link);
			URL.revokeObjectURL(url);
		})
		.catch((err) => {
			console.error('Failed to create blob from canvas:', err);
		});
};
