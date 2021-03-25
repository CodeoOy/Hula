<template>
	<main>
		<Header />
		<FlashMessage position="right top" />
		<h1>Rust Vue POC</h1>
		<p class="routermessage"><router-view /></p>
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
