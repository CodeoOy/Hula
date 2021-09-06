<template>
	<div id="mainwrap">
		<TheHeader v-on:loggedout="logOut" v-if="this.$store.state.loggeduser"/>
		<FlashMessage position="right top" />
		<TheModal />
		<main>
			<router-view />
		</main>
		<TheFooter />
	</div>
</template>

<script>
	import TheHeader from './components/TheHeader.vue' 
	import TheFooter from './components/TheFooter.vue' 
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
