import router from '../router.js'

const ensureArray = value => {
	if (!Array.isArray(value)) {
		console.error(`Expecting an array, got "${typeof value}"`, value)
		return []
	}
	return value
}

export default {
	setUser(state, data) {
		state.loggeduser = data
		if (data) {
			localStorage.setItem('user', JSON.stringify(data))
		} else {
			localStorage.removeItem('user')
		}
	},

	setChosenProject(state, data) {
		state.chosenproject = data
	},

	setProjects(state, data) {
		state.projects = ensureArray(data)
	},

	setSkills(state, data) {
		state.skills = ensureArray(data)
	},

	setSkillLevels(state, data) {
		state.skillLevels = ensureArray(data)
	},



	// TODO: Get rid of this
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
						title: 'Item may be in use.',
						time: 5000
					}
					break;
				default:
					console.log(errObject)
			}
		})
	},
}
