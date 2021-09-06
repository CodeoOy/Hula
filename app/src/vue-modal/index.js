import { ref, shallowRef } from 'vue'
import TheModal from '../components/TheModal.vue'
import confirm from './confirm.js'

export default {
	install: (app, options) => {
		let id = 0
		const modals = ref([])

		app.component('TheModal', {
			...TheModal,
			setup() {
				return {
					modals,
				}
			}
		})

		const modal = ({ title, component, props = {}, backdrop = true } = {}) => {
			props = Object.keys(component.props).reduce((used, key) => ({ ...used, [key]: props[key] }), {})

			return new Promise(resolve => {
				modals.value.push({
					id: ++id,
					resolve,
					title,
					component: shallowRef(component),
					props,
					backdrop,
				})
			})
		}

		app.config.globalProperties.$modal = modal
		app.config.globalProperties.$confirm = confirm(modal)
	},
}