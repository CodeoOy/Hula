import { createApp } from 'vue'
import FlashMessage from '@smartweb/vue-flash-message';
import setupRouter from './router.js'
import App from './App.vue'

const router = setupRouter()
const app = createApp(App)
	.use(router)
	.use(FlashMessage)
router.isReady()
	.then(() => app.mount('#aninmals'))