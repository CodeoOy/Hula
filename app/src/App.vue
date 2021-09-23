<template>
	<TheHeader v-on:loggedout="logOut" v-if="this.$store.state.loggeduser"/>
	<FlashMessage position="right top" />
	<TheModal />
	<main id='main'>
		<router-view />
	</main>
	<TheFooter />
</template>

<script>
	import TheHeader from '@components/TheHeader.vue' 
	import TheFooter from '@components/TheFooter.vue' 
	export default {
		name: 'App',
		components: {
			TheHeader,
			TheFooter,
  		},
		methods: {
			logOut() {
				this.$router.push({ name: 'login' })
			},
		},
	}
</script>
