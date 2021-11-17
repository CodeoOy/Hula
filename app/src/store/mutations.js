import router from '@root/router.js'

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

	addFavorite(state, data) {
		state.loggeduser.favorites.push(data)

		if (state.chosenproject.id == data) {
			state.chosenproject.favorites.push(state.loggeduser.id)
		}
	},

	removeFavorite(state, data) {
		const removeIdfromArray = (id, array) => {
			const index = array.indexOf(id)
			if (index > -1) array.splice(index, 1)
		}

		removeIdfromArray(data, state.loggeduser.favorites)

		if (state.chosenproject.id == data) {
			removeIdfromArray(state.loggeduser.id, state.chosenproject.favorites)
		}
	},

	setChosenProject(state, data) {
		state.chosenproject = data
	},

	setCookieConsent(state, data) {
		state.cookieConsent = data
		if (data) {
			localStorage.setItem("cookieConsent", JSON.stringify(data))
		} else {
			localStorage.removeItem("cookieConsent")
		}
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
