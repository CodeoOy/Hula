<template>
	<div class="container">
		<div class="row mt-4">
			<div class="col">
				<div class="tropical-box rounded-2 p-3">
					<a href="#" v-on:click="getRustData">Get om noms</a>
					<p v-if="message.length">{{ message }}</p>
				</div>
			</div>
		</div>
	</div>
</template>

<script>
	export default {
		name: 'Cheeseboi',
		data() {
			return {
				message: JSON
			}
		},
		methods: {
			getRustData: function() {
				console.log("getRustData fired.");  
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