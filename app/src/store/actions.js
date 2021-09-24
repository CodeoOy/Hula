import { api } from '@root/api.js'

import router from '@root/router.js'

export default {
	async setUser(context, data) {
		if (typeof data == 'string') {
			try {
				const [ user, favorites ] = await Promise.all([
					api.users.get({ id: data }),
					api.users.favorites.get({ id: data }),
				])

				user.favorites = favorites.map(favorite => favorite.project_id)
				data = user
			} catch (error) {
				data = null
			}
		}

		context.commit('setUser', data)
	},

	async setFavorite(context, data) {
		const { state, ...payload } = data

		if (state) {
			const result = await api.users.favorites.save(payload)
			if (result) context.commit('addFavorite', payload.project_id)
		} else {
			const result = await api.users.favorites.delete(payload)
			if (result) context.commit('removeFavorite', payload.project_id)
		}

		return state
	},

	async setChosenProject(context, data) {
		if (typeof data == 'string') {
			const promises = [
				api.projects.get({ id: data }),
			]

			if (context.state.loggeduser.isadmin) promises.push(api.projects.favorites.get({ id: data }))
			if (!context.state.skills.length) promises.push(context.dispatch('getSkills'))
			if (!context.state.skillLevels.length) promises.push(context.dispatch('getSkillLevels'))

			const [
				project,
				favorites,
			] = await Promise.all(promises)

			if (favorites) project.favorites = favorites.map(favorite => favorite.user_id)

			project.needs.forEach(need => {
				need.skills.forEach(skill => {
					Object.entries({
						skills: 'skill_id',
						skillLevels: 'skillscopelevel_id',
					}).forEach(([ item, prop ]) => {
						const result = context.state[item].find(({ id }) => skill[prop] == id)
						skill[prop.replace('id', 'label')] = result ? result.label : ''
					})
				})
			})

			data = project
		}

		context.commit('setChosenProject', data)
	},

	async getProjects(context) {
		const data = await api.projects.get()
		context.commit('setProjects', data)
	},

	async getSkillCategories(context) {
		const data = await api.skills.categories.get()
		context.commit('setSkillCategories', data)
	},

	async getSkillScopes(context) {
		const data = await api.skills.scopes.get()
		context.commit('setSkillScopes', data)
	},

	async getSkills(context) {
		const data = await api.skills.get()
		context.commit('setSkills', data)
	},

	async getSkillLevels(context) {
		const data = await api.skills.levels.get()
		context.commit('setSkillLevels', data)
	},

	async saveSkillLevel(context, data) {
		const item = await api.skills.levels.save(data)
		if (item) await context.dispatch('getSkillLevels')
	},

	async login(context, data) {
		try {
			const userId = await api.users.log.in(data)
			if (userId) await context.dispatch('setUser', userId)
		} catch (error) {
			console.warn(`Login failed: ${error.message}`)
			return false
		}
		return true
	},

	async logout(context) {
		try {
			await api.users.log.out()
			await context.dispatch('setUser', null)
			router.push({ name: 'login' })
		} catch (error) {
			console.warn(`Logout failed: ${error.message}`)
			return false
		}
		return true
	},
}
