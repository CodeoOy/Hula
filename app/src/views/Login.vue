<template>

</template>

<script>
	import FormLogin from '../forms/FormLogin.vue'

	export default {
		name: 'Login',

		async mounted() {
			// Forgot password will return untrue
			const success = await this.$modal({
				title: 'Log in',
				component: FormLogin,
				backdrop: 'static',
			})

			if (success) this.$router.replace(this.$route.query.redirect || { name: 'home' })
		},
	}
</script>