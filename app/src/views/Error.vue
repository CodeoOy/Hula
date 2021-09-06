<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<h1 class="h1">{{ title }}</h1>
					<p>{{ message }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'PageError',
		computed: {
			title () {
				if (this.$store.state.errorObject) {
					return this.$store.state.errorObject.title
				} else {
					return "Page not found."
				}
			},
			message() {
				if (this.$store.state.errorObject) {
					return this.$store.state.errorObject.title
				} else {
					return "Try the navigation."
				}
			}
		}	
	}
</script>