import { createRouter, createWebHistory } from 'vue-router'
import state from './store/state.js'
import { Modal } from 'bootstrap'

import Confirm from './views/Confirm.vue'
import ForgotPassword from './views/ForgotPassword.vue'
import UserHome from './views/UserHome.vue'
import UserProfile from './views/UserProfile.vue'
import Admin from './views/Admin.vue'
import AdminHome from './views/AdminHome.vue'
import AdminProjects from './views/AdminProjects.vue'
import AdminSkills from './views/AdminSkills.vue'
import AdminUsers from './views/AdminUsers.vue'
import AdminScopes from './views/AdminScopes.vue'
import AdminOffers from './views/AdminOffers.vue'
import Error from './views/Error.vue'
import Project from './views/Project.vue'
import Gdpr from './views/Gdpr.vue'
import Login from './views/Login.vue'

const router = createRouter({
	routes: [
		{ path: '/', name: 'home', redirect: () => ({ name: state.loggeduser.isadmin ? 'admin-home' : 'user-home' }) },
		{ path: '/app/confirm', component: Confirm, name: 'confirm' },
		{ path: '/app/forgotpassword', component: ForgotPassword, name: 'forgot-password' },
		{ path: '/app/user', component: UserHome, name: 'user-home' },
		{ path: '/app/user/:id', component: UserProfile, name: 'user' },
		{ path: '/app/admin', component: AdminHome, name: 'admin-home' },
		{ path: '/app/admin', component: Admin, name: 'admin', children: [
			{ path: 'projects', component: AdminProjects, name: 'admin-projects' },
			{ path: 'skills', component: AdminSkills, name: 'admin-skills' },
			{ path: 'users', component: AdminUsers, name: 'admin-users' },
			{ path: 'scopes', component: AdminScopes, name: 'admin-scopes' },
			{ path: 'offers', component: AdminOffers, name: 'admin-offers' },
		] },
		{ path: '/app/gdpr', component: Gdpr, name: 'gdpr' },
		{ path: '/app/login', component: Login, name: 'login' },
		{ path: '/app/project/:id', component: Project, name: 'project' },
		{ path: '/app/:pathMatch(.*)*', component: Error, name: 'error' },
	],
	history: createWebHistory(),
	linkActiveClass: 'active',
	linkExactActiveClass: ''
})

// Close modal before navigating
router.beforeEach((to, from, next) => {
	const modal = document.querySelector('.modal.show')
	if (!modal) {
		next()
	} else {
		modal.addEventListener('hidden.bs.modal', () => next())
		Modal.getInstance(modal).hide()
	}
})

router.beforeEach((to, from, next) => {
	if (to.name !== 'login' 
		&& to.name !== 'confirm'
		&& to.name !== 'forgot-password'
		&& !state.loggeduser) {
			next({ name: 'login', query: { redirect: to.fullPath } })
	}
	else next()
})

export default router
