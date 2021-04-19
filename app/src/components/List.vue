<template>
	<div>
		<a href="#" v-on:click="getUserData">{{ message }}</a>
		<p>{{ user.email }}</p>
	</div>
</template>

<script>
	export default {
		name: 'List',
		data() {
			return {
				message: "Get Tuomas",
				user: {}
			}
		},
		methods: {
			getUserData: function() {
				console.log("getUserData fired.");   
				fetch('api/query', {
					method: 'POST',
					headers: {"Content-Type": "application/json"},
					body: JSON.stringify({"uid": "09c66d46-1dc3-405c-8511-10485fa30b3c"})
				})
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