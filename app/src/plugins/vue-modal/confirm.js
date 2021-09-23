import FormConfirm from '../../forms/FormConfirm.vue'
import { api } from '../../api.js'

export default modal => {
	const confirm = ({ title, ok = 'OK', cancel = 'Cancel' } = {}) => modal({
		component: FormConfirm,
		title,
		props: {
			ok,
			cancel,
		},
	})

	confirm.delete = async (type, data) => {
		let title, apiCall
	
		switch (type) {
			case 'project':
				title = data.name
				apiCall = api.projects.delete.bind(null, data.id)
				break
	
			case 'need':
				title = data.label
				apiCall = api.needs.delete.bind(null, data.id)
				break
	
			case 'need.skill':
				title = data.skill_label
				apiCall = api.needs.skills.delete.bind(null, data.id)
				break
	
			case 'user':
				title = 'profile'
				apiCall = api.users.delete.bind(null, data.id)
				break
	
			case 'user.file':
				title = `file ${data.filename}`
				apiCall = api.users.files.delete.bind(null, data.id)
				break
	
			case 'user.skill':
				title = data.skill_label
				apiCall = api.users.skills.delete.bind(null, {
					id: data.id,
					user_id: data.user_id,
				})
				break
	
			case 'user.reservation':
				title = data.description
				apiCall = api.users.reservations.delete.bind(null, {
					id: data.id,
					user_id: data.user_id,
				})
				break
	
			case 'skill':
				title = data.label
				apiCall = api.skills.delete.bind(null, data.id)
				break
	
			case 'skill.category':
				title = data.label
				apiCall = api.skills.categories.delete.bind(null, data.id)
				break
	
			case 'skill.scope':
				title = data.label
				apiCall = api.skills.scopes.delete.bind(null, data.id)
				break
	
			case 'skill.level':
				title = data.label
				apiCall = api.skills.levels.delete.bind(null, data.id)
				break
	
			case 'offer':
				title = `${data.project_name}: ${data.user_name}`
				apiCall = api.offers.delete.bind(null, data.id)
				break
		}
	
		const confirmed = await confirm({
			title: `Delete ${title}?`,
			ok: 'Delete',
		})
	
		return confirmed
			? apiCall()
			: confirmed
	}

	return confirm
}
