<template>
	<div v-if='confirmed' class='container mt-5'>
		<div class='card shadow' :class='$colorScheme.card'>
			<div class='card-header'>
				<h1 class='h3 mb-0'>Account confirmed</h1>
			</div>
			<div class='card-body'>
				<p class='mb-0'>Your account has been confirmed. You can now <router-link :to='{ name: "login" }'>log in</router-link>.</p>
			</div>
		</div>
	</div>
</template>

<script>
	import FormResetPassword from '../forms/FormResetPassword.vue'

	export default {
		name: 'Confirm',

		data() {
			return {
				confirmed: false,
			}
		},

		async mounted() {
			const data = this.$route.query

			if (data.type == 'reset') {
				this.resetPassword(data)
			} else {
				this.confirmAccount(data)
			}
		},

		methods: {
			async resetPassword(data) {
				await this.$modal({
					title: 'Enter new password',
					component: FormResetPassword,
					props: data,
					backdrop: 'static',
				})

				this.$router.push({ name: 'login' })
			},

			async confirmAccount(data) {
				this.confirmed = await this.$api.users.registration.confirm(data)

				const message = this.confirmed ? {
					type: 'success',
					title: 'Account confirmed',
				} : {
					type: 'error',
					title: 'Account confirmation failed',
				}

				this.$flashMessage.show({
					...message,
					time: 5000,
				})

				if (!this.confirmed) this.$router.replace({
					name: 'error',
					params: {
						title: 'Account confirmation failed',
						message: 'If your account is already confirmed you can try logging in.',
					},
				})

				if (data.reset_request_id) {
					this.confirmed = false // Hide confirmation info in the backround
					this.resetPassword({
						id: data.reset_request_id,
						email: data.email,
						type: 'reset',
					})
				}
			},
		},
	}
</script>
