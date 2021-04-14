<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="tropical-box rounded-2 p-3">
					<p v-if="message.length">{{ message }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Dashboard',
		data() {
			return {
				message: "Dashboard"
			}
		}
	}
</script>