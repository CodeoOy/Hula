import { nodeResolve } from '@rollup/plugin-node-resolve'
import vue from 'rollup-plugin-vue'
import postcss from 'rollup-plugin-postcss'
import autoprefixer from 'autoprefixer'
import replace from '@rollup/plugin-replace'
import html from '@rollup/plugin-html'
import copy from 'rollup-plugin-copy'
import { terser } from "rollup-plugin-terser"
import skinner from './rollup-plugin-hula-skinner.js'
import htmlTemplate from './index.html.js'

const production = !process.env.ROLLUP_WATCH

export default {
	input: 'src/main.js',
	output: {
		dir: '../public',
		format: 'iife',
		sourcemap: !production,
		entryFileNames: production ? '[name]-[hash].js' : '[name].js',
	},

	plugins: [

		nodeResolve({ browser: true }),
		
		replace({
			preventAssignment: true,
			'process.env.NODE_ENV': JSON.stringify(production ? 'production' : 'development'),
			'__VUE_PROD_DEVTOOLS__': !production,
			'__VUE_OPTIONS_API__': true,
		}),
		
		vue(),

		skinner(),

		production && terser(),

		postcss({
			extract: true,
			minimize: production,
			sourceMap: !production,
			plugins: [
				autoprefixer(),
			]
		}),

		copy({
			copyOnce: true,
			targets: [
				{ src: './assets/**/*', dest: '../public/assets' },
				{ src: './src/styles/hula/assets/**/*', dest: '../public/assets' },
				{ src: './src/styles/custom/assets/**/*', dest: '../public/assets' },
				{ src: './node_modules/bootstrap-icons/font/fonts/*', dest: '../public/fonts' },
			]
		}),

		html({
			title: 'Hula',
			publicPath: '/public',
			meta: [
				{ charset: 'utf-8' },
				{ name: 'viewport', content: 'width=device-width, initial-scale=1' },
			],
			template: htmlTemplate,
		}),
	]
}
