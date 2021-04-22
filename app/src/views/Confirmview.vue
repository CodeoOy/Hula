<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="content-box rounded-2 p-3">
					<Confirm />
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	import Confirm from '../components/Confirm.vue'
	export default {
		name: 'Confirmview',
		data() {
			return {
				message: JSON,
				registration_data: String,
			}
		},
		components: {
			Confirm: Confirm
		}
	}
</script>