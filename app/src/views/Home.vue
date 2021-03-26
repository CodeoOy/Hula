<template>
	<div class="container-sm mt-4">
		<div class="row gx-4">
			<div class="col-sm">
				<div class="bg-light p-3">
					<Login />
				</div>
			</div>
			<div class="col-sm">
				<div class="bg-light p-3">
					<List />
				</div>
			</div>
			<div class="col-sm">
				<div class="bg-light p-3">
					Disco chikcen
				</div>
			</div>
		</div>
	</div>
</template>

<script>
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
			'Login': Login
  		}
	}
</script>