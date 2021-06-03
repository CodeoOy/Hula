<template>
	<main>
		<Header v-on:loggedout="checkLogin" />
		<FlashMessage position="right top" />
		<router-view :logged='logged' v-on:checklogin="checkLogin" />
		<p>{{ this.$store.state.loggeduser }}</p>
		<p>{{ user }}</p>
	</main>
</template>

<script>
	import Header from './components/Header.vue' 
	export default {
		name: 'App',

		data() {
			return {
				currentpath: this.$router.currentRoute.value.path,
				logged: false,
				user: {},
				projects: {}
			}
		},
		components: {
			'Header': Header,
  		},
		methods: {
			checkLogin: function() {
				fetch('/api/auth', {method: 'GET'})
				.then((response) => {
					if(response.ok) {
						this.logged = true;
						this.$flashMessage.show({
							type: 'success',
							title: 'Some kind of success',
							time: 1000
						});
					} else {
						fetch('/api/auth', {method: 'DELETE'})
						this.logged = false;
						this.$flashMessage.show({
							type: 'error',
							title: 'Some kind of error or unauthorized',
							time: 1000
						});
					}
				})
				this.user = this.$store.state.loggeduser
			},
			getProjects: function() {
				fetch('/api/projects', {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.then(response => {
					let self = this
					this.projects = response
					this.projects.forEach(function (project) {
						fetch(`/api/projectneeds/${project.id}`, {
							method: 'GET',
							headers: {"Content-Type": "application/json"},
							credentials: 'include'
						})
						.then((response) => response.json())
						.then((response) => {
							project.needs = response
						})
						//self.$store.commit('setProjects', self.projects)
					});
					self.$store.commit('setProjects', self.projects)
					//console.log(self.$store.state.projects)
				})
			},
		},
		mounted() {
			this.checkLogin()
			this.getProjects()
		},
		updated() {
			this.checkLogin()
		}
	}
</script>
