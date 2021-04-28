import { createApp } from 'vue'
import { createStore } from 'vuex'
import FlashMessage from '@smartweb/vue-flash-message';
import setupRouter from './router.js'
import App from './App.vue'
import 'bootstrap'

// Create a new store instance.
const store = createStore({
	state () {
		return {
			loggeduser: {},
		}
	},
	mutations: {
		setUser (state, data) {
			fetch(`http://localhost:8086/api/user/${data}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then((response) => response.json())
			.then(response => { 
				state.loggeduser = response;
			})   
		}
	}
})

const router = setupRouter()
const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)
router.isReady()
	.then(() => app.mount('#aninmals'))