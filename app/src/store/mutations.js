import router from '../router.js'

export default {
	setUser(state, data) {
		state.loggeduser = data;
		localStorage.setItem('user', JSON.stringify(data));
	},
	setChosenProject(state, data) {
		state.chosenproject = data
	},
	getProjects (state) {
		let projectsExist = true
		let projects = []
		fetch('/api/projects?is_include_skills_and_matches=true', {
			method: 'GET',
			headers: {"Content-Type": "application/json"}
		})
		.then(response => {
			if (response.status == 204) {
				projects = []
				projectsExist = false
			} else {
				return response.json()
			}
		})
		.catch((errors) => {
			console.log(errors);
		})
		.then(response => {
			if (projectsExist === true) {
				projects = response
				projects.forEach(function (project) {
					fetch(`/api/projectneeds/${project.id}`, {
						method: 'GET',
						headers: {"Content-Type": "application/json"},
						credentials: 'include'
					})
					.then((response) => response.json())
					.catch((errors) => {
						console.log(errors)
						project.needs = {}
					})
					.then((response) => {
						project.needs = response
					})
				});
				localStorage.setItem('projects', JSON.stringify(projects));
				state.projects = projects
			} else {
				projects = []
				localStorage.setItem('projects', JSON.stringify(projects));
				state.projects = projects
			}
		})
	},
	resetChosenProject (state) {
		state.chosenproject = {}
	},
	errorHandling (state, error) {
		if(error.status == 401) {
			state.loggeduser = null
			localStorage.removeItem('user');
			router.push({ name: 'page-login' })
			return "Unauthorized"
		}
		if(error.status == 500) {
			state.errorObject = {
				type: 'error',
				title: 'Error 500',
				time: 1000
			}
			router.push({ name: 'page-error' })
			return "Unauthorized"
		}
		if(error.status == 400) {
			state.errorObject = {
				type: 'error',
				title: 'Error 400',
				time: 1000
			}
		}
		let errorObject = Promise.resolve(error)
		errorObject.then((resError) => resError.json())
		.then(errObject => {
			switch (errObject.error_type) {
				case 'UniqueViolation':
					state.errorObject = {
						type: 'error',
						title: 'Item already exists.',
						time: 5000
					}
					break;
				case 'AdminRequired':
					state.loggeduser = null
					localStorage.removeItem('user');
					router.push({ name: 'page-login' })
					break;
				case 'ForeignKeyViolation':
					state.errorObject = {
						type: 'error',
						title: 'Foreign key violation',
						time: 5000
					}
					break;
				default:
					console.log(errObject)
			}
		})
	},
	deleteUser (state) {
		state.loggeduser = null
		localStorage.removeItem('user');
		localStorage.removeItem('projects');
		router.push({ name: 'page-login' })
	}
}
