<template>
	<main>
		<Header />
		<FlashMessage position="right top" />
		<router-view />
		{{ logged }}
	</main>
</template>

<script>
	import Header from './components/Header.vue' 
	export default {
		name: 'App',

		data() {
			return {
				variable: false,
				currentpath: this.$router.currentRoute.value.path,
				logged: "nope"
			}
		},
		components: {
			'Header': Header,
  		},
		methods: {
			checkLogin: function() {
				fetch('api/auth', {method: 'GET'})
				.then((response) => {
					if(response.ok) {
						this.logged = "henlo";
					} else {
						this.logged = "nope";
					}
				})
			}
		},
		mounted() {
			this.checkLogin();
		}
	}
</script>
