<template>
	<div class="container-fluid mt-4">
		<AdminList v-if="this.$store.state.loggeduser" />
	</div>
</template>

<script>
	import AdminList from '../components/AdminList.vue'
	export default {
		name: 'Home',
		components: {
			AdminList,
  		}
	}
</script>