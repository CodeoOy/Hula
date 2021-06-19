<template>
	<div id="mainwrap">
		<TheHeader v-on:loggedout="checkLogin" />
		<FlashMessage position="right top" />
		<main>
			<router-view v-on:checklogin="checkLogin" />
		</main>
		<TheFooter />
	</div>
</template>

<script>
	import TheHeader from './components/TheHeader.vue' 
	import TheFooter from './components/TheFooter.vue' 
	export default {
		name: 'App',

		data() {
			return {
				currentpath: this.$router.currentRoute.value.path,
				user: {}
			}
		},
		components: {
			TheHeader,
			TheFooter,
  		},
		methods: {
			checkLogin: function() {
				console.log("Checking log in")
				fetch('/api/auth', {method: 'GET'})
				.then(response => response.json()
				.catch((errors) => {
					console.log(errors);
					this.$store.commit('deleteUser')
					this.user = {}
					this.$flashMessage.show({
						type: 'error',
						title: 'Unauthorized',
						time: 1000
					});
					this.$router.push({ path: '/' })
				})
				.then(data => ({status: response, body: data})))
				.then(obj => {
					if(obj.status.ok) {
						this.$store.commit('setUser', obj.body)
						this.user = this.$store.state.loggeduser
					} else {
						console.log("not ok")
					}
				})
			},
		},
		mounted() {
			this.checkLogin()
			this.$store.commit('getProjects')
		}
	}
</script>
