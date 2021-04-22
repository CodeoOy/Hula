<template>
	<div class="container mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ userid }}</h1>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Profile',
		data() {
			return {
				userid: {}
			}
		},
		methods: {
			getMyData: function () {
				this.userid = JSON.parse(localStorage.getItem('user'))
			}
		},
		mounted() {
			this.getMyData()
		}
	}
</script>