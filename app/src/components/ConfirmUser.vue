<template>
	<div class="p-3 rounded shadow bg-dark text-light">
		<VModal ref='modal' :modalTitle="'Enter new password'" modalBackdrop="static">
			<FormResetPassword v-on:success="changePassword" />
		</VModal>
		<div v-if="confirmed">
			<h2 class="h2">Account confirmed.</h2>
			<p>registration data:</p>
			<pre>{{ registrationData }}</pre>
		</div>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormResetPassword from '../forms/FormResetPassword.vue'
	import { useRoute } from 'vue-router'

	export default {
		name: 'ConfirmUser',
		data: function() {
			return {
				registrationData: {},
				noPassword: false,
				modal: null,
				confirmed: false,
			};
		},
		components: {
			VModal,
			FormResetPassword,
		},
		methods: {
			changePassword (value) {
				this.registrationData.password = value
				this.noPassword = false
				if (this.registrationData.type == 'reset') {
					this.setPassword()
				} else {
					this.confirmRegistration()
				}
				this.$router.push('/')
			},
			async confirmRegistration() {  
				const success = await this.$api.users.registration.confirm(this.registrationData)
				
				const message = success ? {
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

				if (success) this.confirmed = true
			},
			async setPassword() {
				const success = await this.$api.users.password.save(this.registrationData)
				
				const message = success ? {
					type: 'success',
					title: 'Password changed',
				} : {
					type: 'error',
					title: 'Updating password failed',
				}

				this.$flashMessage.show({
					...message,
					time: 5000,
				})
				
				if (success) this.confirmed = true
			}
		},
		mounted() {
			const route = useRoute()
			this.registrationData = route.query
			if (this.registrationData.password == '') {
				this.noPassword = true
				this.$refs.modal.show()
			} else {
				this.confirmRegistration()
			}
		}
	}
</script>