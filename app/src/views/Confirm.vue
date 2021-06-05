<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<ConfirmUser />
			</div>
		</div>
	</div>
</template>

<script>
	import ConfirmUser from '../components/ConfirmUser.vue'
	export default {
		name: 'ConfirmUserview',
		components: {
			ConfirmUser: ConfirmUser
		}
	}
</script>