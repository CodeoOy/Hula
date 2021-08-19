export default {
	async setUser(context, data) {
		const userData = await fetch(`/api/users/${data}`, {
			method: 'GET',
			headers: {"Content-Type": "application/json"}
		})
		.then((response) => response.json())
		context.commit('setUser', userData)
	},
	async setChosenProject (context, data) {
		try {
			let hasNeeds = true
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
				//console.log(errors)
				project.needs = {}
				hasNeeds = false
			})
			if (hasNeeds === true) {
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
						//console.log(errors)
						need.skills = {}
					})
				))
			}
			context.commit('setChosenProject', project)
		} catch (errors) {
			console.log(errors)
		}
	},
}