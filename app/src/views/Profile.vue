<template>
	<div class="container mt-4">
		<div class="row gx-4">
			<div class="col-md-4">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h1>{{ user.firstname }} {{ user.lastname }}</h1>
					<a href="#">Edit</a>
					<img src="" alt="">
				</div>
			</div>
			<div class="col-md-8">
				<div class="p-3 mb-4 rounded-2 content-box bg-dark text-light">
					<h2>Professional profile</h2>
					<a href="#">Edit</a>
					<p>Available: {{ user.available }}</p>
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
				userid: {},
				user: {}
			}
		},
		methods: {
			getMyData: function () {
				this.userid = JSON.parse(localStorage.getItem('user'))

			},
			getUserData: function(uid) { 
				fetch('http://localhost:8086/api/user', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": uid})
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.user = response;
				})    
			},
		},
		mounted() {
			this.getMyData()
			this.getUserData(this.userid)
		}
	}
</script>