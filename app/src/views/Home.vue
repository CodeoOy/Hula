<template>
	<div class="container-sm">
		<div class="row">
			<div class="col-sm">
				<Login />
			</div>
			<div class="col-sm">
				<List />
			</div>
			<div class="col-sm">
				Disco chikcen
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