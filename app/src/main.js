import { createApp } from 'vue'
import store from './store'
import api from './api.js'
import confirmDialog from './vue-confirm-dialog'
import FlashMessage from '@smartweb/vue-flash-message';
import router from './router.js'
import App from './App.vue'
import 'bootstrap'

const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)
	.use(api)
	.use(confirmDialog)

router.isReady()
	.then(() => app.mount('#hula'))
