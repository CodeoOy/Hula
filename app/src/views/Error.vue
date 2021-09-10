<template>
	<div class="container">
		<div class="card shadow" :class='$colorScheme.card'>
			<div class='card-body'>
				<h1 class="h1">{{ title }}</h1>
				<p>{{ message }}</p>
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