import path from 'path'
import fs from 'fs'

export default function hulaSkinner() {
	return ({
		name: 'rollup-plugin-hula-skinner',

		transform(code, id) {
			if (id != path.resolve('.', 'src/styles/main.scss')) return

			const styles = path.dirname(id)

			let skin = fs.existsSync(path.join(styles, 'custom'))
				? path.join(styles, 'custom')
				: path.join(styles, 'hula')

			if (fs.existsSync(path.join(skin, '_variables.scss'))) {
				code = `@import '${path.relative(styles, skin)}/variables';\n${code}`
			}

			if (fs.existsSync(path.join(skin, '_main.scss'))) {
				code = `${code}\n@import '${path.relative(styles, skin)}/main';\n`
			}

			return {
				code,
				map: null,
			}
		},
	})
}
