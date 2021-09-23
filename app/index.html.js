import { makeHtmlAttributes } from '@rollup/plugin-html'

export default function template({ attributes, files, meta, publicPath, title }) {
	const createTags = entries => entries
		.map(({ tag, ...attrs }) => {
			attrs = makeHtmlAttributes(attrs)
			const open = `<${tag}`
			const close = tag == 'script' ? '></script>' : ' />'
			return `${open}${attrs}${close}`
		})
		.join('\n')

	meta = meta.map(attrs => ({ ...attrs, tag: 'meta' }))

	const css = (files.css || []).map(({ fileName }) => ({
		tag: 'link',
		rel: 'stylesheet',
		href: `${publicPath}/${fileName}`,
	}))

	const js = (files.js || []).map(({ fileName }) => ({
		tag: 'script',
		src: `${publicPath}/${fileName}`,
	}))

	const styles = files.css.find(({ fileName }) => 'main.css').source
	const themeColor = styles.match(/--bs-primary:([^;]*?);/).pop().trim()

	const extras = [
		{ tag: 'meta', name: 'theme-color', content: themeColor },
		{ tag: 'link', rel: 'icon', sizes: 'any', href: `${publicPath}/assets/favicon.ico` },
		{ tag: 'link', rel: 'icon', type: 'image/svg+xml', href: `${publicPath}/assets/icon.svg` },
		{ tag: 'link', rel: 'apple-touch-icon', href: `${publicPath}/assets/apple-touch-icon.png` },
		{ tag: 'link', rel: 'manifest', href: `${publicPath}/assets/site.webmanifest` }
	]

	const metas = createTags(meta.concat(extras.filter(({ tag }) => tag == 'meta')))
	const links = createTags(css.concat(extras.filter(({ tag }) => tag == 'link')))
	const scripts = createTags(js)

	return `<!doctype html>
<html${makeHtmlAttributes(attributes.html)}>
	<head>
		${metas}
		<title>${title}</title>
		${links}
	</head>
	<body>
		<div id='hula'></div>
		${scripts}
	</body>
</html>`
}
