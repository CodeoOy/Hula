import { nodeResolve } from '@rollup/plugin-node-resolve'
import vue from 'rollup-plugin-vue'
//import commonjs from '@rollup/plugin-commonjs';
import copy from 'rollup-plugin-copy-assets';

export default {
	input: `src/main.js`,
	output: {
		//file: '../public/main.js',
		dir: '../public',
		entryFileNames: '[name].js',
		format: "iife",
		name: "pocRustVue"
	},

	plugins: [

		nodeResolve({ browser: true}),

		vue({target: 'browser'}),
		//commonjs(),
		copy({
			assets: [
				"assets",
				"index.html"
			],
		}),
	]
}
