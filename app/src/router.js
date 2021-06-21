import { createRouter, createWebHistory } from 'vue-router'
import * as views from './views.js'

const setupRoute = entry => ({
	path: entry.path,
	name: `${entry.id}`,
	component: views[entry.component],
	meta: {
		id: entry.id,
		title: entry.title,
		hidden: entry.hidden,
	},
})

export default function setupRouter() {
	const router = createRouter({
		routes: [
			{ path: '/app/:pathMatch(.*)*', component: views.Page404 },
			{ path: '/', component: views.Home, name: 'Home'},
			{ path: '/app/confirm', component: views.Confirm },
			{ path: '/app/dashboard', component: views.Dashboard },
			{ path: '/app/profile', component: views.Profile },
			{ path: '/app/admin', component: views.Admin },
			{ path: '/app/gdpr', component: views.Gdpr },
			{ path: '/app/login', component: views.Login, name: 'Login' },
			{ path: '/app/project/:id', component: views.Project, name: 'project' },
		],
		history: createWebHistory(),
		linkActiveClass: 'active',
		linkExactActiveClass: ''
	})
	return router
}
