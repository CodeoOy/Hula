<template>
	<div>
		<a href="#" v-on:click="getUserData">{{ message }}</a>
		<p>{{ user.name }}</p>
	</div>
</template>

<script>
	export default {
		name: 'List',
		data() {
			return {
				message: "Click here",
				user: {}
			}
		},
		methods: {
			getUserData: function() {
				console.log("getUserData fired.");   
				fetch('/user/cc7cd70e-bbe0-4a63-8bec-9c63f60a91c4', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.user = response;
					this.$flashMessage.show({
						type: 'success',
						title: 'Successfully fetched data',
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