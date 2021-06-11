<template>
	<main>
		<TheHeader v-on:loggedout="checkLogin" />
		<FlashMessage position="right top" />
		<router-view :logged='logged' v-on:checklogin="checkLogin" />
		<p>{{ this.$store.state.loggeduser }}</p>
		<p>{{ user }}</p>
		<TheFooter />
	</main>
</template>

<script>
	import TheHeader from './components/TheHeader.vue' 
	import TheFooter from './components/TheFooter.vue' 
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
			TheHeader,
			TheFooter,
  		},
		methods: {
			checkLogin: function() {
				fetch('/api/auth', {method: 'GET'})
				.then((response) => {
					if(response.ok) {
						this.logged = true;
					} else {
						this.logged = false;
						this.$flashMessage.show({
							type: 'error',
							title: 'Unauthorized',
							time: 1000
						});
						this.$router.push({ path: '/' })
					}
				})
				.catch((errors) => {
					console.log(errors);
				})
				this.user = this.$store.state.loggeduser
			},
			getProjects: function() {
				fetch('/api/projects', {
					method: 'GET',
					headers: {"Content-Type": "application/json"}
				})
				.then((response) => response.json())
				.catch((errors) => {
					console.log(errors);
				})
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
						.catch((errors) => {
							console.log("No needs for project: " + project.id)
							console.log(errors)
							project.needs = {}
						})
						.then((response) => {
							project.needs = response
						})
					});
					self.$store.commit('setProjects', self.projects)
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
