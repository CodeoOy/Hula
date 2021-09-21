import '../scss/main.scss'

import { createApp, toRef } from 'vue'
import store from './store'
import api from './api.js'
import modal from './vue-modal'
import colorScheme from './vue-color-scheme'
import FlashMessage from '@smartweb/vue-flash-message'
import router from './router.js'
import App from './App.vue'
import 'bootstrap'
import { Tooltip } from 'bootstrap'
import { Form, Field, ErrorMessage, configure } from 'vee-validate'
import './validation.js'

// Enable Bootstrap tooltips
new Tooltip(document.body, { selector: '[title]' })

configure({
	validateOnBlur: false, // Empty required fields make the forms grow -> Forgot password link etc. will dodge the click
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
