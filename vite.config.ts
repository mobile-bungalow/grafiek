import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import wasmPack from "vite-plugin-wasm-pack";

wasmPack("./grafiek_wasm");
export default defineConfig({
	plugins: [sveltekit()],
	server: {
		fs: {
			allow: ['..']
		}
	}
});
