import { createRouter, createWebHistory } from 'vue-router'
import state from './store/state.js'
import { Modal } from 'bootstrap'

import Confirm from './views/Confirm.vue'
import ForgotPassword from './views/ForgotPassword.vue'
import UserHome from './views/UserHome.vue'
import UserProfile from './views/UserProfile.vue'
import Admin from './views/Admin.vue'
import AdminProjects from './views/AdminProjects.vue'
import AdminSkills from './views/AdminSkills.vue'
import AdminUsers from './views/AdminUsers.vue'
import AdminScopes from './views/AdminScopes.vue'
import AdminOffers from './views/AdminOffers.vue'
import Error from './views/Error.vue'
import Project from './views/Project.vue'
import Gdpr from './views/Gdpr.vue'
import Login from './views/Login.vue'
import Register from './views/Register.vue'

const error = (to, props = {}) => ({
	name: 'error',
	params: {
		...props,
		pathMatch: to.path.split('/').slice(1),
	}
})

const needLogin = to => state.loggeduser ? true : { name: 'login', query: { redirect: to.fullPath } }

const needAdmin = to => state.loggeduser.isadmin ? true : error(to)

const needAdminOrSelf = to => state.loggeduser.isadmin || to.params.id == state.loggeduser.id ? true : error(to)

const router = createRouter({
	routes: [
		{ path: '/', name: 'home', redirect: () => ({ name: !state.loggeduser ? 'login' : state.loggeduser.isadmin ? 'admin-projects' : 'user-home' }) },
		{ path: '/app/confirm', component: Confirm, name: 'confirm' },
		{ path: '/app/forgotpassword', component: ForgotPassword, name: 'forgot-password' },
		{ path: '/app/gdpr', component: Gdpr, name: 'gdpr' },
		{ path: '/app/login', component: Login, name: 'login' },
		{ path: '/app/register', component: Register, name: 'register' },
		{ path: '/app/user', component: UserHome, name: 'user-home', beforeEnter: [needLogin] },
		{ path: '/app/user/:id', component: UserProfile, name: 'user', beforeEnter: [needLogin, needAdminOrSelf] },
		{ path: '/app/admin', component: Admin, beforeEnter: [needLogin, needAdmin], children: [
			{ path: 'projects', component: AdminProjects, name: 'admin-projects' },
			{ path: 'skills', component: AdminSkills, name: 'admin-skills' },
			{ path: 'users', component: AdminUsers, name: 'admin-users' },
			{ path: 'scopes', component: AdminScopes, name: 'admin-scopes' },
			{ path: 'offers', component: AdminOffers, name: 'admin-offers' },
		] },
		{ path: '/app/project/:id', component: Project, name: 'project', beforeEnter: [needLogin] },
		{ path: '/:pathMatch(.*)*', component: Error, name: 'error', props: true },
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

export default router
