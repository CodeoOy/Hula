import path from 'path'
import fs from 'fs'

export default function hulaSkinner() {
	return ({
		name: 'rollup-plugin-hula-skinner',

		transform(code, id) {
			if (id != path.resolve('.', 'scss/main.scss')) return

			if (fs.existsSync(path.resolve('.', 'scss/custom/_variables.scss'))) {
				code = `@import 'custom/variables';\n${code}`
			}

			if (fs.existsSync(path.resolve('.', 'scss/custom/_main.scss'))) {
				code = `${code}\n@import 'custom/main';\n`
			}

			return {
				code,
				map: null,
			}
		},
	})
}
