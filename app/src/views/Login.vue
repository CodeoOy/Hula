<template>
	<div class="container-fluid mt-4">
		<VModal :modalID="'login'" :modalStatic="true">
			<FormLogin v-if="showSignup == false" v-on:hide-modal="hideModal"/>
			<FormRegister v-else/>
			<p>
				<a href="#" v-if="showSignup == false" v-on:click="showSignup = true">Or sign up here.</a>
				<a href="#" v-else v-on:click="showSignup = false">Already a user? Log in here.</a>
			</p>
			<p><a href="/app/forgotpassword">Forgot password?</a></p>
		</VModal>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormRegister from '../forms/FormRegister.vue'
	import FormLogin from '../forms/FormLogin.vue'
	import { Modal } from 'bootstrap'
	export default {
		name: 'Login',
		data() {
			return {
				showSignup: false,
				modal: null,
			}
		},
		components: {
			FormRegister,
			FormLogin,
			VModal,
  		},
		methods: {
			hideModal() {
				this.modal.hide()
			}
		},
		mounted() {
			this.modal = new Modal(document.getElementById('hulaModallogin'))
			if (!this.$store.state.loggeduser) {
				this.modal.show()
			} 
		},
	}
</script>