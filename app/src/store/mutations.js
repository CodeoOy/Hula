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

	setSkillCategories(state, data) {
		state.skillCategories = ensureArray(data)
	},

	setSkillScopes(state, data) {
		state.skillScopes = ensureArray(data)
	},

	setSkills(state, data) {
		state.skills = ensureArray(data)
	},

	setSkillLevels(state, data) {
		state.skillLevels = ensureArray(data)
	},
}
