import { ref } from 'vue'
import ConfirmDialog from '../components/TheConfirmDialog.vue'
import confirmDelete from './confirm-delete.js'

export default {
	install: (app, options) => {
		let id = 0
		const dialogs = ref([])

		app.component('ConfirmDialog', {
			...ConfirmDialog,
			setup() {
				return {
					dialogs,
				}
			}
		})
		
		const confirm = title => {
			return new Promise(resolve => {
				dialogs.value.push({
					id: ++id,
					title,
					resolve,
				})
			})
		}

		app.config.globalProperties.$confirm = confirm
		app.config.globalProperties.$confirm.delete = confirmDelete(confirm)
	},
}