<template>
	<VModal ref='modal' :modalTitle='title' modalBackdrop="static">
		<FormLogin v-if="showSignup == false" @success='navigate' />
		<FormRegister v-else/>
		<div class='mt-3'>
			<button class='btn btn-unstyled' v-if="showSignup == false" v-on:click="showSignup = true">Or sign up here.</button>
			<button class='btn btn-unstyled' v-else v-on:click="showSignup = false">Already a user? Log in here.</button>
		</div>
		<div class='mt-3'>
			<router-link :to='{ name: "forgot-password" }'>Forgot password?</router-link>
		</div>
	</VModal>
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
		methods: {
			navigate() {
				this.$router.replace(this.$route.query.redirect || { name: 'home' })
			},
		},
	}
</script>