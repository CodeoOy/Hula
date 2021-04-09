<template>
	<div class="container-sm mt-4">
		<div class="row gx-4">
			<div class="col-md">
				<div class="p-3 rounded-2 tropical-box">
					<Register />
				</div>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 tropical-box">
					<List />
				</div>
			</div>
			<div class="col-md">
				<div class="p-3 rounded-2 tropical-box">
					<Login />
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
				message: "Treeflooooof"
			}
		},
		components: {
			'List': List,
			'Register': Register,
			'Login': Login
  		}
	}
</script>