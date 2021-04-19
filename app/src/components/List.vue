<template>
	<div>
		<p><a href="#" v-on:click="getAllUsers">Get all users</a></p>
		<p>{{ user.uid }}</p>
		<ul>
			<li v-for="user in users" :key="user.email">
				<a href="#" v-on:click="getUserData(user.uid)">{{user.email}}</a>
			</li>
		</ul>
	</div>
</template>

<script>
	export default {
		name: 'List',
		data() {
			return {
				message: "Get Tuomas",
				user: {},
				users: {}
			}
		},
		methods: {
			getUserData: function(uid) {
				console.log("getUserData fired.");   
				fetch('api/query', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": uid})
				})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.user = response;
					this.$flashMessage.show({
						type: 'success',
						title: 'Successfully fetched Tuomas',
						time: 1000
					});
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
					//this.$router.push('/create'); 
				})
			},
			getAllUsers: function() {
				fetch('api/users', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.users = response;
					this.$flashMessage.show({
						type: 'success',
						title: 'Successfully fetched users',
						time: 1000
					});
				})    
				.catch((errors) => {    
					console.log("Could not get data");
					console.log(errors);
					//this.$router.push('/create'); 
				})
			}
		}
	}
</script>