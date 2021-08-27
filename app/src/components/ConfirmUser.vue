<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Enter new password'" :modalID="'Password'" :modalStatic="true">
			<FormResetPassword v-on:form-sent="changePassword" />
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
	import { Modal } from 'bootstrap'
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
			hideModalUpdate() {
				let modal = Modal.getInstance(document.querySelector('#hulaModalPassword'))
				modal.hide()
			},
			changePassword (value) {
				this.registrationData.password = value
				this.noPassword = false
				this.hideModalUpdate()
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
					this.modal = new Modal(document.getElementById('hulaModalPassword'))
					this.modal.show()
			} else {
				this.confirmRegistration()
			}
		}
	}
</script>