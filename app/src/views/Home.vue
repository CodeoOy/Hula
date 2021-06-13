<template>
	<div class="container-fluid mt-4">
		<VModal v-on:loggedin="hideModal">
			{{ logged }}
			<FormLogin v-if="show_signup == false" v-on:checklogin="isLogged"/>
			<FormRegister v-else/>
			<a href="#" v-if="show_signup == false" v-on:click="show_signup = true">Or sign up here.</a>
			<a href="#" v-else v-on:click="show_signup = false">Already a user? Log in here.</a>
		</VModal>
		<AdminList v-if="logged" />
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
				modal: null
			}
		},
		props: {
			logged: Boolean
		},
		components: {
			FormRegister,
			FormLogin,
			AdminList,
			VModal,
  		},
		methods: {
			isLogged () {
				this.$emit('checklogin')
			},
			hideModal () {
				console.log("hidemodal")
				//this.modal.hide()
			}
		},
		mounted() {
			this.modal = new Modal(document.getElementById('hulaModal'))
			this.modal.show()
		}
	}
</script>