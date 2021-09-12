let store, router, flashMessage

const errorMessages = {
	UniqueViolation: 'Item already exists',
}

const debounceFlashMessage = {}

function splitPascalCase(word) {
	const pattern = /($[a-z])|[A-Z][^A-Z]+/g
	const words = word.match(pattern).join(" ")
	return words[0] + words.slice(1).toLowerCase()
}

function debounce(func, timeout = 300) {
	let timer
	return (...args) => {
		if (!timer) func.apply(this, args)
		clearTimeout(timer)
		timer = setTimeout(() => {
			timer = undefined
		}, timeout)
	}
}

async function handleHttpStatus(response) {
	if (response.ok) return response

	let errorMessage = response.statusText

	let error = await response.json().catch(() => {})
	if (error) {
		error = error.error_type
		switch (error) {
			default:
				errorMessage = errorMessages[error] || splitPascalCase(error)
				console.error(error)
		}
	}

	if (!(errorMessage in debounceFlashMessage)) {
		debounceFlashMessage[errorMessage] = debounce(() => flashMessage.show({
			type: 'error',
			title: `Error ${response.status}`,
			text: errorMessage,
			time: 5000,
		}))
	}

	debounceFlashMessage[errorMessage]()

	switch (response.status) {
		case 401:
			store.dispatch('setUser', null)
			router.push({ name: 'login', query: {
				redirect: router.currentRoute.value.fullPath,
			}})
			break
		case 500:
			router.push({ name: 'error', params: {
				title: 'Error 500',
				message: errorMessage,
			}})
			break
	}

	throw Error(response.statusText)
}

const request = ({ url, onError, ...options } = {}) => fetch(url.replace(/\/+$/, ''), options)
	.then(handleHttpStatus)
	.catch(error => {
		if (onError !== undefined) return onError
		throw error
	})

const sendJson = ({ method = 'POST', ...args } = {}) => request({
	...args,
	method,
	headers: { 'Content-Type': 'application/json' },
	body: JSON.stringify(args.body),
})

const save = (url, { post = '', put = 'id' } = {}) => data => sendJson({
	url: `${url}/${data[put] || data[post] || ''}`,
	method: data[put] ? 'PUT' : 'POST',
	body: data,
})

const returnBoolean = promise => promise.then(() => true).catch(() => false)
const returnObject = promise => promise.then(response => response.json()).catch(() => null)
const returnArray = promise => promise.then(response => response.json()).catch(() => [])

const getArray = url => id => returnArray(request({ url: `${url}/${id || ''}` }))
const getObject = url => id => returnObject(request({ url: `${url}/${id || ''}` }))
const get = url => id => (id ? getObject(url) : getArray(url))(id)

const remove = url => data => returnBoolean(
	typeof data == 'string'
		? request({ url: `${url}/${data || ''}`, method: 'DELETE' })
		: sendJson({ url: `${url}/${data.id || ''}`, method: 'DELETE', body: data })
)

export const api = {

	projects: {
		get: async id => {
			if (!id) return returnArray(request({ url: '/api/projects?is_include_skills_and_matches=true' }))

			const [
				project,
				needs,
			] = await Promise.all([
				returnObject(request({ url: `/api/projects/${id}` })),
				api.needs.get(id),
			])

			if (project) project.needs = needs

			return project
		},

		save: save('/api/projects'),
		delete: remove('/api/projects'),
	},

	needs: {
		get: async id => {
			const needs = await returnArray(request({ url: `/api/projectneeds/${id}` }))

			const addSkills = async need => need.skills = await api.needs.skills.get(need.id)
			await Promise.all(needs.map(addSkills))

			return needs
		},

		save: save('/api/projectneeds'),
		delete: remove('/api/projectneeds'),

		skills: {
			get: getArray('/api/projectskills'),
			save: save('/api/projectskills'),
			delete: remove('/api/projectskills'),
		},
	},

	skills: {
		get: get('/api/skills'),
		save: save('/api/skills'),
		delete: remove('/api/skills'),

		levels: {
			get: get('/api/skills/levels'),
			save: save('/api/skills/levels'),
			delete: remove('/api/skills/levels'),
		},

		scopes: {
			get: get('/api/skills/scopes'),
			save: save('/api/skills/scopes'),
			delete: remove('/api/skills/scopes'),
		},

		categories: {
			get: get('/api/skills/categories'),
			save: save('/api/skills/categories'),
			delete: remove('/api/skills/categories'),
		},
	},

	users: {
		get: get('/api/users'),
		save: save('/api/users'),
		delete: remove('/api/users'),

		skills: {
			save: save('/api/userskills', { post: 'user_id' }),
			delete: remove('/api/userskills'),
		},

		reservations: {
			get: getArray('/api/userreservations'),
			save: save('/api/userreservations', { post: 'user_id' }),
			delete: remove('/api/userreservations'),
		},

		files: {
			get: getArray('/api/useruploads'),

			save: data => {
				const body = new FormData()
				body.append('id', data.id)
				if (data.files.length) data.files.forEach(file => body.append('files[]', file))
				return returnBoolean(request({
					url: '/api/upload',
					method: 'POST',
					body,
				}))
			},

			delete: remove('/api/useruploads'),
		},

		password: {
			requestReset: body => returnBoolean(sendJson({ url: '/api/resetpassword', body })),
			save: body => returnBoolean(sendJson({ url: '/api/updatepassword', method: 'PUT', body })),
		},

		registration: {
			invite: body => sendJson({ url: '/api/invitations', body }),
			confirm: body => returnBoolean(sendJson({ url: `/api/register/${body.id}`, body })),
		},

		log: {
			in: async body => {
				await sendJson({ url: '/api/auth', body })
				return returnObject(request({ url: '/api/auth' }))
			},

			out: () => request({ url: '/api/auth', method: 'DELETE' }),
		},
	},

	matches: {
		get: async id => {
			const matches = await returnObject(request({ url: `/api/matches/${id}` }))

			return !matches ? matches : matches.reduce((users, match) => {
				const { user, skill } = Object.entries(match).reduce((match, [prop, value]) => {
					match[prop in match.user ? 'user' : 'skill'][prop] = value
					return match
				}, { user: {
					user_id: null,
					user_first_name: null,
					user_last_name: null,
					user_is_hidden: null,
					user_favorite: false,
					project_id: null,
					project_name: null,
				}, skill: {} })

				if (user.user_id in users) {
					users[user.user_id].skills.push(skill)
				} else {
					users[user.user_id] = { ...user, skills: [skill] }
				}

				return users
			}, {})
		}
	},

	offers: {
		get: getArray('/api/offers'),
		save: save('/api/offers'),
		delete: remove('/api/offers'),
	},
}

export default {
	install: (app, options) => {
		store = app.config.globalProperties.$store
		router = app.config.globalProperties.$router
		flashMessage = app.config.globalProperties.$flashMessage

		app.config.globalProperties.$api = api
	},
}
