<template>
	<div class="container mt-5">
		<div class="card shadow" :class='$colorScheme.card'>
			<div class='card-header'>
				<h1 class='h3 mb-0'>{{ title }}</h1>
			</div>
			<div class='card-body'>
				<p class='mb-0'>{{ message }}</p>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Error',

		props: {
			title: {
				type: String,
				default: 'Page not found',
			},
			message: {
				type: String,
				default: 'Try the navigation',
			},
		},
	}
</script>