<template>
	<div class="container-fluid mt-4">
		<div class="modal fade" v-bind:class="{ 'show db': !logged, '': logged }">
			<div class="modal-dialog">
				<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
					<div>
						<div v-if="show_signup == false">
							<h2>Log in</h2>
							<Login v-on:loggedin="isLogged"/>
							<a href="#" v-on:click="show_signup = true">Or sign up here.</a>
						</div>
						<div v-else>
							<h2>Sign up</h2>
							<Register />
							<a href="#" v-on:click="show_signup = false">Already a user? Log in here.</a>
						</div>
					</div>
				</div>
			</div>
		</div>
		<List />
	</div>
</template>

<script>
	import Register from '../components/Register.vue'
	import Login from '../components/Login.vue'
	import List from '../components/List.vue'
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
		watch: {
			logged: function() {
				//console.log(this.logged)
			}
		},
		components: {
			'List': List,
			'Register': Register,
			'Login': Login
  		},
		methods: {
			isLogged () {
				//console.log("Emit caught")
				this.$emit('loggedin')
			},
			logoutUser () {
				fetch('http://localhost:8086/api/auth', {method: 'DELETE'})
				this.$emit('loggedin')
			},
		}
	}
</script>