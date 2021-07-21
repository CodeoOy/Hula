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
			{ path: '/', component: views.Home, name: 'page-home'},
			{ path: '/app/confirm', component: views.Confirm, name: 'page-confirm' },
			{ path: '/app/dashboard', component: views.Dashboard, name: 'page-dashboard' },
			{ path: '/app/user/:id', component: views.Profile, name: 'page-profile' },
			{ path: '/app/admin', component: views.Admin, name: 'page-admin' },
			{ path: '/app/gdpr', component: views.Gdpr, name: 'page-gdpr' },
			{ path: '/app/login', component: views.Login, name: 'page-login' },
			{ path: '/app/project/:id', component: views.Project, name: 'page-project' },
			{ path: '/app/:pathMatch(.*)*', component: views.PageError, name: 'page-error' },
		],
		history: createWebHistory(),
		linkActiveClass: 'active',
		linkExactActiveClass: ''
	})
	return router
}
