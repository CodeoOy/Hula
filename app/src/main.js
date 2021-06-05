import { createApp } from 'vue'
import { createStore } from 'vuex'
import FlashMessage from '@smartweb/vue-flash-message';
import setupRouter from './router.js'
import App from './App.vue'
import 'bootstrap'

// Create a new store instance.

function getProjectNeeds(id) {
	console.log("Fetching needs")
	fetch(`/api/projectneeds/${id}`, {
		method: 'GET',
		headers: {"Content-Type": "application/json"},
		credentials: 'include'
	})
	.then((response) => response.json())
	.then((response) => {
		return response
	})
}

const store = createStore({
	state () {
		return {
			loggeduser: JSON.parse(localStorage.getItem('user')),
			chosenproject: JSON.parse(localStorage.getItem('chosenproject')),
			projects: JSON.parse(localStorage.getItem('projects')),
		}
	},
	mutations: {
		setUser (state, data) {
			fetch(`/api/users/${data}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then((response) => response.json())
			.then(response => { 
				localStorage.setItem('user', JSON.stringify(response));
				state.loggeduser = response;
			})   
		},
		setProjects (state, data) {
			localStorage.setItem('projects', JSON.stringify(data));
			state.projects = data
		},
		setChosenProject (state, data) {
			localStorage.setItem('chosenproject', JSON.stringify(data));
			state.chosenproject = data
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