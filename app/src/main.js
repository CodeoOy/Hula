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
			nextpage: ''
		}
	},
	actions: {
		async setUser(context, data) {
			const userData = await fetch(`/api/users/${data}`, {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then((response) => response.json())
			context.commit('setUser', userData)
		},
	},
	mutations: {
		setUser(state, data) {
			state.loggeduser = data;
			localStorage.setItem('user', JSON.stringify(data));
		},
		getProjects (state) {
			fetch('/api/projects', {
				method: 'GET',
				headers: {"Content-Type": "application/json"}
			})
			.then((response) => response.json())
			.catch((errors) => {
				console.log(errors);
			})
			.then(response => {
				state.projects = response
				state.projects.forEach(function (project) {
					fetch(`/api/projectneeds/${project.id}`, {
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
					})
				});
			})
			localStorage.setItem('projects', JSON.stringify(state.projects));
		},
		async setChosenProject (state, data) {
			try {
				let hasProjects = true
				let project = await fetch(`/api/projects/${data}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.then((response) => response.json())
				project.needs = await fetch(`/api/projectneeds/${data}`, {
					method: 'GET',
					headers: {"Content-Type": "application/json"},
					credentials: 'include'
				})
				.then((response) => response.json())
				.catch((errors) => {
					console.log("No needs for project: " + project.id)
					console.log(errors)
					project.needs = {}
					hasProjects = false
				})
				console.log("Project from state:")
				console.log(project)
				if (hasProjects === true) {
					await Promise.all(project.needs.map(need =>
						fetch(`/api/projectskills/${need.id}`, {
							method: 'GET',
							headers: {"Content-Type": "application/json"},
							credentials: 'include'
						})
						.then((response) => response.json())
						.then(response => {
							need.skills = response
						})
						.catch((errors) => {
							console.log("No skills for need" + need.id)
							console.log(errors)
							need.skills = {}
						})
					))
				}
				localStorage.setItem('chosenproject', JSON.stringify(project));
				state.chosenproject = project
			} catch (errors) {
				console.log(errors)
			}
		},
		resetChosenProject (state) {
			state.chosenproject = {}
		},
		deleteUser (state) {
			state.loggeduser = null
			localStorage.removeItem('user');
		}
	}
})

const router = setupRouter()
const app = createApp(App)
	.use(router)
	.use(FlashMessage)
	.use(store)
router.beforeEach((to, from, next) => {
	if (to.name !== 'page-login' && !store.state.loggeduser) {
		store.state.nextpage = to.name
		next({ name: 'page-login' })
	}
	else next()
})
router.isReady()
	.then(() => app.mount('#hula'))
