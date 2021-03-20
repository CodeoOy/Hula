<template>
	<div>
		<h1>Rust Vue POC</h1>
		<p class="routermessage"><router-view /></p>
	</div>
</template>

<script>
	export default {
		name: 'App',

		data() {
			return {
				variable: false
			}
		}
	}
</script>
