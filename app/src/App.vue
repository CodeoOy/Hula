<template>
	<div>
		<FlashMessage position="right top" />
		<h1>Rust Vue POC</h1>
		<p class="routermessage"><router-view /></p>
	</div>
</template>

<script>
	export default {
		name: 'App',

		data() {
			return {
				variable: false,
				currentpath: this.$router.currentRoute.value.path
			}
		}
	}
</script>
