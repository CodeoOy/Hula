<template>
	<nav class="navbar navbar-expand-sm shadow mb-4 mb-md-5" :class='$colorScheme.navbar'>
		<div class="container-fluid">
			<router-link :to='{ name: "home" }' class="navbar-brand hula-logo py-0">
				<svg height='48' xmlns='http://www.w3.org/2000/svg' viewBox='0 0 79 35'>
					<path fill-rule='nonzero' d='M72.46 15.53c2.01 0 3.25 1.16 3.25 3.03v.75h-3.89c-2.97 0-4.82 1.46-4.82 3.88 0 2.54 1.85 3.94 4.77 3.94 1.65 0 3.25-.69 3.94-1.98v1c0 .46.39.85.86.85.46 0 .85-.39.85-.86v-7.58c0-2.81-1.77-4.68-4.96-4.68-2.32 0-3.8.96-4.58 2.17a.85.85 0 0 0-.14.47c0 .44.36.8.8.8.25 0 .44-.08.72-.39a3.82 3.82 0 0 1 3.2-1.4Zm-.53 10c-2.15 0-3.14-.9-3.14-2.34 0-1.6 1.24-2.34 3.03-2.34h3.89V22c0 2.34-1.85 3.52-3.78 3.52ZM61.88 7.15a.9.9 0 0 0-.88.88v15.74C61 25.9 62.02 27 63.87 27a.83.83 0 0 0 0-1.65c-.75 0-1.1-.48-1.1-1.58V8.03a.89.89 0 0 0-.89-.88Zm-14 6.89c-.5 0-.88.39-.88.88v7.2c0 2.61.88 5.01 5.13 5.01 4.27 0 5.12-2.4 5.12-5.01v-7.2c0-.5-.38-.88-.85-.88a.9.9 0 0 0-.91.88v7.2c0 1.79-.58 3.33-3.36 3.33-2.79 0-3.37-1.54-3.37-3.33v-7.2c0-.5-.4-.88-.88-.88Zm-6.41 4.08v7.94c0 .5.41.94.93.94.53 0 .94-.45.94-.94V8.94c0-.5-.41-.94-.94-.94a.95.95 0 0 0-.93.94v7.47h-9.6V8.94c0-.5-.4-.94-.93-.94a.95.95 0 0 0-.94.94v17.12c0 .5.41.94.94.94.52 0 .93-.45.93-.94v-7.94h9.6Z'/>
					<circle id='hoop' cx='11.82' cy='17.5' r='8.77' fill='none' stroke-width='1.44'/>
					<circle cx='14.62' cy='14.51' r='4.31'/>
				</svg>
			</router-link>
			<ul class="navbar-nav">
				<li class="nav-item dropdown">
					<button id="usermenu" class='nav-link btn btn-unstyled fs-2 rounded-circle' data-bs-toggle="dropdown" aria-expanded="false">
						<i class='bi-person-circle'></i>
					</button>
					<ul v-if="loggedUser" class="dropdown-menu dropdown-menu-end position-absolute" aria-labelledby="usermenu">
						<template v-if='loggedUser.isadmin'>
							<li v-for='{ name, label } of navigation' :key='name'>
								<router-link :to='{ name }' class='dropdown-item'>{{ label }}</router-link>
							</li>
							<li><hr class='dropdown-divider'></li>
						</template>
						<li>
							<router-link :to='{ name: "user", params: { id: loggedUser.id } }' class="dropdown-item">Profile</router-link>
						</li>
						<li>
							<button v-on:click="logOut()" class="dropdown-item">Log out</button>
						</li>
					</ul>
				</li>
			</ul>
		</div>
	</nav>
</template>

<script>
import { navigation } from '@views/Admin.vue'

export default {
	name: 'TheHeader',
	data() {
		return {
			navigation,
			loggedUser: this.$store.state.loggeduser // Here loggedUser will result to menu rendering vs. not if you use the state directly
		}
	},

	methods: {
		async logOut() {
			const success = await this.$store.dispatch('logout')

			if (success) this.$flashMessage.show({
				type: 'success',
				title: 'Successfully logged out',
				time: 5000,
			})
		},
	}
};
</script>
