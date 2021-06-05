<template>
	<div class="container-fluid mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': !logged, '': logged }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<div v-if="show_signup == false">
							<h2>Log in</h2>
							<FormLogin v-on:checklogin="isLogged"/>
							<a href="#" v-on:click="show_signup = true">Or sign up here.</a>
						</div>
						<div v-else>
							<h2>Sign up</h2>
							<FormRegister />
							<a href="#" v-on:click="show_signup = false">Already a user? Log in here.</a>
						</div>
					</div>
				</div>
			</div>
		</div>
		<AdminList v-if="logged" />
	</div>
</template>

<script>
	import FormRegister from '../forms/FormRegister.vue'
	import FormLogin from '../forms/FormLogin.vue'
	import AdminList from '../components/AdminList.vue'
	export default {
		name: 'Home',
		data() {
			return {
				show_signup: false
			}
		},
		props: {
			logged: false
		},
		components: {
			FormRegister,
			FormLogin,
			AdminList
  		},
		methods: {
			isLogged () {
				this.$emit('checklogin')
			}
		}
	}
</script>