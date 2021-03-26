<template>
	<header class="p-3 bg-secondary">
	<div class="container">
		<div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
		<a href="/" class="d-flex align-items-center mb-2 mb-lg-0 text-decoration-none">
			<svg class="bi me-2" width="40" height="32"><use xlink:href="#bootstrap"></use></svg>
		</a>

		<ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
			<li><a href="#" class="nav-link px-2 font-heading font-heading--large">Tropical ERP</a></li>
		</ul>

		<div class="text-end">
			<button type="button" class="btn btn-outline-dark me-2">Login</button>
			<button type="button" class="btn btn-warning">Sign-up</button>
		</div>
		</div>
	</div>
	</header>
</template>

<script>
export default {
	name: 'Header'
};
</script>