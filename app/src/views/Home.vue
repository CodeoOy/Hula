<template>
	<div class="container-fluid mt-4">
		<div class="row gx-4">
			<div class="modal fade" v-bind:class="{ 'show db': !logged, '': logged }">
				<div class="modal-dialog">
					<div class="modal-content p-3 rounded-2 content-box bg-dark text-light">
						<div>
							<div v-if="show_signup == false">
								<h2>Log in</h2>
								<Login v-on:loggedin="isLogged"/>
								<a href="#" v-on:click="show_signup = true">Sign up here.</a>
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
		<div class="row">
			<div class="p-3 rounded-2 content-box bg-dark text-light">
				<p>
					<button @click="show = !show" class="btn btn-primary text-white">
						Transition
					</button>
				</p>
				<p>
					<button @click="logoutUser()" class="btn btn-primary text-white">
						Log out
					</button>
				</p>
				<transition name="fadeHeight">
					<p v-if="show">hello</p>
				</transition>
				<p>{{ message }}</p>
			</div>
		</div>
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
				message: "Kylpynalle",
				show_signup: false,
				show: false
			}
		},
		props: {
			logged: false
		},
		components: {
			'List': List,
			'Register': Register,
			'Login': Login
  		},
		methods: {
			isLogged () {
				console.log("Emit caught")
				this.$emit('loggedin')
			},
			logoutUser () {
				fetch('api/auth', {method: 'DELETE'})
				this.$emit('loggedin')
			},
		}
	}
</script>