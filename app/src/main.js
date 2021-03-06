import '@root/styles/main.scss'
import '~bootstrap-icons/font/bootstrap-icons.css'

import { createApp, toRef } from 'vue'
import store from '@store'
import api from '@root/api.js'
import modal from '@root/plugins/vue-modal'
import colorScheme from '@root/plugins/vue-color-scheme'
import FlashMessage from '@smartweb/vue-flash-message'
import router from '@root/router.js'
import App from '@root/App.vue'
import 'bootstrap'
import { Tooltip } from 'bootstrap'
import { Form, Field, ErrorMessage } from 'vee-validate'
import '@root/validation.js'

// Enable Bootstrap tooltips
new Tooltip(document.body, {
	selector: '[title]',
	delay: { show: 300, hide: 0 },
})

// Close tooltips on click (otherwise they stay on top of modals)
document.body.addEventListener('click', () => {
	for (const el of document.querySelectorAll('.tooltip')) {
		Tooltip.getInstance(el).hide()
	}
})

const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)
	.use(api)
	.use(modal)
	.use(colorScheme, { scheme: toRef(store.state, 'colorScheme') })
	.component('VForm', Form)
	.component('VField', Field)
	.component('ErrorMessage', ErrorMessage)

router.isReady()
	.then(() => {
		if (store.state.loggeduser) {
			return store.dispatch('setUser', store.state.loggeduser.id)
		}
	})
	.then(() => app.mount('#hula'))
