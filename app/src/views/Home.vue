<template>
	<div class="container-fluid mt-4">
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
		components: {
			FormRegister,
			FormLogin,
			AdminList,
			VModal,
  		}
	}
</script>