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
			fetch(`/api/projects/${data}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"},
				credentials: 'include'
			})
			.then((response) => response.json())
			.then((response) => {
				var project = response
				fetch(`/api/projectneeds/${data}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.then((response) => response.json())
				.catch((errors) => {
					console.log("No needs for project: " + project.id)
					console.log(errors)
					project.needs = {}
				})
				.then((response) => {
					project.needs = response
					project.needs.forEach(function (projectneed) {
						fetch(`/api/projectskills/${projectneed.id}`, {
							method: 'GET',
							headers: {"Content-Type": "application/json"},
							credentials: 'include'
						})
						.then((response) => response.json())
						.catch((errors) => {
							console.log("No skills for need: " + projectneed.id)
							console.log(errors)
							projectneed.skills = {}
						})
						.then((response) => {
							projectneed.skills = response
						})
					});
				})
				.catch((errors) => {
					console.log("No needs for project: " + project.id)
					console.log(errors)
					project.needs = {}
				})
				.then((response) => {
					console.log("Project from state:")
					console.log(project)
					localStorage.setItem('chosenproject', JSON.stringify(project));
					state.chosenproject = project
				})
			})
		},
		resetChosenProject (state) {
			state.chosenproject = {}
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