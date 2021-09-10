import { reactive, computed, watch, unref } from 'vue'

const classes = {
	navbar: {
		light: ['navbar-light', 'bg-light'],
		dark: ['navbar-dark', 'bg-dark'],
	},

	table: {
		dark: ['table-dark'],
	},

	card: {
		dark: ['bg-dark', 'text-light'],
	},

	modal: {
		dark: ['bg-dark', 'text-light'],
	},
}

export default {
	install: (app, { scheme }) => {
		const colorScheme = reactive(Object.entries(classes).reduce((classes, [key, value]) => {
			classes[key] = computed(() => value[unref(scheme)])
			return classes
		}, {}))

		watch(scheme, (value, previous) => {
			document.body.classList.add(value)
			document.body.classList.remove(previous)
		})

		app.config.globalProperties.$colorScheme = colorScheme
	},
}