<template>
	<footer class="navbar navbar-dark mt-4 mt-md-5" :class='$colorScheme.navbar'>
		<div class="container-fluid">
			<div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
				<router-link to="/app/Gdpr">Privacy Policy</router-link>
			</div>
		</div>
		<div v-if="!cookieConsent" class="cookie-consent d-flex d-wrap align-items-center bg-dark shadow">
			<div class="text-light"><span>We use cookies on this website to distinguish you from other users. For details, please see our <router-link to="/app/Gdpr">Privacy Policy</router-link>.</span></div>
			<div><button v-on:click='giveCookieConsent()' class="btn btn-primary">Got it!</button></div>
		</div>
	</footer>
</template>

<script>
	export default {
		name: 'TheFooter',

		computed: {
				cookieConsent() {
					return this.$store.state.cookieConsent
				}
		},

		methods: {
			giveCookieConsent() {
				this.$store.dispatch("setCookieConsent", true)
			}
		}
	}
</script>