<template>
	<main>
		<Header v-on:loggedout="checkLogin" />
		<FlashMessage position="right top" />
		<router-view :logged='logged' v-on:checklogin="checkLogin" />
		{{ this.$store.state.loggeduser }}
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
			}
		},
		components: {
			'Header': Header,
  		},
		methods: {
			checkLogin: function() {
				fetch('http://localhost:8086/api/auth', {method: 'GET'})
				.then((response) => {
					if(response.ok) {
						this.logged = true;
						this.$flashMessage.show({
							type: 'success',
							title: 'Some kind of success',
							time: 1000
						});
					} else {
						fetch('http://localhost:8086/api/auth', {method: 'DELETE'})
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
		},
		mounted() {
			this.checkLogin()
		},
		updated() {
			this.checkLogin()
		}
	}
</script>
