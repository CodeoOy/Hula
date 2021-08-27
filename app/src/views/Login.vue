<template>
	<div class="container-fluid mt-4">
		<VModal ref='modal' :modalTitle='title' modalBackdrop="static">
			<FormLogin v-if="showSignup == false" />
			<FormRegister v-else/>
			<p>
				<a href="#" v-if="showSignup == false" v-on:click.prevent="showSignup = true">Or sign up here.</a>
				<a href="#" v-else v-on:click.prevent="showSignup = false">Already a user? Log in here.</a>
			</p>
			<p><a href="/app/forgotpassword">Forgot password?</a></p>
		</VModal>
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormRegister from '../forms/FormRegister.vue'
	import FormLogin from '../forms/FormLogin.vue'

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
		computed: {
			title() { return this.showSignup ? 'Sign up' : 'Log in' },
		},
		mounted() {
			if (!this.$store.state.loggeduser) {
				this.$refs.modal.show()
			} 
		},
	}
</script>