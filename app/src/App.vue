<template>
	<div id="mainwrap" :class="this.$route.name">
		<TheHeader v-on:loggedout="logOut" v-if="this.$store.state.loggeduser"/>
		<FlashMessage position="right top" />
		<main>
			<router-view />
		</main>
		{{ this.$store.state.loggeduser }}
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
				this.$store.commit('deleteUser')
				this.$router.push({ name: 'page-login' })
			}
		}
	}
</script>
