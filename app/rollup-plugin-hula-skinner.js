import path from 'path'
import fs from 'fs'

export default function hulaSkinner() {
	return ({
		name: 'rollup-plugin-hula-skinner',

		transform(code, id) {
			if (id != path.resolve('.', 'scss/main.scss')) return

			const scss = path.dirname(id)

			let skin = fs.existsSync(path.join(scss, 'custom'))
				? path.join(scss, 'custom')
				: path.join(scss, 'hula')

			if (fs.existsSync(path.join(skin, '_variables.scss'))) {
				code = `@import '${path.relative(scss, skin)}/variables';\n${code}`
			}

			if (fs.existsSync(path.join(skin, '_main.scss'))) {
				code = `${code}\n@import '${path.relative(scss, skin)}/main';\n`
			}

			return {
				code,
				map: null,
			}
		},
	})
}
