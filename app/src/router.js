import { createRouter, createWebHistory } from 'vue-router'
import state from './store/state.js'
import { Modal } from 'bootstrap'

import Home from './views/Home.vue'
import Confirm from './views/Confirm.vue'
import ForgotPassword from './views/ForgotPassword.vue'
import Dashboard from './views/Dashboard.vue'
import Profile from './views/Profile.vue'
import Admin from './views/Admin.vue'
import PageError from './views/PageError.vue'
import Project from './views/Project.vue'
import Gdpr from './views/Gdpr.vue'
import Login from './views/Login.vue'

const router = createRouter({
	routes: [
		{ path: '/', component: Home, name: 'page-home'},
		{ path: '/app/confirm', component: Confirm, name: 'page-confirm' },
		{ path: '/app/forgotpassword', component: ForgotPassword, name: 'page-forgot-password' },
		{ path: '/app/dashboard', component: Dashboard, name: 'page-dashboard' },
		{ path: '/app/user/:id', component: Profile, name: 'page-profile' },
		{ path: '/app/admin', component: Admin, name: 'page-admin' },
		{ path: '/app/gdpr', component: Gdpr, name: 'page-gdpr' },
		{ path: '/app/login', component: Login, name: 'page-login' },
		{ path: '/app/project/:id', component: Project, name: 'page-project' },
		{ path: '/app/:pathMatch(.*)*', component: PageError, name: 'page-error' },
	],
	history: createWebHistory(),
	linkActiveClass: 'active',
	linkExactActiveClass: ''
})

// Close modal before navigating
router.beforeEach((to, from, next) => {
	const modal = document.querySelector('.show[id^=hulaModal]')
	if (!modal) {
		next()
	} else {
		modal.addEventListener('hidden.bs.modal', () => next())
		Modal.getInstance(modal).hide()
	}
})

router.beforeEach((to, from, next) => {
	console.log(to)
	if (to.name !== 'page-login' 
		&& to.name !== 'page-confirm'
		&& to.name !== 'page-forgot-password'
		&& !state.loggeduser) {
			state.nextpage = to.fullPath
			next({ name: 'page-login' })
	}
	else next()
})

export default router
