import { api } from '../api.js'

export default confirm => async (type, data) => {
	let title, apiCall

	switch (type) {
		case 'project':
			title = data.name
			apiCall = api.projects.delete
			break

		case 'need':
			title = data.label
			apiCall = api.needs.delete
			break

		case 'need.skill':
			title = data.label
			apiCall = api.needs.skills.delete
			break

		case 'user':
			title = 'profile'
			apiCall = api.users.delete
			break

		case 'user.file':
			title = `file ${data.filename}`
			apiCall = api.users.files.delete
			break

		case 'user.skill':
			title = data.skill_label
			apiCall = api.users.skills.delete
			break

		case 'user.reservation':
			title = data.description
			apiCall = api.users.reservations.delete
			break

		case 'skill':
			title = data.label
			apiCall = api.skills.delete
			break

		case 'skill.category':
			title = data.label
			apiCall = api.skills.categories.delete
			break

		case 'skill.scope':
			title = data.label
			apiCall = api.skills.scopes.delete
			break

		case 'skill.level':
			title = data.label
			apiCall = api.skills.levels.delete
			break

		case 'offer':
			title = `${data.project_name}: ${data.user_name}`
			apiCall = api.offers.delete
			break
	}

	const confirmed = await confirm(`Delete ${title}`)

	return confirmed
		? apiCall(data.id)
		: confirmed
}