<template>
	<div class="p-3 rounded-2 content-box bg-dark text-light">
		<VModal :modalTitle="'Enter new password'" :modalID="'Password'" :modalStatic="true">
			<FormResetPassword v-on:form-sent="changePassword" />
		</VModal>
		<h2>Raw registration data:</h2>
		<pre>{{ registrationData }}</pre>
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
			},
			confirmRegistration: function() {  
				fetch(`/api/register/${this.registrationData.id}`, {
					method: 'POST', 
					headers: {"Content-Type": "application/json"}, 
					body: JSON.stringify(this.registrationData)
				})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(() => {    
					this.$flashMessage.show({
						type: 'success',
						title: 'Account confirmed.',
						time: 2000
					});
				})
				.catch((errors) => {
					console.log("Error data: " + errors);
					this.$flashMessage.show({
						type: 'error',
						title: 'Account not confirmed',
						time: 2000
					});
				})      
			},
			setPassword () {
				fetch('/api/updatepassword', {
					method: 'PUT', 
					headers: {"Content-Type": "application/json"}, 
					body: JSON.stringify(this.registrationData)
				})
				.then(response => { 
					return (response.status >= 200 && response.status <= 299) ? response.json() : this.$store.commit('errorHandling', response)
				})
				.then(() => {    
					this.$flashMessage.show({
						type: 'success',
						title: 'Account confirmed.',
						time: 2000
					});
				})
				.catch((errors) => {
					console.log("Error data: " + errors);
					this.$flashMessage.show({
						type: 'error',
						title: 'Account not confirmed',
						time: 2000
					});
				})
			}
		},
		mounted() {
			const route = useRoute()
			console.log(route.query)
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