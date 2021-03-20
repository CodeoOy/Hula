import { nodeResolve } from '@rollup/plugin-node-resolve'
import replace from '@rollup/plugin-replace'
import vue from 'rollup-plugin-vue'
import dotenv from 'dotenv'
//import commonjs from '@rollup/plugin-commonjs';
import copy from 'rollup-plugin-copy-assets';

const variables = {
	...dotenv.config({ path: "./src/.env" }).parsed
}

const production = !process.env.ROLLUP_WATCH

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

		replace({
			'process.env.NODE_ENV': JSON.stringify('development'),
			'__VUE_PROD_DEVTOOLS__': !production,
			'__VUE_OPTIONS_API__': true,
			...Object.entries(variables).reduce((env, [key, value]) => ({ ...env, [`process.env.${key}`]: JSON.stringify(value) }), {})
		}),

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
