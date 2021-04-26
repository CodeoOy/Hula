<template>
	<header class="bg-dark">
	<div class="container-fluid">
		<div class="d-flex flex-wrap align-items-center justify-content-center justify-content-lg-start">
		<a href="/" class="d-flex align-items-center mb-2 mb-lg-0 text-decoration-none">
			<!--<svg class="bi me-2" width="40" height="32"><use xlink:href="#bootstrap"></use></svg>-->
			<span class="font-heading font-heading--large">Hula</span>
		</a>

		<ul class="nav col-12 col-lg-auto me-lg-auto mb-2 justify-content-center mb-md-0">
			<li><a href="/app/dashboard" class="nav-link px-2">Dashboard</a></li>
		</ul>

		<div class="dropdown text-end">
			<a href="#" class="d-block" id="usermenu" data-bs-toggle="dropdown" aria-expanded="false">
				<svg class="icon" width="16" height="16" viewbox="0 0 16 16" fill="currentColor">
					<path d="M11 6a3 3 0 11-6 0 3 3 0 016 0z"></path><path fill-rule="evenodd" d="M0 8a8 8 0 1116 0A8 8 0 010 8zm8-7a7 7 0 00-5.468 11.37C3.242 11.226 4.805 10 8 10s4.757 1.225 5.468 2.37A7 7 0 008 1z"></path>
				</svg>
			</a>
			<ul class="dropdown-menu" aria-labelledby="usermenu">
				<li>
					<a href="#" v-on:click="logOut()" class="dropdown-item">Log out</a>
				</li>
				<li>
					<a href="http://localhost:8086/app/profile" class="dropdown-item">Profile</a>
				</li>
			</ul>
		</div>
		</div>
	</div>
	</header>
</template>

<script>
export default {
	name: 'Header',
	methods: {
		logOut () {
			fetch('http://localhost:8086/api/auth', {method: 'DELETE'})
			.then((response) => {
				this.message = response;
				this.$flashMessage.show({
					type: 'success',
					title: 'Successfully logged out',
					time: 1000
				});
			})
			this.$emit('loggedout')
		}
	}
};
</script>