<template>
	<footer class="navbar navbar-dark mt-4 mt-md-5" :class='$colorScheme.navbar'>
	<div class="container-fluid">
		<div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
			<router-link to="/app/Gdpr">Our terms of Service</router-link>
		</div>
	</div>
	</footer>
</template>

<script>
	export default {
		name: 'TheFooter',
	}
</script>