<template>
	<div class="container-sm mt-4">
		<div class="row gx-4">
			<div class="col-md">
				<transition name="fadeHeight" mode="out-in">
					<div class="p-3 rounded-2 content-box bg-dark text-light">
						<div v-if="just_logged == false">
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
						<div v-else>
							<p>Looks like you're logged in</p>
							<p><a href="#" v-on:click="logoutUser">Log out test link</a></p>
						</div>
					</div>
				</transition>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<List />
				</div>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<p>
						<button @click="show = !show" class="btn btn-primary text-white">
							Transition
						</button>
					</p>
					<transition name="fadeHeight">
						<p v-if="show">hello</p>
					</transition>
					<p>{{ message }}</p>
				</div>
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
				just_logged: false,
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
				this.just_logged = true;
			},
			logoutUser () {
				fetch('api/auth', {method: 'DELETE'})
				this.just_logged = false;
			},
		}
	}
</script>