<template>
	<div>
		<a href="#" v-on:click="getRustData">Get om noms</a>
		{{ message }}
	</div>
</template>

<script>
	export default {
		name: 'Cheeseboi',
		data() {
			return {
				message: ""
			}
		},
		methods: {
			getUserData: function() {
				console.log("getUserData fired.");   
				fetch('/omnom', {method: 'GET'})
				.then((response) => response.json())
				.then(response => { 
					console.log(response);
					this.message = response;
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