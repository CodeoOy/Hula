<template>
	<main>
		<Header />
		<FlashMessage position="right top" />
		<router-view />
	</main>
</template>

<script>
	import Header from './components/Header.vue' 
	export default {
		name: 'App',

		data() {
			return {
				variable: false,
				currentpath: this.$router.currentRoute.value.path
			}
		},
		components: {
			'Header': Header,
  		}
	}
</script>
