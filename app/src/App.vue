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
				fetch('http://localhost:8086/api/auth', {method: 'GET'})
				//.then((response) => response.json())
				.then((response) => {
					//console.log(response)
					if(response.ok) {
						this.logged = "henlo";
					} else {
						this.logged = "nope";
					}
				})
			}
		},
		mounted() {
			this.checkLogin()
			console.log("mounted")
		},
		updated() {
			this.checkLogin()
			console.log("updated")
		}
	}
</script>
