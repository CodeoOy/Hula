<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="p-3 rounded-2 content-box bg-dark text-light">
					<h1>{{ title }}</h1>
					<p>{{ message }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'PageError',
		props: {
			title: {
				type: String,
				default: "Page not found!"
			},
			message: {
				type: String,
				default: "Try the navigation."
			},
		}
	}
</script>