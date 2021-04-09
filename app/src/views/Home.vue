<template>
	<div class="container-sm mt-4">
		<div class="row gx-4">
			<div class="col-md">
				<div v-if="show_signup == true" class="p-3 rounded-2 tropical-box">
					<Login />
					<a href="#" v-on:click="show_signup = false">Sign up here.</a>
				</div>
				<div v-else class="p-3 rounded-2 tropical-box">
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
					Kylpynalle
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
				message: "Treeflooooof",
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