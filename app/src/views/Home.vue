<template>
	<div class="container-sm mt-4">
		<div class="row gx-4">
			<div class="col-md">
				<div v-if="show_signup == false" class="p-3 rounded-2 tropical-box">
					<h2>Log in</h2>
					<Login />
					<a href="#" v-on:click="show_signup = false">Sign up here.</a>
				</div>
				<div v-else class="p-3 rounded-2 tropical-box">
					<h2>Sign up</h2>
					<Register />
					<a href="#" v-on:click="show_signup = true">Already a user? Log in here.</a>
				</div>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 tropical-box">
					<List />
				</div>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 tropical-box">
					{{ message }}
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
			}
		},
		components: {
			'List': List,
			'Register': Register,
			'Login': Login
  		}
	}
</script>