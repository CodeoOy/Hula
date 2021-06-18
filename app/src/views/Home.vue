<template>
	<div class="container-fluid mt-4">
		<!-- <VModal v-if="logged == false"> -->
		<VModal>
			<FormLogin v-if="show_signup == false" v-on:hide-modal="hideModal"/>
			<FormRegister v-else/>
			<a href="#" v-if="show_signup == false" v-on:click="show_signup = true">Or sign up here.</a>
			<a href="#" v-else v-on:click="show_signup = false">Already a user? Log in here.</a>
		</VModal>
		<AdminList v-if="this.$store.state.loggeduser" />
	</div>
</template>

<script>
	import VModal from '../components/VModal.vue'
	import FormRegister from '../forms/FormRegister.vue'
	import FormLogin from '../forms/FormLogin.vue'
	import AdminList from '../components/AdminList.vue'
	import { Modal } from 'bootstrap'
	export default {
		name: 'Home',
		data() {
			return {
				show_signup: false,
				modal: null,
			}
		},
		components: {
			FormRegister,
			FormLogin,
			AdminList,
			VModal,
  		},
		methods: {
			hideModal () {
				this.modal.hide()
			}
		},
		mounted() {
			this.modal = new Modal(document.getElementById('hulaModal'))
			if (!this.$store.state.loggeduser) {
				this.modal.show()
			} 
		}
	}
</script>