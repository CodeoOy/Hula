<template>
	<div id="mainwrap" :class="this.$route.name">
		<TheHeader v-on:loggedout="logOut" v-if="this.$store.state.loggeduser"/>
		<FlashMessage position="right top" />
		<main>
			<router-view />
		</main>
		{{ this.$store.state.projects }}
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
				this.$store.commit('errorHandling')
				this.$router.push({ name: 'page-login' })
			},
			flashIt(errorObject) {
				this.$flashMessage.show({
					type: errorObject.type,
					title: errorObject.title,
					time: errorObject.time,
				})
			}
		},
		computed: {
			errorObject: {
				get () {
					return this.$store.state.errorObject
				}
			}
		},
		watch: {
			'errorObject'(newObject) {
				this.flashIt(newObject)
			}
		},
	}
</script>
