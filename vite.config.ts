import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';
import { paraglide } from '@inlang/paraglide-sveltekit/vite';
import svg from '@poppanator/sveltekit-svg';
import tailwindcss from '@tailwindcss/vite';
import wasm from 'vite-plugin-wasm';

export default defineConfig({
	plugins: [
		sveltekit(),
		paraglide({
			project: './project.inlang',
			outdir: './src/lib/paraglide'
		}),
		svg({
			includePaths: ['./src/lib/icons/'],
			svgoOptions: {
				multipass: true,
				plugins: [
					{
						name: 'preset-default',
						params: { overrides: { removeViewBox: false } }
					}
				]
			}
		}),
		tailwindcss(),
		wasm(),
	],
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	build: {
		target: 'esnext',
	},
	optimizeDeps: {
   		exclude: ['wasm-games'],
	},
	server: {
		fs: {
		// Allow serving files from one level up (where the pkg folder might be)
		allow: ['..'],
		},
	},
});
