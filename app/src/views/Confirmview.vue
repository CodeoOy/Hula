<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<Confirm />
			</div>
		</div>
	</div>
</template>

<script>
	import Confirm from '../components/Confirm.vue'
	export default {
		name: 'Confirmview',
		components: {
			Confirm: Confirm
		}
	}
</script>