import { createApp } from 'vue'
import setupRouter from './router.js'
import App from './App.vue'

const router = setupRouter()
const app = createApp(App)
	.use(router)

app.mount('#app')