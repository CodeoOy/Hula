import { createApp } from 'vue'
import store from './store'
import FlashMessage from '@smartweb/vue-flash-message';
import router from './router.js'
import App from './App.vue'
import 'bootstrap'
import { Modal } from 'bootstrap'

const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)

router.isReady()
	.then(() => app.mount('#hula'))
